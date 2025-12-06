use aocd::*;

#[aocd(2025, 6)]
fn main() {
    let input = input!();
    let sum = solve(&input);
    dbg!(sum);
}

fn parse_sheet(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn solve(input: &str) -> u64 {
    let sheet = parse_sheet(input);
    let mut sum = 0;
    let mut col: Vec<u64> = Vec::new();
    let mut x = sheet[0].len() as isize - 1;
    while x >= 0 {
        let mut num = 0;
        for y in 0..sheet.len() {
            if y == sheet.len() - 1 {
                col.push(num);
                match sheet[y][x as usize] {
                    '+' => {
                        sum += col.iter().sum::<u64>();
                        col.clear();
                        x -= 1;
                    }
                    '*' => {
                        sum += col.iter().product::<u64>();
                        col.clear();
                        x -= 1;
                    }
                    _ => (),
                }
            } else {
                if let Some(n) = sheet[y][x as usize].to_digit(10) {
                    num = num * 10 + n as u64;
                }
            }
        }
        x -= 1;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input), 3263827);
    }
}
