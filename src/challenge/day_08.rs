const VISITED_MASK: u8 = 1 << 7;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let trees = Vec::from_iter(input.iter().flat_map(|line| line.bytes()).map(|x| x - b'0'));
    Ok(TreeCounter::new(input.len(), trees).count())
}

struct TreeCounter {
    count: usize,
    max_height: u8,
    size: usize,
    trees: Vec<u8>,
}

impl TreeCounter {
    fn new(size: usize, trees: Vec<u8>) -> Self {
        TreeCounter {
            count: 4 * (size - 1),
            max_height: 0,
            size,
            trees,
        }
    }

    fn count(mut self) -> usize {
        for x in 1..self.size - 1 {
            self.max_height = self.trees[x];

            for y in 1..self.size - 1 {
                self.check_tree(x + y * self.size);
            }

            self.max_height = self.trees[x + (self.size - 1) * self.size];

            for y in (1..self.size - 1).rev() {
                self.check_tree(x + y * self.size);
            }
        }

        for y in 1..self.size - 1 {
            self.max_height = self.trees[y * self.size];

            for x in 1..self.size - 1 {
                self.check_tree(x + y * self.size);
            }

            self.max_height = self.trees[(self.size - 1) + (y * self.size)];

            for x in (1..self.size - 1).rev() {
                self.check_tree(x + y * self.size);
            }
        }

        self.count
    }

    fn check_tree(&mut self, index: usize) {
        let height = self.trees[index] & 0xf;

        if height > self.max_height {
            self.max_height = height;

            if self.trees[index] & VISITED_MASK != VISITED_MASK {
                self.trees[index] |= VISITED_MASK;
                self.count += 1;
            }
        }
    }
}
