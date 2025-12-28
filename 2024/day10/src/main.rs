use std::collections::{HashMap, HashSet};
use std::fs;

type Position = (usize, usize);
type Memo = HashMap<Position, HashSet<Position>>;

fn find_nines_from(pos: Position, grid: &Vec<Vec<u32>>, memo: &mut Memo) -> HashSet<Position> {
    if let Some(cached_result) = memo.get(&pos) {
        return cached_result.clone();
    }

    let height = grid[pos.0][pos.1];

    if height == 9 {
        let mut nines = HashSet::new();
        nines.insert(pos);
        return nines;
    }

    let mut reachable_nines = HashSet::new();
    let (r, c) = pos;

    let coords_to_check: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for (dr, dc) in coords_to_check {
        let nr = r as isize + dr;
        let nc = c as isize + dc;

        if nr >= 0 && nr < grid.len() as isize && nc >= 0 && nc < grid[0].len() as isize {
            let next_pos = (nr as usize, nc as usize);
            if grid[next_pos.0].len() == 0 {
                continue;
            }
            if grid[next_pos.0][next_pos.1] == height + 1 {
                let nines = find_nines_from(next_pos, grid, memo);
                reachable_nines.extend(nines);
            }
        }
    }

    memo.insert(pos, reachable_nines.clone());
    reachable_nines
}

fn solve(grid: &Vec<Vec<u32>>) -> usize {
    if grid.is_empty() || grid[0].is_empty() {
        return 0;
    }

    let mut trailheads = Vec::new();
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r].len() == 0 {
                continue;
            }
            if grid[r][c] == 0 {
                trailheads.push((r, c));
            }
        }
    }

    let mut memo: Memo = HashMap::new();
    let mut total_score = 0;

    for trailhead in trailheads {
        let nines = find_nines_from(trailhead, grid, &mut memo);
        total_score += nines.len();
    }

    total_score
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).expect("Invalid character in input"))
                .collect()
        })
        .collect()
}

fn solve_part1(input: &str) -> usize {
    let grid = parse_input(input);

    solve(&grid)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("err");

    let solution = solve_part1(input.as_str());
    println!("Sum of scores: {}", solution);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_example() {
        let input = r#"
        0123
        1234
        8765
        9876
        "#;

        assert_eq!(solve_part1(input), 1);
    }
}
