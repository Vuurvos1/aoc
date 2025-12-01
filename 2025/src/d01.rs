use crate::Solution;

pub struct Day01;

impl Solution for Day01 {
    type Part1 = u32;
    type Part2 = u32;

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        let mut dial = 50;
        let mut count = 0;

        for line in input.lines() {
            let mut chars = line.chars();
            let direction = chars.next().unwrap();
            let amount: i32 = chars.as_str().parse().unwrap();

            if direction == 'R' {
                dial += amount;
            } else {
                dial -= amount;
            }

            dial = dial % 100;

            if dial == 0 {
                count += 1;
            }
        }

        count
    }

    fn solve_p2(&self, input: &str) -> Self::Part2 {
        let mut dial = 50;
        let mut count: u32 = 0;

        for line in input.lines() {
            let mut chars = line.chars();
            let direction = chars.next().unwrap();
            let mut amount: i32 = chars.as_str().parse().unwrap();

            if direction == 'R' {
                // dial += amount;
            } else {
                // amount = -amount;
                // dial -= amount;
            }

            for _ in 0..amount {
                if direction == 'R' {
                    dial = (dial + 1) % 100;
                } else {
                    dial = (dial - 1) % 100;
                }

                if dial == 0 {
                    count += 1;
                }
            }

            // let pass_count = dial / 100;
            // count += pass_count as u32;

            dial = dial % 100;

            // if dial == 0 {
            //     count += 1;
            // }
        }

        count
    }
}
