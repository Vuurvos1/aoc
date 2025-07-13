use std::collections::HashMap;

use crate::Solution;

pub struct Day01;

impl Solution for Day01 {
    type Part1 = u32;
    type Part2 = u32;

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        let mut c1 = Vec::new();
        let mut c2 = Vec::new();

        for line in input.lines() {
            let mut split = line.split_whitespace().map(|x| x.parse::<u32>().unwrap());
            c1.push(split.next());
            c2.push(split.next());
        }

        c1.sort();
        c2.sort();

        c1.iter()
            .zip(c2.iter())
            .map(|(a, b)| a.unwrap().abs_diff(b.unwrap()))
            .sum()
    }

    fn solve_p2(&self, input: &str) -> Self::Part2 {
        let mut sum: u32 = 0;

        let mut c1 = Vec::new();
        let mut c2 = Vec::new();

        for line in input.lines() {
            let mut split = line.split_whitespace().map(|x| x.parse::<u32>().unwrap());
            c1.push(split.next().unwrap());
            c2.push(split.next().unwrap());
        }

        let mut occurances: HashMap<u32, u32> = HashMap::new();
        for b in c2 {
            let count = occurances.entry(b).or_insert(0);
            *count += 1;
        }

        for a in c1 {
            let count = occurances.entry(a).or_default();
            sum += *count * a;
        }

        sum
    }
}
