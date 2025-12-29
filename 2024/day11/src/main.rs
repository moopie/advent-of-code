use std::{collections::HashMap, fs::read_to_string, vec};

fn blink(stones: Vec<u64>) -> Vec<u64> {
    let mut nstones = vec![];

    for &stone in stones.iter() {
        if stone == 0 {
            nstones.push(1);
            continue;
        }

        let len = number_of_digits(stone);
        if len % 2 == 0 {
            let s = stone.to_string();
            let idx = len / 2;

            let (first, second) = s.split_at(idx as usize);
            let fnum = first.parse::<u64>().expect("err");
            let snum = second.parse::<u64>().expect("err");
            nstones.push(fnum);
            nstones.push(snum);
        } else {
            nstones.push(stone * 2024);
        }
    }

    nstones
}

fn number_of_digits(num: u64) -> u64 {
    if num == 0 { 1 } else { num.ilog10() as u64 + 1 }
}

fn blink_counts(stones: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut next = HashMap::new();

    for (stone, count) in stones {
        if stone == 0 {
            *next.entry(1).or_insert(0) += count;
            continue;
        }

        let len = number_of_digits(stone);

        if len % 2 == 0 {
            let s = stone.to_string();
            let mid = s.len() / 2;

            let left = s[..mid].parse::<u64>().unwrap();
            let right = s[mid..].parse::<u64>().unwrap();

            *next.entry(left).or_insert(0) += count;
            *next.entry(right).or_insert(0) += count;
        } else {
            *next.entry(stone * 2024).or_insert(0) += count;
        }
    }

    next
}

fn solve_p1(input: &str) -> u64 {
    let stones = parse_input(input);
    let mut once = blink(stones);
    for _ in 0..24 {
        once = blink(once);
    }

    once.len() as u64
}

fn solve_p2(input: &str) -> u64 {
    let mut stones = HashMap::<u64, u64>::new();

    for n in input.trim().split_whitespace() {
        let val = n.parse::<u64>().expect("err");
        *stones.entry(val).or_insert(0) += 1;
    }
    for _ in 0..75 {
        stones = blink_counts(stones);
    }

    stones.values().sum()
}

fn parse_input(input: &str) -> Vec<u64> {
    let mut res = vec![];

    let nums: Vec<_> = input.trim().split_whitespace().collect();

    for n in nums.iter() {
        res.push(n.parse::<u64>().expect("not a num"));
    }

    res
}

fn main() {
    println!("AOC 2024 day 11!");

    let input = read_to_string("input.txt").expect("err");

    let p1_ans = solve_p1(input.as_str());
    println!("Part 1 solution: {}", p1_ans);

    let p2_ans = solve_p2(input.as_str());
    println!("Part 2 solution: {}", p2_ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = r#"
        125 17
        "#;

        assert_eq!(solve_p1(input), 55312);
    }
}
