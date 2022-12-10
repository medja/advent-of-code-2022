use std::cmp::Ordering;

const CYCLES: [usize; 6] = [20, 60, 100, 140, 180, 220];
const SCREEN_WIDTH: usize = 40;
const SCREEN_HEIGHT: usize = 6;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut i = 0;
    let mut cycle = 0usize;
    let mut register = 1isize;
    let mut result = 0isize;

    for instruction in input {
        let value = register;
        execute(instruction, &mut cycle, &mut register);

        if cycle < CYCLES[i] {
            continue;
        }

        result += value * CYCLES[i] as isize;
        i += 1;

        if i == CYCLES.len() {
            break;
        }
    }

    Ok(result)
}

fn execute(instruction: &str, cycle: &mut usize, register: &mut isize) {
    match instruction.as_bytes()[0] {
        b'n' => {
            *cycle += 1;
        }
        b'a' => {
            *cycle += 2;
            *register += instruction[5..].parse::<isize>().unwrap();
        }
        _ => unreachable!(),
    }
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut screen = String::with_capacity(SCREEN_WIDTH * SCREEN_HEIGHT);
    let mut position = 1isize;

    for instruction in input {
        match instruction.as_bytes()[0] {
            b'n' => {
                draw(position, &mut screen);
            }
            b'a' => {
                draw(position, &mut screen);
                draw(position, &mut screen);
                position += instruction[5..].parse::<isize>()?;
            }
            _ => unreachable!(),
        }
    }

    for i in 0..SCREEN_HEIGHT {
        println!("{}", &screen[i * SCREEN_WIDTH..(i + 1) * SCREEN_WIDTH]);
    }

    Ok("picture")
}

fn draw(position: isize, screen: &mut String) {
    let i = (screen.len() % SCREEN_WIDTH) as isize;

    let visible = match i.cmp(&position) {
        Ordering::Less => i == position - 1,
        Ordering::Equal => true,
        Ordering::Greater => i == position + 1,
    };

    if visible {
        screen.push('#');
    } else {
        screen.push('.');
    }
}
