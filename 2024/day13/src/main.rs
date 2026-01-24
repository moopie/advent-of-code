use std::fs::read_to_string;

#[derive(Debug)]
struct Machine {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    px: i64,
    py: i64,
}

fn main() {
    println!("AOC 2024 day 12!");

    let input = read_to_string("input.txt").expect("err");

    let result = solve_part1(input.as_str());

    println!("Part 1 solution: {}", result);
}

fn solve_part1(input: &str) -> i64 {
    let machines = parse_input(input);

    let mut sum = 0;

    for m in machines {
        let det = m.ax * m.by - m.bx * m.ay;
        if det == 0 {
            continue; // no unique solution
        }

        let a_num = m.px * m.by - m.bx * m.py;
        let b_num = m.ax * m.py - m.px * m.ay;

        // must be divisible
        if a_num % det != 0 || b_num % det != 0 {
            continue;
        }

        let a = a_num / det;
        let b = b_num / det;

        // part 1: only solutions from 0..=100 count
        if a >= 0 && a <= 100 && b >= 0 && b <= 100 {
            sum += 3 * a + b;
        }
    }

    sum
}
fn parse_input(input: &str) -> Vec<Machine> {
    input
        .trim()
        .split("\n\n") // split blocks
        .map(|block| parse_machine(block))
        .collect()
}

fn parse_machine(block: &str) -> Machine {
    let mut lines = block.lines().map(|l| l.trim());

    let a = lines.next().unwrap();
    let b = lines.next().unwrap();
    let p = lines.next().unwrap();

    let (ax, ay) = parse_xy(a);
    let (bx, by) = parse_xy(b);
    let (px, py) = parse_xy(p);

    Machine {
        ax,
        ay,
        bx,
        by,
        px,
        py,
    }
}

// Extract X+94, Y+34 or X=8400, Y=5400
fn parse_xy(line: &str) -> (i64, i64) {
    let (_, rest) = line.split_once("X").unwrap(); // after the 'X'
    let (xpart, ypart) = rest.split_once(", Y").unwrap();

    let x = xpart[1..].parse().unwrap(); // skip '+' or '='
    let y = ypart[1..].parse().unwrap();

    (x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
        Button A: X+94, Y+34
        Button B: X+22, Y+67
        Prize: X=8400, Y=5400

        Button A: X+26, Y+66
        Button B: X+67, Y+21
        Prize: X=12748, Y=12176

        Button A: X+17, Y+86
        Button B: X+84, Y+37
        Prize: X=7870, Y=6450

        Button A: X+69, Y+23
        Button B: X+27, Y+71
        Prize: X=18641, Y=10279
        "#;

    #[test]
    fn part1_should_be_480() {
        let result = solve_part1(INPUT);

        assert_eq!(480, result);
    }
}
