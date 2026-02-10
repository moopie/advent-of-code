use std::fs::read_to_string;

fn main() {
    println!("AOC 2024 day 25!");

    let input = read_to_string("input.txt").unwrap();

    println!("Part 1: {}", solve_part_1(input.as_str()));
}

fn solve_part_1(input: &str) -> usize {
    let blocks: Vec<Vec<&str>> = input
        .trim()
        .split("\n\n")
        .map(|b| b.lines().map(str::trim).collect())
        .collect();

    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for block in blocks {
        if block[0].chars().all(|c| c == '#') {
            locks.push(parse_lock(&block));
        } else {
            keys.push(parse_key(&block));
        }
    }

    let mut count = 0;
    for lock in &locks {
        for key in &keys {
            if fits(lock, key) {
                count += 1;
            }
        }
    }

    count
}

fn parse_lock(block: &[&str]) -> [usize; 5] {
    let mut heights = [0; 5];
    for col in 0..5 {
        for row in 1..block.len() {
            if block[row].as_bytes()[col] == b'#' {
                heights[col] += 1;
            }
        }
    }
    heights
}

fn parse_key(block: &[&str]) -> [usize; 5] {
    let mut heights = [0; 5];
    for col in 0..5 {
        for row in (0..block.len() - 1).rev() {
            if block[row].as_bytes()[col] == b'#' {
                heights[col] += 1;
            }
        }
    }
    heights
}

fn fits(lock: &[usize; 5], key: &[usize; 5]) -> bool {
    (0..5).all(|i| lock[i] + key[i] <= 5)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"
    #####
    .####
    .####
    .####
    .#.#.
    .#...
    .....

    #####
    ##.##
    .#.##
    ...##
    ...#.
    ...#.
    .....

    .....
    #....
    #....
    #...#
    #.#.#
    #.###
    #####

    .....
    .....
    #.#..
    ###..
    ###.#
    ###.#
    #####

    .....
    .....
    .....
    #....
    #.#..
    #.#.#
    #####
    "#;

    #[test]
    fn part_1_should_be_3() {
        assert_eq!(3, solve_part_1(EXAMPLE));
    }
}
