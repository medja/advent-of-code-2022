pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(solve(input[0].as_bytes(), 4))
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(solve(input[0].as_bytes(), 14))
}

fn solve(sequence: &[u8], length: usize) -> usize {
    let result = sequence
        .windows(length)
        .position(|bytes| {
            bytes
                .iter()
                .map(|byte| 1u32 << (byte - b'a'))
                .fold(0u32, |acc, value| acc | value)
                .count_ones() as usize == length
        })
        .unwrap();

    result + length
}
