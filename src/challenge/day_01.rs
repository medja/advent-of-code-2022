use anyhow::Context;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    input
        .split(|line| line.is_empty())
        .map(total_calories)
        .max()
        .context("Input is empty")
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut calories = input
        .split(|line| line.is_empty())
        .map(total_calories)
        .collect::<Vec<_>>();

    calories.sort();
    Ok(calories.iter().rev().take(3).sum::<usize>())
}

fn total_calories(inventory: &[&str]) -> usize {
    inventory
        .iter()
        .filter_map(|calories| calories.parse::<usize>().ok())
        .sum::<usize>()
}
