use std::fs::read_to_string;

fn main() {
    println!("AOC 2024 day 22!");

    let input = read_to_string("input.txt").unwrap();

    println!("Part 1: {}", solve_part_1(input.as_str()));
}

fn solve_part_1(input: &str) -> i64 {
    let mut numbers = parse_input(input);

    for num in &mut numbers {
        let mut n = *num;
        for _ in 0..2000 {
            n = calculate_secret(n);
        }

        *num = n;
    }
    numbers.iter().sum()
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

    const EXAMPLE: &str = r#"
    1
    10
    100
    2024
    "#;

    #[test]
    fn part_1_should_be_37327623() {
        let actual = solve_part_1(EXAMPLE);

        assert_eq!(37327623, actual);
    }
}
