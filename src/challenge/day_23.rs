use std::ops::Add;

const PADDING: usize = 10;

const DIRECTIONS: [(i8, i8); 8] = [
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

const MOVES: [[usize; 3]; 4] = [
    [7, 0, 1], // north
    [3, 4, 5], // south
    [5, 6, 7], // west
    [1, 2, 3], // east
];

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut moves = MOVES;
    let mut map = Map::new(input);

    for _ in 0..10 {
        map.simulate(moves);
        moves.rotate_left(1);
    }

    Ok(map.score())
}

#[derive(Copy, Clone)]
struct Position {
    x: u8,
    y: u8,
}

impl Position {
    fn index(&self, width: usize) -> usize {
        self.x as usize + (self.y as usize) * width
    }
}

impl Add<(i8, i8)> for Position {
    type Output = Position;

    fn add(self, rhs: (i8, i8)) -> Self::Output {
        Position {
            x: self.x.wrapping_add_signed(rhs.0),
            y: self.y.wrapping_add_signed(rhs.1),
        }
    }
}

struct Elf {
    current_position: Position,
    next_position: Position,
}

struct Map {
    width: usize,
    elves: Vec<Elf>,
    grid: Vec<u8>,
}

impl Map {
    fn new(input: &[&str]) -> Self {
        let width = input[0].len() + PADDING * 2;
        let height = input.len() + PADDING * 2;

        let mut elves = Vec::new();
        let mut grid = vec![0; width * height];

        for (y, row) in input.iter().enumerate() {
            for (x, cell) in row.bytes().enumerate() {
                if cell == b'#' {
                    let x = x + PADDING;
                    let y = y + PADDING;

                    let position = Position {
                        x: x as u8,
                        y: y as u8,
                    };

                    let elf = Elf {
                        current_position: position,
                        next_position: position,
                    };

                    elves.push(elf);
                    grid[x + y * width] = u8::MAX;
                }
            }
        }

        Map {
            width,
            elves,
            grid,
        }
    }

    fn simulate(&mut self, moves: [[usize; 3]; 4]) {
        for elf in &mut self.elves {
            let neighbors = DIRECTIONS.map(|direction| elf.current_position + direction);

            if neighbors
                .iter()
                .all(|position| self.grid[position.index(self.width)] != u8::MAX)
            {
                continue;
            }

            let next_position = moves
                .iter()
                .find(|directions| {
                    directions.iter().all(|direction| {
                        self.grid[neighbors[*direction].index(self.width)] != u8::MAX
                    })
                })
                .map(|directions| neighbors[directions[1]]);

            if let Some(next_position) = next_position {
                self.grid[next_position.index(self.width)] += 1;
                elf.next_position = next_position;
            }
        }

        for elf in &mut self.elves {
            let index = elf.next_position.index(self.width);
            let count = self.grid[index];

            if count == 1 {
                self.grid[elf.current_position.index(self.width)] = 0;
                self.grid[index] = u8::MAX;
                elf.current_position = elf.next_position;
            } else if count > 1 && count != u8::MAX {
                self.grid[index] = 0;
            }
        }
    }

    fn score(&self) -> usize {
        let mut min_x = u8::MAX;
        let mut max_x = 0;
        let mut min_y = u8::MAX;
        let mut max_y = 0;

        for elf in &self.elves {
            min_x = min_x.min(elf.current_position.x);
            max_x = max_x.max(elf.current_position.x);
            min_y = min_y.min(elf.current_position.y);
            max_y = max_y.max(elf.current_position.y);
        }

        (max_x - min_x + 1) as usize * (max_y - min_y + 1) as usize - self.elves.len()
    }
}
