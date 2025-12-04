use crate::Solution;

pub struct Day03;

impl Solution for Day03 {
    type Part1 = u64;
    type Part2 = u64;

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        let mut sum: u64 = 0;

        for line in input.lines() {
            let digits: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();

            sum += find_max_digit(2, &digits);
        }

        sum
    }

    fn solve_p2(&self, input: &str) -> Self::Part2 {
        let mut sum: u64 = 0;

        for line in input.lines() {
            let digits: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();

            sum += find_max_digit(12, &digits);
        }

        sum
    }
}

fn find_max_digit(k: usize, digits: &Vec<u32>) -> u64 {
    let mut max = 0;
    let mut start = 0;

    for pos in 0..k {
        let mut best_idx = start;
        let remaining = k - pos; // digits left to add (including this one)

        // find the largest digit in the remaining range
        for i in start..=(digits.len() - remaining) {
            if digits[i] > digits[best_idx] {
                best_idx = i;
            }
        }

        max = max * 10 + digits[best_idx] as u64;
        start = best_idx + 1; // starts after the chosen digit
    }
    max
}
