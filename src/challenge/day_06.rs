pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(solve(input[0].as_bytes(), 4))
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(solve(input[0].as_bytes(), 14))
}

fn solve(sequence: &[u8], length: usize) -> usize {
    let result = sequence
        .windows(length)
        .enumerate()
        .find(|(_, bytes)| {
            bytes
                .iter()
                .enumerate()
                .all(|(i, x)| bytes[i + 1..].iter().all(|y| x != y))
        })
        .map(|(i, _)| i)
        .unwrap();

    result + length
}
