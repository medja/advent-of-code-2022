use anyhow::Context;
use std::str::FromStr;

const GRID_SIZE: usize = 22;
const CUBE_SIDES: usize = 6;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let grid = build_grid(input)?;

    let surface = Coordinate::all()
        .filter(|coordinate| grid.get(*coordinate) == Block::Lava)
        .map(|coordinate| {
            let count = coordinate
                .neighbors()
                .map(|neighbor| grid.get(neighbor))
                .filter(|block| *block == Block::Lava)
                .count();

            CUBE_SIDES - count
        })
        .sum::<usize>();

    Ok(surface)
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut grid = build_grid(input)?;
    Ok(calculate_surface(Coordinate::new(0, 0, 0), &mut grid))
}

fn calculate_surface(coordinate: Coordinate, grid: &mut Grid) -> usize {
    if !grid.replace(coordinate, Block::Air, Block::Water) {
        return 0;
    }

    coordinate
        .neighbors()
        .map(|neighbor| match grid.get(neighbor) {
            Block::Air => calculate_surface(neighbor, grid),
            Block::Lava => 1,
            Block::Water => 0,
        })
        .sum()
}

fn build_grid(input: &[&str]) -> anyhow::Result<Grid> {
    input.iter().try_fold(Grid::new(), |mut grid, line| {
        grid.set(line.parse()?, Block::Lava);
        Ok(grid)
    })
}

#[derive(Copy, Clone)]
struct Coordinate {
    x: u8,
    y: u8,
    z: u8,
}

impl Coordinate {
    fn new(x: u8, y: u8, z: u8) -> Self {
        Coordinate { x, y, z }
    }

    fn index(self) -> usize {
        self.z as usize * GRID_SIZE * GRID_SIZE + self.y as usize * GRID_SIZE + self.x as usize
    }

    fn all() -> impl Iterator<Item = Coordinate> {
        (0..GRID_SIZE as u8).flat_map(|z| {
            (0..GRID_SIZE as u8)
                .flat_map(move |y| (0..GRID_SIZE as u8).map(move |x| Coordinate::new(x, y, z)))
        })
    }

    fn neighbors(self) -> impl Iterator<Item = Coordinate> {
        const MAX: u8 = GRID_SIZE as u8 - 1;

        let neighbors = [
            if self.z > 0 {
                Some(Coordinate::new(self.x, self.y, self.z - 1))
            } else {
                None
            },
            if self.z < MAX {
                Some(Coordinate::new(self.x, self.y, self.z + 1))
            } else {
                None
            },
            if self.y > 0 {
                Some(Coordinate::new(self.x, self.y - 1, self.z))
            } else {
                None
            },
            if self.y < MAX {
                Some(Coordinate::new(self.x, self.y + 1, self.z))
            } else {
                None
            },
            if self.x > 0 {
                Some(Coordinate::new(self.x - 1, self.y, self.z))
            } else {
                None
            },
            if self.x < MAX {
                Some(Coordinate::new(self.x + 1, self.y, self.z))
            } else {
                None
            },
        ];

        neighbors.into_iter().flatten()
    }
}

impl FromStr for Coordinate {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut iterator = input.split(',');

        let x: u8 = iterator
            .next()
            .context("Unexpected end of input")?
            .parse()?;
        let y: u8 = iterator
            .next()
            .context("Unexpected end of input")?
            .parse()?;
        let z: u8 = iterator
            .next()
            .context("Unexpected end of input")?
            .parse()?;

        Ok(Coordinate::new(x + 1, y + 1, z + 1))
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Block {
    Air,
    Lava,
    Water,
}

struct Grid(Vec<Block>);

impl Grid {
    fn new() -> Self {
        Grid(vec![Block::Air; GRID_SIZE * GRID_SIZE * GRID_SIZE])
    }

    fn get(&self, coordinate: Coordinate) -> Block {
        self.0[coordinate.index()]
    }

    fn set(&mut self, coordinate: Coordinate, block: Block) {
        self.0[coordinate.index()] = block;
    }

    fn replace(&mut self, coordinate: Coordinate, block: Block, replace_with: Block) -> bool {
        let index = coordinate.index();

        if self.0[index] == block {
            self.0[index] = replace_with;
            true
        } else {
            false
        }
    }
}
