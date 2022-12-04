use std::str::FromStr;
use anyhow::Context;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let count = input.iter()
        .filter_map(|line| line.split_once(','))
        .filter(|(first, second)| contains_overlap(first, second).unwrap_or(false))
        .count();

    Ok(count)
}

fn contains_overlap(first: &str, second: &str) -> anyhow::Result<bool> {
    let first_range = Range::from_str(first)?;
    let second_range = Range::from_str(second)?;

    Ok(first_range.overlaps(&second_range) || second_range.overlaps(&first_range))
}

struct Range {
    start: usize,
    end: usize
}

impl Range {
    fn overlaps(&self, other: &Self) -> bool {
        self.start >= other.start && self.end <= other.end
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