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
        let mut dial: i32 = 50;
        let mut count: u32 = 0;

        for line in input.lines() {
            let mut chars = line.chars();
            let direction = chars.next().unwrap();
            let amount: i32 = chars.as_str().parse().unwrap();

            let delta = if direction == 'R' { amount } else { -amount };
            let new_dial = dial + delta;

            // Count how many multiples of 100 we pass through or land on
            // = floor(new_dial/100) - floor(dial/100) for positive movement
            // or ceil(dial/100) - ceil(new_dial/100) for negative movement
            let crosses = if delta >= 0 {
                new_dial.div_euclid(100) - dial.div_euclid(100)
            } else {
                dial.div_euclid(100) - new_dial.div_euclid(100)
            };

            count += crosses as u32;
            dial = new_dial.rem_euclid(100);
        }

        count
    }
}
