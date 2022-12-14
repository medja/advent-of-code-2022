use anyhow::Context;
use std::ops::RangeInclusive;
use std::str::FromStr;

// Assume we won't go out of these bounds for any input
const MIN_X: usize = 300;
const MAX_X: usize = 700;
const MIN_Y: usize = 0;
const MAX_Y: usize = 180;

const WIDTH: usize = MAX_X - MIN_X + 1;
const HEIGHT: usize = MAX_Y - MIN_Y + 1;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(Cave::new(input)?.simulate_sand_without_floor())
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(Cave::new(input)?.simulate_sand_with_floor())
}

#[derive(Clone)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    const SAND_SOURCE: Coordinate = Coordinate {
        x: (500 - MIN_X),
        y: 0,
    };
}

impl FromStr for Coordinate {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (x, y) = input
            .split_once(',')
            .with_context(|| format!("Could not find , in {}", input))?;

        let x = x.parse()?;
        let y = y.parse()?;

        Ok(Coordinate { x, y })
    }
}

struct Cave {
    grid: Vec<Vec<bool>>,
    bottom: usize,
}

impl Cave {
    fn new(input: &[&str]) -> anyhow::Result<Self> {
        let mut bottom = 0;
        let mut grid = vec![vec![false; WIDTH]; HEIGHT];

        for line in input {
            let mut iterator = line.split_ascii_whitespace().step_by(2);

            let start = iterator.next().context("Unexpected end of input")?;
            let mut start = Coordinate::from_str(start)?;

            for end in iterator {
                let end = Coordinate::from_str(end)?;

                if start.x != end.x {
                    let y = start.y - MIN_Y;

                    if y > bottom {
                        bottom = y;
                    }

                    for x in build_range(start.x - MIN_X, end.x - MIN_X) {
                        grid[y][x] = true;
                    }
                } else {
                    let x = start.x - MIN_X;

                    for y in build_range(start.y - MIN_Y, end.y - MIN_Y) {
                        grid[y][x] = true;
                    }
                }

                start = end;
            }
        }

        bottom += 1;
        Ok(Cave { grid, bottom })
    }

    fn simulate_sand_with_floor(&mut self) -> usize {
        let mut count = 0;
        let mut queue = Vec::new();

        queue.push(Coordinate::SAND_SOURCE);

        while let Some(mut position) = queue.pop() {
            if self.grid[position.y][position.x] {
                continue;
            }

            self.grid[position.y][position.x] = true;
            count += 1;

            if position.y == self.bottom {
                continue;
            }

            position.y += 1;
            queue.push(position.clone());
            position.x -= 1;
            queue.push(position.clone());
            position.x += 2;
            queue.push(position);
        }

        count
    }

    fn simulate_sand_without_floor(&mut self) -> usize {
        (1..).take_while(|_| self.drop_sand()).count()
    }

    fn drop_sand(&mut self) -> bool {
        let mut position = Coordinate::SAND_SOURCE;

        if self.grid[position.y][position.x] {
            return false;
        }

        loop {
            position.y += 1;

            if self.grid[position.y][position.x] {
                if !self.grid[position.y][position.x - 1] {
                    position.x -= 1;
                } else if !self.grid[position.y][position.x + 1] {
                    position.x += 1;
                } else {
                    position.y -= 1;
                    break;
                }
            }

            if position.y == self.bottom {
                return false;
            }
        }

        self.grid[position.y][position.x] = true;
        true
    }
}

fn build_range(start: usize, end: usize) -> RangeInclusive<usize> {
    if start < end {
        start..=end
    } else {
        end..=start
    }
}
