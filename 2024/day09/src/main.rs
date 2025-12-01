fn defrag(input: String) -> i64 {
    let parse = parse_file(input);
    0
}

fn parse_file(input: String) -> Vec<Option<i64>> {
    let mut fill: Vec<Option<i64>> = vec![];

    let mut index = 0;
    for i in 0..input.len() {
        let is_space = i % 2 > 0;

        let n = input.chars().nth(i).unwrap_or_default();
        let num = n.to_string().parse::<i64>().expect("Not a number");
        let mut index = num;
        while index > 0 {
            fill.push(Some(num));
            index -= 1;
        }
    }

    todo!()
}

fn main() {
    println!("Hello, world!");
}

mod tests {
    use super::*;

    #[test]
    fn init() {
        let input = "2333133121414131402".to_string();
        assert_eq!(defrag(input), 1928);
    }
}
