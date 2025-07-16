use std::collections::HashMap;

use crate::Solution;

pub struct Day01;

impl Solution for Day01 {
    type Part1 = u32;
    type Part2 = u32;

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        let (mut c1, mut c2) = parse_input(input);

        c1.sort_unstable();
        c2.sort_unstable();

        c1.iter().zip(c2.iter()).map(|(a, b)| a.abs_diff(*b)).sum()
    }

    fn solve_p2(&self, input: &str) -> Self::Part2 {
        let (c1, c2) = parse_input(input);

        let mut occurances: HashMap<u32, u32> = HashMap::new();
        for b in c2 {
            *occurances.entry(b).or_insert(0) += 1;
        }

        c1.iter()
            .map(|&a| occurances.get(&a).unwrap_or(&0) * a)
            .sum()
    }
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut c1 = Vec::with_capacity(1000);
    let mut c2 = Vec::with_capacity(1000);

    for line in input.lines() {
        if let Some((a, b)) = line.split_once("   ") {
            c1.push(a.parse::<u32>().unwrap());
            c2.push(b.parse::<u32>().unwrap());
        }
    }

    (c1, c2)
}
