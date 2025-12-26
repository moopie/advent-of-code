use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn get_node_count(str_map: String) -> i64 {
    // Build grid
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

fn main() {
    println!("AOC 2024 day 8!");

    let input = read_to_string("input.txt").expect("err");
    let node_count = get_node_count(input);

    println!("Part 1 solution: {}", node_count);
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
}
