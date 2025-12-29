use std::collections::{HashMap, HashSet};
use std::fs;

type Position = (usize, usize);
type Memo = HashMap<Position, HashSet<Position>>;
type RatingMemo = HashMap<Position, usize>;

fn count_trails_from(pos: Position, grid: &Vec<Vec<Option<u32>>>, memo: &mut RatingMemo) -> usize {
    if let Some(cached_count) = memo.get(&pos) {
        return *cached_count;
    }

    let height = grid[pos.0][pos.1];

    if height == Some(9) {
        return 1;
    }

    let mut total_paths = 0;
    let (r, c) = pos;

    let coords_to_check: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for (dr, dc) in coords_to_check {
        let nr = r as isize + dr;
        let nc = c as isize + dc;

        if nr >= 0 && nr < grid.len() as isize && nc >= 0 && nc < grid[nr as usize].len() as isize {
            if let Some(current) = height {
                let rr = nr as usize;
                let cc = nc as usize;
                if grid[rr][cc] == Some(current + 1) {
                    total_paths += count_trails_from((rr, cc), grid, memo);
                }
            }
        }
    }
    memo.insert(pos, total_paths);
    total_paths
}

fn find_nines_from(
    pos: Position,
    grid: &Vec<Vec<Option<u32>>>,
    memo: &mut Memo,
) -> HashSet<Position> {
    if let Some(cached_result) = memo.get(&pos) {
        return cached_result.clone();
    }

    let height = grid[pos.0][pos.1];

    if height == Some(9) {
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
            if let Some(next_height) = height {
                if grid[next_pos.0].len() == 0 {
                    continue;
                }
                if grid[next_pos.0][next_pos.1] == Some(next_height + 1) {
                    let nines = find_nines_from(next_pos, grid, memo);
                    reachable_nines.extend(nines);
                }
            }
        }
    }

    memo.insert(pos, reachable_nines.clone());
    reachable_nines
}

fn solve(grid: &Vec<Vec<Option<u32>>>) -> usize {
    if grid.is_empty() || grid[0].is_empty() {
        return 0;
    }

    let trailheads = get_trailheads(grid);

    let mut memo: Memo = HashMap::new();
    let mut total_score = 0;

    for trailhead in trailheads {
        let nines = find_nines_from(trailhead, grid, &mut memo);
        total_score += nines.len();
    }

    total_score
}

fn get_trailheads(grid: &Vec<Vec<Option<u32>>>) -> Vec<(usize, usize)> {
    if grid.is_empty() || grid[0].is_empty() {
        return Vec::new();
    }

    let mut trailheads = Vec::new();
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r].len() == 0 {
                continue;
            }
            if grid[r][c] == Some(0) {
                trailheads.push((r, c));
            }
        }
    }
    trailheads
}

fn parse_input(input: &str) -> Vec<Vec<Option<u32>>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| {
                    if c == '.' {
                        None
                    } else {
                        Some(c.to_digit(10).expect("Invalid character in input"))
                    }
                })
                .collect()
        })
        .collect()
}

fn solve_part1(input: &str) -> u32 {
    let grid = parse_input(input);

    solve(&grid) as u32
}

fn solve_part2(input: &str) -> u32 {
    let map = parse_input(input);
    if map.is_empty() || map[0].is_empty() {
        return 0;
    }

    let trailheads = get_trailheads(&map);

    let mut memo: RatingMemo = HashMap::new();
    let mut total_rating = 0;
    for th in trailheads {
        let rating = count_trails_from(th, &map, &mut memo);
        total_rating += rating;
    }

    total_rating as u32
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("err");

    let solution = solve_part1(input.clone().as_str());
    println!("Sum of scores: {}", solution);

    let solution_p2 = solve_part2(input.clone().as_str());
    println!("Sum of scores: {}", solution_p2);
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

    #[test]
    fn p2_example() {
        let input = r#"
        .....0.
        ..4321.
        ..5..2.
        ..6543.
        ..7..4.
        ..8765.
        ..9....
        "#;

        assert_eq!(solve_part2(input), 3);
    }

    #[test]
    fn p2_example_1() {
        let input = r#"
        ..90..9
        ...1.98
        ...2..7
        6543456
        765.987
        876....
        987....
        "#;

        assert_eq!(solve_part2(input), 13);
    }

    #[test]
    fn p2_example_2() {
        let input = r#"
        .....0.
        ..4321.
        ..5..2.
        ..6543.
        ..7..4.
        ..8765.
        ..9....
        "#;

        assert_eq!(solve_part2(input), 3);
    }
}
