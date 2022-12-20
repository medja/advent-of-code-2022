use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Context;
use lazy_static::lazy_static;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use regex::Regex;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;

lazy_static! {
    static ref CHALLENGE_PATTERN: Regex =
        Regex::new("(?i)(?:Day\\W*)?(\\d\\d?)\\W*([AB])").unwrap();
    static ref SOLUTIONS: Solutions = {
        use Day::*;
        use Part::*;

        let mut solutions = Solutions::new();

        solutions.add(Day01, PartA, day_01::part_a);
        solutions.add(Day01, PartB, day_01::part_b);
        solutions.add(Day02, PartA, day_02::part_a);
        solutions.add(Day02, PartB, day_02::part_b);
        solutions.add(Day03, PartA, day_03::part_a);
        solutions.add(Day03, PartB, day_03::part_b);
        solutions.add(Day04, PartA, day_04::part_a);
        solutions.add(Day04, PartB, day_04::part_b);
        solutions.add(Day05, PartA, day_05::part_a);
        solutions.add(Day05, PartB, day_05::part_b);
        solutions.add(Day06, PartA, day_06::part_a);
        solutions.add(Day06, PartB, day_06::part_b);
        solutions.add(Day07, PartA, day_07::part_a);
        solutions.add(Day07, PartB, day_07::part_b);
        solutions.add(Day08, PartA, day_08::part_a);
        solutions.add(Day08, PartB, day_08::part_b);
        solutions.add(Day09, PartA, day_09::part_a);
        solutions.add(Day09, PartB, day_09::part_b);
        solutions.add(Day10, PartA, day_10::part_a);
        solutions.add(Day10, PartB, day_10::part_b);
        solutions.add(Day11, PartA, day_11::part_a);
        solutions.add(Day11, PartB, day_11::part_b);
        solutions.add(Day12, PartA, day_12::part_a);
        solutions.add(Day12, PartB, day_12::part_b);
        solutions.add(Day13, PartA, day_13::part_a);
        solutions.add(Day13, PartB, day_13::part_b);
        solutions.add(Day14, PartA, day_14::part_a);
        solutions.add(Day14, PartB, day_14::part_b);
        solutions.add(Day15, PartA, day_15::part_a);
        solutions.add(Day15, PartB, day_15::part_b);
        solutions.add(Day16, PartA, day_16::part_a);
        solutions.add(Day16, PartB, day_16::part_b);
        solutions.add(Day17, PartA, day_17::part_a);
        solutions.add(Day17, PartB, day_17::part_b);
        solutions.add(Day18, PartA, day_18::part_a);
        solutions.add(Day18, PartB, day_18::part_b);
        solutions.add(Day19, PartA, day_19::part_a);
        solutions.add(Day19, PartB, day_19::part_b);
        solutions.add(Day20, PartA, day_20::part_a);

        solutions
    };
}

#[derive(
    IntoPrimitive, TryFromPrimitive, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug,
)]
#[repr(u8)]
pub enum Day {
    Day01 = 1,
    Day02 = 2,
    Day03 = 3,
    Day04 = 4,
    Day05 = 5,
    Day06 = 6,
    Day07 = 7,
    Day08 = 8,
    Day09 = 9,
    Day10 = 10,
    Day11 = 11,
    Day12 = 12,
    Day13 = 13,
    Day14 = 14,
    Day15 = 15,
    Day16 = 16,
    Day17 = 17,
    Day18 = 18,
    Day19 = 19,
    Day20 = 20,
}

impl Day {
    fn name(&self) -> &'static str {
        use Day::*;

        match self {
            Day01 => "Calorie Counting",
            Day02 => "Rock Paper Scissors",
            Day03 => "Rucksack Reorganization",
            Day04 => "Camp Cleanup",
            Day05 => "Supply Stacks",
            Day06 => "Tuning Trouble",
            Day07 => "No Space Left On Device",
            Day08 => "Treetop Tree House",
            Day09 => "Rope Bridge",
            Day10 => "Cathode-Ray Tube",
            Day11 => "Monkey in the Middle",
            Day12 => "Hill Climbing Algorithm",
            Day13 => "Distress Signal",
            Day14 => "Regolith Reservoir",
            Day15 => "Beacon Exclusion Zone",
            Day16 => "Proboscidea Volcanium",
            Day17 => "Pyroclastic Flow",
            Day18 => "Boiling Boulders",
            Day19 => "Not Enough Minerals",
            Day20 => "Grove Positioning System",
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
            _ => Err(anyhow::anyhow!(
                "{} is not a valid part, expecting A or B",
                string
            )),
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
