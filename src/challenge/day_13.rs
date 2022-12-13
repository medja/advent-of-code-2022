use anyhow::Context;
use std::cmp::Ordering;
use std::iter::Peekable;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let result = input
        .chunks(3)
        .enumerate()
        .filter(|(_, chunk)| compare(chunk[0], chunk[1]).unwrap_or(false))
        .map(|(index, _)| index + 1)
        .sum::<usize>();

    Ok(result)
}

enum Item {
    Number(u8),
    List(Vec<Item>),
}

fn compare(left: &str, right: &str) -> anyhow::Result<bool> {
    let left = parse(&mut left.bytes().peekable())?;
    let right = parse(&mut right.bytes().peekable())?;
    Ok(compare_items(left, right).unwrap_or(false))
}

fn compare_items(left: Item, right: Item) -> Option<bool> {
    use Item::*;

    match (left, right) {
        (Number(left), Number(right)) => compare_numbers(left, right),
        (Number(left), List(right)) => compare_lists(vec![Number(left)], right),
        (List(left), Number(right)) => compare_lists(left, vec![Number(right)]),
        (List(left), List(right)) => compare_lists(left, right),
    }
}

fn compare_lists(left: Vec<Item>, right: Vec<Item>) -> Option<bool> {
    let default = compare_numbers(left.len(), right.len());

    left.into_iter()
        .zip(right)
        .find_map(|(left, right)| compare_items(left, right))
        .or(default)
}

fn compare_numbers<N: Ord>(left: N, right: N) -> Option<bool> {
    match left.cmp(&right) {
        Ordering::Less => Some(true),
        Ordering::Equal => None,
        Ordering::Greater => Some(false),
    }
}

fn parse(input: &mut Peekable<impl Iterator<Item = u8>>) -> anyhow::Result<Item> {
    let next = input.next().context("Unexpected end of input")?;

    if next == b'[' {
        let mut items = Vec::new();

        while matches!(input.peek(), Some(char) if *char != b']') {
            items.push(parse(input)?);

            if matches!(input.peek(), Some(b',')) {
                input.next();
            }
        }

        input.next();

        Ok(Item::List(items))
    } else {
        let mut number = next - b'0';

        while matches!(input.peek(), Some(char) if char.is_ascii_digit()) {
            number = 10 * number + input.next().unwrap() - b'0';
        }

        Ok(Item::Number(number))
    }
}
