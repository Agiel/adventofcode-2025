use aocd::*;

#[aocd(2025, 3)]
fn main() {
    let input = input!();
    let sum = solve(&input);
    dbg!(sum);
}

fn solve(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .map(|bank| {
            bank.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .map(|bank| {
            let mut tens = 0;
            let mut index = 0;
            for i in 0..bank.len() - 1 {
                if bank[i] > tens {
                    tens = bank[i];
                    index = i;
                }
            }

            let mut ones = 0;
            for i in index + 1..bank.len() {
                if bank[i] > ones {
                    ones = bank[i];
                }
            }

            tens * 10 + ones
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
        assert_eq!(solve(input), 357);
    }
}
