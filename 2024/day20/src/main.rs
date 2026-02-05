use std::{collections::VecDeque, fs::read_to_string};

const INF: u64 = u64::MAX / 2;

fn main() {
    println!("AOC 2024 day 20!");

    let input = read_to_string("input.txt").unwrap();

    let part1 = solve_part1(input.as_str(), 100);

    println!("Part 1 solution: {}", part1);
}

fn solve_part1(input: &str, limit: u64) -> i64 {
    let (grid, start, end) = parse_input(input);

    let dist_s = distance(&grid, start);
    let dist_e = distance(&grid, end);

    let h = grid.len();
    let w = grid[0].len();

    let norm = dist_s[end.1][end.0];

    let mut count = 0;

    for y1 in 0..h {
        for x1 in 0..w {
            let d1 = dist_s[y1][x1];

            if d1 == INF {
                continue;
            }

            for dy in -2i32..=2 {
                for dx in -2i32..=2 {
                    let cheat = dx.abs() + dy.abs();
                    if cheat == 0 || cheat > 2 {
                        continue;
                    }

                    let x2 = x1 as i32 + dx;
                    let y2 = y1 as i32 + dy;

                    if x2 < 0 || y2 < 0 || x2 >= w as i32 || y2 >= h as i32 {
                        continue;
                    }

                    let x2 = x2 as usize;
                    let y2 = y2 as usize;

                    if grid[y2][x2] == '#' {
                        continue;
                    }

                    let d2 = dist_e[y2][x2];

                    if d2 == INF {
                        continue;
                    }

                    let cheated = d1 + cheat as u64 + d2;

                    if norm >= cheated + limit {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn distance(grid: &[Vec<char>], start: (usize, usize)) -> Vec<Vec<u64>> {
    let h = grid.len();
    let w = grid[0].len();

    let mut q = VecDeque::new();

    let mut dist = vec![vec![INF; w]; h];
    dist[start.1][start.0] = 0;

    q.push_front((start.0, start.1));

    while let Some((x, y)) = q.pop_front() {
        let cost = dist[y][x];

        for (dx, dy) in [(0, -1), (-1, 0), (1, 0), (0, 1)] {
            let cx = x as i32 + dx;
            let cy = y as i32 + dy;

            if cx < 0 || cy < 0 || cx >= w as i32 || cy >= h as i32 {
                continue;
            }

            let ux = cx as usize;
            let uy = cy as usize;

            if grid[uy][ux] == '#' {
                continue;
            }

            if dist[uy][ux] != INF {
                continue;
            }

            dist[uy][ux] = cost + 1;

            q.push_back((ux, uy));
        }
    }

    dist
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, (usize, usize), (usize, usize)) {
    let mut start = None;
    let mut end = None;
    let mut grid = Vec::new();

    for (y, l) in input.trim().lines().enumerate() {
        let mut line = Vec::new();
        for (x, c) in l.trim().chars().enumerate() {
            line.push(match c {
                'S' => {
                    start = Some((x, y));
                    '.'
                }
                'E' => {
                    end = Some((x, y));
                    '.'
                }
                _ => c,
            });
        }

        grid.push(line);
    }

    (grid, start.unwrap(), end.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"
    ###############
    #...#...#.....#
    #.#.#.#.#.###.#
    #S#...#.#.#...#
    #######.#.#.###
    #######.#.#...#
    #######.#.###.#
    ###..E#...#...#
    ###.#######.###
    #...###...#...#
    #.#####.#.###.#
    #.#...#.#.#...#
    #.#.#.#.#.#.###
    #...#...#...###
    ###############
    "#;

    #[test]
    fn part1_should_be_44() {
        let actual = solve_part1(EXAMPLE, 2);

        assert_eq!(44, actual);
    }
}
