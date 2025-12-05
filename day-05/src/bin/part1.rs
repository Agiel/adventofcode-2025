use aocd::*;

#[aocd(2025, 5)]
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

fn parse_ranges(input: &str) -> (Vec<Range>, Vec<u64>) {
    let (ranges, ids) = input.trim().split_once("\n\n").unwrap();

    let ranges = ranges
        .lines()
        .map(|range| {
            let (min, max) = range.split_once('-').unwrap();
            let range = Range {
                min: min.parse().unwrap(),
                max: max.parse().unwrap(),
            };
            range
        })
        .collect();

    let ids = ids.lines().map(|l| l.parse().unwrap()).collect();

    (ranges, ids)
}

fn solve(input: &str) -> usize {
    let (ranges, ids) = parse_ranges(input);
    ids.iter()
        .filter_map(|id| ranges.iter().any(|r| r.is_inside(*id)).then_some(()))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input), 3);
    }
}
