use std::{
    collections::{BTreeSet, BinaryHeap},
    time::Instant,
};

use aocd::*;

#[aocd(2025, 9)]
fn main() {
    let input = input!();

    let now = Instant::now();
    let sum = solve(&input);
    println!("{sum} in {:?}", now.elapsed());
}

fn boundary_set(corners: &Vec<(u64, u64)>) -> BTreeSet<(u64, u64)> {
    let mut boundary = BTreeSet::new();
    corners
        .iter()
        .zip(corners.iter().cycle().skip(1))
        .for_each(|(a, b)| {
            if a.0 == b.0 {
                (a.1.min(b.1)..=a.1.max(b.1)).for_each(|y| {
                    boundary.insert((a.0, y));
                });
            } else {
                (a.0.min(b.0)..=a.0.max(b.0)).for_each(|x| {
                    boundary.insert((x, a.1));
                });
            }
        });
    boundary
}

fn solve(input: &str) -> u64 {
    let tiles = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect::<Vec<(u64, u64)>>();

    let boundary = boundary_set(&tiles);

    let mut heap = tiles
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            tiles
                .iter()
                .skip(i)
                .map(|b| ((a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1), a, b))
                .collect::<Vec<_>>()
        })
        .collect::<BinaryHeap<_>>();

    while let Some((area, &a, &b)) = heap.pop() {
        let top = (a.0.min(b.0) + 1, a.1.min(b.1) + 1);
        let bottom = (a.0.max(b.0) - 1, a.1.max(b.1) - 1);
        let rect = boundary_set(&vec![top, (bottom.0, top.1), bottom, (top.0, bottom.1)]);

        // for y in 0..9 {
        //     for x in 0..14 {
        //         if rect.contains(&(x, y)) {
        //             print!("#");
        //         } else if boundary.contains(&(x, y)) {
        //             print!("X");
        //         } else {
        //             print!(".");
        //         }
        //     }
        //     print!("\n");
        // }

        if boundary.is_disjoint(&rect) {
            return area;
        }
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input), 24);
    }
}
