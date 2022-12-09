pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut head = Position { x: 0, y: 0 };
    let mut tail = Position { x: 0, y: 0 };
    let mut positions = vec![tail];

    for line in input {
        let distance = line[2..].parse::<i16>()?;

        match line.as_bytes()[0] {
            b'U' => head.y += distance,
            b'D' => head.y -= distance,
            b'L' => head.x -= distance,
            b'R' => head.x += distance,
            _ => unreachable!(),
        }

        while head.x.abs_diff(tail.x) > 1 || head.y.abs_diff(tail.y) > 1 {
            tail.x += (head.x - tail.x).signum();
            tail.y += (head.y - tail.y).signum();
            positions.push(tail);
        }
    }

    positions.sort();
    positions.dedup_by(|prev, next| prev == next);

    Ok(positions.len())
}

#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
struct Position {
    x: i16,
    y: i16,
}
