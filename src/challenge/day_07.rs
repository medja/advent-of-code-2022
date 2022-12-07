use std::collections::HashMap;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let result = compute_directory_sizes(input)?
        .values()
        .filter(|size| **size <= 100000)
        .sum::<usize>();

    Ok(result)
}

fn compute_directory_sizes(input: &[&str]) -> anyhow::Result<HashMap<String, usize>> {
    let mut working_directory = Vec::with_capacity(16);
    let mut directory_sizes = HashMap::<String, usize>::new();

    for line in input {
        if line.starts_with(|char: char| char.is_ascii_digit()) {
            let size = line
                .split_ascii_whitespace()
                .next()
                .unwrap()
                .parse::<usize>()?;

            for i in 0..=working_directory.len() {
                *directory_sizes
                    .entry(build_path(&working_directory[..i]))
                    .or_default() += size;
            }
        } else if line.starts_with("$ cd") {
            change_directory(&mut working_directory, &line[5..]);
        }
    }

    Ok(directory_sizes)
}

fn build_path(segments: &[&str]) -> String {
    let length = segments.iter().map(|segment| segment.len() + 1).sum();

    let mut path = String::with_capacity(length);

    for segment in segments {
        path.push('/');
        path.push_str(segment);
    }

    path
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
