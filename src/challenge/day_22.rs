use num_enum::{FromPrimitive, IntoPrimitive};
use std::iter::Peekable;
use std::str::Bytes;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(solve(input, false))
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(solve(input, true))
}

fn solve(input: &[&str], cube: bool) -> usize {
    let map = Map::new(&input[..input.len() - 2]);
    let mut position = map.start();
    let mut direction = Direction::Right;

    for command in Commands::new(input[input.len() - 1]) {
        let move_amount = match command {
            Command::Move(move_amount) => move_amount,
            Command::TurnLeft => {
                direction = direction.turn_left();
                continue;
            }
            Command::TurnRight => {
                direction = direction.turn_right();
                continue;
            }
        };

        for _ in 0..move_amount {
            let next_position = position.move_one(direction);

            match map.get(next_position) {
                Tile::Air => {
                    match map.find_opposite_tile(position, direction, cube) {
                        None => break, // found a wall
                        Some((next_position, next_direction)) => {
                            position = next_position;
                            direction = next_direction;
                        }
                    }
                }
                Tile::Wall => break,
                Tile::Ground => position = next_position,
            }
        }
    }

    1000 * position.y as usize + 4 * position.x as usize + u8::from(direction) as usize
}

#[derive(IntoPrimitive, FromPrimitive, Copy, Clone)]
#[repr(u8)]
enum Direction {
    #[default]
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn turn_left(self) -> Self {
        self.turn(3)
    }

    fn turn_right(self) -> Self {
        self.turn(1)
    }

    fn turn(self, count: u8) -> Self {
        ((u8::from(self) + count) % 4).into()
    }
}

#[derive(Copy, Clone)]
struct Position {
    x: u8,
    y: u8,
}

impl Position {
    fn new(x: u8, y: u8) -> Self {
        Position { x, y }
    }

    fn move_one(self, direction: Direction) -> Self {
        match direction {
            Direction::Right => Position::new(self.x + 1, self.y),
            Direction::Down => Position::new(self.x, self.y + 1),
            Direction::Left => Position::new(self.x - 1, self.y),
            Direction::Up => Position::new(self.x, self.y - 1),
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Tile {
    Air,
    Wall,
    Ground,
}

struct Map {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl Map {
    fn new(input: &[&str]) -> Map {
        let height = input.len();
        let width = input.iter().map(|line| line.len()).max().unwrap();

        let mut tiles = vec![Tile::Air; width * height];

        for (y, row) in input.iter().enumerate() {
            for (x, cell) in row.bytes().enumerate() {
                let tile = match cell {
                    b'#' => Tile::Wall,
                    b'.' => Tile::Ground,
                    _ => continue,
                };

                tiles[x + y * width] = tile;
            }
        }

        Map {
            width,
            height,
            tiles,
        }
    }

    fn start(&self) -> Position {
        let index = self.tiles[..self.width]
            .iter()
            .position(|tile| *tile == Tile::Ground)
            .unwrap();

        Position::new(index as u8 + 1, 1)
    }

    fn get(&self, position: Position) -> Tile {
        let x = position.x as usize;
        let y = position.y as usize;

        if x == 0 || x > self.width || y == 0 || y > self.height {
            Tile::Air
        } else {
            self.tiles[(x - 1) + (y - 1) * self.width]
        }
    }

    // Returns None if the tile on the opposite side of the map is a wall
    fn find_opposite_tile(
        &self,
        position: Position,
        direction: Direction,
        cube: bool,
    ) -> Option<(Position, Direction)> {
        if cube {
            self.find_opposite_tile_cube(position, direction)
        } else {
            self.find_opposite_tile_flat(position, direction)
        }
    }

    // Finds the first non-Air tile wrapping around the map
    fn find_opposite_tile_flat(
        &self,
        position: Position,
        direction: Direction,
    ) -> Option<(Position, Direction)> {
        let (x, y, tile) = match direction {
            Direction::Right => {
                let y = position.y as usize - 1;
                let (x, tile) = (0..self.width)
                    .map(|x| (x, self.tiles[x + y * self.width]))
                    .find(|(_, tile)| *tile != Tile::Air)
                    .unwrap();
                (x, y, tile)
            }
            Direction::Down => {
                let x = position.x as usize - 1;
                let (y, tile) = (0..self.height)
                    .map(|y| (y, self.tiles[x + y * self.width]))
                    .find(|(_, tile)| *tile != Tile::Air)
                    .unwrap();
                (x, y, tile)
            }
            Direction::Left => {
                let y = position.y as usize - 1;
                let (x, tile) = (0..self.width)
                    .rev()
                    .map(|x| (x, self.tiles[x + y * self.width]))
                    .find(|(_, tile)| *tile != Tile::Air)
                    .unwrap();
                (x, y, tile)
            }
            Direction::Up => {
                let x = position.x as usize - 1;
                let (y, tile) = (0..self.height)
                    .rev()
                    .map(|y| (y, self.tiles[x + y * self.width]))
                    .find(|(_, tile)| *tile != Tile::Air)
                    .unwrap();
                (x, y, tile)
            }
        };

        if tile == Tile::Ground {
            Some((Position::new(x as u8 + 1, y as u8 + 1), direction))
        } else {
            None
        }
    }

    // Finds the first position over the edge (along with the new direction relative to the map)
    // It doesn't feel like this is supposed to support generic layout, but only the provided one
    fn find_opposite_tile_cube(
        &self,
        position: Position,
        direction: Direction,
    ) -> Option<(Position, Direction)> {
        let (x, y, direction) = match direction {
            Direction::Right => match position.y {
                1..=50 => (99, 150 - position.y as usize, Direction::Left),
                51..=100 => ((position.y as usize - 1) + 50, 49, Direction::Up),
                101..=150 => (149, 150 - position.y as usize, Direction::Left),
                151..=200 => ((position.y as usize - 1) - 100, 149, Direction::Up),
                _ => unreachable!(),
            },
            Direction::Down => match position.x {
                1..=50 => ((position.x as usize - 1) + 100, 0, Direction::Down),
                51..=100 => (49, (position.x as usize - 1) + 100, Direction::Left),
                101..=150 => (99, (position.x as usize - 1) - 50, Direction::Left),
                _ => unreachable!(),
            },
            Direction::Left => match position.y {
                1..=50 => (0, 150 - position.y as usize, Direction::Right),
                51..=100 => ((position.y as usize - 1) - 50, 100, Direction::Down),
                101..=150 => (50, 150 - position.y as usize, Direction::Right),
                151..=200 => ((position.y as usize - 1) - 100, 0, Direction::Down),
                _ => unreachable!(),
            },
            Direction::Up => match position.x {
                1..=50 => (50, (position.x as usize - 1) + 50, Direction::Right),
                51..=100 => (0, (position.x as usize - 1) + 100, Direction::Right),
                101..=150 => ((position.x as usize - 1) - 100, 199, Direction::Up),
                _ => unreachable!(),
            },
        };

        if self.tiles[x + y * self.width] == Tile::Ground {
            Some((Position::new(x as u8 + 1, y as u8 + 1), direction))
        } else {
            None
        }
    }
}

#[derive(Eq, PartialEq)]
enum Command {
    Move(usize),
    TurnLeft,
    TurnRight,
}

struct Commands<'a>(Peekable<Bytes<'a>>);

impl<'a> Commands<'a> {
    fn new(input: &'a str) -> Self {
        Commands(input.bytes().peekable())
    }
}

impl Iterator for Commands<'_> {
    type Item = Command;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.0.next()?;

        let mut amount = match next {
            b'L' => return Some(Command::TurnLeft),
            b'R' => return Some(Command::TurnRight),
            _ => (next - b'0') as usize,
        };

        while matches!(self.0.peek(), Some(next) if next.is_ascii_digit()) {
            amount = amount * 10 + (self.0.next().unwrap() - b'0') as usize;
        }

        Some(Command::Move(amount))
    }
}
