pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(input.iter().map(|line| Rucksack::new(line.as_bytes()).score()).sum::<usize>())
}

struct Compartment(u64);

impl Compartment {
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

        Compartment(flags)
    }

    fn score(&self, other: &Self) -> usize {
        (0..52)
            .filter(|index| {
                let mask = 1 << *index;
                self.0 & mask == mask && other.0 & mask == mask
            })
            .map(|index| index + 1)
            .sum::<usize>()
    }
}

struct Rucksack(Compartment, Compartment);

impl Rucksack {
    fn new(items: &[u8]) -> Self {
        let length = items.len() / 2;
        Rucksack(Compartment::new(&items[..length]), Compartment::new(&items[length..]))
    }

    fn score(&self) -> usize {
        self.0.score(&self.1)
    }
}
