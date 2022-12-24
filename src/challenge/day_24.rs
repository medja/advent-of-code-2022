use std::collections::VecDeque;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(find_path(Valley::new(input)))
}

fn find_path(mut valley: Valley) -> usize {
    let exit = valley.exit();
    let mut minute = 0;
    valley.predict();

    while !valley.get(Position::START).will_be_empty() {
        valley.simulate();
        valley.predict();
        minute += 1;
    }

    let mut queue = VecDeque::new();
    let mut next_positions = vec![false; valley.width * valley.height];
    queue.push_back(State::new(minute + 1, Position::START));

    while let Some(state) = queue.pop_front() {
        if state.position == exit {
            return state.minute as usize + 1;
        }

        if state.minute > minute {
            valley.simulate();
            valley.predict();
            minute = state.minute;
            next_positions.fill(false);
        }

        for position in state.position.find_moves(valley.width, valley.height) {
            if valley.get(position).will_be_empty() {
                let index = position.x as usize + (position.y as usize) * valley.width;

                if !next_positions[index] {
                    queue.push_back(State::new(state.minute + 1, position));
                    next_positions[index] = true;
                }
            }
        }
    }

    unreachable!()
}

struct State {
    minute: u16,
    position: Position,
}

impl State {
    fn new(minute: u16, position: Position) -> Self {
        State { minute, position }
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
struct Position {
    x: u8,
    y: u8,
}

impl Position {
    const fn new(x: u8, y: u8) -> Self {
        Position { x, y }
    }

    const START: Position = Position::new(0, 0);

    fn find_moves(self, max_x: usize, max_y: usize) -> Moves {
        let mut moves = Moves::new();
        let right = self.x + 1;
        let down = self.y + 1;

        moves.push(self);

        if self.x > 0 {
            moves.push(Position::new(self.x - 1, self.y));
        }

        if self.y > 0 {
            moves.push(Position::new(self.x, self.y - 1));
        }

        if right < max_x as u8 {
            moves.push(Position::new(right, self.y))
        }

        if down < max_y as u8 {
            moves.push(Position::new(self.x, down))
        }

        moves
    }
}

struct Moves {
    positions: [Position; 5],
    length: usize,
}

impl Moves {
    fn new() -> Self {
        Moves {
            positions: [Position::START; 5],
            length: 0,
        }
    }

    fn push(&mut self, position: Position) {
        self.positions[self.length] = position;
        self.length += 1;
    }
}

impl Iterator for Moves {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.length == 0 {
            None
        } else {
            self.length -= 1;
            Some(self.positions[self.length])
        }
    }
}

#[derive(Copy, Clone)]
struct Blizzard {
    mask: u8,
    dx: i8,
    dy: i8,
}

impl Blizzard {
    const fn new(mask: u8, dx: i8, dy: i8) -> Self {
        Blizzard { mask, dx, dy }
    }

    const UP: Blizzard = Blizzard::new(1 << 0, 0, -1);
    const RIGHT: Blizzard = Blizzard::new(1 << 1, 1, 0);
    const DOWN: Blizzard = Blizzard::new(1 << 2, 0, 1);
    const LEFT: Blizzard = Blizzard::new(1 << 3, -1, 0);

    const ALL: [Blizzard; 4] = [
        Blizzard::UP,
        Blizzard::RIGHT,
        Blizzard::DOWN,
        Blizzard::LEFT,
    ];
}

#[derive(Copy, Clone)]
struct Tile(u8);

impl Tile {
    fn new(byte: u8) -> Self {
        match byte {
            b'^' => Tile(Blizzard::UP.mask),
            b'>' => Tile(Blizzard::RIGHT.mask),
            b'v' => Tile(Blizzard::DOWN.mask),
            b'<' => Tile(Blizzard::LEFT.mask),
            b'.' => Tile(0),
            _ => unreachable!(),
        }
    }

    fn will_be_empty(self) -> bool {
        self.0 & 0b11110000 == 0
    }

    fn contains(self, blizzard: Blizzard) -> bool {
        self.0 & blizzard.mask == blizzard.mask
    }

    fn enqueue(&mut self, blizzard: Blizzard) {
        self.0 |= blizzard.mask << 4
    }

    fn update(&mut self) {
        self.0 >>= 4
    }
}

struct Valley {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl Valley {
    fn new(input: &[&str]) -> Self {
        let width = input[0].len() - 2;
        let height = input.len() - 2;

        let tiles = input[1..=height]
            .iter()
            .flat_map(|row| &row.as_bytes()[1..=width])
            .map(|byte| Tile::new(*byte))
            .collect::<Vec<_>>();

        Valley {
            width,
            height,
            tiles,
        }
    }

    fn get(&self, position: Position) -> Tile {
        self.tiles[position.x as usize + (position.y as usize) * self.width]
    }

    fn predict(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let tile = self.tiles[x + y * self.width];

                for blizzard in Blizzard::ALL {
                    if !tile.contains(blizzard) {
                        continue;
                    }

                    let x = add_coordinate(x, blizzard.dx, self.width);
                    let y = add_coordinate(y, blizzard.dy, self.height);
                    self.tiles[x + y * self.width].enqueue(blizzard);
                }
            }
        }
    }

    fn simulate(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.tiles[x + y * self.width].update();
            }
        }
    }

    fn exit(&self) -> Position {
        Position {
            x: self.width as u8 - 1,
            y: self.height as u8 - 1,
        }
    }
}

fn add_coordinate(value: usize, delta: i8, max: usize) -> usize {
    (value + max).wrapping_add_signed(delta as isize) % max
}
