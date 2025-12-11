use std::time::Instant;

use aocd::*;
use z3::{Optimize, ast::Int};

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
    buttons: Vec<Vec<usize>>,
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
                                .map(|btn| btn.parse().unwrap())
                                .collect(),
                        );
                    }
                    Some('{') => {
                        jolts = word[1..word.len() - 1]
                            .split(",")
                            .map(|jlt| jlt.parse().unwrap())
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

fn min_presses(buttons: &Vec<Vec<usize>>, jolts: &Vec<u32>) -> u64 {
    let opt = Optimize::new();
    let n = (0..buttons.len())
        .map(|b| {
            let n = Int::new_const(format!("button{b}"));
            opt.assert(&n.ge(0));
            n
        })
        .collect::<Vec<_>>();
    jolts.iter().enumerate().for_each(|(i, jolts)| {
        let eq: Int = buttons
            .iter()
            .enumerate()
            .filter_map(|(j, b)| b.contains(&i).then_some(j))
            .map(|j| &n[j])
            .sum();
        opt.assert(&eq.eq(*jolts));
    });

    let total: Int = n.iter().sum();
    opt.minimize(&total);

    if let z3::SatResult::Sat = opt.check(&[]) {
        // println!("{opt:?}");
        let model = opt.get_model().unwrap();
        return model.eval(&total, true).unwrap().as_u64().unwrap();
    }
    panic!()
}

fn solve(input: &str) -> u64 {
    let machines = parse(input);

    // dbg!(&machines);

    machines
        .iter()
        .map(|m| min_presses(&m.buttons, &m.jolts))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input), 33);
    }
}
