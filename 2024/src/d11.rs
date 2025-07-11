use std::collections::HashMap;

use crate::Solution;

pub struct Day11;

impl Solution for Day11 {
    type Part1 = u64;
    type Part2 = u64;

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        let stones: Vec<u64> = input
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();

        let mut cache: HashMap<(u64, u16), u64> = HashMap::new();

        stones
            .iter()
            .map(|stone| step_stone(*stone, 25, &mut cache))
            .sum()
    }

    fn solve_p2(&self, input: &str) -> Self::Part2 {
        let stones: Vec<u64> = input
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();

        let mut cache: HashMap<(u64, u16), u64> = HashMap::new();
        stones
            .iter()
            .map(|stone| step_stone(*stone, 75, &mut cache))
            .sum()
    }
}

fn step_stone(digit: u64, step: u16, cache: &mut HashMap<(u64, u16), u64>) -> u64 {
    if step == 0 {
        return 1;
    }

    if digit == 0 {
        // not caching the 0 - 1 seems to be faster
        return step_stone(1, step - 1, cache);
    }

    if let Some(c) = cache.get(&(digit, step)) {
        return *c;
    }

    let len = digit.ilog10() + 1;
    if len % 2 == 0 {
        // push halfs to new stones
        let divisor = 10_u64.pow(len / 2);
        let left = digit / divisor;
        let right = digit % divisor;

        let c = step_stone(left, step - 1, cache) + step_stone(right, step - 1, cache);
        cache.insert((digit, step), c);
        return c;
    }

    let c = step_stone(digit * 2024, step - 1, cache);
    cache.insert((digit, step), c);
    c
}
