use std::str::FromStr;
use anyhow::Context;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(solve(input, Pair::contains_overlap))
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(solve(input, Pair::contains_partial_overlap))
}

fn solve(input: &[&str], filter: fn(&Pair) -> bool) -> usize {
    input.iter()
        .filter_map(|line| Pair::from_str(line).ok())
        .filter(filter)
        .count()
}

struct Pair(Range, Range);

impl Pair {
    fn contains_overlap(&self) -> bool {
        self.0.overlaps(&self.1) || self.1.overlaps(&self.0)
    }

    fn contains_partial_overlap(&self) -> bool {
        self.0.partially_overlaps(&self.1) || self.1.partially_overlaps(&self.0)
    }
}

impl FromStr for Pair {
    type Err = anyhow::Error;

    fn from_str(pair: &str) -> Result<Self, Self::Err> {
        let (first, second) = pair.split_once(',')
            .with_context(|| format!("{} is not a valid pair", pair))?;

        Ok(Pair(first.parse()?, second.parse()?))
    }
}

struct Range {
    start: usize,
    end: usize
}

impl Range {
    fn overlaps(&self, other: &Self) -> bool {
        self.start >= other.start && self.end <= other.end
    }

    fn partially_overlaps(&self, other: &Self) -> bool {
        self.start >= other.start && self.start <= other.end
    }
}

impl FromStr for Range {
    type Err = anyhow::Error;

    fn from_str(range: &str) -> Result<Self, Self::Err> {
        let (start, end) = range.split_once('-')
            .with_context(|| format!("{} is not a valid range", range))?;

        let result = Range {
            start: start.parse()?,
            end: end.parse()?,
        };

        Ok(result)
    }
}
