use std::{collections::VecDeque, fs::read_to_string};

fn main() {
    println!("AOC 2024 day 18!");

    let input = read_to_string("input.txt").unwrap();

    let part1 = solve_part1(input.as_str(), 71, 1024);

    println!("Part 1 solution: {}", part1);

    let part2 = solve_part2(input.as_str(), 71, 1024);

    println!("Part 2 solution: {}", part2);
}

fn solve_part2(input: &str, size: usize, _unused: usize) -> String {
    let corrupt = parse_input(input);

    let mut lo = 0usize;
    let mut hi = corrupt.len();

    while lo < hi {
        let mid = (lo + hi) / 2;

        let dist = solve_part1(input, size, mid);

        if dist != 0 {
            // still reachable → need MORE corruption
            lo = mid + 1;
        } else {
            // unreachable → too far
            hi = mid;
        }
    }

    // lo is the FIRST index that breaks the path
    let (x, y) = corrupt[lo - 1];
    format!("{},{}", x, y)
}

fn solve_part1(input: &str, size: usize, limit: usize) -> i32 {
    let corrupt = parse_input(input);
    let mut map = vec![vec![false; size]; size];

    for &(x, y) in corrupt.iter().take(limit) {
        map[y][x] = true;
    }

    let mut dq = VecDeque::new();
    let mut visited = vec![vec![false; size]; size];

    dq.push_back((0, (0usize, 0usize)));

    while let Some((cost, (x, y))) = dq.pop_front() {
        if (x, y) == (size - 1, size - 1) {
            return cost;
        }

        if visited[y][x] {
            continue;
        }

        visited[y][x] = true;

        for (dx, dy) in [(0, -1), (-1, 0), (1, 0), (0, 1)] {
            let cx = x as i32 + dx;
            let cy = y as i32 + dy;

            if cx < 0 || cy < 0 || cy as usize >= map.len() || cx as usize >= map[cy as usize].len()
            {
                continue;
            }

            if !map[cy as usize][cx as usize] {
                dq.push_back((cost + 1, (cx as usize, cy as usize)));
            }
        }
    }
    0
}

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let (x, y) = line.trim().split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"
    5,4
    4,2
    4,5
    3,0
    2,1
    6,3
    2,4
    1,5
    0,6
    3,3
    2,6
    5,1
    1,2
    5,5
    2,5
    6,5
    1,4
    0,4
    6,4
    1,1
    6,1
    1,0
    0,5
    1,6
    2,0
    "#;

    #[test]
    fn part1_should_be_22() {
        let ret = solve_part1(EXAMPLE, 7, 12);

        assert_eq!(22, ret);
    }
}
