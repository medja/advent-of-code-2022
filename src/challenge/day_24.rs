use std::collections::VecDeque;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut valley = Valley::new(input);
    valley.predict();
    Ok(find_path(Position::START, valley.exit(), &mut valley))
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut valley = Valley::new(input);
    let exit = valley.exit();
    valley.predict();

    let there = find_path(Position::START, exit, &mut valley);
    let back = find_path(exit, Position::START, &mut valley);
    let finish = find_path(Position::START, exit, &mut valley);

    Ok(there + back + finish)
}

// BFS search for best path
fn find_path(start: Position, end: Position, valley: &mut Valley) -> usize {
    let mut minute = 0;

    // Wait until we can make out first move
    while !valley.get(start).will_be_empty() {
        minute += 1;
        valley.simulate();
        valley.predict();
    }

    let mut queue = VecDeque::new();
    let mut next_positions = vec![false; valley.width * valley.height];
    queue.push_back(State::new(minute + 1, start));

    loop {
        let state = queue.pop_front();

        // None means we couldn't find a path
        // But that's not the end since it's valid and safe to stay before the starting position
        if !matches!(&state, Some(state) if state.minute == minute) {
            minute += 1;

            // Simulate waiting before the starting position
            if valley.get(start).will_be_empty() {
                let index = start.x as usize + (start.y as usize) * valley.width;

                if !next_positions[index] {
                    queue.push_back(State::new(minute, start));
                }
            }

            valley.simulate();
            valley.predict();
            next_positions.fill(false);
        }

        // Process next move (if one exists)
        let state = match state {
            Some(state) => state,
            None => continue,
        };

        if state.position == end {
            // Simulate moving out of the valley
            valley.simulate();
            valley.predict();
            return state.minute as usize + 1;
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

#[derive(Eq, PartialEq, Copy, Clone)]
struct Blizzard(u8);

impl Blizzard {
    const UP: Blizzard = Blizzard(1 << 0);
    const RIGHT: Blizzard = Blizzard(1 << 1);
    const DOWN: Blizzard = Blizzard(1 << 2);
    const LEFT: Blizzard = Blizzard(1 << 3);

    const ALL: [Blizzard; 4] = [
        Blizzard::UP,
        Blizzard::RIGHT,
        Blizzard::DOWN,
        Blizzard::LEFT,
    ];
}

// Stores the current blizzards are stored in the 4 least significant bits and the upcoming
// blizzards in the next 4 bits
#[derive(Copy, Clone)]
struct Tile(u8);

impl Tile {
    fn new(byte: u8) -> Self {
        match byte {
            b'^' => Tile(Blizzard::UP.0),
            b'>' => Tile(Blizzard::RIGHT.0),
            b'v' => Tile(Blizzard::DOWN.0),
            b'<' => Tile(Blizzard::LEFT.0),
            b'.' => Tile(0),
            _ => unreachable!(),
        }
    }

    fn will_be_empty(self) -> bool {
        self.0 & 0b11110000 == 0
    }

    fn contains(self, blizzard: Blizzard) -> bool {
        self.0 & blizzard.0 == blizzard.0
    }

    fn enqueue(&mut self, blizzard: Blizzard) {
        self.0 |= blizzard.0 << 4
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

                    let (x, y) = self.find_next_position(x, y, blizzard);
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

    fn find_next_position(&self, x: usize, y: usize, blizzard: Blizzard) -> (usize, usize) {
        match blizzard {
            Blizzard::UP => {
                if y == 0 {
                    (x, self.height - 1)
                } else {
                    (x, y - 1)
                }
            }
            Blizzard::RIGHT => {
                let next_x = x + 1;

                if next_x == self.width {
                    (0, y)
                } else {
                    (next_x, y)
                }
            }
            Blizzard::DOWN => {
                let next_y = y + 1;

                if next_y == self.height {
                    (x, 0)
                } else {
                    (x, next_y)
                }
            }
            Blizzard::LEFT => {
                if x == 0 {
                    (self.width - 1, y)
                } else {
                    (x - 1, y)
                }
            }
            _ => unreachable!(),
        }
    }
}
