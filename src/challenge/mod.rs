use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Context;
use lazy_static::lazy_static;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use regex::Regex;

mod day_01;

lazy_static! {
    static ref CHALLENGE_PATTERN: Regex =
        Regex::new("(?i)(?:Day\\W*)?(\\d\\d?)\\W*([AB])").unwrap();

    static ref SOLUTIONS: Solutions = {
        use Day::*;
        use Part::*;

        let mut solutions = Solutions::new();

        solutions.add(Day01, PartA, day_01::part_a);
        solutions.add(Day01, PartB, day_01::part_b);

        solutions
    };
}

#[derive(IntoPrimitive, TryFromPrimitive, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(u8)]
pub enum Day {
    Day01 = 1,
}

impl Day {
    fn name(&self) -> &'static str {
        use Day::*;

        match self {
            Day01 => "Calorie Counting",
        }
    }

    async fn input(&self) -> anyhow::Result<String> {
        let index = u8::from(*self);
        crate::http::get(format!("https://adventofcode.com/2022/day/{}/input", index)).await
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Day {:02}: {}", u8::from(*self), self.name())
    }
}

impl FromStr for Day {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let index = string.parse::<u8>()?;

        index
            .try_into()
            .with_context(|| format!("Day {} is out of range", index))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Part {
    PartA,
    PartB,
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Part::PartA => write!(f, "Part A"),
            Part::PartB => write!(f, "Part B"),
        }
    }
}

impl FromStr for Part {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "a" | "A" => Ok(Part::PartA),
            "b" | "B" => Ok(Part::PartB),
            _ => Err(anyhow::anyhow!("{} is not a valid part, expecting A or B", string)),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Challenge(Day, Part);

impl Challenge {
    pub fn new(day: Day, part: Part) -> Self {
        Challenge(day, part)
    }
}

impl FromStr for Challenge {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let captures = CHALLENGE_PATTERN
            .captures(string)
            .with_context(|| format!("{} is not a valid challenge, expecting \\d+[AB]", string))?;

        let day = captures
            .get(1)
            .context("Day capture group is missing")?
            .as_str()
            .parse()?;

        let part = captures
            .get(2)
            .context("Part capture group is missing")?
            .as_str()
            .parse()?;

        Ok(Challenge(day, part))
    }
}

impl Display for Challenge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.0, self.1)
    }
}

pub async fn solve(challenge: &Challenge) -> anyhow::Result<()> {
    SOLUTIONS.solve(challenge).await
}

trait Solution {
    fn run(&self, challenge: &Challenge, input: &[&str]) -> anyhow::Result<()>;
}

impl<R: Display> Solution for fn(&[&str]) -> anyhow::Result<R> {
    fn run(&self, challenge: &Challenge, input: &[&str]) -> anyhow::Result<()> {
        let start = Instant::now();
        let result = self(input)?;
        let duration = start.elapsed();
        println!("{}: {} (duration = {:?})", challenge, result, duration);
        Ok(())
    }
}

struct Solutions(HashMap<Challenge, Box<dyn Solution + Sync + 'static>>);

impl Solutions {
    fn new() -> Self {
        Solutions(HashMap::new())
    }

    fn add<R: Display + 'static>(
        &mut self,
        day: Day,
        part: Part,
        func: fn(&[&str]) -> anyhow::Result<R>,
    ) {
        self.0.insert(Challenge::new(day, part), Box::new(func));
    }

    async fn solve(&self, challenge: &Challenge) -> anyhow::Result<()> {
        let solution = self
            .0
            .get(challenge)
            .with_context(|| format!("Cannot find solution for {}", challenge))?;

        let input = challenge.0.input().await?;
        solution.run(challenge, &input.lines().collect::<Vec<_>>())
    }
}
