use std::cmp::Ordering;
use std::iter::Peekable;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let result = parse(input)
        .chunks(2)
        .enumerate()
        .filter(|(_, chunk)| chunk[0].cmp(&chunk[1]) == Ordering::Less)
        .map(|(index, _)| index + 1)
        .sum::<usize>();

    Ok(result)
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let divider_2 = Signal::divider(2);
    let divider_6 = Signal::divider(6);

    let mut signals = parse(input);
    signals.push(divider_2.clone());
    signals.push(divider_6.clone());
    signals.sort();

    let first = signals.iter().position(|item| item == &divider_2).unwrap() + 1;
    let second = signals.iter().position(|item| item == &divider_6).unwrap() + 1;
    Ok(first * second)
}

#[derive(Eq, PartialEq, Clone)]
enum Token {
    ArrayStart(u8),
    ArrayEnd(u8),
    Value(u8),
}

enum Item<'a> {
    Value(u8),
    Array(&'a [Token]),
}

#[derive(Eq, PartialEq, Clone)]
struct Signal(Vec<Token>);

impl Signal {
    fn divider(value: u8) -> Self {
        let tokens = vec![
            Token::ArrayStart(4),
            Token::ArrayStart(2),
            Token::Value(value),
            Token::ArrayEnd(1),
            Token::ArrayEnd(0),
        ];

        Signal(tokens)
    }
}

impl Ord for Signal {
    fn cmp(&self, other: &Self) -> Ordering {
        compare(&self.0, &other.0)
    }
}

impl PartialOrd for Signal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(input: &[&str]) -> Vec<Signal> {
    input
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| parse_signal(&mut line.bytes().peekable(), line.len()))
        .collect::<Vec<_>>()
}

fn parse_signal(input: &mut Peekable<impl Iterator<Item = u8>>, size_hint: usize) -> Signal {
    let mut tokens = Vec::with_capacity(size_hint);
    let mut depth = 0;

    while let Some(char) = input.next() {
        match char {
            b'[' => {
                tokens.push(Token::ArrayStart(depth));
                depth += 1;
            }
            b']' => {
                depth -= 1;
                tokens.push(Token::ArrayEnd(depth));
            }
            b',' => {}
            _ if char.is_ascii_digit() => {
                let mut value = char - b'0';

                if matches!(input.peek(), Some(char) if char.is_ascii_digit()) {
                    value = 10 * value + input.next().unwrap() - b'0';
                }

                tokens.push(Token::Value(value));
            }
            _ => unreachable!(),
        }
    }

    replace_depths_with_lengths(&mut tokens);

    Signal(tokens)
}

fn replace_depths_with_lengths(tokens: &mut [Token]) {
    for i in 0..tokens.len() {
        let (head, tail) = tokens.split_at_mut(i + 1);

        if let Token::ArrayStart(depth) = &mut head[i] {
            *depth = tail
                .iter()
                .position(|token| matches!(token, Token::ArrayEnd(i) if i == depth))
                .unwrap() as u8
                + 1;
        }
    }
}

fn compare(mut left: &[Token], mut right: &[Token]) -> Ordering {
    while !left.is_empty() && !right.is_empty() {
        let (left_item, left_remainder) = take_item(left);
        let (right_item, right_remainder) = take_item(right);

        let ordering = compare_items(left_item, right_item);

        if ordering != Ordering::Equal {
            return ordering;
        }

        left = left_remainder;
        right = right_remainder;
    }

    if !left.is_empty() {
        Ordering::Greater
    } else if !right.is_empty() {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

fn take_item(tokens: &[Token]) -> (Item, &[Token]) {
    match &tokens[0] {
        Token::Value(value) => (Item::Value(*value), &tokens[1..]),
        Token::ArrayStart(length) => {
            let length = *length as usize;
            (Item::Array(&tokens[1..length]), &tokens[length + 1..])
        }
        _ => unreachable!(),
    }
}

fn compare_items(left: Item, right: Item) -> Ordering {
    match (left, right) {
        (Item::Array(left), Item::Array(right)) => compare(left, right),
        (Item::Array(left), Item::Value(right)) => {
            compare(left, std::slice::from_ref(&Token::Value(right)))
        }
        (Item::Value(left), Item::Array(right)) => {
            compare(std::slice::from_ref(&Token::Value(left)), right)
        }
        (Item::Value(left), Item::Value(right)) => left.cmp(&right),
    }
}
