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

    let part2 = solve_part_2(contents.as_str());

    println!("Part 2 solution: {}", part2);
}

fn solve_part_2(input: &str) -> i32 {
    let (grid, start_pos, end_pos) = parse_input(input);

    let start = State {
        x: start_pos.0,
        y: start_pos.1,
        dir: Dir::E,
    };

    let dist_start = dijkstra_from_start(&grid, start);
    let dist_end = dijkstra_from_end(&grid, end_pos);

    let best = dist_start
        .iter()
        .filter(|(s, _)| (s.x, s.y) == end_pos)
        .map(|(_, &c)| c)
        .min()
        .unwrap();

    let mut tiles = std::collections::HashSet::new();

    for (s, &ds) in &dist_start {
        if let Some(&de) = dist_end.get(s) {
            if ds + de == best {
                tiles.insert((s.x, s.y));
            }
        }
    }

    tiles.len() as i32
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

fn dijkstra_from_start(grid: &Grid, start: State) -> HashMap<State, i32> {
    let mut heap = BinaryHeap::new();
    let mut dist = HashMap::new();

    heap.push((Reverse(0), start));
    dist.insert(start, 0);

    while let Some((Reverse(cost), state)) = heap.pop() {
        if cost > dist[&state] {
            continue;
        }

        // forward
        let (nx, ny) = forward(state.x, state.y, state.dir);
        if grid[ny][nx] != '#' {
            let ns = State {
                x: nx,
                y: ny,
                dir: state.dir,
            };
            let nc = cost + 1;
            if dist.get(&ns).map_or(true, |&c| nc < c) {
                dist.insert(ns, nc);
                heap.push((Reverse(nc), ns));
            }
        }

        // turns
        for nd in [turn_left(state.dir), turn_right(state.dir)] {
            let ns = State {
                x: state.x,
                y: state.y,
                dir: nd,
            };
            let nc = cost + 1000;
            if dist.get(&ns).map_or(true, |&c| nc < c) {
                dist.insert(ns, nc);
                heap.push((Reverse(nc), ns));
            }
        }
    }

    dist
}

fn dijkstra_from_end(grid: &Grid, end: (usize, usize)) -> HashMap<State, i32> {
    let mut heap = BinaryHeap::new();
    let mut dist = HashMap::new();

    for dir in [Dir::N, Dir::E, Dir::S, Dir::W] {
        let s = State {
            x: end.0,
            y: end.1,
            dir,
        };
        heap.push((Reverse(0), s));
        dist.insert(s, 0);
    }

    while let Some((Reverse(cost), state)) = heap.pop() {
        if cost > dist[&state] {
            continue;
        }

        // backward move (reverse of forward)
        let (bx, by) = match state.dir {
            Dir::N => (state.x, state.y + 1),
            Dir::S => (state.x, state.y - 1),
            Dir::E => (state.x - 1, state.y),
            Dir::W => (state.x + 1, state.y),
        };

        if grid[by][bx] != '#' {
            let ns = State {
                x: bx,
                y: by,
                dir: state.dir,
            };
            let nc = cost + 1;
            if dist.get(&ns).map_or(true, |&c| nc < c) {
                dist.insert(ns, nc);
                heap.push((Reverse(nc), ns));
            }
        }

        // turns
        for nd in [turn_left(state.dir), turn_right(state.dir)] {
            let ns = State {
                x: state.x,
                y: state.y,
                dir: nd,
            };
            let nc = cost + 1000;
            if dist.get(&ns).map_or(true, |&c| nc < c) {
                dist.insert(ns, nc);
                heap.push((Reverse(nc), ns));
            }
        }
    }

    dist
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

    #[test]
    fn part2_should_be_45() {
        let val = solve_part_2(EXAMPLE_INPUT1);

        assert_eq!(45, val);
    }

    #[test]
    fn part2_should_be_64() {
        let val = solve_part_2(EXAMPLE_INPUT2);

        assert_eq!(64, val);
    }
}
