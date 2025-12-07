use std::collections::BTreeMap;

use aocd::*;

#[aocd(2025, 7)]
fn main() {
    let input = input!();
    let sum = solve(&input);
    dbg!(sum);
}

fn solve(input: &str) -> u64 {
    let mut beams = BTreeMap::new();
    for line in input.lines() {
        for (col, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    beams.insert(col, 1);
                }
                '^' if beams.contains_key(&col) => {
                    let worlds = beams.remove(&col).unwrap();
                    beams
                        .entry(col - 1)
                        .and_modify(|w| *w += worlds)
                        .or_insert(worlds);
                    beams
                        .entry(col + 1)
                        .and_modify(|w| *w += worlds)
                        .or_insert(worlds);
                }
                _ => (),
            }
        }
    }
    beams.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input), 40);
    }
}
