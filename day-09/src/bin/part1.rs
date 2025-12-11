use std::{collections::BinaryHeap, time::Instant};

use aocd::*;

#[aocd(2025, 9)]
fn main() {
    let input = input!();

    let now = Instant::now();
    let sum = solve(&input);
    println!("{sum} in {:?}", now.elapsed());
}

fn solve(input: &str) -> u64 {
    let tiles = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect::<Vec<(u64, u64)>>();

    let mut heap = tiles
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            tiles
                .iter()
                .skip(i)
                .map(|b| (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1))
                .collect::<Vec<_>>()
        })
        .collect::<BinaryHeap<_>>();

    heap.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input), 50);
    }
}
