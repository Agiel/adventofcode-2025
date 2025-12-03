use aocd::*;

#[aocd(2025, 3)]
fn main() {
    let input = input!();
    let sum = solve(&input);
    dbg!(sum);
}

fn solve(input: &str) -> u64 {
    input
        .trim()
        .lines()
        .map(|bank| {
            bank.chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect::<Vec<_>>()
        })
        .map(|bank| {
            let mut start = 0;
            let mut end = bank.len() - 12;
            let mut joltage = Vec::new();
            while joltage.len() < 12 {
                let mut max = 0;
                let mut index = 0;
                for i in start..=end {
                    if bank[i] > max {
                        max = bank[i];
                        index = i;
                    }
                }
                joltage.push(max);
                start = index + 1;
                end = bank.len() - 12 + joltage.len();
            }

            joltage
                .iter()
                .enumerate()
                .map(|(i, n)| n * 10u64.pow(11 - i as u32))
                .sum::<u64>()
        })
        .inspect(|v| println!("{v}"))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input), 3121910778619);
    }
}
