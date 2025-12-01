use std::fs::{read_to_string};

fn main() {
    println!("Hello aoc 2024 day 3!");
    let contents = read_to_string("input.txt").expect("Can't open file");
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut acc = 0;

    for line in lines.iter() {
        let res = parse_math_eq(line);

        println!("line: {}\nresult: {}", line, res);
        acc += res;
    }

    println!("Day 3 part 1 result: {}", acc);
}

#[derive(Debug, PartialEq)]
enum Token {
    OpenParen,
    CloseParen,
    Number,
    Separator,
}

fn parse_math_eq(line: &str) -> i32 {
    let split: Vec<&str> = line.split("mul").collect();
    let mut ret = 0;
    let mut fin: Vec<i32> = vec![];
    let mut nums: Vec<i32> = vec![];
    let mut buf: Vec<char> = vec![];

    for eq in split.iter() {
        let mut first_paren = false;
        for char in eq.chars() {
            let token = parse_token(char);

            match token {
                Some((ref t, v)) => println!("{}: {:?}", v, t),
                None => ()
            }

            match token {
                Some((t, c)) => {
                    match t {
                        Token::OpenParen => {
                            buf.clear();
                            nums.clear();
                            if first_paren {
                                break;
                            }
                            first_paren = true;
                        },
                        Token::Separator | Token::CloseParen => {
                            if buf.is_empty() {
                                continue;
                            }

                            let mut bufstr = "".to_string();
                            for n in buf.iter() {
                                bufstr.push_str(&n.to_string());
                            }
                            buf.clear();

                            println!("buf: {:?}", bufstr);
                            nums.push(bufstr.parse::<i32>().expect("err"));

                            if t == Token::CloseParen {
                                println!("num size: {}", nums.len());
                                if nums.len() == 2{
                                    fin.extend(nums.clone());
                                }
                                nums.clear();
                                break;
                            }
                        },
                        Token::Number => {
                            buf.push(c);
                        },
                    }

                }
                _ => {
                    buf.clear();
                }
            }
        }
    }

    println!("nums = {:?}", fin);

    for pair in fin.chunks_exact(2) {
        let a = pair [0];
        let b = pair[1];
        let calc = a * b;
        println!("{} * {} = {}", a, b, calc);
        ret += calc;
    }

    return ret;
}

fn parse_token(item: char) -> Option<(Token, char)> {
    match item {
        '(' => Some((Token::OpenParen, item)),
        ')' => Some((Token::CloseParen, item)),
        '0'..'9' => Some((Token::Number, item)),
        ',' => Some((Token::Separator, item)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_part1() {
        assert_eq!(parse_math_eq("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"), 161);
    }

    #[test]
    fn correct_handle_of_mul() {
        assert_eq!(parse_math_eq("mul(34)mul(2,3)mul((2,4))"), 6);
    }
}