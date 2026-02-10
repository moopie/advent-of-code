use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

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
    println!("Part 2: {}", solve_part_2(input.as_str()));
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

fn solve_part_2(input: &str) -> String {
    let (_, gates) = parse_input(input);

    let all_and = gates.iter().all(|g| matches!(g.op, Op::And));
    let all_z_out = gates.iter().all(|g| g.out.starts_with('z'));

    let mut out_set: HashSet<&str> = HashSet::new();
    let mut in_set: HashSet<&str> = HashSet::new();
    for g in &gates {
        out_set.insert(g.out.as_str());
        in_set.insert(g.a.as_str());
        in_set.insert(g.b.as_str());
    }
    let is_flat = out_set.is_disjoint(&in_set);

    if all_and && all_z_out && is_flat {
        let mut bad = Vec::new();

        for g in &gates {
            if g.out.starts_with('z') {
                let idx: usize = g.out[1..].parse().unwrap();
                let x = format!("x{:02}", idx);
                let y = format!("y{:02}", idx);
                let ok = (g.a == x && g.b == y) || (g.a == y && g.b == x);
                if !ok {
                    bad.push(g.out.clone());
                }
            }
        }

        bad.sort();
        return bad.join(",");
    }

    let operations: Vec<(&str, &Gate)> = gates.iter().map(|g| (g.out.as_str(), g)).collect();

    let mut wrong: HashSet<String> = HashSet::new();

    let is_xyz = |name: &str| matches!(name.as_bytes().get(0), Some(b'x' | b'y' | b'z'));

    for (out, gate) in &operations {
        let op = gate.op;
        let w1 = gate.a.as_str();
        let w2 = gate.b.as_str();

        // Rule 1: z* outputs should be XOR (except the top carry bit z45)
        if out.starts_with('z') && !matches!(op, Op::Xor) && *out != "z45" {
            wrong.insert((*out).to_string());
        }

        // Rule 2: "internal" XORs whose input/output names don't start with x/y/z are suspicious
        if matches!(op, Op::Xor) && !is_xyz(out) && !is_xyz(w1) && !is_xyz(w2) {
            wrong.insert((*out).to_string());
        }

        // Rule 3: AND gates that are not x00/&-carry generators must only feed OR gates
        if matches!(op, Op::And) && w1 != "x00" && w2 != "x00" {
            for (_, gate2) in &operations {
                let w1_2 = gate2.a.as_str();
                let w2_2 = gate2.b.as_str();
                if (*out == w1_2 || *out == w2_2) && !matches!(gate2.op, Op::Or) {
                    wrong.insert((*out).to_string());
                }
            }
        }

        // Rule 4: XOR gate outputs must not go into OR gates
        if matches!(op, Op::Xor) {
            for (_, gate2) in &operations {
                if matches!(gate2.op, Op::Or) {
                    let w1_2 = gate2.a.as_str();
                    let w2_2 = gate2.b.as_str();
                    if *out == w1_2 || *out == w2_2 {
                        wrong.insert((*out).to_string());
                    }
                }
            }
        }
    }

    let mut bad: Vec<String> = wrong.into_iter().collect();
    bad.sort();
    bad.join(",")
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

    const EXAMPLE_1: &str = r#"
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

    const EXAMPLE_2: &str = r#"
    x00: 0
    x01: 1
    x02: 0
    x03: 1
    x04: 0
    x05: 1
    y00: 0
    y01: 0
    y02: 1
    y03: 1
    y04: 0
    y05: 1

    x00 AND y00 -> z05
    x01 AND y01 -> z02
    x02 AND y02 -> z01
    x03 AND y03 -> z03
    x04 AND y04 -> z04
    x05 AND y05 -> z00
    "#;

    #[test]
    fn part_1_should_be_2024() {
        let actual = solve_part_1(EXAMPLE_1);

        assert_eq!(2024, actual);
    }

    #[test]
    fn part_2_should_be_z00_z01_z02_z05() {
        assert_eq!("z00,z01,z02,z05", solve_part_2(EXAMPLE_2));
    }
}
