use std::{error, fs::read_to_string};

fn defrag(input: String) -> i64 {
    let p = parse_line(input, format_part1).expect("err");
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

fn defrag_part2(input: String) -> i64 {
    let p = parse_line(input, format_part2).expect("err");
    checksum(&p)
}

fn checksum(arr: &[Option<i64>]) -> i64 {
    arr.iter()
        .enumerate()
        .filter_map(|(pos, v)| v.map(|id| pos as i64 * id))
        .sum()
}

fn find_file(arr: &[Option<i64>], file_id: i64) -> Option<(usize, usize)> {
    let mut start = None;

    for i in 0..arr.len() {
        if arr[i] == Some(file_id) {
            if start.is_none() {
                start = Some(i);
            }
        } else if start.is_some() {
            return Some((start.unwrap(), i)); // [start, end)
        }
    }

    start.map(|s| (s, arr.len()))
}

fn find_free_span(arr: &[Option<i64>], max_end: usize, size: usize) -> Option<usize> {
    let mut count = 0;

    for i in 0..max_end {
        if arr[i].is_none() {
            count += 1;
            if count == size {
                return Some(i + 1 - size);
            }
        } else {
            count = 0;
        }
    }

    None
}

fn move_file(arr: &mut Vec<Option<i64>>, from: usize, to: usize, size: usize, id: i64) {
    for i in 0..size {
        arr[to + i] = Some(id);
        arr[from + i] = None;
    }
}

fn format_part2(mut arr: Vec<Option<i64>>) -> Vec<Option<i64>> {
    let max_id = arr.iter().filter_map(|x| *x).max().unwrap();

    for file_id in (0..=max_id).rev() {
        let (start, end) = match find_file(&arr, file_id) {
            Some(v) => v,
            None => continue,
        };

        let size = end - start;

        if let Some(target) = find_free_span(&arr, start, size) {
            move_file(&mut arr, start, target, size, file_id);
        }
    }

    arr
}

fn format_part1(mut arr: Vec<Option<i64>>) -> Vec<Option<i64>> {
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
    arr
}

fn parse_line(
    input: String,
    formatter: fn(Vec<Option<i64>>) -> Vec<Option<i64>>,
) -> Result<Vec<Option<i64>>, Box<dyn error::Error>> {
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

    Ok(formatter(arr))
}

fn main() {
    println!("AOC 2024 day 9!");

    let input = read_to_string("input.txt").expect("err");

    let part1_answer = defrag(input.clone());

    println!("Part 1 answer: {}", part1_answer);

    let part2_answer = defrag_part2(input);

    println!("Part 2 answer: {}", part2_answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let input = "2333133121414131402".to_string();
        assert_eq!(defrag(input), 1928);
    }

    #[test]
    fn init_p2() {
        let input = "2333133121414131402".to_string();
        assert_eq!(defrag_part2(input), 2858);
    }
}
