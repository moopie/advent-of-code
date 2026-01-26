use std::fs::read_to_string;

type Grid = Vec<Vec<char>>;
type Move = char;
type Robot = (i32, i32);

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let result = solve_part_1(&input);
    println!("Part 1 solution: {}", result);
}

fn solve_part_1(input: &str) -> u32 {
    let (mut grid, moves, mut robot) = parse_input(input);

    for mov in moves {
        move_robot(&mut grid, mov, &mut robot);
    }

    score(&grid)
}

fn parse_input(input: &str) -> (Grid, Vec<Move>, Robot) {
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

fn dir(c: char) -> (i32, i32) {
    match c {
        '^' => (0, -1),
        'v' => (0, 1),
        '<' => (-1, 0),
        '>' => (1, 0),
        _ => unreachable!(),
    }
}

fn move_robot(grid: &mut Grid, mov: Move, robot: &mut Robot) {
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

fn score(grid: &Grid) -> u32 {
    let mut sum = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'O' {
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
}
