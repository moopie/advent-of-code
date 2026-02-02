use std::{collections::HashMap, fs::read_to_string};

fn main() {
    println!("AOC 2024 day 19!");

    let input = read_to_string("input.txt").unwrap();

    let part1 = solve_part1(input.as_str());

    println!("Part 1 solution: {}", part1);
}

fn solve_part1(input: &str) -> i32 {
    let (patterns, designs) = parse_input(input);

    let mut sum = 0;
    for design in designs {
        let mut memo = HashMap::new();
        let count = count_builds(design, &patterns, &mut memo);
        if count > 0 {
            sum += 1;
        }
    }
    sum
}

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (p, d) = input.trim().split_once("\n\n").unwrap();

    let patterns = p.split(",").map(str::trim).collect();
    let designs = d.lines().map(str::trim).collect();

    (patterns, designs)
}

fn count_builds<'a>(design: &'a str, patterns: &[&str], memo: &mut HashMap<&'a str, u64>) -> u64 {
    if design.is_empty() {
        return 1;
    }

    if let Some(&v) = memo.get(design) {
        return v;
    }

    let mut total = 0;

    for &p in patterns {
        if let Some(rest) = design.strip_prefix(p) {
            total += count_builds(rest, patterns, memo);
        }
    }

    memo.insert(design, total);
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"
    r, wr, b, g, bwu, rb, gb, br

    brwrr
    bggr
    gbbr
    rrbgbr
    ubwu
    bwurrg
    brgr
    bbrgwb
    "#;

    #[test]
    fn part1_should_be_6() {
        let a = solve_part1(EXAMPLE);

        assert_eq!(6, a);
    }
}
