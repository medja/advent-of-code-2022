use std::collections::VecDeque;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut acc = 0;
    let mut exhausted;
    let mut result = VecDeque::new();

    let mut numbers = input
        .iter()
        .map(|number| number.bytes().rev())
        .collect::<Vec<_>>();

    loop {
        exhausted = true;

        for number in &mut numbers {
            let digit = match number.next() {
                Some(digit) => digit,
                None => continue,
            };

            acc += parse_snafu(digit);
            exhausted = false;
        }

        if acc == 0 && exhausted {
            break;
        }

        let delta = match acc {
            _ if acc > 2 => 2,
            _ if acc < -2 => -2,
            _ => 0,
        };

        acc += delta;
        let carry = acc / 5;
        acc = acc % 5 - delta;
        result.push_front(format_snafu(acc));
        acc = carry;
    }

    Ok(std::str::from_utf8(result.make_contiguous())?.to_string())
}

fn parse_snafu(digit: u8) -> isize {
    match digit {
        b'2' => 2,
        b'1' => 1,
        b'0' => 0,
        b'-' => -1,
        b'=' => -2,
        _ => unreachable!(),
    }
}

fn format_snafu(digit: isize) -> u8 {
    match digit {
        2 => b'2',
        1 => b'1',
        0 => b'0',
        -1 => b'-',
        -2 => b'=',
        _ => unreachable!(),
    }
}
