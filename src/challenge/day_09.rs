pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(simulate::<2>(input))
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(simulate::<10>(input))
}

#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
struct Position {
    x: i16,
    y: i16,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn simulate<const N: usize>(input: &[&str]) -> usize {
    let mut knots = [Position::default(); N];
    let mut positions = vec![Position::default()];

    for line in input {
        let direction = match line.as_bytes()[0] {
            b'U' => Direction::Up,
            b'D' => Direction::Down,
            b'L' => Direction::Left,
            b'R' => Direction::Right,
            _ => unreachable!(),
        };

        for _ in 0..line[2..].parse::<i16>().unwrap() {
            match direction {
                Direction::Up => knots[0].y += 1,
                Direction::Down => knots[0].y -= 1,
                Direction::Left => knots[0].x -= 1,
                Direction::Right => knots[0].x += 1,
            }

            for i in 1..N {
                let previous = knots[i - 1];
                let current = &mut knots[i];

                if previous.x.abs_diff(current.x) > 1 || previous.y.abs_diff(current.y) > 1 {
                    current.x += (previous.x - current.x).signum();
                    current.y += (previous.y - current.y).signum();

                    if i + 1 == N {
                        positions.push(*current);
                    }
                }
            }
        }
    }

    positions.sort();
    positions.dedup_by(|prev, next| prev == next);
    positions.len()
}
