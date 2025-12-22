use std::fs::read_to_string;
use nom::{
    IResult,
    Parser,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::recognize,
    sequence::{delimited, separated_pair},
    error::{Error, ErrorKind},
};

fn number_1_to_3(input: &str) -> IResult<&str, i32> {
    let (rest, digits) = recognize(digit1).parse(input)?;

    if digits.len() > 3 {
        return Err(nom::Err::Error(Error::new(input, ErrorKind::Digit)));
    }

    let value = digits.parse::<i32>().unwrap();
    Ok((rest, value))
}

fn mul_parser(input: &str) -> IResult<&str, i32> {
    delimited(
        tag("mul("),
        separated_pair(number_1_to_3, tag(","), number_1_to_3),
        tag(")"),
    )
    .map(|(a, b)| a * b)
    .parse(input)
}

pub fn solve(input: &str) -> i32 {
    let mut sum = 0;
    let mut i = 0;

    while i < input.len() {
        let slice = &input[i..];

        if let Ok((rest, value)) = mul_parser(slice) {
            sum += value;
            i += slice.len() - rest.len();
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
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_part1() {
        assert_eq!(
            solve(
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
            ),
            161
        );
    }

    #[test]
    fn correct_handle_of_mul() {
        assert_eq!(solve("mul(34)mul(2,3)mul((2,4))"), 6);
    }
}
