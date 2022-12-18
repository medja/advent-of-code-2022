use anyhow::Context;
use std::str::FromStr;

const GRID_SIZE: usize = 20;
const MAX_COORDINATE: usize = GRID_SIZE - 1;
const CUBE_SIDES: usize = 6;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let grid = build_grid(input)?;

    let surface = Coordinate::all()
        .filter(|coordinate| grid.get(coordinate.clone()))
        .map(|coordinate| CUBE_SIDES - grid.count_neighbors(coordinate))
        .sum::<usize>();

    Ok(surface)
}

fn build_grid(input: &[&str]) -> anyhow::Result<Grid> {
    input.iter().try_fold(Grid::new(), |mut grid, line| {
        grid.set(line.parse()?);
        Ok(grid)
    })
}

#[derive(Clone)]
struct Coordinate {
    x: usize,
    y: usize,
    z: usize,
}

impl Coordinate {
    fn index(self) -> usize {
        self.z * GRID_SIZE * GRID_SIZE + self.y * GRID_SIZE + self.x
    }

    fn all() -> impl Iterator<Item = Coordinate> {
        (0..GRID_SIZE).flat_map(|z| {
            (0..GRID_SIZE).flat_map(move |y| (0..GRID_SIZE).map(move |x| Coordinate { x, y, z }))
        })
    }
}

impl FromStr for Coordinate {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut iterator = input.split(',');

        Ok(Coordinate {
            x: iterator
                .next()
                .context("Unexpected end of input")?
                .parse()?,
            y: iterator
                .next()
                .context("Unexpected end of input")?
                .parse()?,
            z: iterator
                .next()
                .context("Unexpected end of input")?
                .parse()?,
        })
    }
}

struct Grid(Vec<bool>);

impl Grid {
    fn new() -> Self {
        Grid(vec![false; GRID_SIZE * GRID_SIZE * GRID_SIZE])
    }

    fn get(&self, coordinate: Coordinate) -> bool {
        self.0[coordinate.index()]
    }

    fn set(&mut self, coordinate: Coordinate) {
        self.0[coordinate.index()] = true;
    }

    fn count_neighbors(&self, coordinate: Coordinate) -> usize {
        let mut count = 0;

        if coordinate.z > 0
            && self.get(Coordinate {
                x: coordinate.x,
                y: coordinate.y,
                z: coordinate.z - 1,
            })
        {
            count += 1;
        }

        if coordinate.z < MAX_COORDINATE
            && self.get(Coordinate {
                x: coordinate.x,
                y: coordinate.y,
                z: coordinate.z + 1,
            })
        {
            count += 1;
        }

        if coordinate.y > 0
            && self.get(Coordinate {
                x: coordinate.x,
                y: coordinate.y - 1,
                z: coordinate.z,
            })
        {
            count += 1;
        }

        if coordinate.y < MAX_COORDINATE
            && self.get(Coordinate {
                x: coordinate.x,
                y: coordinate.y + 1,
                z: coordinate.z,
            })
        {
            count += 1;
        }

        if coordinate.x > 0
            && self.get(Coordinate {
                x: coordinate.x - 1,
                y: coordinate.y,
                z: coordinate.z,
            })
        {
            count += 1;
        }

        if coordinate.x < MAX_COORDINATE
            && self.get(Coordinate {
                x: coordinate.x + 1,
                y: coordinate.y,
                z: coordinate.z,
            })
        {
            count += 1;
        }

        count
    }
}
