use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    println!("AOC 2024 day 22!");

    let input = read_to_string("input.txt").unwrap();

    println!("Part 1: {}", solve_part_1(input.as_str()));
    println!("Part 2: {}", solve_part_2(input.as_str()));
}

fn solve_part_1(input: &str) -> i64 {
    let mut numbers = parse_input(input);

    for num in &mut numbers {
        let mut n = *num;
        for _ in 0..2_000 {
            n = calculate_secret(n);
        }

        *num = n;
    }
    numbers.iter().sum()
}

fn solve_part_2(input: &str) -> i64 {
    let numbers = parse_input(input);

    let mut profit: HashMap<(i8, i8, i8, i8), i64> = HashMap::new();

    for &start in &numbers {
        let mut secret = start;

        let mut prices = [0i8; 2000];
        for p in prices.iter_mut() {
            secret = calculate_secret(secret);
            *p = (secret % 10) as i8;
        }

        let mut seen: HashSet<(i8, i8, i8, i8)> = HashSet::new();

        for i in 0..prices.len() - 4 {
            let a = prices[i];
            let b = prices[i + 1];
            let c = prices[i + 2];
            let d = prices[i + 3];
            let e = prices[i + 4];

            let pattern = (b - a, c - b, d - c, e - d);

            if seen.insert(pattern) {
                *profit.entry(pattern).or_insert(0) += e as i64;
            }
        }
    }

    *profit.values().max().unwrap_or(&0)
}

fn parse_input(input: &str) -> Vec<i64> {
    input
        .trim()
        .lines()
        .filter(|x| !x.trim().is_empty())
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

fn calculate_secret(mut num: i64) -> i64 {
    num = num ^ (num * 64);
    num %= 16777216;

    num = num ^ (num / 32);
    num %= 16777216;

    num = num ^ (num * 2048);
    num %= 16777216;

    num
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = r#"
    1
    10
    100
    2024
    "#;

    const EXAMPLE_2: &str = r#"
    1
    2
    3
    2024
    "#;

    #[test]
    fn part_1_should_be_37327623() {
        let actual = solve_part_1(EXAMPLE_1);

        assert_eq!(37327623, actual);
    }

    #[test]
    fn part_2_should_be_23() {
        let actual = solve_part_2(EXAMPLE_2);

        assert_eq!(23, actual);
    }
}
