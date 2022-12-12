use anyhow::Context;
use std::ops::{Index, IndexMut};

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    find_path(Map::new(input)).context("Could not find path")
}

fn find_path(map: Map) -> Option<usize> {
    let mut neighbors = Vec::with_capacity(4);

    let mut queue = Vec::new();
    queue.push(map.start);

    let mut scores = vec![vec![u16::MAX; map.width as usize]; map.height as usize];
    scores[map.start] = 0;

    while let Some(position) = queue.pop() {
        let score = scores[position] + 1;
        map.fill_neighbors(position, &mut neighbors);

        for &neighbor in &neighbors {
            if score >= scores[neighbor] {
                continue;
            }

            scores[neighbor] = score;

            if neighbor != map.end {
                queue.push(neighbor);
            }
        }
    }

    match scores[map.end] {
        u16::MAX => None,
        score => Some(score as usize),
    }
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

    fn fill_neighbors(&self, position: Position, buffer: &mut Vec<Position>) {
        buffer.clear();

        let right = position.x + 1;
        let down = position.y + 1;
        let max_height = self.grid[position] + 1;

        if right < self.width {
            let position = Position::new(right, position.y);

            if self.grid[position] <= max_height {
                buffer.push(position);
            }
        }

        if down < self.height {
            let position = Position::new(position.x, down);

            if self.grid[position] <= max_height {
                buffer.push(position);
            }
        }

        if position.y > 0 {
            let position = Position::new(position.x, position.y - 1);

            if self.grid[position] <= max_height {
                buffer.push(position);
            }
        }

        if position.x > 0 {
            let position = Position::new(position.x - 1, position.y);

            if self.grid[position] <= max_height {
                buffer.push(position);
            }
        }
    }
}
