const VISITED_MASK: u8 = 1 << 7;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(TreeCounter::new(input).count())
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(SpotSelector::new(input).select(6))
}

fn parse_grid(input: &[&str]) -> Vec<u8> {
    Vec::from_iter(input.iter().flat_map(|line| line.bytes()).map(|x| x - b'0'))
}

struct TreeCounter {
    count: usize,
    max_height: u8,
    size: usize,
    trees: Vec<u8>,
}

impl TreeCounter {
    fn new(input: &[&str]) -> Self {
        TreeCounter {
            count: 4 * (input.len() - 1),
            max_height: 0,
            size: input.len(),
            trees: parse_grid(input),
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

struct SpotSelector {
    size: usize,
    trees: Vec<u8>,
}

impl SpotSelector {
    fn new(input: &[&str]) -> Self {
        let grid = parse_grid(input);

        SpotSelector {
            size: input.len(),
            trees: grid,
        }
    }

    fn select(self, min_height: u8) -> usize {
        let mut best_score = 0usize;

        for x in 1..(self.size - 1) {
            for y in 1..(self.size - 1) {
                let height = self.trees[x + y * self.size];

                if height < min_height {
                    continue;
                }

                let up = (0..y)
                    .rev()
                    .position(|i| self.trees[x + i * self.size] >= height)
                    .map(|i| i + 1)
                    .unwrap_or(y);

                let down = (y + 1..self.size)
                    .position(|i| self.trees[x + i * self.size] >= height)
                    .map(|i| i + 1)
                    .unwrap_or(self.size - y - 1);

                let left = (0..x)
                    .rev()
                    .position(|i| self.trees[i + y * self.size] >= height)
                    .map(|i| i + 1)
                    .unwrap_or(x);

                let right = (x + 1..self.size)
                    .position(|i| self.trees[i + y * self.size] >= height)
                    .map(|i| i + 1)
                    .unwrap_or(self.size - x - 1);

                let score = up * down * left * right;

                if score > best_score {
                    best_score = score;
                }
            }
        }

        best_score
    }
}
