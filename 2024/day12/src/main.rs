use std::collections::VecDeque;
use std::fs::read_to_string;

struct Region {
    name: char,
    area: usize,
    perimeter: usize,
}

fn solve_p1(input: &str) -> u32 {
    let mut score = 0;
    let fields = parse_map(input);
    let regions = get_regions(fields);

    for r in regions.iter() {
        score += r.area * r.perimeter;
    }

    score as u32
}

fn parse_map(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .split_whitespace()
        .map(|x| x.chars().collect())
        .collect()
}

fn get_regions(fields: Vec<Vec<char>>) -> Vec<Region> {
    let mut regions: Vec<Region> = vec![];
    let mut visited_area: Vec<Vec<bool>> = fields
        .clone()
        .iter()
        .map(|f| f.iter().map(|_| false).collect())
        .collect();

    let mut visited_perimeter: Vec<Vec<bool>> = fields
        .clone()
        .iter()
        .map(|f| f.iter().map(|_| false).collect())
        .collect();

    for col in 0..fields.clone().len() {
        for row in 0..fields[col].clone().len() {
            if visited_area[col][row] == true {
                continue;
            }
            let ch = fields[col][row];
            let area = get_region_area(fields.clone(), col, row, &mut visited_area);
            let perimeter = get_region_perimeter(fields.clone(), col, row, &mut visited_perimeter);
            regions.push(Region {
                name: ch,
                area: area,
                perimeter: perimeter,
            })
        }
    }

    regions
}

fn get_region_area(
    grid: Vec<Vec<char>>,
    start_c: usize,
    start_r: usize,
    visited: &mut Vec<Vec<bool>>,
) -> usize {
    let target = grid[start_c][start_r];
    let mut stack = VecDeque::new();
    stack.push_back((start_c, start_r));
    visited[start_c][start_r] = true;

    let dirs: [(isize, isize); 4] = [
        (-1, 0), // up
        (1, 0),  // down
        (0, -1), // left
        (0, 1),  // right
    ];
    let mut count = 0;

    while let Some((c, r)) = stack.pop_back() {
        count += 1;

        for (dc, dr) in dirs {
            let nc = (c as isize) + dc;
            let nr = (r as isize) + dr;
            if nc < 0 || nr < 0 {
                continue;
            }
            let (uc, ur) = (nc as usize, nr as usize);
            if ur >= grid.len() || uc >= grid[ur].len() {
                continue;
            }

            if !visited[uc][ur] && grid[uc][ur] == target {
                visited[uc][ur] = true;
                stack.push_back((uc, ur));
            }
        }
    }

    count
}

fn get_region_perimeter(
    grid: Vec<Vec<char>>,
    start_c: usize,
    start_r: usize,
    visited: &mut Vec<Vec<bool>>,
) -> usize {
    let target = grid[start_c][start_r];
    let mut stack = VecDeque::new();
    stack.push_back((start_c, start_r));
    visited[start_c][start_r] = true;

    let dirs: [(isize, isize); 4] = [
        (-1, 0), // up
        (1, 0),  // down
        (0, -1), // left
        (0, 1),  // right
    ];
    let mut count = 0;

    while let Some((c, r)) = stack.pop_back() {
        for (dc, dr) in dirs {
            let nc = (c as isize) + dc;
            let nr = (r as isize) + dr;
            if nc < 0 || nr < 0 {
                count += 1;
                continue;
            }
            let (uc, ur) = (nc as usize, nr as usize);
            if ur >= grid.len() || uc >= grid[ur].len() {
                count += 1;
                continue;
            }

            if !visited[uc][ur] && grid[uc][ur] == target {
                visited[uc][ur] = true;
                stack.push_back((uc, ur));
            } else if grid[uc][ur] != target {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    println!("AOC 2023 day 12!");

    let input = read_to_string("input.txt").expect("err");

    let p1_result = solve_p1(input.as_str());

    println!("Part 1 solution: {}", p1_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_simple() {
        let input = r#"
        RRRRIICCFF
        RRRRIICCCF
        VVRRRCCFFF
        VVRCCCJFFF
        VVVVCJJCFE
        VVIVCCJJEE
        VVIIICJJEE
        MIIIIIJJEE
        MIIISIJEEE
        MMMISSJEEE
        "#;

        assert_eq!(solve_p1(input), 1930);
    }

    #[test]
    fn p1_simple_2() {
        let input = r#"
        OOOOO
        OXOXO
        OOOOO
        OXOXO
        OOOOO
        "#;

        assert_eq!(solve_p1(input), 772);
    }

    #[test]
    fn p1_simple_3() {
        let input = r#"
        AAAA
        BBCD
        BBCC
        EEEC
        "#;

        assert_eq!(solve_p1(input), 140);
    }
}
