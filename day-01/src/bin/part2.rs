use aocd::*;

#[aocd(2025, 1)]
fn main() {
    let input = input!();
    let sum = solve(&input);
    dbg!(sum);
}

fn solve(input: &str) -> i32 {
    let (_, password) = input.lines().fold((50, 0), |(position, password), line| {
        if line.len() == 0 {
            return (position, password);
        }
        let (dir, steps) = line.split_at(1);
        let steps = match dir {
            "L" => -steps.parse::<i32>().unwrap(),
            "R" => steps.parse().unwrap(),
            _ => panic!(),
        };
        let mut add = 0;

        // When we start at 0 the next step will count it when going left
        if position == 0 && steps < 0 {
            add -= 1;
        }

        let position = position + steps;
        add += position.div_euclid(100).abs();
        let position = position.rem_euclid(100);

        // When we end at 0 the previous step will miss it when going left
        if position == 0 && steps < 0 {
            add += 1;
        }

        (position, password + add)
    });
    return password;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input), 6);
    }
}
