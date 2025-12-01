use std::fs::read_to_string;

fn main() {
    println!("Advent of code day 4!");

    let file = read_to_string("input.txt").expect("err");

    let part1 = count_xmas(&file);

    println!("part 1: {}", part1);

    let part2 = count_mas_x(&file);

    println!("part 2: {}", part2);
}

fn count_xmas(input: &str) -> u32 {
    let mut count: u32 = 0;

    let m: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim().chars().collect()) // Convert each line into chars
        .filter(|x: &Vec<char>| x.is_empty() == false)
        .collect();

    for i in 0..m.len() {
        for j in 0..m[i].len() {
            let ilen = i + 3 < m.len();
            let jlen = j + 3 < m[i].len();
            let jneg = j >= 3;

            let results: Vec<bool> = vec![
                jlen && match_pattern(m[i][j], m[i][j+1], m[i][j+2], m[i][j+3]),
                ilen && match_pattern(m[i][j], m[i+1][j], m[i+2][j], m[i+3][j]),
                ilen && jlen && match_pattern(m[i][j], m[i+1][j+1], m[i+2][j+2], m[i+3][j+3]),
                ilen && jneg && match_pattern(m[i][j], m[i+1][j-1], m[i+2][j-2], m[i+3][j-3]),
            ];

            for res in results {
                if res {
                    count = count + 1;
                }
            }
        }
    }

    return count;
}

fn count_mas_x(input: &str) -> u32 {
    let mut count: u32 = 0;

    let m: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim().chars().collect()) // Convert each line into chars
        .filter(|x: &Vec<char>| x.is_empty() == false)
        .collect();

    for i in 0..m.len() {
        for j in 0..m[i].len() {
            let ilen = i + 2 < m.len();
            let jlen = j + 2 < m[i].len();

            if ilen && jlen {
                if match_pattern_x(m[i][j], m[i+1][j+1], m[i+2][j+2]) &&
                    match_pattern_x(m[i][j+2], m[i+1][j+1], m[i+2][j]) {
                    count = count + 1;
                }
            }
        }
    }

    return count;
}

fn match_pattern(a: char, b: char, c: char, d: char) -> bool {
    a == 'X' && b == 'M' && c == 'A' && d == 'S' ||
    a == 'S' && b == 'A' && c == 'M' && d == 'X'
}

fn match_pattern_x(a: char, b: char, c: char) -> bool {
    a == 'M' && b == 'A' && c == 'S' ||
    a == 'S' && b == 'A' && c == 'M'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = r#"
            MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX
        "#;

        assert_eq!(count_xmas(input), 18);
    }

    #[test]
    fn test2_part_1() {
        let input = r#"
        ..X...
        .SAMX.
        .A..A.
        XMAS.S
        .X...."#;

        assert_eq!(count_xmas(input), 4);
    }

    #[test]
    fn test_part_2() {
        let input = r#"
            MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX
        "#;

        assert_eq!(count_mas_x(input), 9);
    }
}