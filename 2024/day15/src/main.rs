use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
};

type Grid = Vec<Vec<char>>;
type Move = char;
type Robot = (i32, i32);

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let part1 = solve_part_1(&input);

    println!("Part 1 solution: {}", part1);

    let part2 = solve_part_2(input.as_str());

    println!("Part 2 solution: {}", part2);
}

fn solve_part_1(input: &str) -> u32 {
    let (mut grid, moves, mut robot) = parse_input_part1(input);

    for mov in moves {
        move_robot_part1(&mut grid, mov, &mut robot);
    }

    score(&grid)
}

fn solve_part_2(input: &str) -> u32 {
    let (mut grid, moves, mut robot) = parse_input_part2(input);

    for mov in moves {
        move_robot_part2(&mut grid, mov, &mut robot);
    }

    score(&grid)
}

fn parse_input_part1(input: &str) -> (Grid, Vec<Move>, Robot) {
    let (map_part, moves_part) = input.split_once("\n\n").unwrap();
    let mut robot = (0, 0);

    let grid = map_part
        .lines()
        .filter(|l| !l.trim().is_empty())
        .enumerate()
        .map(|(y, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == '@' {
                        robot = (x as i32, y as i32);
                    }
                    c
                })
                .collect()
        })
        .collect();

    let moves = moves_part
        .lines()
        .filter(|l| !l.trim().is_empty())
        .flat_map(|l| l.trim().chars())
        .collect();

    (grid, moves, robot)
}

fn parse_input_part2(input: &str) -> (Grid, Vec<Move>, Robot) {
    let (map_part, moves_part) = input.split_once("\n\n").unwrap();
    let mut robot = (0, 0);

    let grid = map_part
        .lines()
        .filter(|l| !l.trim().is_empty())
        .enumerate()
        .map(|(y, line)| {
            let mut row = Vec::new();

            for c in line.trim().chars() {
                match c {
                    '#' => {
                        row.push('#');
                        row.push('#');
                    }
                    '.' => {
                        row.push('.');
                        row.push('.');
                    }
                    'O' => {
                        row.push('[');
                        row.push(']');
                    }
                    '@' => {
                        robot = (row.len() as i32, y as i32);
                        row.push('@');
                        row.push('.');
                    }
                    _ => unreachable!(),
                }
            }

            row
        })
        .collect();

    let moves = moves_part
        .lines()
        .filter(|l| !l.trim().is_empty())
        .flat_map(|l| l.trim().chars())
        .collect();

    (grid, moves, robot)
}

fn dir(c: char) -> (i32, i32) {
    match c {
        '^' => (0, -1),
        'v' => (0, 1),
        '<' => (-1, 0),
        '>' => (1, 0),
        _ => unreachable!(),
    }
}

fn move_robot_part1(grid: &mut Grid, mov: Move, robot: &mut Robot) {
    let (dx, dy) = dir(mov);

    let nx = robot.0 + dx;
    let ny = robot.1 + dy;

    let next = grid[ny as usize][nx as usize];

    // Simple move
    if next == '.' {
        grid[robot.1 as usize][robot.0 as usize] = '.';
        grid[ny as usize][nx as usize] = '@';
        *robot = (nx, ny);
        return;
    }

    // Blocked immediately
    if next == '#' {
        return;
    }

    // Push logic
    let mut cx = nx;
    let mut cy = ny;

    // Scan forward
    loop {
        match grid[cy as usize][cx as usize] {
            'O' => {
                cx += dx;
                cy += dy;
            }
            '.' => break,
            '#' => return,
            _ => unreachable!(),
        }
    }

    // Shift boxes backward
    while (cx, cy) != (nx, ny) {
        let px = cx - dx;
        let py = cy - dy;
        grid[cy as usize][cx as usize] = grid[py as usize][px as usize];
        cx = px;
        cy = py;
    }

    // Move robot
    grid[robot.1 as usize][robot.0 as usize] = '.';
    grid[ny as usize][nx as usize] = '@';
    *robot = (nx, ny);
}

fn move_robot_part2(grid: &mut Grid, mov: Move, robot: &mut Robot) {
    let (dx, dy) = dir(mov);
    let nx = robot.0 + dx;
    let ny = robot.1 + dy;

    match grid[ny as usize][nx as usize] {
        '.' => {
            grid[robot.1 as usize][robot.0 as usize] = '.';
            grid[ny as usize][nx as usize] = '@';
            *robot = (nx, ny);
        }
        '#' => return,
        '[' | ']' => {
            let start = box_left(nx, ny, grid);

            let boxes = if dy != 0 {
                // vertical push
                let Some(b) = collect_boxes_vertical(start, dy, grid) else {
                    return;
                };
                b
            } else {
                // horizontal push
                let Some(b) = collect_boxes_horizontal(start, dx, dy, grid) else {
                    return;
                };
                b
            };

            move_boxes(&boxes, dx, dy, grid);

            grid[robot.1 as usize][robot.0 as usize] = '.';
            grid[ny as usize][nx as usize] = '@';
            *robot = (nx, ny);
        }
        _ => unreachable!(),
    }
}

fn box_left(x: i32, y: i32, grid: &Grid) -> (i32, i32) {
    if grid[y as usize][x as usize] == '[' {
        (x, y)
    } else {
        (x - 1, y)
    }
}

fn collect_boxes_vertical(start: (i32, i32), dy: i32, grid: &Grid) -> Option<HashSet<(i32, i32)>> {
    let mut boxes = HashSet::new();
    let mut queue = VecDeque::new();

    boxes.insert(start);
    queue.push_back(start);

    while let Some((lx, y)) = queue.pop_front() {
        let ny = y + dy;

        for x in [lx, lx + 1] {
            let cell = grid[ny as usize][x as usize];
            match cell {
                '.' => {}
                '#' => return None,
                '[' | ']' => {
                    let next = box_left(x, ny, grid);
                    if boxes.insert(next) {
                        queue.push_back(next);
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    Some(boxes)
}

fn collect_boxes_horizontal(
    start: (i32, i32),
    dx: i32,
    _dy: i32,
    grid: &Grid,
) -> Option<HashSet<(i32, i32)>> {
    let mut boxes = HashSet::new();
    let (mut x, y) = start;

    loop {
        // We are always positioned at the LEFT bracket
        boxes.insert((x, y));

        let check_x = if dx > 0 { x + 2 } else { x - 1 };

        match grid[y as usize][check_x as usize] {
            '.' => break,       // space to push into
            '#' => return None, // blocked
            '[' => {
                x = check_x; // next box (already left bracket)
            }
            ']' => {
                x = check_x - 1; // normalize to left bracket
            }
            _ => unreachable!(),
        }
    }

    Some(boxes)
}

fn move_boxes(boxes: &HashSet<(i32, i32)>, dx: i32, dy: i32, grid: &mut Grid) {
    let mut list: Vec<_> = boxes.iter().cloned().collect();

    // Move farthest first
    list.sort_by_key(|&(x, y)| if dy != 0 { -y * dy } else { -x * dx });

    for (x, y) in list {
        // clear old
        grid[y as usize][x as usize] = '.';
        grid[y as usize][(x + 1) as usize] = '.';

        // write new
        grid[(y + dy) as usize][(x + dx) as usize] = '[';
        grid[(y + dy) as usize][(x + dx + 1) as usize] = ']';
    }
}

fn score(grid: &Grid) -> u32 {
    let mut sum = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'O' || grid[y][x] == '[' {
                sum += 100 * y as u32 + x as u32;
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = r#"
    ##########
    #..O..O.O#
    #......O.#
    #.OO..O.O#
    #..O@..O.#
    #O#..O...#
    #O..O..O.#
    #.OO.O.OO#
    #....O...#
    ##########

    <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
    vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
    ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
    <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
    ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
    ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
    >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
    <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
    ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
    v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
    "#;

    const EXAMPLE2: &str = r#"
    ########
    #..O.O.#
    ##@.O..#
    #...O..#
    #.#.O..#
    #...O..#
    #......#
    ########

    <^^>>>vv<v>>v<<
    "#;

    #[test]
    fn part1_should_be_10092() {
        let res = solve_part_1(EXAMPLE1);

        assert_eq!(10092, res);
    }

    #[test]
    fn part1_should_be_2028() {
        let res = solve_part_1(EXAMPLE2);

        assert_eq!(2028, res);
    }

    #[test]
    fn part2_should_be_9021() {
        let res = solve_part_2(EXAMPLE1);

        assert_eq!(9021, res);
    }
}
