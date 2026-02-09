use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    println!("AOC 2024 day 23!");
    let input = read_to_string("input.txt").unwrap();

    println!("Part 1: {}", solve_part_1(input.as_str()));
    println!("Part 2: {}", solve_part_2(input.as_str()));
}

fn solve_part_1(input: &str) -> i64 {
    let pairs = parse_input(input);

    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();

    for (a, b) in pairs {
        graph.entry(a).or_default().insert(b);
        graph.entry(b).or_default().insert(a);
    }

    let nodes: Vec<&str> = graph.keys().copied().collect();
    let mut count = 0;

    for i in 0..nodes.len() {
        for j in i + 1..nodes.len() {
            for k in j + 1..nodes.len() {
                let a = nodes[i];
                let b = nodes[j];
                let c = nodes[k];

                if graph[a].contains(b)
                    && graph[a].contains(c)
                    && graph[b].contains(c)
                    && (a.starts_with('t') || b.starts_with('t') || c.starts_with('t'))
                {
                    count += 1;
                }
            }
        }
    }

    count
}

fn solve_part_2(input: &str) -> String {
    let pairs = parse_input(input);
    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();

    for (a, b) in pairs {
        graph.entry(a).or_default().insert(b);
        graph.entry(b).or_default().insert(a);
    }

    let nodes: Vec<&str> = graph.keys().copied().collect();

    let mut best: Vec<&str> = Vec::new();
    let mut current: Vec<&str> = Vec::new();

    expand(&graph, nodes, &mut current, &mut best);

    best.sort();
    best.join(",")
}

fn parse_input(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .filter(|x| !x.trim().is_empty())
        .map(|x| x.trim().split_once("-").unwrap())
        .collect()
}

fn expand<'a>(
    graph: &HashMap<&'a str, HashSet<&'a str>>,
    candidates: Vec<&'a str>,
    current: &mut Vec<&'a str>,
    best: &mut Vec<&'a str>,
) {
    if current.len() > best.len() {
        *best = current.clone();
    }

    for (i, &v) in candidates.iter().enumerate() {
        if current.iter().all(|&u| graph[u].contains(v)) {
            current.push(v);

            let next_candidates = candidates[i + 1..]
                .iter()
                .copied()
                .filter(|&u| graph[v].contains(u))
                .collect();

            expand(graph, next_candidates, current, best);
            current.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"
    kh-tc
    qp-kh
    de-cg
    ka-co
    yn-aq
    qp-ub
    cg-tb
    vc-aq
    tb-ka
    wh-tc
    yn-cg
    kh-ub
    ta-co
    de-co
    tc-td
    tb-wq
    wh-td
    ta-ka
    td-qp
    aq-cg
    wq-ub
    ub-vc
    de-ta
    wq-aq
    wq-vc
    wh-yn
    ka-de
    kh-ta
    co-tc
    wh-qp
    tb-vc
    td-yn
    "#;

    #[test]
    fn part_1_should_be_7() {
        let actual = solve_part_1(EXAMPLE);

        assert_eq!(7, actual);
    }

    #[test]
    fn part_2_should_be_co_de_ka_ta() {
        let actual = solve_part_2(EXAMPLE);

        assert_eq!("co,de,ka,ta", actual);
    }
}
