use std::{
    cell::RefCell,
    collections::{BTreeMap, BTreeSet},
    panic,
    rc::Rc,
};

use aocd::*;

#[aocd(2025, 8)]
fn main() {
    let input = input!();
    let sum = solve(&input);
    dbg!(sum);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn distance(&self, other: &Point) -> i64 {
        ((self.x - other.x) * (self.x - other.x)
            + (self.y - other.y) * (self.y - other.y)
            + (self.z - other.z) * (self.z - other.z))
            .isqrt()
    }
}

fn parse(input: &str) -> Vec<Point> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split(",")
                .map(|num| num.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|vec| Point {
            x: vec[0],
            y: vec[1],
            z: vec[2],
        })
        .collect()
}

fn solve(input: &str) -> i64 {
    let boxes = parse(input);
    let mut pairs = Vec::new();
    for (i, a) in boxes.iter().enumerate() {
        for b in boxes.iter().skip(i + 1) {
            pairs.push((a.distance(&b), a, b));
        }
    }
    pairs.sort();

    let circuits: Vec<Rc<RefCell<BTreeSet<Point>>>> = boxes
        .iter()
        .map(|b| Rc::new(RefCell::new(BTreeSet::from([b.clone()]))))
        .collect();

    let mut box_to_circuit =
        BTreeMap::from_iter(boxes.iter().enumerate().map(|(i, b)| (b.clone(), i)));

    for (_, a, b) in pairs.iter() {
        let c_a_id = *box_to_circuit.get(a).unwrap();
        let c_b_id = *box_to_circuit.get(b).unwrap();
        if c_a_id == c_b_id {
            continue;
        }

        let c_a = circuits[c_a_id].clone();
        let c_b = circuits[c_b_id].clone();
        let (larger, smaller, larger_id) = if c_a.borrow().len() >= c_b.borrow().len() {
            (c_a, c_b, c_a_id)
        } else {
            (c_b, c_a, c_b_id)
        };

        smaller.borrow().iter().for_each(|b| {
            box_to_circuit.insert(b.clone(), larger_id);
        });

        larger.borrow_mut().append(&mut smaller.borrow_mut());
        if larger.borrow().len() == boxes.len() {
            return a.x * b.x;
        }
    }
    panic!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input), 25272);
    }
}
