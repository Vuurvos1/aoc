use regex::Regex;

use crate::Solution;

pub struct Day03;

impl Solution for Day03 {
    type Part1 = i32;
    type Part2 = i32;

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("Invalid regex");

        let mut sum = 0;
        for (_, [d1, d2]) in re.captures_iter(input).map(|c| c.extract()) {
            sum += d1.parse::<i32>().unwrap() * d2.parse::<i32>().unwrap()
        }

        sum
    }

    fn solve_p2(&self, input: &str) -> Self::Part2 {
        let re = Regex::new(r"(mul\(\d+,\d+\)|do\(\)|don't\(\))").expect("Invalid regex");
        let digit_re = Regex::new(r"(\d+),(\d+)").expect("Invalid digit regex");

        let mut sum = 0;
        let mut mult = true;
        for (_, [token]) in re.captures_iter(input).map(|c| c.extract()) {
            match token {
                "do()" => mult = true,
                "don't()" => mult = false,
                _ => {
                    if !mult {
                        continue;
                    }

                    for (_, [d1, d2]) in digit_re.captures_iter(token).map(|c| c.extract()) {
                        sum += d1.parse::<i32>().unwrap() * d2.parse::<i32>().unwrap()
                    }
                }
            }
        }

        sum
    }
}
