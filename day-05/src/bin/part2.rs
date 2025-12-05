use aocd::*;

#[aocd(2025, 5)]
fn main() {
    let input = input!();
    let sum = solve(&input);
    dbg!(sum);
}

fn parse_ranges(input: &str) -> Vec<(u64, u64)> {
    let (ranges, _) = input.trim().split_once("\n\n").unwrap();

    ranges
        .lines()
        .map(|range| {
            let (min, max) = range.split_once('-').unwrap();
            (min.parse().unwrap(), max.parse().unwrap())
        })
        .collect::<Vec<_>>()
}

fn solve(input: &str) -> u64 {
    let mut ranges = parse_ranges(input);
    ranges.sort();

    let mut i = 0;
    while i < ranges.len() - 1 {
        if ranges[i + 1].0 <= ranges[i].1 + 1 {
            ranges[i].1 = ranges[i].1.max(ranges[i + 1].1);
            ranges.remove(i + 1);
        } else {
            i += 1;
        }
    }
    ranges.iter().map(|range| range.1 - range.0 + 1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input), 14);
    }
}
