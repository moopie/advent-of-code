use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;

struct Region {
    name: char,
    area: usize,
    perimeter: usize,
    sides: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    U,
    D,
    L,
    R,
}

fn solve_p1(input: &str) -> u32 {
    let mut score = 0;
    let fields = parse_map(input);
    let regions = get_regions(fields);

    for r in regions.iter() {
        score += r.area * r.perimeter;
    }

    score as u32
}

fn solve_p2(input: &str) -> u32 {
    let mut score = 0;
    let fields = parse_map(input);
    let regions = get_regions(fields);

    for r in regions.iter() {
        score += r.area * r.sides;
    }

    score as u32
}

fn parse_map(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .split_whitespace()
        .map(|x| x.chars().collect())
        .collect()
}

fn get_regions(fields: Vec<Vec<char>>) -> Vec<Region> {
    let rows = fields.len();
    let cols = fields[0].len();

    let mut regions: Vec<Region> = Vec::new();

    let mut visited_area = vec![vec![false; cols]; rows];
    let mut visited_perimeter = vec![vec![false; cols]; rows];
    let mut visited_sides = vec![vec![false; cols]; rows];

    for r in 0..rows {
        for c in 0..cols {
            if visited_area[r][c] {
                continue;
            }
            let ch = fields[r][c];

            let area = get_region_area(&fields, r, c, &mut visited_area);
            let perimeter = get_region_perimeter(&fields, r, c, &mut visited_perimeter);
            let sides = get_region_sides(&fields, r, c, &mut visited_sides);

            regions.push(Region {
                name: ch,
                area,
                perimeter,
                sides,
            });
        }
    }

    regions
}

fn get_region_area(
    grid: &Vec<Vec<char>>,
    start_r: usize,
    start_c: usize,
    visited: &mut Vec<Vec<bool>>,
) -> usize {
    let target = grid[start_r][start_c];
    let mut stack = VecDeque::new();
    stack.push_back((start_r, start_c));
    visited[start_r][start_c] = true;

    let dirs: [(isize, isize); 4] = [
        (-1, 0), // up
        (1, 0),  // down
        (0, -1), // left
        (0, 1),  // right
    ];

    let rows = grid.len();
    let cols = grid[0].len();

    let mut count = 0;

    while let Some((r, c)) = stack.pop_back() {
        count += 1;

        for (dr, dc) in dirs {
            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if nr < 0 || nc < 0 {
                continue;
            }
            let (ur, uc) = (nr as usize, nc as usize);
            if ur >= rows || uc >= cols {
                continue;
            }

            if !visited[ur][uc] && grid[ur][uc] == target {
                visited[ur][uc] = true;
                stack.push_back((ur, uc));
            }
        }
    }

    count
}

fn get_region_perimeter(
    grid: &Vec<Vec<char>>,
    start_r: usize,
    start_c: usize,
    visited: &mut Vec<Vec<bool>>,
) -> usize {
    let target = grid[start_r][start_c];
    let mut stack = VecDeque::new();
    stack.push_back((start_r, start_c));
    visited[start_r][start_c] = true;

    let dirs: [(isize, isize); 4] = [
        (-1, 0), // up
        (1, 0),  // down
        (0, -1), // left
        (0, 1),  // right
    ];

    let rows = grid.len();
    let cols = grid[0].len();

    let mut count = 0;

    while let Some((r, c)) = stack.pop_back() {
        for (dr, dc) in dirs {
            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if nr < 0 || nc < 0 {
                count += 1;
                continue;
            }
            let (ur, uc) = (nr as usize, nc as usize);
            if ur >= rows || uc >= cols {
                count += 1;
                continue;
            }

            if !visited[ur][uc] && grid[ur][uc] == target {
                visited[ur][uc] = true;
                stack.push_back((ur, uc));
            } else if grid[ur][uc] != target {
                count += 1;
            }
        }
    }

    count
}

fn get_region_sides(
    grid: &Vec<Vec<char>>,
    start_r: usize,
    start_c: usize,
    visited: &mut Vec<Vec<bool>>,
) -> usize {
    let target = grid[start_r][start_c];

    let rows = grid.len();
    let cols = grid[0].len();
    let dirs: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut q = VecDeque::new();
    q.push_back((start_r, start_c));
    visited[start_r][start_c] = true;

    let mut cells = vec![(start_r, start_c)];

    // Collect all cells in this region
    while let Some((r, c)) = q.pop_front() {
        for (dr, dc) in dirs {
            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if nr < 0 || nc < 0 {
                continue;
            }
            let (ur, uc) = (nr as usize, nc as usize);
            if ur >= rows || uc >= cols {
                continue;
            }
            if !visited[ur][uc] && grid[ur][uc] == target {
                visited[ur][uc] = true;
                q.push_back((ur, uc));
                cells.push((ur, uc));
            }
        }
    }

    // Region mask so we don't confuse same-letter *other* regions
    let mut in_region = vec![vec![false; cols]; rows];
    for &(r, c) in &cells {
        in_region[r][c] = true;
    }

    // Helper: is this tile inside *this* region?
    let inside = |r: isize, c: isize| -> bool {
        if r < 0 || c < 0 {
            return false;
        }
        let (ur, uc) = (r as usize, c as usize);
        if ur >= rows || uc >= cols {
            return false;
        }
        in_region[ur][uc]
    };

    let mut corners = 0;

    // Count convex + concave corners
    for &(r, c) in &cells {
        let r = r as isize;
        let c = c as isize;

        // external corners
        let nw = inside(r, c) && !inside(r - 1, c) && !inside(r, c - 1);
        let ne = inside(r, c) && !inside(r - 1, c) && !inside(r, c + 1);
        let sw = inside(r, c) && !inside(r + 1, c) && !inside(r, c - 1);
        let se = inside(r, c) && !inside(r + 1, c) && !inside(r, c + 1);

        // internal (concave) corners
        let nw_in = inside(r, c) && inside(r - 1, c) && inside(r, c - 1) && !inside(r - 1, c - 1);
        let ne_in = inside(r, c) && inside(r - 1, c) && inside(r, c + 1) && !inside(r - 1, c + 1);
        let sw_in = inside(r, c) && inside(r + 1, c) && inside(r, c - 1) && !inside(r + 1, c - 1);
        let se_in = inside(r, c) && inside(r + 1, c) && inside(r, c + 1) && !inside(r + 1, c + 1);

        corners += nw as usize
            + ne as usize
            + sw as usize
            + se as usize
            + nw_in as usize
            + ne_in as usize
            + sw_in as usize
            + se_in as usize;
    }

    corners
}

fn main() {
    println!("AOC 2023 day 12!");

    let input = read_to_string("input.txt").expect("err");

    let p1_result = solve_p1(input.as_str());

    println!("Part 1 solution: {}", p1_result);

    let p2_result = solve_p2(input.as_str());

    println!("Part 2 solution: {}", p2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_sample() {
        let input = r#"
        RRRRIICCFF
        RRRRIICCCF
        VVRRRCCFFF
        VVRCCCJFFF
        VVVVCJJCFE
        VVIVCCJJEE
        VVIIICJJEE
        MIIIIIJJEE
        MIIISIJEEE
        MMMISSJEEE
        "#;

        assert_eq!(solve_p1(input), 1930);
    }

    #[test]
    fn p1_sample_2() {
        let input = r#"
        OOOOO
        OXOXO
        OOOOO
        OXOXO
        OOOOO
        "#;

        assert_eq!(solve_p1(input), 772);
    }

    #[test]
    fn p1_sample_3() {
        let input = r#"
        AAAA
        BBCD
        BBCC
        EEEC
        "#;

        assert_eq!(solve_p1(input), 140);
    }

    #[test]
    fn p2_sample_2() {
        let input = r#"
        OOOOO
        OXOXO
        OOOOO
        OXOXO
        OOOOO
        "#;

        assert_eq!(solve_p2(input), 436);
    }

    #[test]
    fn p2_sample_3() {
        let input = r#"
        AAAAAA
        AAABBA
        AAABBA
        ABBAAA
        ABBAAA
        AAAAAA
        "#;

        assert_eq!(solve_p2(input), 368);
    }
}
