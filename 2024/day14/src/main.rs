use std::fs::read_to_string;

fn main() {
    println!("AOC 2024 day 14!");

    let input = read_to_string("input.txt").expect("err");

    let part1 = solve_part_1(input.as_str(), 101, 103);

    println!("Part 1 solution: {}", part1);

    solve_part_2(input.as_str(), 101, 103)
}

#[derive(Debug)]
struct Robot {
    pos: (i64, i64),
    vel: (i64, i64),
}

fn solve_part_1(input: &str, width: i64, height: i64) -> u64 {
    let robots = parse_input(input);

    get_safety_factor(&robots, width, height, 100)
}

fn get_safety_factor(robots: &[Robot], width: i64, height: i64, step: u64) -> u64 {
    let mut sums = [0u64; 4];

    let mid_x = width / 2;
    let mid_y = height / 2;

    for robot in robots {
        let dx = wrap(robot.pos.0 + (robot.vel.0 * step as i64) % width, width);
        let dy = wrap(robot.pos.1 + (robot.vel.1 * step as i64) % height, height);

        if dx == mid_x || dy == mid_y {
            continue;
        }

        let quad_x = dx < mid_x;
        let quad_y = dy < mid_y;

        let quad = match (quad_x, quad_y) {
            (false, false) => 0,
            (false, true) => 1,
            (true, false) => 2,
            (true, true) => 3,
        };

        sums[quad] += 1;
    }
    sums.iter().product()

}

fn solve_part_2(input: &str, width: i64, height: i64) {
    let robots = parse_input(input);
    let mut val = u64::MAX;
    let mut min_pos = 0;

    for i in 0u64..(width*height) as u64 {
        let sf = get_safety_factor(&robots, width, height, i);
        if sf < val {
            val = sf;
            min_pos = i;
        }
    }
    println!("Part 2 solution: {}", min_pos);
}

fn wrap(val: i64, max: i64) -> i64 {
    ((val % max) + max) % max
}

fn parse_input(input: &str) -> Vec<Robot> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let (p, v) = l.trim().split_once(' ').expect("line has p and v");
            let p = p.strip_prefix("p=").unwrap();
            let v = v.strip_prefix("v=").unwrap();

            let (px, py) = p.split_once(',').unwrap();
            let (vx, vy) = v.split_once(',').unwrap();

            Robot {
                pos: (px.parse().unwrap(), py.parse().unwrap()),
                vel: (vx.parse().unwrap(), vy.parse().unwrap()),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = r#"
    p=0,4 v=3,-3
    p=6,3 v=-1,-3
    p=10,3 v=-1,2
    p=2,0 v=2,-1
    p=0,0 v=1,3
    p=3,0 v=-2,-2
    p=7,6 v=-1,-3
    p=3,0 v=-1,-2
    p=9,3 v=2,3
    p=7,3 v=-1,2
    p=2,4 v=2,-3
    p=9,5 v=-3,-3
    "#;

    #[test]
    fn part1_should_be_12() {
        let result = solve_part_1(EXAMPLE_INPUT, 11, 7);

        assert_eq!(12, result);
    }
}
