use std::collections::HashMap;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let result = compute_directory_sizes(input)?
        .values()
        .filter(|size| **size <= 100000)
        .sum::<usize>();

    Ok(result)
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let root: &[&str] = &[];
    let directories = compute_directory_sizes(input)?;
    let min_size = directories.get(root).unwrap() - 40000000; // (+ 30000000 - 70000000)

    let result = *directories
        .values()
        .filter(|size| **size >= min_size)
        .min()
        .unwrap();

    Ok(result)
}

fn compute_directory_sizes<'a>(input: &[&'a str]) -> anyhow::Result<HashMap<Vec<&'a str>, usize>> {
    let mut working_directory = Vec::with_capacity(16);
    let mut directory_sizes = HashMap::new();

    for line in input {
        if line.starts_with(|char: char| char.is_ascii_digit()) {
            let size = line
                .split_ascii_whitespace()
                .next()
                .unwrap()
                .parse::<usize>()?;

            for i in 0..=working_directory.len() {
                match directory_sizes.get_mut(&working_directory[..i]) {
                    Some(sum) => *sum += size,
                    None => {
                        directory_sizes.insert(working_directory[..i].to_vec(), size);
                    }
                }
            }
        } else if line.starts_with("$ cd") {
            change_directory(&mut working_directory, &line[5..]);
        }
    }

    Ok(directory_sizes)
}

fn change_directory<'a>(working_directory: &mut Vec<&'a str>, mut path: &'a str) {
    if path.starts_with('/') {
        working_directory.clear();
        path = &path[1..];
    }

    if path.is_empty() {
        return;
    }

    for segment in path.split('/') {
        if segment == ".." {
            working_directory.pop();
        } else {
            working_directory.push(segment);
        }
    }
}
