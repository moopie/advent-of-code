use std::collections::HashSet;
use std::fs::read_to_string;

fn get_node_count(str_map: String) -> i64 {
    let lines = str_map.lines();

    let freqs: HashSet<char> = str_map.chars()
        // remove empty space
        .filter(|x|
            *x != '.' &&
            *x != '\n' &&
            *x != ' ')
        .collect();

    let map: Vec<Vec<char>> = lines
        .map(|x| x.to_string().trim().chars().collect())
        .filter(|x: &Vec<char>| !x.is_empty())
        .collect();

    let lenx = map[0].len() as i64;
    let leny = map.len() as i64;
    let mut set: HashSet<(i64, i64)> = HashSet::new();
    let mut freq_locs: Vec<(i64, i64, char)> = vec![];

    for freq in freqs {
        for x  in 0..map.len() {
            for y in 0..map[x].len() {
                if freq == map[y][x] {
                    freq_locs.push((x as i64, y as i64, freq.clone()));
                }
            }
        }
    }

    for (x, (x1, y1, _)) in freq_locs.iter().enumerate() {
        // the set of other frequency towers

        for y in x..freq_locs.len() {
            let (x2, y2, _) = freq_locs[y];

            if *x1 == x2 && *y1 == y2 {
                continue;
            }

            let dx = (x2 - x1).abs();
            let dy = (y2 - y1).abs();

            let ax = x1 - dx;
            let ay = y1 - dy;

            let bx = x2 + dx;
            let by = y2 + dy;

            if ax >= 0 && ax < lenx && ay >= 0 && ay < leny {
                set.insert((ax, ay));
            }
            if bx >= 0 && bx < lenx && by >= 0 && by < leny {
                set.insert((bx, by));
            }
        }
    }
    println!("\nset: {:?}", set);
    set.len() as i64
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