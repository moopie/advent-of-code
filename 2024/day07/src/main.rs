use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, space0, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated},
    IResult, Parser,
};
use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct Line {
    target: u64,
    values: Vec<u64>,
}

// parse a signed or unsigned integer
fn number(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse::<u64>).parse(input)
}

// parse one line: "190: 10 19"
fn line(input: &str) -> IResult<&str, Line> {
    let (input, (target, values)) = preceded(
        space0,
        separated_pair(number, tag(": "), separated_list1(space1, number)),
    )
    .parse(input)?;

    Ok((input, Line { target, values }))
}

fn lines(input: &str) -> IResult<&str, Vec<Line>> {
    preceded(
        nom::combinator::opt(newline),
        terminated(
            separated_list1(newline, line),
            nom::combinator::opt(newline),
        ),
    )
    .parse(input)
}

fn concat(a: u64, b: u64) -> u64 {
    let mut pow = 10;
    let mut t = b;
    while t >= 10 {
        pow *= 10;
        t /= 10;
    }
    a * pow + b
}

fn can_make(target: u64, nums: &Vec<u64>) -> bool {
    fn dfs(idx: usize, current: u64, target: u64, nums: &Vec<u64>) -> bool {
        if idx == nums.len() {
            return current == target;
        }

        let n = nums[idx];

        // +
        if dfs(idx + 1, current + n, target, nums) {
            return true;
        }

        // *
        if dfs(idx + 1, current * n, target, nums) {
            return true;
        }

        // ||
        if dfs(idx + 1, concat(current, n), target, nums) {
            return true;
        }

        false
    }
    dfs(1, nums[0], target, nums)
}

fn solve_part_2(input: &str) -> u64 {
    let (_, parsed) = lines(input.trim()).expect("Couldn't parse");

    parsed
        .iter()
        .filter_map(|x| {
            let nums = &x.values;
            let ans = can_make(x.target, nums);

            if ans {
                Some(x.target)
            } else {
                None
            }
        })
        .sum()
}

fn parse_eqs(input: String) -> u64 {
    let mut ret = 0;

    let eqs: Vec<String> = input
        .lines()
        .filter(|x| !x.to_string().trim().is_empty())
        .map(|x| x.to_string())
        .collect();

    for eq in eqs {
        let f: Vec<String> = eq.split(":").map(|x| x.trim().to_string()).collect();
        let target = f[0].parse::<u64>().expect("err");
        let arr = f[1]
            .split(" ")
            .map(|x| x.parse::<u64>().expect("err"))
            .collect();

        if get_solutions(arr, target) {
            ret += target;
        }
    }

    ret
}

fn get_solutions(equation: Vec<u64>, target: u64) -> bool {
    let n = equation.len();
    let total_size = 1 << (n - 1);

    for i in 0..total_size {
        let mut result = equation[0];
        let mut ops: Vec<char> = vec![];

        for ii in 0..equation.len() - 1 {
            let num = equation[ii + 1];
            let is_eq = i & (1 << ii) != 0;

            if is_eq {
                result += num;
                ops.push('+');
            } else {
                result *= num;
                ops.push('*');
            }
        }

        if result == target {
            return true;
        }
    }

    false
}

fn main() {
    println!("AOC day 7!");

    let content = read_to_string("input.txt").expect("err");
    let result_part_1 = parse_eqs(content.clone());

    println!("result for part 1: {}", result_part_1);

    let result_part_2 = solve_part_2(content.as_str());
    println!("result for part 2: {}", result_part_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let input = r#"
            190: 10 19
            3267: 81 40 27
            83: 17 5
            156: 15 6
            7290: 6 8 6 15
            161011: 16 10 13
            192: 17 8 14
            21037: 9 7 18 13
            292: 11 6 16 20
        "#;

        assert_eq!(parse_eqs(input.to_string()), 3749);
    }

    #[test]
    fn init_190() {
        let input = r#"
            190: 10 19
        "#;

        assert_eq!(parse_eqs(input.to_string()), 190);
    }

    #[test]
    fn init_3267() {
        let input = r#"
            3267: 81 40 27
        "#;

        assert_eq!(parse_eqs(input.to_string()), 3267);
    }

    #[test]
    fn init_292() {
        let input = r#"
            292: 11 6 16 20
        "#;

        assert_eq!(parse_eqs(input.to_string()), 292);
    }

    #[test]
    fn init_part_2() {
        let input = r#"
            190: 10 19
            3267: 81 40 27
            83: 17 5
            156: 15 6
            7290: 6 8 6 15
            161011: 16 10 13
            192: 17 8 14
            21037: 9 7 18 13
            292: 11 6 16 20
        "#;

        assert_eq!(solve_part_2(input), 11387);
    }
}
