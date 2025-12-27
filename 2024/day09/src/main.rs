use std::{error, fs::read_to_string};

fn defrag(input: String) -> i64 {
    let p = parse_line(input).expect("err");
    let mut ans = 0;
    let mut cursor = 0;
    for item in p {
        if let Some(val) = item {
            let current = cursor * val;
            cursor += 1;
            ans += current;
        }
    }

    ans
}

fn parse_line(input: String) -> Result<Vec<Option<i64>>, Box<dyn error::Error>> {
    let mut arr: Vec<Option<i64>> = vec![];

    let mut file_id = 0;

    for i in 0..input.len() {
        let is_space = i % 2 != 0;

        let n = input.chars().nth(i).unwrap_or_default();
        if n == '\n' {
            continue;
        }
        let num = n.to_string().parse::<i64>()?;
        let mut index = num;
        while index > 0 {
            if !is_space {
                arr.push(Some(file_id));
            } else {
                arr.push(None);
            }
            index -= 1;
        }

        if !is_space {
            file_id += 1;
        }
    }

    let mut left = 0;
    let mut right = arr.len() - 1;
    while left < right {
        if arr[left].is_some() {
            left += 1;
            continue;
        }

        if arr[right].is_none() {
            right -= 1;
            continue;
        }

        arr[left] = arr[right];
        arr[right] = None;

        left += 1;
        right -= 1;
    }

    Ok(arr)
}

fn main() {
    println!("AOC 2024 day 9!");

    let input = read_to_string("input.txt").expect("err");

    let part1_answer = defrag(input);

    println!("Part 1 answer: {}", part1_answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let input = "2333133121414131402".to_string();
        assert_eq!(defrag(input), 1928);
    }
}
