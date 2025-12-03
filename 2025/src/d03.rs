use crate::Solution;

pub struct Day03;

impl Solution for Day03 {
    type Part1 = u32;
    type Part2 = u64;

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        let mut sum: u32 = 0;

        for line in input.lines() {
            let digits = line
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>();

            let mut max = 0;

            for i in 0..digits.len() {
                for j in i + 1..digits.len() {
                    let digit = (digits[i] * 10) + digits[j];

                    if digit > max {
                        max = digit;
                    }
                }
            }

            sum += max;
        }

        sum
    }

    fn solve_p2(&self, input: &str) -> Self::Part2 {
        let mut sum: u64 = 0;

        for line in input.lines() {
            let digits = line
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>();

            let k = 12; // amount of digits to consider
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

            sum += max as u64;
        }

        sum
    }
}
