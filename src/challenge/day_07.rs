pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let result = compute_directory_sizes(input)
        .into_iter()
        .filter(|size| *size <= 100000)
        .sum::<usize>();

    Ok(result)
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let directories = compute_directory_sizes(input);
    let min_size = directories[0] - 40000000; // (+ 30000000 - 70000000)

    let result = directories
        .into_iter()
        .filter(|size| *size >= min_size)
        .min()
        .unwrap();

    Ok(result)
}

// Turns out the commands do a DFS traversal of the file system
fn compute_directory_sizes(input: &[&str]) -> Vec<usize> {
    let mut working_directory = Vec::with_capacity(16);
    let mut directory_sizes = Vec::with_capacity(200);

    for line in input {
        if line.starts_with(|char: char| char.is_ascii_digit()) {
            let size = line.split_once(' ').unwrap().0.parse::<usize>().unwrap();

            for &i in &working_directory {
                directory_sizes[i] += size;
            }
        } else if line.starts_with("$ cd") {
            if &line[5..] == ".." {
                working_directory.pop();
            } else {
                working_directory.push(directory_sizes.len());
                directory_sizes.push(0);
            }
        }
    }

    directory_sizes
}
