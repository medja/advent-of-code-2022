use anyhow::{bail, Context};
use std::ops::{Index, IndexMut};

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let map = Map::new(input);
    let scores = find_path(&map, Direction::Up);

    match scores[map.end] {
        u16::MAX => bail!("Could not find path"),
        score => Ok(score as usize),
    }
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let map = Map::new(input);
    let scores = find_path(&map, Direction::Down);

    let score = map
        .grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, height)| **height == b'a')
                .map(move |(x, _)| (x, y))
        })
        .map(|(x, y)| scores[y][x])
        .min()
        .context("Could not find path")?;

    Ok(score as usize)
}

fn find_path(map: &Map, direction: Direction) -> Vec<Vec<u16>> {
    let mut queue = Vec::new();
    let mut neighbors = Vec::with_capacity(4);
    let mut scores = vec![vec![u16::MAX; map.width as usize]; map.height as usize];

    let start = match direction {
        Direction::Up => map.start,
        Direction::Down => map.end,
    };

    queue.push(start);
    scores[start] = 0;

    while let Some(position) = queue.pop() {
        let score = scores[position] + 1;
        map.fill_neighbors(position, direction, &mut neighbors);

        for &neighbor in &neighbors {
            if score >= scores[neighbor] {
                continue;
            }

            scores[neighbor] = score;

            let is_end = match direction {
                Direction::Up => neighbor == map.end,
                Direction::Down => map.grid[neighbor] == b'a',
            };

            if !is_end {
                queue.push(neighbor);
            }
        }
    }

    scores
}

#[derive(Eq, PartialEq, Copy, Clone)]
struct Position {
    x: u16,
    y: u16,
}

impl Position {
    const MIN: Position = Position::new(0, 0);

    const fn new(x: u16, y: u16) -> Self {
        Position { x, y }
    }
}

impl<T> Index<Position> for Vec<Vec<T>> {
    type Output = T;

    fn index(&self, index: Position) -> &Self::Output {
        &self[index.y as usize][index.x as usize]
    }
}

impl<T> IndexMut<Position> for Vec<Vec<T>> {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        &mut self[index.y as usize][index.x as usize]
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
}

struct Map {
    width: u16,
    height: u16,
    start: Position,
    end: Position,
    grid: Vec<Vec<u8>>,
}

impl Map {
    fn new(input: &[&str]) -> Self {
        let width = input[0].len() as u16;
        let height = input.len() as u16;

        let mut start = Position::MIN;
        let mut end = Position::MIN;
        let mut grid = vec![Vec::with_capacity(width as usize); height as usize];

        for (i, line) in input.iter().enumerate() {
            let row = line.as_bytes();
            grid[i].extend_from_slice(row);

            if let Some(offset) = row.iter().position(|x| *x == b'S') {
                start = Position::new(offset as u16, i as u16);
                grid[i][offset] = b'a';
            }

            if let Some(offset) = row.iter().position(|x| *x == b'E') {
                end = Position::new(offset as u16, i as u16);
                grid[i][offset] = b'z';
            }
        }

        Map {
            width,
            height,
            start,
            end,
            grid,
        }
    }

    fn fill_neighbors(&self, position: Position, direction: Direction, buffer: &mut Vec<Position>) {
        buffer.clear();

        let right = position.x + 1;
        let down = position.y + 1;

        let (min, max) = match direction {
            Direction::Up => (b'a', self.grid[position] + 1),
            Direction::Down => (self.grid[position] - 1, b'z'),
        };

        if right < self.width {
            let position = Position::new(right, position.y);
            let height = self.grid[position];

            if height >= min && height <= max {
                buffer.push(position);
            }
        }

        if down < self.height {
            let position = Position::new(position.x, down);
            let height = self.grid[position];

            if height >= min && height <= max {
                buffer.push(position);
            }
        }

        if position.y > 0 {
            let position = Position::new(position.x, position.y - 1);
            let height = self.grid[position];

            if height >= min && height <= max {
                buffer.push(position);
            }
        }

        if position.x > 0 {
            let position = Position::new(position.x - 1, position.y);
            let height = self.grid[position];

            if height >= min && height <= max {
                buffer.push(position);
            }
        }
    }
}
