use std::collections::VecDeque;

const MONKEY_LINE_LENGTH: usize = 7;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(solve(input, 20, true))
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(solve(input, 10000, false))
}

fn solve(input: &[&str], rounds: usize, relief: bool) -> usize {
    let length = (input.len() + 1) / MONKEY_LINE_LENGTH;
    let mut monkeys = Vec::with_capacity(length);

    for i in 0..length {
        let input = &input[i * MONKEY_LINE_LENGTH..(i + 1) * MONKEY_LINE_LENGTH - 1];
        monkeys.push(Monkey::parse(input));
    }

    let modulo = monkeys.iter().fold(1, |acc, monkey| acc * monkey.divisor);

    for _ in 0..rounds {
        for i in 0..length {
            while let Some(throw) = monkeys[i].throw_next_item(relief, modulo) {
                monkeys[throw.monkey].catch_item(throw.item);
            }
        }
    }

    let mut counts = monkeys
        .into_iter()
        .map(|monkey| monkey.throw_count)
        .collect::<Vec<usize>>();

    counts.sort();
    counts[length - 1] * counts[length - 2]
}

#[derive(Debug)]
struct Throw {
    monkey: usize,
    item: u64,
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    divisor: u64,
    if_true: usize,
    if_false: usize,
    throw_count: usize,
}

impl Monkey {
    fn parse(input: &[&str]) -> Self {
        let items = input[1][17..]
            .split(',')
            .map(|item| item[1..].parse::<u64>().unwrap());

        Monkey {
            items: VecDeque::from_iter(items),
            operation: Operation::parse(&input[2][23..]),
            divisor: input[3][21..].parse::<u64>().unwrap(),
            if_true: input[4][29..].parse::<usize>().unwrap(),
            if_false: input[5][30..].parse::<usize>().unwrap(),
            throw_count: 0,
        }
    }

    fn throw_next_item(&mut self, relief: bool, modulo: u64) -> Option<Throw> {
        let item = self
            .operation
            .calculate_worry_level(self.items.pop_front()?, relief, modulo);

        let monkey = if item % self.divisor == 0 {
            self.if_true
        } else {
            self.if_false
        };

        self.throw_count += 1;

        Some(Throw { item, monkey })
    }

    fn catch_item(&mut self, item: u64) {
        self.items.push_back(item);
    }
}

#[derive(Debug)]
enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

impl Operation {
    fn parse(input: &str) -> Self {
        if input.as_bytes()[2] == b'o' {
            return Operation::Square;
        }

        let value = input[2..].parse::<u64>().unwrap();

        match input.as_bytes()[0] {
            b'+' => Operation::Add(value),
            b'*' => Operation::Multiply(value),
            _ => unreachable!(),
        }
    }

    fn calculate_worry_level(&self, item: u64, relief: bool, modulo: u64) -> u64 {
        let result = match self {
            Operation::Add(value) => value.overflowing_add(item).0,
            Operation::Multiply(value) => value.overflowing_mul(item).0,
            Operation::Square => item.overflowing_mul(item).0,
        };

        if relief {
            result / 3
        } else {
            result % modulo
        }
    }
}
