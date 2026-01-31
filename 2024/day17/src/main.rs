use std::fs::read_to_string;

struct Cpu {
    a: u64,
    b: u64,
    c: u64,
    ip: usize,
    program: Vec<u64>,
    output: Vec<u64>,
}

fn main() {
    println!("AOC 2024 day 17!");

    let input = read_to_string("input.txt").expect("Couldn't read input.txt");

    let part1 = solve_part1(input.as_str());

    println!("Part 1 solution: {}", part1);

    let part2 = solve_part2(input.as_str());

    println!("Part 2 solution: {}", part2);
}

fn solve_part2(input: &str) -> String {
    "".to_string()
}

fn solve_part1(input: &str) -> String {
    let mut cpu = parse_input(input);

    let mut cpu = Cpu {
        a: cpu.a,
        b: cpu.b,
        c: cpu.c,
        ip: 0,
        program: cpu.program,
        output: Vec::new(),
    };

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

#[cfg(test)]
mod tests {
    use crate::solve_part1;

    const EXAMPLE_INPUT: &str = r#"
    Register A: 729
    Register B: 0
    Register C: 0

    Program: 0,1,5,4,3,0
    "#;

    #[test]
    fn part1_should_be_4_6_3_5_6_3_5_2_1_0() {
        let solution = solve_part1(EXAMPLE_INPUT);

        assert_eq!("4,6,3,5,6,3,5,2,1,0", solution);
    }
}
