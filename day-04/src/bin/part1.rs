use std::collections::BTreeSet;

use aocd::*;

#[aocd(2025, 4)]
fn main() {
    let input = input!();
    let sum = solve(&input);
    dbg!(sum);
}

fn solve(input: &str) -> usize {
    let rolls = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, c)| (c == '@').then_some((x as i32, y as i32)))
        })
        .collect::<BTreeSet<_>>();

    rolls
        .iter()
        .filter_map(|(x, y)| {
            (vec![
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ]
            .iter()
            .filter_map(|(x2, y2)| rolls.contains(&(x + x2, y + y2)).then_some(()))
            .count()
                < 4)
            .then_some(())
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input), 13);
    }
}
