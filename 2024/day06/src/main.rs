use std::fs::read_to_string;
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn main() {
    println!("AOC 2024 day 6!");

    let input = read_to_string("input.txt").expect("err");

    let count = count_visits(input.clone());

    println!("part 1 count: {}", count);

    let part2 = solve_part_2(input);

    println!("part 2 count: {}", part2);
}

fn count_visits(input: String) -> u32 {
    let map: Vec<Vec<char>> = input.lines()
        .map(|x| x.to_string().trim().chars().collect())
        .filter(|line: &Vec<char>| !line.is_empty())
        .collect();

    for (i, line) in map.iter().enumerate() {
        for (j, b) in line.iter().enumerate() {
            match get_direction(b.clone()) {
                Some(dir) => {
                    return traverse(map, i, j, dir);
                },
                _ => ()
            }
        }
    }
    return 0;
}

fn print_map(map: Vec<Vec<char>>) {
    for ix in map.iter() {
        for jx in ix.iter() {
            print!("{jx}");
        }
        print!("\n");
    }
    print!("\n");
}

fn traverse(map: Vec<Vec<char>>, i: usize, j: usize, dir: Direction) -> u32 {
    let mut map = map;
    let mut i = i;
    let mut j = j;
    let mut dir = dir;
    let mut incr = 0;
    let mut moves = HashSet::new();

    while i < map.len() || j < map[0].len() {
        let (next_i, next_j) = match dir {
            Direction::Up => (i.checked_sub(1), Some(j)),
            Direction::Down => (i.checked_add(1), Some(j)),
            Direction::Left => (Some(i), j.checked_sub(1)),
            Direction::Right => (Some(i), j.checked_add(1))
        };

        moves.insert((i, j));

        match (next_i, next_j) {
            (Some(ni), Some(nj)) => {
                if ni < map.len() && nj < map[0].len() {
                    let next = map[ni][nj];

                    map[i][j] = 'X';

                    match next {
                        'X' => {
                            i = ni;
                            j = nj;
                            incr += 1;
                        },
                        '.' | '^' => {
                            i = ni;
                            j = nj;
                        },
                        _ => {
                            dir = change_direction(dir);
                        }
                    }
                }
                else {
                    return moves.len().try_into().unwrap();
                }
            },
            _ => {
                return moves.len().try_into().unwrap();
            }
        }

    }

    println!("incr: {}", incr);

    0
}

fn causes_loop(
    map: &Vec<Vec<char>>,
    start: (usize, usize),
    start_dir: Direction,
    obstacle: (usize, usize),
) -> bool {
    let height = map.len();
    let width = map[0].len();

    let mut x = start.0;
    let mut y = start.1;
    let mut dir = start_dir;

    let mut seen: HashSet<(usize, usize, Direction)> = HashSet::new();

    loop {
        // loop detected
        if !seen.insert((x, y, dir)) {
            return true;
        }

        let (nx, ny) = next_pos(x, y, &dir);

        // exiting map = no loop
        if nx >= width || ny >= height {
            return false;
        }

        // check obstacle or wall
        if (nx, ny) == obstacle || map[ny][nx] == '#' {
            dir = change_direction(dir);
        } else {
            x = nx;
            y = ny;
        }
    }
}

fn next_pos(x: usize, y: usize, dir: &Direction) -> (usize, usize) {
    match dir {
        Direction::Up => (x, y.wrapping_sub(1)),
        Direction::Down => (x, y + 1),
        Direction::Left => (x.wrapping_sub(1), y),
        Direction::Right => (x + 1, y),
    }
}

fn solve_part_2(input: String) -> u32 {
    let map: Vec<Vec<char>> = input.lines()
        .map(|x| x.trim().chars().collect())
        .filter(|line: &Vec<char>| !line.is_empty())
        .collect();

    let height = map.len();
    let width = map[0].len();

    // locate guard start
    let mut start = (0usize, 0usize);
    let mut start_dir = Direction::Up;

    for y in 0..height {
        for x in 0..width {
            if let Some(d) = get_direction(map[y][x]) {
                start = (x, y);
                start_dir = d;
            }
        }
    }

    let mut count = 0;

    // try placing an obstacle in every empty cell except start
    for y in 0..height {
        for x in 0..width {
            if map[y][x] != '.' {
                continue;
            }
            if (x, y) == start {
                continue;
            }

            // simulate with a virtual obstacle at (x, y)
            if causes_loop(&map, start, start_dir, (x, y)) {
                count += 1;
            }
        }
    }

    count
}

fn change_direction(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn get_direction(guard: char) -> Option<Direction> {
    match guard {
        '^' => Some(Direction::Up),
        'v' => Some(Direction::Down),
        '<' => Some(Direction::Left),
        '>' => Some(Direction::Right),
        _ => None
    }
}

fn get_new_direction(cell: char, current: Direction) -> Direction {
    return match cell {
        '#' | 'O' => change_direction(current),
        _ => current
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_test() {
        let input = r#"
            ....#.....
            .........#
            ..........
            ..#.......
            .......#..
            ..........
            .#..^.....
            ........#.
            #.........
            ......#...
        "#;

        assert_eq!(count_visits(input.to_string()), 41);
    }

    #[test]
    fn zid_test() {
        let input = r#"
        .....
        ..#..
        ...#.
        .....
        ..^..
        "#;

        assert_eq!(count_visits(input.to_string()), 3);
    }

    #[test]
    fn zid_test2() {
        let input = r#"
        .....
        ..#..
        ....#
        .....
        ..^#.
        "#;

        assert_eq!(count_visits(input.to_string()), 7);
    }

    #[test]
    fn part2_test() {
        let input = r#"
            ....#.....
            .........#
            ..........
            ..#.......
            .......#..
            ..........
            .#..^.....
            ........#.
            #.........
            ......#...
        "#;

        assert_eq!(solve_part_2(input.to_string()), 6);
    }
}
