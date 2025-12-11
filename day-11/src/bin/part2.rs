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

fn count_ways<'a>(
    pos: &'a str,
    goal: &str,
    hits: &(bool, bool),
    graph: &'a BTreeMap<String, BTreeSet<String>>,
    cache: &mut BTreeMap<(&'a str, (bool, bool)), u64>,
) -> u64 {
    if pos == goal {
        if hits.0 && hits.1 {
            return 1;
        } else {
            return 0;
        }
    }

    if let Some(&count) = cache.get(&(pos, *hits)) {
        return count;
    }

    let mut hits = *hits;
    if pos == "fft" {
        hits.0 = true;
    } else if pos == "dac" {
        hits.1 = true;
    }

    let count = graph[pos]
        .iter()
        .map(|connection| count_ways(connection, goal, &hits, graph, cache))
        .sum();
    cache.insert((pos, hits), count);
    count
}

fn solve(input: &str) -> u64 {
    let devices = parse(input);
    let mut cache = BTreeMap::new();
    let hits = (false, false);
    count_ways("svr", "out", &hits, &devices, &mut cache)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example2.txt");
        assert_eq!(solve(input), 2);
    }
}
