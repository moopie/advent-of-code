use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fs::read_to_string,
};

type Grid = Vec<Vec<char>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
enum Dir {
    N,
    E,
    S,
    W,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
struct State {
    x: usize,
    y: usize,
    dir: Dir,
}

fn main() {
    println!("AOC 2024 day 16!");

    let contents = read_to_string("input.txt").unwrap();

    let part1 = solve_part_1(contents.as_str());

    println!("Part 1 solution: {}", part1);
}

fn solve_part_1(input: &str) -> i32 {
    let (grid, start_pos, end_pos) = parse_input(input);

    let mut heap = BinaryHeap::new();
    let mut dist = HashMap::new();

    let start = State {
        x: start_pos.0,
        y: start_pos.1,
        dir: Dir::E,
    };

    heap.push((Reverse(0), start));

    dist.insert(start, 0);

    while let Some((Reverse(cost), state)) = heap.pop() {
        if (state.x, state.y) == (end_pos.0, end_pos.1) {
            return cost;
        }

        if let Some(&best) = dist.get(&state) {
            if cost > best {
                continue; // stale entry
            }
        }

        let (nx, ny) = forward(state.x, state.y, state.dir);

        if grid[ny][nx] != '#' {
            let next_cost = cost + 1;
            let next_state = State {
                x: nx,
                y: ny,
                dir: state.dir,
            };
            if dist.get(&next_state).map_or(true, |&c| next_cost < c) {
                dist.insert(next_state, next_cost);
                heap.push((Reverse(next_cost), next_state));
            }
        }

        for next_dir in [turn_left(state.dir), turn_right(state.dir)] {
            let next_cost = cost + 1000;
            let next_state = State {
                x: state.x,
                y: state.y,
                dir: next_dir,
            };

            if dist.get(&next_state).map_or(true, |&c| next_cost < c) {
                dist.insert(next_state, next_cost);
                heap.push((Reverse(next_cost), next_state));
            }
        }
    }

    0
}

fn turn_left(d: Dir) -> Dir {
    match d {
        Dir::N => Dir::W,
        Dir::W => Dir::S,
        Dir::S => Dir::E,
        Dir::E => Dir::N,
    }
}

fn turn_right(d: Dir) -> Dir {
    match d {
        Dir::N => Dir::E,
        Dir::E => Dir::S,
        Dir::S => Dir::W,
        Dir::W => Dir::N,
    }
}

fn forward(x: usize, y: usize, d: Dir) -> (usize, usize) {
    match d {
        Dir::N => (x, y - 1),
        Dir::S => (x, y + 1),
        Dir::W => (x - 1, y),
        Dir::E => (x + 1, y),
    }
}

fn parse_input(input: &str) -> (Grid, (usize, usize), (usize, usize)) {
    let mut grid = Vec::new();
    let mut start = None;
    let mut end = None;

    for (y, line) in input.lines().enumerate() {
        let current_row: Vec<char> = line.chars().collect();
        let mut new_row = Vec::new();

        for (x, c) in current_row.iter().enumerate() {
            let current = match c {
                'S' => {
                    start = Some((x, y));
                    '.'
                }
                'E' => {
                    end = Some((x, y));
                    '.'
                }
                _ => *c,
            };
            new_row.push(current);
        }

        grid.push(new_row);
    }

    (
        grid,
        start.expect("No start (S) found"),
        end.expect("No end (E) found"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT1: &str = r#"
    ###############
    #.......#....E#
    #.#.###.#.###.#
    #.....#.#...#.#
    #.###.#####.#.#
    #.#.#.......#.#
    #.#.#####.###.#
    #...........#.#
    ###.#.#####.#.#
    #...#.....#.#.#
    #.#.#.###.#.#.#
    #.....#...#.#.#
    #.###.#.#.#.#.#
    #S..#.....#...#
    ###############
    "#;

    const EXAMPLE_INPUT2: &str = r#"
    #################
    #...#...#...#..E#
    #.#.#.#.#.#.#.#.#
    #.#.#.#...#...#.#
    #.#.#.#.###.#.#.#
    #...#.#.#.....#.#
    #.#.#.#.#.#####.#
    #.#...#.#.#.....#
    #.#.#####.#.###.#
    #.#.#.......#...#
    #.#.###.#####.###
    #.#.#...#.....#.#
    #.#.#.#####.###.#
    #.#.#.........#.#
    #.#.#.#########.#
    #S#.............#
    #################
    "#;

    #[test]
    fn part1_should_be_7036() {
        let val = solve_part_1(EXAMPLE_INPUT1);

        assert_eq!(7036, val);
    }

    #[test]
    fn part1_should_be_11048() {
        let val = solve_part_1(EXAMPLE_INPUT2);

        assert_eq!(11048, val);
    }
}
