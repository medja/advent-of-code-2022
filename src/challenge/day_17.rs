const SHAPES: [Shape; 5] = [
    Shape([0b11110, 0, 0, 0]),
    Shape([0b1000, 0b11100, 0b1000, 0]),
    Shape([0b11100, 0b100, 0b100, 0]),
    Shape([0b10000, 0b10000, 0b10000, 0b10000]),
    Shape([0b11000, 0b11000, 0, 0]),
];

// Assume the pattern will show up after at most this many shapes
const SCAN_SHAPE_COUNT: usize = 2000;
// Assume the pattern can be identified using only these many rows
const PATTERN_LOOKUP_HEIGHT: usize = 10;
// Assume the pattern will after this many rows
const PATTERN_LOOKUP_OFFSET: usize = 10;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut chamber = Chamber::new(input[0]);

    for _ in 0..2022 {
        chamber.drop();
    }

    Ok(chamber.height())
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut chamber = Chamber::new(input[0]);

    for _ in 0..SCAN_SHAPE_COUNT {
        chamber.drop();
    }

    // search for a cycle at an offset
    let pattern_start = chamber.rocks.len() - PATTERN_LOOKUP_OFFSET;
    let pattern_lookup_end = pattern_start - PATTERN_LOOKUP_HEIGHT;
    let search = &chamber.rocks[pattern_lookup_end..pattern_start];

    let pattern_height = (search.len()..)
        .find(|i| search == &chamber.rocks[pattern_start - search.len() - i..pattern_start - i])
        .unwrap();

    let next_pattern_start = pattern_start + pattern_height;
    let next_pattern_lookup_end = next_pattern_start - PATTERN_LOOKUP_HEIGHT;

    // build up the last cycle
    while chamber.rocks.len() < next_pattern_start
        || chamber.rocks[next_pattern_lookup_end..next_pattern_start]
            != chamber.rocks[pattern_lookup_end..pattern_start]
    {
        chamber.drop();
    }

    let current_shape_count = chamber.shape_count;
    let next_pattern_start = next_pattern_start + pattern_height;
    let next_pattern_lookup_end = next_pattern_start - PATTERN_LOOKUP_HEIGHT;

    // build up another cycle to count the shapes
    while chamber.rocks.len() < next_pattern_start
        || chamber.rocks[next_pattern_lookup_end..next_pattern_start]
            != chamber.rocks[pattern_lookup_end..pattern_start]
    {
        chamber.drop();
    }

    let pattern_shape_count = chamber.shape_count - current_shape_count;
    let remaining = 1000000000000 - chamber.shape_count;

    // assume we've repeated the cycle as many times as we can, without exceeding 1000000000000 shapes
    // simulate the last few shapes which don't add up to a full cycle
    for _ in 0..remaining % pattern_shape_count {
        chamber.drop();
    }

    Ok(chamber.height() + remaining / pattern_shape_count * pattern_height)
}

struct Shapes(usize);

impl Shapes {
    fn next(&mut self) -> Shape {
        let index = self.0;
        self.0 = (self.0 + 1) % SHAPES.len();
        SHAPES[index]
    }
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
    shape_count: usize,
    rocks: Vec<u8>,
    shapes: Shapes,
    shifts: Shifts<'a>,
}

impl<'a> Chamber<'a> {
    fn new(shifts: &'a str) -> Self {
        Chamber {
            shape_count: 0,
            rocks: vec![0b1111111; 4],
            shapes: Shapes(0),
            shifts: Shifts(0, shifts.as_bytes()),
        }
    }

    fn height(&self) -> usize {
        self.rocks.len() - 4
    }

    fn drop(&mut self) {
        let mut shape = self.shapes.next();
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

        self.shape_count += 1;
    }

    fn check_collision(&self, shape: Shape, height: usize) -> bool {
        let overlap = self.rocks.len().saturating_sub(height).min(4);

        shape.0[..overlap]
            .iter()
            .enumerate()
            .any(|(i, row)| self.rocks[height + i] & row != 0)
    }
}
