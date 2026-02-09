use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;

const KEYPAD: [[Option<char>; 3]; 4] = [
    [Some('7'), Some('8'), Some('9')],
    [Some('4'), Some('5'), Some('6')],
    [Some('1'), Some('2'), Some('3')],
    [None, Some('0'), Some('A')],
];

const ARROW_KEYS: [[Option<char>; 3]; 2] = [
    [None, Some('^'), Some('A')],
    [Some('<'), Some('v'), Some('>')],
];

fn main() {
    let input = read_to_string("input.txt").unwrap();
    println!("Part 1: {}", solve_part_1(&input));
    println!("Part 2: {}", solve_part_2(&input));
}

fn parse_input(input: &str) -> Vec<String> {
    input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(String::from)
        .collect()
}

fn find_pos<const H: usize, const W: usize>(
    pad: &[[Option<char>; W]; H],
    target: char,
) -> (i32, i32) {
    for y in 0..H {
        for x in 0..W {
            if pad[y][x] == Some(target) {
                return (x as i32, y as i32);
            }
        }
    }
    panic!("key not found: {target}");
}

fn bfs<const H: usize, const W: usize>(
    pad: &[[Option<char>; W]; H],
    from: char,
    to: char,
) -> Vec<String> {
    let (sx, sy) = find_pos(pad, from);
    let (tx, ty) = find_pos(pad, to);

    let mut q = VecDeque::new();
    let mut best: HashMap<(i32, i32), usize> = HashMap::new();
    let mut out = Vec::new();

    q.push_back((sx, sy, String::new()));
    best.insert((sx, sy), 0);

    while let Some((x, y, path)) = q.pop_front() {
        let d = path.len();

        if (x, y) == (tx, ty) {
            out.push(format!("{path}A"));
            continue;
        }

        for (dx, dy, c) in [(0, -1, '^'), (0, 1, 'v'), (-1, 0, '<'), (1, 0, '>')] {
            let nx = x + dx;
            let ny = y + dy;

            if nx < 0 || ny < 0 || nx >= W as i32 || ny >= H as i32 {
                continue;
            }
            if pad[ny as usize][nx as usize].is_none() {
                continue;
            }

            let nd = d + 1;
            if best.get(&(nx, ny)).map_or(true, |&v| nd <= v) {
                best.insert((nx, ny), nd);
                let mut p = path.clone();
                p.push(c);
                q.push_back((nx, ny, p));
            }
        }
    }

    out
}

fn cost(seq: &str, depth: usize, memo: &mut HashMap<(String, usize), u64>) -> u64 {
    if depth == 0 {
        return seq.len() as u64;
    }

    if let Some(&v) = memo.get(&(seq.to_string(), depth)) {
        return v;
    }

    let mut total = 0;
    let mut cur = 'A';

    for ch in seq.chars() {
        let paths = if is_numeric_seq(seq) {
            bfs(&KEYPAD, cur, ch)
        } else {
            bfs(&ARROW_KEYS, cur, ch)
        };

        let best = paths
            .iter()
            .map(|p| cost(p, depth - 1, memo))
            .min()
            .unwrap();

        total += best;
        cur = ch;
    }

    memo.insert((seq.to_string(), depth), total);
    total
}

fn is_numeric_seq(seq: &str) -> bool {
    seq.chars().any(|c| c.is_ascii_digit())
}

fn solve_part_1(input: &str) -> u32 {
    let lines = parse_input(input);
    let mut memo = HashMap::new();

    lines
        .iter()
        .map(|line| {
            let n: u64 = line[..line.len() - 1].parse().unwrap();
            let presses = cost(line, 3, &mut memo);
            n * presses
        })
        .sum::<u64>() as u32
}

fn solve_part_2(input: &str) -> u64 {
    let lines = parse_input(input);

    lines
        .iter()
        .map(|line| {
            let mut memo = HashMap::new();
            let n: u64 = line[..line.len() - 1].parse().unwrap();
            let presses = cost(line, 26, &mut memo);
            n * presses
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"
    029A
    980A
    179A
    456A
    379A
    "#;

    #[test]
    fn part_1_should_be_126384() {
        assert_eq!(126384, solve_part_1(EXAMPLE));
    }

    #[test]
    fn part_2_zhould_be_154115708116294() {
        assert_eq!(154115708116294, solve_part_2(EXAMPLE));
    }
}
