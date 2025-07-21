use crate::{utils::concat_numbers, Solution};

pub struct Day07;

impl Solution for Day07 {
    type Part1 = u64;
    type Part2 = u64;

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        let mut sum = 0;

        for line in input.lines() {
            // get all numbers in line
            let (s1, s2) = line.split_once(": ").unwrap();
            let awnser = s1.parse::<u64>().unwrap();
            let digits = s2
                .split(" ")
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            // try all combinations of operations on all digits to see if one procudes the awnser
            if math_numbers(digits[0], awnser, &digits, 1, &['+', '*']) {
                sum += awnser;
            }
        }

        sum
    }

    fn solve_p2(&self, input: &str) -> Self::Part2 {
        let mut sum = 0;

        for line in input.lines() {
            // get all numbers in line
            let (s1, s2) = line.split_once(": ").unwrap();
            let awnser = s1.parse::<u64>().unwrap();
            let digits = s2
                .split(" ")
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            // try all combinations of operations on all digits to see if one procudes the awnser
            if math_numbers(digits[0], awnser, &digits, 1, &['+', '*', '|']) {
                sum += awnser;
            }
        }

        sum
    }
}

/// try all combinations of operations on all digits
fn math_numbers(
    sum: u64,
    awnser: u64,
    digits: &Vec<u64>,
    index: usize,
    operations: &[char],
) -> bool {
    if index == digits.len() {
        return sum == awnser;
    }

    // prune branches that are already too big
    if sum > awnser {
        return false;
    }

    let digit = digits[index];

    for &op in operations {
        let new_sum = match op {
            '+' => sum + digit,
            '*' => sum * digit,
            '|' => concat_numbers(sum, digit),
            _ => panic!("Unknown operation"),
        };

        if math_numbers(new_sum, awnser, digits, index + 1, operations) {
            return true;
        }
    }

    false
}

#[test]
fn part1_example() {
    let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    assert_eq!(Day07.solve_p1(&input), 3749);
}
