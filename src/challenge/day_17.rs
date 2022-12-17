const SHAPES: [Shape; 5] = [
    Shape([0b11110, 0, 0, 0]),
    Shape([0b1000, 0b11100, 0b1000, 0]),
    Shape([0b11100, 0b100, 0b100, 0]),
    Shape([0b10000, 0b10000, 0b10000, 0b10000]),
    Shape([0b11000, 0b11000, 0, 0]),
];

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut chamber = Chamber::new(input[0]);

    for i in 0..2022 {
        chamber.drop(SHAPES[i % SHAPES.len()]);
    }

    Ok(chamber.height())
}

struct Shifts<'a>(usize, &'a [u8]);

impl Shifts<'_> {
    fn next(&mut self) -> Direction {
        let index = self.0;
        self.0 = (self.0 + 1) % self.1.len();

        match self.1[index] {
            b'<' => Direction::Left,
            b'>' => Direction::Right,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Left,
    Right,
}

#[derive(Copy, Clone)]
struct Shape([u8; 4]);

impl Shape {
    fn shift(self, direction: Direction) -> Self {
        match direction {
            Direction::Left => {
                if self.0.iter().all(|row| (row & 0b1000000) == 0) {
                    Shape(self.0.map(|row| row << 1))
                } else {
                    self
                }
            }
            Direction::Right => {
                if self.0.iter().all(|row| (row & 0b1) == 0) {
                    Shape(self.0.map(|row| row >> 1))
                } else {
                    self
                }
            }
        }
    }
}

struct Chamber<'a> {
    rocks: Vec<u8>,
    shifts: Shifts<'a>,
}

impl<'a> Chamber<'a> {
    fn new(shifts: &'a str) -> Self {
        Chamber {
            rocks: vec![0b1111111; 4],
            shifts: Shifts(0, shifts.as_bytes()),
        }
    }

    fn height(&self) -> usize {
        self.rocks.len() - 4
    }

    fn drop(&mut self, mut shape: Shape) {
        let mut height = self.rocks.len() + 3;

        loop {
            let shifted = shape.shift(self.shifts.next());

            if !self.check_collision(shifted, height) {
                shape = shifted;
            }

            height -= 1;

            if self.check_collision(shape, height) {
                break;
            }
        }

        height += 1;

        for (i, &row) in shape.0.iter().enumerate() {
            let index = height + i;

            if index < self.rocks.len() {
                self.rocks[index] |= row;
            } else if row != 0 {
                self.rocks.push(row);
            }
        }
    }

    fn check_collision(&self, shape: Shape, height: usize) -> bool {
        let overlap = self.rocks.len().saturating_sub(height).min(4);

        shape.0[..overlap]
            .iter()
            .enumerate()
            .any(|(i, row)| self.rocks[height + i] & row != 0)
    }
}
