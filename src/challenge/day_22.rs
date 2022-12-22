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
            match map.find_next_position(position, direction, cube) {
                Some((next_position, next_direction)) => {
                    position = next_position;
                    direction = next_direction;
                }
                None => break,
            }
        }
    }

    let x = position.x as usize + 1;
    let y = position.y as usize + 1;
    let direction = u8::from(direction) as usize;

    1000 * y + 4 * x + direction
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

        Position::new(index as u8, 0)
    }

    fn is_on_edge(&self, position: Position, direction: Direction) -> bool {
        match direction {
            Direction::Right => position.x as usize + 1 == self.width,
            Direction::Down => position.y as usize + 1 == self.height,
            Direction::Left => position.x == 0,
            Direction::Up => position.y == 0,
        }
    }

    fn find_next_position(
        &self,
        position: Position,
        direction: Direction,
        cube: bool,
    ) -> Option<(Position, Direction)> {
        let (sx, sy) = (position.x as usize, position.y as usize);

        let (dx, dy) = match direction {
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (self.width - 1, 0),
            Direction::Up => (0, self.height - 1),
        };

        let (mut x, mut y) = ((sx + dx) % self.width, (sy + dy) % self.height);
        let mut tile = self.tiles[x + y * self.width];

        if cube && (tile == Tile::Air || self.is_on_edge(position, direction)) {
            return self.find_next_cube_position(position, direction);
        }

        while tile == Tile::Air {
            x = (x + dx) % self.width;
            y = (y + dy) % self.height;
            tile = self.tiles[x + y * self.width];
        }

        if tile == Tile::Ground {
            Some((Position::new(x as u8, y as u8), direction))
        } else {
            None
        }
    }

    fn find_next_cube_position(
        &self,
        position: Position,
        direction: Direction,
    ) -> Option<(Position, Direction)> {
        let sx = position.x as usize;
        let sy = position.y as usize;

        let (x, y, direction) = match direction {
            Direction::Right => match sy {
                0..=49 => (99, 149 - sy, Direction::Left),
                50..=99 => (sy + 50, 49, Direction::Up),
                100..=149 => (149, 149 - sy, Direction::Left),
                150..=199 => (sy - 100, 149, Direction::Up),
                _ => unreachable!(),
            },
            Direction::Down => match sx {
                0..=49 => (sx + 100, 0, Direction::Down),
                50..=99 => (49, sx + 100, Direction::Left),
                100..=149 => (99, sx - 50, Direction::Left),
                _ => unreachable!(),
            },
            Direction::Left => match sy {
                0..=49 => (0, 149 - sy, Direction::Right),
                50..=99 => (sy - 50, 100, Direction::Down),
                100..=149 => (50, 149 - sy, Direction::Right),
                150..=199 => (sy - 100, 0, Direction::Down),
                _ => unreachable!(),
            },
            Direction::Up => match sx {
                0..=49 => (50, sx + 50, Direction::Right),
                50..=99 => (0, sx + 100, Direction::Right),
                100..=149 => (sx - 100, 199, Direction::Up),
                _ => unreachable!(),
            },
        };

        if self.tiles[x + y * self.width] == Tile::Ground {
            Some((Position::new(x as u8, y as u8), direction))
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
