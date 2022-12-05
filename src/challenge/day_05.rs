use anyhow::Context;
use std::str::FromStr;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    solve(input, &CrateMover9000)
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    solve(input, &CrateMover9001)
}

fn solve(input: &[&str], crane: &impl Crane) -> anyhow::Result<impl std::fmt::Display> {
    let index = input
        .iter()
        .position(|line| line.is_empty())
        .context("Input doesn't contain a new line")?;

    let mut stacks = Stacks::new(&input[..index]);

    for line in &input[index + 1..] {
        crane.move_crates(&mut stacks, line.parse()?);
    }

    let result = stacks
        .0
        .iter()
        .filter_map(|stack| stack.last())
        .map(|char| *char as char)
        .collect::<String>();

    Ok(result)
}

struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut iterator = line.split_ascii_whitespace();
        iterator.next();
        let count = next_number(&mut iterator, line)?;
        iterator.next();
        let from = next_number(&mut iterator, line)? - 1;
        iterator.next();
        let to = next_number(&mut iterator, line)? - 1;
        Ok(Instruction { count, from, to })
    }
}

fn next_number<'a>(
    iterator: &mut impl Iterator<Item = &'a str>,
    source: &str,
) -> anyhow::Result<usize> {
    let number = iterator
        .next()
        .with_context(|| format!("{} in not a valid instruction", source))?
        .parse()?;

    Ok(number)
}

struct Stacks(Box<[Vec<u8>]>);

impl Stacks {
    fn new(lines: &[&str]) -> Self {
        let columns = (lines[0].len() + 1) / 4;
        let mut stacks = (0..columns).map(|_| Vec::new()).collect::<Box<[_]>>();

        for line in lines.iter().rev().skip(1) {
            let bytes = line.as_bytes();

            for i in 0..columns {
                let byte = bytes[i * 4 + 1];

                if byte != b' ' {
                    stacks[i].push(byte);
                }
            }
        }

        Stacks(stacks)
    }
}

struct CrateMover9000;
struct CrateMover9001;

trait Crane {
    fn move_crates(&self, stacks: &mut Stacks, instruction: Instruction);
}

impl Crane for CrateMover9000 {
    fn move_crates(&self, stacks: &mut Stacks, instruction: Instruction) {
        for _ in 0..instruction.count {
            let value = stacks.0[instruction.from].pop().unwrap();
            stacks.0[instruction.to].push(value);
        }
    }
}

impl Crane for CrateMover9001 {
    fn move_crates(&self, stacks: &mut Stacks, instruction: Instruction) {
        let length = stacks.0[instruction.from].len();
        let start = length - instruction.count;

        for i in start..length {
            stacks.0[instruction.to].push(stacks.0[instruction.from][i])
        }

        stacks.0[instruction.from].truncate(start);
    }
}
