use std::{collections::VecDeque, fmt, fs::read_to_string};

struct Cpu {
    a: u64,
    b: u64,
    c: u64,
    ip: usize,
    program: Vec<u64>,
    output: Vec<u64>,
}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ip={:<2} A={:<6} B={:<6} C={:<6} | out={:?}",
            self.ip, self.a, self.b, self.c, self.output,
        )
    }
}

fn main() {
    println!("AOC 2024 day 17!");

    let input = read_to_string("input.txt").expect("Couldn't read input.txt");

    let part1 = solve_part1(input.as_str());

    println!("Part 1 solution: {}", part1);

    let part2 = solve_part2(input.as_str());

    println!("Part 2 solution: {}", part2);
}

fn solve_part2(input: &str) -> u64 {
    let base = parse_input(input);
    let program = base.program.clone();
    let n = program.len();

    // queue of (offset, value_for_A)
    let mut queue: VecDeque<(usize, u64)> = VecDeque::new();
    queue.push_back((n - 1, 0));

    while let Some((offset, value)) = queue.pop_front() {
        for digit in 0..8u64 {
            let new_value = (value << 3) | digit;

            let out = run_with_a(&base, new_value);

            // output must equal the suffix program[offset..]
            if out == program[offset..] {
                if offset == 0 {
                    return new_value; // smallest A found
                }

                queue.push_back((offset - 1, new_value));
            }
        }
    }

    panic!("No solution found for part 2");
}

fn solve_part1(input: &str) -> String {
    let mut cpu = parse_input(input);

    run(&mut cpu);

    cpu.output
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn parse_input(input: &str) -> Cpu {
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut program = Vec::new();

    for line in input.lines().map(str::trim).filter(|l| !l.is_empty()) {
        if let Some(v) = line.strip_prefix("Register A: ") {
            a = v.parse().unwrap();
        } else if let Some(v) = line.strip_prefix("Register B: ") {
            b = v.parse().unwrap();
        } else if let Some(v) = line.strip_prefix("Register C: ") {
            c = v.parse().unwrap();
        } else if let Some(v) = line.strip_prefix("Program: ") {
            program = v.split(',').map(|x| x.parse::<u64>().unwrap()).collect();
        }
    }

    Cpu {
        a,
        b,
        c,
        ip: 0,
        program,
        output: Vec::new(),
    }
}

fn combo(cpu: &Cpu, x: u64) -> u64 {
    match x {
        0..=3 => x,
        4 => cpu.a,
        5 => cpu.b,
        6 => cpu.c,
        _ => unreachable!(),
    }
}

fn step(cpu: &mut Cpu) {
    let op = cpu.program[cpu.ip];
    let arg = cpu.program[cpu.ip + 1];

    match op {
        0 => cpu.a >>= combo(cpu, arg),   // adv
        1 => cpu.b ^= arg,                // bxl
        2 => cpu.b = combo(cpu, arg) % 8, // bst
        3 => {
            // jnz
            if cpu.a != 0 {
                cpu.ip = arg as usize;
                return;
            }
        }
        4 => cpu.b ^= cpu.c,                       // bxc
        5 => cpu.output.push(combo(cpu, arg) % 8), // out
        6 => cpu.b = cpu.a >> combo(cpu, arg),     // bdv
        7 => cpu.c = cpu.a >> combo(cpu, arg),     // cdv
        _ => unreachable!(),
    }

    cpu.ip += 2;
}

fn run(cpu: &mut Cpu) {
    while cpu.ip + 1 < cpu.program.len() {
        step(cpu);
    }
}

fn run_with_a(base_cpu: &Cpu, a: u64) -> Vec<u64> {
    let mut cpu = Cpu {
        a,
        b: 0,
        c: 0,
        ip: 0,
        program: base_cpu.program.clone(),
        output: Vec::new(),
    };

    run(&mut cpu);
    cpu.output
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"
    Register A: 729
    Register B: 0
    Register C: 0

    Program: 0,1,5,4,3,0
    "#;

    const EXAMPLE_INPUT2: &str = r#"
    Register A: 2024
    Register B: 0
    Register C: 0

    Program: 0,3,5,4,3,0
    "#;

    #[test]
    fn part1_should_be_4_6_3_5_6_3_5_2_1_0() {
        let solution = solve_part1(EXAMPLE_INPUT);

        assert_eq!("4,6,3,5,6,3,5,2,1,0", solution);
    }

    #[test]
    fn part2_should_be_117440() {
        let solution = solve_part2(EXAMPLE_INPUT2);
        assert_eq!(117440, solution);
    }
}
