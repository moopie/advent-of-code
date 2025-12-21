use std::fs;

#[derive(Debug, PartialEq)]
enum Safety {
    Safe,
    Unsafe,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    None,
    Up,
    Down,
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
        } else if dist > 0 {
            cdir = Direction::Up;
        } else {
            return Safety::Unsafe;
        }

        if dir == Direction::None {
            dir = cdir;
        } else if dir != cdir {
            return Safety::Unsafe;
        }

        if dist.abs() < 1 || dist.abs() > 3 {
            return Safety::Unsafe;
        }
    }

    return Safety::Safe;
}

fn calc_distance_with_tolerance(vec: Vec<i32>) -> Safety {
    // already safe?
    if calc_distance(vec.clone()) == Safety::Safe {
        return Safety::Safe;
    }

    // try removing exactly one element
    for i in 0..vec.len() {
        let mut copy = vec.clone();
        copy.remove(i);

        if calc_distance(copy) == Safety::Safe {
            return Safety::Safe;
        }
    }

    Safety::Unsafe
}

fn main() {
    println!("aoc2024 day 2!");

    let levels = read_file("input.txt".to_string());
    let res = levels.iter().map(|lev| calc_distance(lev.to_vec()));

    let count = res.filter(|x| x == &Safety::Safe).count();

    println!("Part 1 {count}");

    // fast solution because I can't deal with this mess
    // in 2025 i realized that it's easier to clone part 1 and modify it
    // instead of trying to do both parts in the same file

    let res2 = levels.iter().map(|lev| {
        println!();
        let v = lev.to_vec();
        let safety = calc_distance_with_tolerance(v);
        println!("__VEC__({lev:?}) {safety:?}");
        safety
    });

    let mut safe = 0;
    for r in res2 {
        if r == Safety::Safe {
            safe += 1
        }
    }

    println!("Part 2 {safe}");
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

    #[test]
    fn test_calc_tolerant_dist1() {
        let arr = vec![33, 36, 35, 36, 33];
        let res = calc_distance_with_tolerance(arr);
        assert_eq!(res, Safety::Unsafe);
    }

    #[test]
    fn test_calc_tolerant_dist2() {
        let arr = vec![7, 6, 4, 2, 1];
        let res = calc_distance_with_tolerance(arr);
        assert_eq!(res, Safety::Safe);
    }

    #[test]
    fn test_calc_tolerant_dist3() {
        let arr = vec![1, 2, 7, 8, 9];
        let res = calc_distance_with_tolerance(arr);
        assert_eq!(res, Safety::Unsafe);
    }

    #[test]
    fn test_calc_tolerant_dist4() {
        let arr = vec![9, 7, 6, 2, 1];
        let res = calc_distance_with_tolerance(arr);
        assert_eq!(res, Safety::Unsafe);
    }

    #[test]
    fn test_calc_tolerant_dist5() {
        let arr = vec![1, 3, 2, 4, 5];
        let res = calc_distance_with_tolerance(arr);
        assert_eq!(res, Safety::Safe);
    }

    #[test]
    fn test_calc_tolerant_dist6() {
        let arr = vec![8, 6, 4, 4, 1];
        let res = calc_distance_with_tolerance(arr);
        assert_eq!(res, Safety::Safe);
    }

    #[test]
    fn test_calc_tolerant_dist7() {
        let arr = vec![1, 3, 6, 7, 9];
        let res = calc_distance_with_tolerance(arr);
        assert_eq!(res, Safety::Safe);
    }
}
