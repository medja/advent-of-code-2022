use std::cmp::Ordering;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(CircularList::new(input, 1).decrypt(1))
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(CircularList::new(input, 811589153).decrypt(10))
}

struct Node {
    value: i64,
    next: usize,
    previous: usize,
}

struct CircularList(Vec<Node>);

impl CircularList {
    fn new(input: &[&str], decryption_key: i64) -> Self {
        let mut nodes = input
            .iter()
            .enumerate()
            .map(|(i, value)| Node {
                value: value.parse::<i64>().unwrap() * decryption_key,
                next: i + 1,
                previous: i.saturating_sub(1),
            })
            .collect::<Vec<_>>();

        let length = nodes.len();

        if let Some(first) = nodes.first_mut() {
            first.previous = length - 1;
        }

        if let Some(last) = nodes.last_mut() {
            last.next = 0;
        }

        CircularList(nodes)
    }

    fn decrypt(&mut self, iterations: usize) -> i64 {
        // When we move around numbers during decryption, the number being moved is not in the list
        let length = self.0.len() as i64 - 1;

        for _ in 0..iterations {
            for i in 0..self.0.len() {
                let i_next = self.0[i].next;
                let i_previous = self.0[i].previous;

                // Remove current number
                self.0[i_previous].next = i_next;
                self.0[i_next].previous = i_previous;

                let dest = self.find_next_node(i, self.0[i].value, length);
                let dest_next = self.0[dest].next;

                // Insert it at destination
                self.0[dest].next = i;
                self.0[i].previous = dest;
                self.0[i].next = dest_next;
                self.0[dest_next].previous = i;
            }
        }

        let index = self.0.iter().position(|node| node.value == 0).unwrap();
        let length = self.0.len() as i64;

        [1000, 2000, 3000]
            .iter()
            .map(|i| self.0[self.find_next_node(index, *i, length)].value)
            .sum()
    }

    fn find_next_node(&self, start: usize, count: i64, length: i64) -> usize {
        let count = self.normalize_move_count(count, length);

        match count.cmp(&0) {
            Ordering::Less => (count..0).fold(start, |destination, _| self.0[destination].previous),
            Ordering::Equal => self.0[start].previous,
            Ordering::Greater => (0..count).fold(start, |destination, _| self.0[destination].next),
        }
    }

    fn normalize_move_count(&self, mut count: i64, length: i64) -> i64 {
        count = count.rem_euclid(length);

        if count > length / 2 {
            count - self.0.len() as i64
        } else {
            count
        }
    }
}

impl std::fmt::Debug for CircularList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0[0].value)?;
        let mut next = self.0[0].next;

        while next != 0 {
            write!(f, ", {}", self.0[next].value)?;
            next = self.0[next].next;
        }

        Ok(())
    }
}
