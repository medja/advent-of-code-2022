pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let result = input
        .iter()
        .map(|line| {
            let items = line.as_bytes();
            let length = items.len() / 2;
            [
                Rucksack::new(&items[..length]),
                Rucksack::new(&items[length..]),
            ]
            .score()
        })
        .sum::<usize>();

    Ok(result)
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let result = input
        .chunks_exact(3)
        .map(|chunk| {
            std::array::from_fn::<_, 3, _>(|index| Rucksack::new(chunk[index].as_bytes())).score()
        })
        .sum::<usize>();

    Ok(result)
}

struct Rucksack(u64);

impl Rucksack {
    fn new(items: &[u8]) -> Self {
        let mut flags = 0;

        for &item in items {
            let index = if item >= b'a' {
                item - b'a'
            } else {
                item - b'A' + 26
            };

            flags |= 1 << index;
        }

        Rucksack(flags)
    }
}

trait Score {
    fn score(&self) -> usize;
}

impl<const N: usize> Score for [Rucksack; N] {
    fn score(&self) -> usize {
        (0..52)
            .filter(|index| {
                let mask = 1 << *index;
                self.iter().all(|item| item.0 & mask == mask)
            })
            .map(|index| index + 1)
            .sum::<usize>()
    }
}
