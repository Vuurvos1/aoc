use crate::Solution;

pub struct Day06;

impl Solution for Day06 {
    type Part1 = u64;
    type Part2 = u64;

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        // split on all whitespace
        let intput = input
            .lines()
            .map(|line| line.split_whitespace().collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>();

        // transform the input into columns
        let columns = intput[0]
            .iter()
            .enumerate()
            .map(|(i, _)| intput.iter().map(|row| row[i]).collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>();

        let mut total = 0;

        for mut col in columns {
            let operator = col[col.len() - 1];
            // remove the operator from the column
            col.pop();

            let mut sum = if operator == "+" { 0 } else { 1 };
            for num in col {
                match operator {
                    "+" => sum += num.parse::<u64>().unwrap(),
                    "*" => sum *= num.parse::<u64>().unwrap(),
                    _ => panic!("Unknown operator: {}", operator),
                }
            }

            total += sum;
        }

        total
    }

    fn solve_p2(&self, input: &str) -> Self::Part2 {
        // split on all whitespace
        let input = input.lines().collect::<Vec<&str>>();

        let digits_input = input[..input.len() - 1].to_vec();
        let operators = input
            .last()
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>();

        let digit_chars = digits_input
            .iter()
            .map(|row| row.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        // transform the digit row into columns
        let digit_rows = (0..digit_chars[0].len())
            .map(|i| digit_chars.iter().map(|row| row[i]).collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let grouped_rows = digit_rows
            .split(|col| col.iter().all(|&ch| ch == ' '))
            .filter(|group| !group.is_empty())
            .map(|group| group.to_vec())
            .collect::<Vec<Vec<Vec<char>>>>();

        let combined_rows = grouped_rows
            .iter()
            .map(|group| {
                group
                    .iter()
                    .map(|col| col.iter().collect::<String>().trim().to_string())
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>();

        let mut total = 0;

        for (i, group) in combined_rows.iter().enumerate() {
            let operator = operators[i];

            let mut sum = if operator == "+" { 0 } else { 1 };

            for digit in group {
                match operator {
                    "+" => sum += digit.parse::<u64>().unwrap(),
                    "*" => sum *= digit.parse::<u64>().unwrap(),
                    _ => panic!("Unknown operator: {}", operator),
                }
            }

            total += sum;
        }

        total
    }
}
