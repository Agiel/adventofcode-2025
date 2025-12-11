use std::{collections::BTreeMap, time::Instant};

use aocd::*;

#[aocd(2025, 10)]
fn main() {
    let input = input!();

    let now = Instant::now();
    let sum = solve(&input);
    println!("{sum} in {:?}", now.elapsed());
}

#[derive(Debug)]
struct Machine {
    lights: u32,
    buttons: Vec<u32>,
    jolts: Vec<u32>,
}

fn parse(input: &str) -> Vec<Machine> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut lights = 0;
            let mut buttons = Vec::new();
            let mut jolts = Vec::new();
            line.split_whitespace().for_each(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    Some('[') => {
                        lights = word[1..word.len() - 1]
                            .chars()
                            .enumerate()
                            .map(|(i, c)| if c == '#' { 2u32.pow(i as u32) } else { 0 })
                            .sum()
                    }
                    Some('(') => {
                        buttons.push(
                            word[1..word.len() - 1]
                                .split(",")
                                .map(|btn| 2u32.pow(btn.parse::<u32>().unwrap()))
                                .sum(),
                        );
                    }
                    Some('{') => {
                        jolts = word[1..word.len() - 1]
                            .split(",")
                            .map(|jlt| jlt.parse::<u32>().unwrap())
                            .collect()
                    }
                    _ => (),
                }
            });
            Machine {
                lights,
                buttons,
                jolts,
            }
        })
        .collect()
}

fn min_presses(
    state: u32,
    goal: u32,
    buttons: Vec<u32>,
    cache: &mut BTreeMap<(u32, Vec<u32>), u32>,
) -> u32 {
    if state == goal {
        return 0;
    }

    if let Some(_button) = buttons.iter().find(|&b| state ^ b == goal) {
        return 1;
    }

    if let Some(&presses) = cache.get(&(state, buttons.clone())) {
        return presses;
    }

    let candidates = buttons
        .iter()
        .filter(|&b| (state ^ goal) & b != 0)
        .collect::<Vec<_>>();

    if candidates.is_empty() {
        return u32::MAX;
    }

    let presses = 1u32.saturating_add(
        candidates
            .iter()
            .map(|&b| {
                let buttons = buttons.iter().filter(|&b2| b2 != b).cloned().collect();
                min_presses(state ^ b, goal, buttons, cache)
            })
            .min()
            .unwrap(),
    );

    cache.insert((state, buttons), presses);
    return presses;
}

fn solve(input: &str) -> u32 {
    let machines = parse(input);

    // dbg!(&machines);

    machines
        .iter()
        .map(|m| {
            let mut cache = BTreeMap::new();
            min_presses(0, m.lights, m.buttons.clone(), &mut cache)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input), 7);
    }
}
