use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug, Clone, Copy)]
enum Op {
    And,
    Or,
    Xor,
}

#[derive(Debug)]
struct Gate {
    a: String,
    b: String,
    op: Op,
    out: String,
}

fn main() {
    println!("AOC 2024 day 24!");

    let input = read_to_string("input.txt").unwrap();

    println!("Part 1: {}", solve_part_1(input.as_str()));
}

fn solve_part_1(input: &str) -> u64 {
    let (mut wires, mut gates) = parse_input(input);

    while !gates.is_empty() {
        let mut next = Vec::new();

        for gate in gates {
            let (Some(w1), Some(w2)) = (wires.get(&gate.a), wires.get(&gate.b)) else {
                next.push(gate);
                continue;
            };

            let value = match gate.op {
                Op::And => w1 & w2,
                Op::Or => w1 | w2,
                Op::Xor => w1 ^ w2,
            };

            wires.insert(gate.out.clone(), value);
        }

        gates = next;
    }

    build_z_value(&wires)
}

fn parse_input(input: &str) -> (HashMap<String, u8>, Vec<Gate>) {
    let (values_part, gates_part) = input.trim().split_once("\n\n").unwrap();

    let wires = parse_wires(values_part);
    let gates = parse_gates(gates_part);

    (wires, gates)
}

fn parse_wires(s: &str) -> HashMap<String, u8> {
    s.lines()
        .map(|line| {
            let (name, val) = line.split_once(": ").unwrap();
            (name.trim().to_string(), val.trim().parse::<u8>().unwrap())
        })
        .collect()
}

fn parse_gates(s: &str) -> Vec<Gate> {
    s.lines()
        .map(|line| {
            // ntg XOR fgs -> mjb
            let (lhs, out) = line.split_once(" -> ").unwrap();
            let mut it = lhs.split_whitespace();

            let a = it.next().unwrap().to_string();
            let op = match it.next().unwrap() {
                "AND" => Op::And,
                "OR" => Op::Or,
                "XOR" => Op::Xor,
                x => panic!("unknown op {x}"),
            };
            let b = it.next().unwrap().to_string();

            Gate {
                a,
                b,
                op,
                out: out.to_string(),
            }
        })
        .collect()
}

fn build_z_value(wires: &HashMap<String, u8>) -> u64 {
    let mut result = 0u64;

    for (k, &v) in wires {
        if let Some(idx) = k.strip_prefix('z') {
            let bit = idx.parse::<u32>().unwrap();
            result |= (v as u64) << bit;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"
    x00: 1
    x01: 0
    x02: 1
    x03: 1
    x04: 0
    y00: 1
    y01: 1
    y02: 1
    y03: 1
    y04: 1

    ntg XOR fgs -> mjb
    y02 OR x01 -> tnw
    kwq OR kpj -> z05
    x00 OR x03 -> fst
    tgd XOR rvg -> z01
    vdt OR tnw -> bfw
    bfw AND frj -> z10
    ffh OR nrd -> bqk
    y00 AND y03 -> djm
    y03 OR y00 -> psh
    bqk OR frj -> z08
    tnw OR fst -> frj
    gnj AND tgd -> z11
    bfw XOR mjb -> z00
    x03 OR x00 -> vdt
    gnj AND wpb -> z02
    x04 AND y00 -> kjc
    djm OR pbm -> qhw
    nrd AND vdt -> hwm
    kjc AND fst -> rvg
    y04 OR y02 -> fgs
    y01 AND x02 -> pbm
    ntg OR kjc -> kwq
    psh XOR fgs -> tgd
    qhw XOR tgd -> z09
    pbm OR djm -> kpj
    x03 XOR y03 -> ffh
    x00 XOR y04 -> ntg
    bfw OR bqk -> z06
    nrd XOR fgs -> wpb
    frj XOR qhw -> z04
    bqk OR frj -> z07
    y03 OR x01 -> nrd
    hwm AND bqk -> z03
    tgd XOR rvg -> z12
    tnw OR pbm -> gnj
    "#;

    #[test]
    fn part_1_should_be_2024() {
        let actual = solve_part_1(EXAMPLE);

        assert_eq!(2024, actual);
    }
}
