const CYCLES: [usize; 6] = [20, 60, 100, 140, 180, 220];

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
