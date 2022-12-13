use anyhow::Context;
use std::cmp::Ordering;
use std::iter::Peekable;
use std::str::FromStr;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let result = input
        .chunks(3)
        .enumerate()
        .filter(|(_, chunk)| compare(chunk[0], chunk[1]).unwrap_or(false))
        .map(|(index, _)| index + 1)
        .sum::<usize>();

    Ok(result)
}

fn compare(left: &str, right: &str) -> anyhow::Result<bool> {
    let left = Item::from_str(left)?;
    let right = Item::from_str(right)?;
    Ok(left.cmp(&right) == Ordering::Less)
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let divider_2 = Item::List(vec![Item::List(vec![Item::Number(2)])]);
    let divider_6 = Item::List(vec![Item::List(vec![Item::Number(6)])]);
    let mut items = Vec::with_capacity(2 + (input.len() + 1) / 3);

    let input_items = input
        .iter()
        .filter(|line| !line.is_empty())
        .filter_map(|line| Item::from_str(line).ok());

    items.push(divider_2.clone());
    items.push(divider_6.clone());
    items.extend(input_items);
    items.sort();

    let x = items.iter().position(|item| item == &divider_2).unwrap() + 1;
    let y = items.iter().position(|item| item == &divider_6).unwrap() + 1;
    Ok(x * y)
}

#[derive(Eq, PartialEq, Clone)]
enum Item {
    Number(u8),
    List(Vec<Item>),
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        compare_items(self, other)
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Item {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        parse(&mut input.bytes().peekable())
    }
}

fn compare_items(left: &Item, right: &Item) -> Ordering {
    use Item::*;

    match (left, right) {
        (Number(left), Number(right)) => left.cmp(right),
        (Number(left), List(right)) => compare_lists(&vec![Number(*left)], right),
        (List(left), Number(right)) => compare_lists(left, &vec![Number(*right)]),
        (List(left), List(right)) => compare_lists(left, right),
    }
}

fn compare_lists(left: &Vec<Item>, right: &Vec<Item>) -> Ordering {
    let default = left.len().cmp(&right.len());

    left.iter()
        .zip(right)
        .map(|(left, right)| compare_items(left, right))
        .find(|ordering| *ordering != Ordering::Equal)
        .unwrap_or(default)
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
