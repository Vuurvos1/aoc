use crate::Solution;

pub struct Day07;

impl Solution for Day07 {
    type Part1 = i64;
    type Part2 = i64;

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        let operations = vec!['+', '*'];

        let mut sum = 0;

        for line in input.lines() {
            // get all numbers in line
            let s = line.split(": ").collect::<Vec<&str>>();
            let awnser = s[0].parse::<i64>().unwrap();
            let digits = s[1]
                .split(" ")
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            // try all combinations of operations on all digits to see if one procudes the awnser
            let result = math_numbers(digits[0], awnser, &digits[1..].to_vec(), &operations);
            if result {
                sum += awnser;
            }
        }

        sum
    }

    fn solve_p2(&self, input: &str) -> Self::Part2 {
        let operations = vec!['+', '*', '|'];

        let mut sum = 0;

        for line in input.lines() {
            // get all numbers in line
            let s = line.split(": ").collect::<Vec<&str>>();
            let awnser = s[0].parse::<i64>().unwrap();
            let digits = s[1]
                .split(" ")
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            // try all combinations of operations on all digits to see if one procudes the awnser
            let result = math_numbers(digits[0], awnser, &digits[1..].to_vec(), &operations);
            if result {
                sum += awnser;
            }
        }

        sum
    }
}

// recursive function to try all combinations of operations on all digits
fn math_numbers(sum: i64, awnser: i64, digits: &Vec<i64>, operations: &Vec<char>) -> bool {
    if digits.len() == 0 {
        return sum == awnser;
    }

    // prune branches that are already too big
    if sum > awnser {
        return false;
    }

    let mut new_digits = digits.clone();
    let digit = new_digits.remove(0);

    for op in operations {
        let mut new_sum = sum;
        match op {
            '+' => new_sum += digit,
            '*' => new_sum *= digit,
            '|' => new_sum = sum * 10_i64.pow(digit.to_string().len() as u32) + digit,
            _ => panic!("Unknown operation"),
        }

        let new_result = math_numbers(new_sum, awnser, &new_digits, operations);
        if new_result {
            return new_result;
        }
    }

    return false;
}
