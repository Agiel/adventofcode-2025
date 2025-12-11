use std::collections::{BTreeMap, BTreeSet};

use aocd::*;

#[aocd(2025, 11)]
fn main() {
    let input = input!();
    let sum = solve(&input);
    dbg!(sum);
}

fn parse(input: &str) -> BTreeMap<String, BTreeSet<String>> {
    input
        .lines()
        .map(|line| {
            let (key, val) = line.split_once(": ").unwrap();
            let val = val.split_whitespace().map(|s| s.to_string()).collect();
            (key.to_string(), val)
        })
        .collect()
}

fn count_ways(pos: &String, goal: &String, graph: &BTreeMap<String, BTreeSet<String>>) -> u64 {
    if pos == goal {
        return 1;
    }

    graph[pos]
        .iter()
        .map(|connection| count_ways(connection, goal, graph))
        .sum()
}

fn solve(input: &str) -> u64 {
    let devices = parse(input);
    count_ways(&"you".to_string(), &"out".to_string(), &devices)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input), 5);
    }
}
