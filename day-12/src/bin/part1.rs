use aocd::*;

#[aocd(2025, 12)]
fn main() {
    let input = input!();
    let sum = solve(&input);
    dbg!(sum);
}

struct Region {
    size: (u32, u32),
    shapes: Vec<u32>,
}

struct Input {
    shapes: Vec<Vec<Vec<bool>>>,
    regions: Vec<Region>,
}

fn parse(input: &str) -> Input {
    let input = input.split("\n\n").collect::<Vec<_>>();
    let shapes = input
        .iter()
        .take(input.len() - 1)
        .map(|s| {
            s.lines()
                .skip(1)
                .map(|line| line.chars().map(|c| c == '#').collect())
                .collect()
        })
        .collect();

    let regions = input
        .last()
        .unwrap()
        .lines()
        .map(|line| {
            let (size, shapes) = line.split_once(": ").unwrap();
            let size = size
                .split_once("x")
                .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                .unwrap();
            let shapes = shapes
                .split_whitespace()
                .map(|count| count.parse().unwrap())
                .collect();
            Region { size, shapes }
        })
        .collect();
    Input { shapes, regions }
}

fn solve(input: &str) -> usize {
    let input = parse(input);
    input
        .regions
        .iter()
        .filter(|r| {
            let area = r.size.0 * r.size.1;
            if area >= r.shapes.iter().sum::<u32>() * 9 {
                // Fits everything unpacked
                true
            } else if area
                < r.shapes
                    .iter()
                    .enumerate()
                    .map(|(i, s)| {
                        s * input.shapes[i]
                            .iter()
                            .flat_map(|row| row)
                            .filter(|&part| *part)
                            .count() as u32
                    })
                    .sum()
            {
                // No chance of fitting even with perfect packing
                false
            } else {
                // Start packing
                unimplemented!("NP-hard problem. Good luck, have fun.")
            }
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input), 5);
    }
}
