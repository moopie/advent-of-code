use std::fs;

#[derive(Debug, PartialEq)]
enum Safety {
    Safe,
    Unsafe
}

#[derive(PartialEq)]
enum Direction {
    None,
    Up,
    Down
}

fn read_file(path: String) -> Vec<Vec<i32>> {
    let mut arr: Vec<Vec<i32>> = vec![];

    let contents = fs::read_to_string(path).expect("Can't read the file");

    let lines: Vec<&str> = contents.split("\n").collect();

    for line in lines.iter() {
        let nums: Vec<&str> = line.split_whitespace().collect();
        let mut vals: Vec<i32> = vec![];

        for num in nums.iter() {
            let n = num.parse::<i32>().expect("Couldn't parse {num}");
            vals.push(n);
        }

        arr.push(vals);
    }

    return arr;
}

fn calc_distance(vec: Vec<i32>) -> Safety {

    let mut dir = Direction::None;

    if vec.is_empty() {
        return Safety::Unsafe;
    }

    for i in 0..vec.len() - 1 {
        let a = vec[i];
        let b = vec[i + 1];

        let dist = a - b;
        let cdir: Direction;

        if dist < 0 {
            cdir = Direction::Down;
        }
        else if dist > 0 {
            cdir = Direction::Up;
        }
        else {
            return Safety::Unsafe;
        }

        if dir == Direction::None {
            dir = cdir;
        }
        else if dir != cdir {
            return Safety::Unsafe;
        }

        if dist.abs() < 1 || dist.abs() > 3 {
            return Safety::Unsafe;
        }
    }

    return Safety::Safe;
}

fn main() {
    println!("aoc2024 day 2!");

    let levels = read_file("input.txt".to_string());
    let res = levels.iter().map(|lev| calc_distance(lev.to_vec()));

    let count = res.filter(|x| x == &Safety::Safe).count();

    println!("Part 1 {count}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_distance1() {
        let arr = [1, 2, 3, 4, 5, 6].to_vec();
        let res = calc_distance(arr);
        assert_eq!(res, Safety::Safe);
    }

    #[test]
    fn test_calc_distance2() {
        let arr = [1, 2, 2, 4, 5, 6].to_vec();
        let res = calc_distance(arr);
        assert_eq!(res, Safety::Unsafe);
    }

    #[test]
    fn test_calc_distance3() {
        let arr = [2, 1, 3, 4, 5, 6].to_vec();
        let res = calc_distance(arr);
        assert_eq!(res, Safety::Unsafe);
    }
}
