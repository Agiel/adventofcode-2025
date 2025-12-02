use std::collections::BTreeSet;

use aocd::*;

#[aocd(2025, 2)]
fn main() {
    let input = input!();
    let sum = solve(&input);
    dbg!(sum);
}

struct Range {
    min: u64,
    max: u64,
}

impl Range {
    fn is_inside(&self, id: u64) -> bool {
        id >= self.min && id <= self.max
    }
}

fn parse_ranges(input: &str) -> (Vec<Range>, u64) {
    let mut max_id = 0;
    let ranges = input.trim().split(',').map(|range| {
        let (min, max) = range.split_once('-').unwrap();
        let range = Range {
            min: min.parse().unwrap(),
            max: max.parse().unwrap(),
        };
        max_id = max_id.max(range.max);
        range
    }).collect();
    (ranges, max_id)
}

fn solve(input: &str) -> u64 {
    let (ranges, max) = parse_ranges(input);
    let mut i = 1u64;
    let mut found = BTreeSet::new();
    loop {
        // Check one repetition first so we can break early
        let mut id = i;
        id = id * 10u64.pow(i.ilog10() + 1) + i;
        if id > max {
            break;
        }
        if ranges.iter().any(|range| range.is_inside(id)) {
            found.insert(id);
        }
        // Check more repetitions
        loop {
            id = id * 10u64.pow(i.ilog10() + 1) + i;
            if id > max {
                break;
            }
            if ranges.iter().any(|range| range.is_inside(id)) {
                found.insert(id);
            }
        }
        i += 1;
    }
    found.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input), 4174379265);
    }
}
