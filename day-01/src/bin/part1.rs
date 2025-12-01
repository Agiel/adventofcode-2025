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
        let position = (position + steps).rem_euclid(100);
        if position == 0 {
            (position, password + 1)
        } else {
            (position, password)
        }
    });
    return password;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input), 3);
    }
}
