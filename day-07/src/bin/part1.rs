use std::collections::BTreeSet;

use aocd::*;

#[aocd(2025, 7)]
fn main() {
    let input = input!();
    let sum = solve(&input);
    dbg!(sum);
}

fn solve(input: &str) -> u64 {
    let mut beams = BTreeSet::new();
    let mut splits = 0;
    for line in input.lines() {
        for (col, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    beams.insert(col);
                }
                '^' if beams.contains(&col) => {
                    splits += 1;
                    beams.remove(&col);
                    beams.insert(col - 1);
                    beams.insert(col + 1);
                }
                _ => (),
            }
        }
    }
    splits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input), 21);
    }
}
