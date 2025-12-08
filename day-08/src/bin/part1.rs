use std::{
    cell::RefCell,
    collections::{BTreeMap, BTreeSet},
    rc::Rc,
    time::Instant,
};

use aocd::*;

#[aocd(2025, 8)]
fn main() {
    let input = input!();

    let now = Instant::now();
    let sum = solve(&input, 1000);
    println!("{sum} in {:?} with Map<Set>", now.elapsed());

    let now = Instant::now();
    let sum = solve_uf(&input, 1000);
    println!("{sum} in {:?} with UnionFind", now.elapsed());
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

fn solve(input: &str, connections: usize) -> usize {
    let boxes = parse(input);
    let mut pairs = Vec::new();
    for (i, a) in boxes.iter().enumerate() {
        for b in boxes.iter().skip(i + 1) {
            pairs.push((a.distance(&b), a, b));
        }
    }
    pairs.sort();

    let mut circuits: Vec<Rc<RefCell<BTreeSet<Point>>>> = boxes
        .iter()
        .map(|b| Rc::new(RefCell::new(BTreeSet::from([b.clone()]))))
        .collect();

    let mut box_to_circuit =
        BTreeMap::from_iter(boxes.iter().enumerate().map(|(i, b)| (b.clone(), i)));

    pairs.iter().take(connections).for_each(|(_, a, b)| {
        let c_a_id = *box_to_circuit.get(a).unwrap();
        let c_b_id = *box_to_circuit.get(b).unwrap();
        if c_a_id == c_b_id {
            return;
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
    });

    circuits.sort_by(|a, b| b.borrow().len().cmp(&a.borrow().len()));

    circuits.iter().take(3).map(|c| c.borrow().len()).product()
}

struct UnionFind {
    parents: Vec<usize>,
}

impl UnionFind {
    fn new(len: usize) -> Self {
        Self {
            parents: (0..len).collect(),
        }
    }
    fn find(&self, i: usize) -> usize {
        if self.parents[i] == i {
            return i;
        }
        return self.find(self.parents[i]);
    }

    fn union(&mut self, i: usize, j: usize) {
        let p = self.find(j);
        self.parents[p] = self.find(i);
    }
}

fn solve_uf(input: &str, connections: usize) -> i64 {
    let boxes = parse(input);
    let mut pairs = Vec::new();
    for (i, a) in boxes.iter().enumerate() {
        for (j, b) in boxes.iter().enumerate().skip(i + 1) {
            pairs.push((a.distance(&b), i, j));
        }
    }
    pairs.sort();

    let mut uf = UnionFind::new(boxes.len());
    for &(_, a, b) in pairs.iter().take(connections) {
        if uf.find(a) == uf.find(b) {
            continue;
        }
        uf.union(a, b);
    }

    let mut circuits = vec![0; boxes.len()];
    for i in 0..boxes.len() {
        circuits[uf.find(i)] += 1;
    }
    circuits.sort();
    circuits.iter().rev().take(3).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve(input, 10), 40);
    }
}
