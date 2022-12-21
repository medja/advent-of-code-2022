use std::collections::HashMap;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let monkeys = input
        .iter()
        .map(|line| (&line[..4], &line[6..]))
        .collect::<HashMap<_, _>>();

    Ok(evaluate("root", &monkeys))
}

fn evaluate(name: &str, monkeys: &HashMap<&str, &str>) -> i64 {
    let value = monkeys[name];

    if value.as_bytes()[0].is_ascii_digit() {
        return value.parse().unwrap();
    }

    let mut iterator = value.split_ascii_whitespace();

    let left = evaluate(iterator.next().unwrap(), monkeys);
    let operator = iterator.next().unwrap().as_bytes()[0];
    let right = evaluate(iterator.next().unwrap(), monkeys);

    match operator {
        b'+' => left + right,
        b'-' => left - right,
        b'*' => left * right,
        b'/' => left / right,
        _ => unreachable!(),
    }
}
