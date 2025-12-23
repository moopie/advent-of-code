use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::recognize,
    error::{Error, ErrorKind},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};
use std::fs::read_to_string;

#[derive(Debug)]
enum Calc {
    Mul(i32, i32),
    Do,
    Dont,
}

fn number_1_to_3(input: &str) -> IResult<&str, i32> {
    let (rest, digits) = recognize(digit1).parse(input)?;

    if digits.len() > 3 {
        return Err(nom::Err::Error(Error::new(input, ErrorKind::Digit)));
    }

    let value = digits.parse::<i32>().unwrap();
    Ok((rest, value))
}

fn do_parser(input: &str) -> IResult<&str, Calc> {
    tag("do()").map(|_| Calc::Do).parse(input)
}

fn dont_parser(input: &str) -> IResult<&str, Calc> {
    tag("don't()").map(|_| Calc::Dont).parse(input)
}

fn mul_parser(input: &str) -> IResult<&str, Calc> {
    delimited(
        tag("mul("),
        separated_pair(number_1_to_3, tag(","), number_1_to_3),
        tag(")"),
    )
    .map(|(a, b)| Calc::Mul(a, b))
    .parse(input)
}

pub fn solve(input: &str) -> i32 {
    let mut sum = 0;
    let mut i = 0;

    while i < input.len() {
        let slice = &input[i..];

        if let Ok((rest, value)) = mul_parser(slice) {
            match value {
                Calc::Mul(a, b) => sum += a * b,
                _ => (),
            };
            i += slice.len() - rest.len();
        } else {
            i += 1; // skip corrupted character
        }
    }

    sum
}

fn parse_next(input: &str) -> Option<(usize, Calc)> {
    let mut parsers = alt((dont_parser, do_parser, mul_parser));

    if let Ok((rest, instr)) = parsers.parse(input) {
        let consumed = input.len() - rest.len();
        Some((consumed, instr))
    } else {
        None
    }
}

pub fn solve_part2(input: &str) -> i32 {
    let mut sum = 0;
    let mut i = 0;
    let mut cont = true;

    while i < input.len() {
        let slice = &input[i..];

        if let Some((consumed, value)) = parse_next(slice) {
            match value {
                Calc::Mul(a, b) => {
                    if cont {
                        sum += a * b
                    }
                }
                Calc::Do => cont = true,
                Calc::Dont => cont = false,
            };
            i += consumed;
        } else {
            i += 1; // skip corrupted character
        }
    }

    sum
}

fn main() {
    println!("Hello aoc 2024 day 3!");
    let contents = read_to_string("input.txt").expect("Can't open file");

    let ans = solve(&contents.trim());
    println!("Day 3 part 1 result: {}", ans);

    let ans = solve_part2(&contents.trim());
    println!("Day 3 part 2 result: {}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_part1() {
        assert_eq!(
            solve("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            161
        );
    }

    #[test]
    fn correct_handle_of_mul() {
        assert_eq!(solve("mul(34)mul(2,3)mul((2,4))"), 6);
    }

    #[test]
    fn initial_part2() {
        assert_eq!(
            solve_part2(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
            ),
            48
        );
    }
}
