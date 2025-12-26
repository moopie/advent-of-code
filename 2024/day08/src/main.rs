use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn get_node_count(str_map: String) -> i64 {
    let map: Vec<Vec<char>> = str_map
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect();

    let height = map.len() as i64;
    let width = map[0].len() as i64;

    let mut freq_map: HashMap<char, Vec<(i64, i64)>> = HashMap::new();

    for y in 0..height {
        for x in 0..width {
            let ch = map[y as usize][x as usize];
            if ch != '.' {
                freq_map.entry(ch).or_default().push((x, y));
            }
        }
    }

    let mut antinodes: HashSet<(i64, i64)> = HashSet::new();

    for (_freq, pts) in freq_map {
        for i in 0..pts.len() {
            for j in (i + 1)..pts.len() {
                let (x1, y1) = pts[i];
                let (x2, y2) = pts[j];

                let a1 = (2 * x1 - x2, 2 * y1 - y2);
                let a2 = (2 * x2 - x1, 2 * y2 - y1);

                if a1.0 >= 0 && a1.0 < width && a1.1 >= 0 && a1.1 < height {
                    antinodes.insert(a1);
                }
                if a2.0 >= 0 && a2.0 < width && a2.1 >= 0 && a2.1 < height {
                    antinodes.insert(a2);
                }
            }
        }
    }

    antinodes.len() as i64
}

fn solve_part_2(input: &str) -> u32 {
    let map: Vec<Vec<char>> = input
        .to_string()
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect();

    let height = map.len() as i64;
    let width = map[0].len() as i64;

    let mut freq_map: HashMap<char, Vec<(i64, i64)>> = HashMap::new();

    for y in 0..height {
        for x in 0..width {
            let ch = map[y as usize][x as usize];
            if ch != '.' {
                freq_map.entry(ch).or_default().push((x, y));
            }
        }
    }

    let mut antinodes: HashSet<(i64, i64)> = HashSet::new();

    for (_freq, pts) in freq_map {
        for i in 0..pts.len() {
            for j in (i + 1)..pts.len() {
                let (x1, y1) = pts[i];
                let (x2, y2) = pts[j];

                let dx = x2 - x1;
                let dy = y2 - y1;
                let g = gcd(dx.abs(), dy.abs());

                let step_x = dx / g;
                let step_y = dy / g;

                let mut cx = x1;
                let mut cy = y1;

                while cx >= 0 && cx < width && cy >= 0 && cy < height {
                    antinodes.insert((cx, cy));
                    cx += step_x;
                    cy += step_y;
                }

                let mut cx = x1 - step_x;
                let mut cy = y1 - step_y;

                while cx >= 0 && cx < width && cy >= 0 && cy < height {
                    antinodes.insert((cx, cy));
                    cx -= step_x;
                    cy -= step_y;
                }
            }
        }
    }

    antinodes.len() as u32
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a.abs()
}

fn main() {
    println!("AOC 2024 day 8!");

    let input = read_to_string("input.txt").expect("err");
    let node_count = get_node_count(input.clone());

    println!("Part 1 solution: {}", node_count);

    let part2 = solve_part_2(input.as_str());

    println!("Part 2 solution: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let input = r#"
        ............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............
        "#;

        assert_eq!(get_node_count(input.to_string()), 14);
    }

    #[test]
    fn init_two_nodes() {
        let input = r#"
        ..........
        ..........
        ..........
        ....a.....
        ..........
        .....a....
        ..........
        ..........
        ..........
        ..........
        "#;

        assert_eq!(get_node_count(input.to_string()), 2);
    }

    #[test]
    fn init_three_nodes() {
        let input = r#"
        ..........
        ..........
        ..........
        ....a.....
        ........a.
        .....a....
        ..........
        ..........
        ..........
        ..........
        "#;

        assert_eq!(get_node_count(input.to_string()), 4);
    }

    #[test]
    fn part_2_first() {
        let input = r#"
        T.........
        ...T......
        .T........
        ..........
        ..........
        ..........
        ..........
        ..........
        ..........
        ..........
        "#;

        assert_eq!(solve_part_2(input), 9);
    }

    #[test]
    fn part_2_second() {
        let input = r#"
        ............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............
        "#;

        assert_eq!(solve_part_2(input), 34);
    }
}
