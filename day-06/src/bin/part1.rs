use aocd::*;

#[aocd(2025, 6)]
fn main() {
    let input = input!();
    let sum = solve(&input);
    dbg!(sum);
}

fn parse_sheet(input: &str) -> Vec<(Vec<u64>, char)> {
    let mut columns: Vec<(Vec<u64>, char)> = Vec::new();
    input.trim().lines().for_each(|line| {
        line.split_whitespace()
            .enumerate()
            .for_each(|(col, entry)| {
                if let Ok(num) = entry.parse() {
                    if let Some(nums) = columns.get_mut(col) {
                        nums.0.push(num);
                    } else {
                        columns.push((vec![num], '-'));
                    }
                } else {
                    columns[col].1 = entry.chars().next().unwrap();
                }
            });
    });
    columns
}

fn solve(input: &str) -> u64 {
    let sheet = parse_sheet(input);
    sheet
        .iter()
        .map(|col| match col.1 {
            '+' => col.0.iter().sum::<u64>(),
            '*' => col.0.iter().product(),
            _ => panic!(),
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input), 4277556);
    }
}
