use std::collections::HashMap;

const ROOT: &str = "root";
const HUMAN: &str = "humn";

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(evaluate(ROOT, false, &parse_monkeys(input)).unwrap())
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let monkeys = parse_monkeys(input);

    let mut iterator = monkeys[ROOT].split_ascii_whitespace();
    let left = iterator.next().unwrap();
    let right = iterator.nth(1).unwrap();

    let left_result = evaluate(left, true, &monkeys);
    let right_result = evaluate(right, true, &monkeys);

    let solution = match (left_result, right_result) {
        (Some(result), _) => solve(right, result, &monkeys),
        (_, Some(result)) => solve(left, result, &monkeys),
        _ => unreachable!(),
    };

    Ok(solution)
}

fn parse_monkeys<'a>(input: &[&'a str]) -> HashMap<&'a str, &'a str> {
    input
        .iter()
        .map(|line| (&line[..4], &line[6..]))
        .collect::<HashMap<_, _>>()
}

fn evaluate(name: &str, human: bool, monkeys: &HashMap<&str, &str>) -> Option<u64> {
    if human && name == HUMAN {
        return None;
    }

    let value = monkeys[name];

    if value.as_bytes()[0].is_ascii_digit() {
        return Some(value.parse().unwrap());
    }

    let mut iterator = value.split_ascii_whitespace();

    let left = evaluate(iterator.next().unwrap(), human, monkeys)?;
    let operator = iterator.next().unwrap().as_bytes()[0];
    let right = evaluate(iterator.next().unwrap(), human, monkeys)?;

    let result = match operator {
        b'+' => left + right,
        b'-' => left - right,
        b'*' => left * right,
        b'/' => left / right,
        _ => unreachable!(),
    };

    Some(result)
}

fn solve(name: &str, result: u64, monkeys: &HashMap<&str, &str>) -> u64 {
    if name == HUMAN {
        return result;
    }

    let mut iterator = monkeys[name].split_ascii_whitespace();

    let left = iterator.next().unwrap();
    let operator = iterator.next().unwrap().as_bytes()[0];
    let right = iterator.next().unwrap();

    if let Some(value) = evaluate(left, true, monkeys) {
        solve(right, solve_right(value, result, operator), monkeys)
    } else if let Some(value) = evaluate(right, true, monkeys) {
        solve(left, solve_left(value, result, operator), monkeys)
    } else {
        unreachable!()
    }
}

fn solve_left(value: u64, result: u64, operator: u8) -> u64 {
    match operator {
        b'+' => result - value,
        b'-' => result + value,
        b'*' => result / value,
        b'/' => result * value,
        _ => unreachable!(),
    }
}

fn solve_right(value: u64, result: u64, operator: u8) -> u64 {
    match operator {
        b'+' => result - value,
        b'-' => value - result,
        b'*' => result / value,
        b'/' => value / result,
        _ => unreachable!(),
    }
}
