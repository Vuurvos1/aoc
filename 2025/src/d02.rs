use crate::Solution;

pub struct Day02;

impl Solution for Day02 {
    type Part1 = u64;
    type Part2 = u64;

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        let mut sum: u64 = 0;

        for line in input.split(",") {
            let mut parts = line.split("-");
            let start = parts.next().unwrap().parse::<u64>().unwrap();
            let end = parts.next().unwrap().parse::<u64>().unwrap();

            for i in start..=end {
                let i_str = i.to_string();

                let half1 = i_str.chars().take(i_str.len() / 2).collect::<String>();
                let half2 = i_str.chars().skip(i_str.len() / 2).collect::<String>();
                if half1 == half2 {
                    sum += i;
                }
            }
        }

        sum
    }

    fn solve_p2(&self, input: &str) -> Self::Part2 {
        let mut sum: u64 = 0;

        for line in input.split(",") {
            let mut parts = line.split("-");
            let start = parts.next().unwrap().parse::<u64>().unwrap();
            let end = parts.next().unwrap().parse::<u64>().unwrap();

            for i in start..=end {
                let i_str = i.to_string();

                for j in 1..=i_str.len() / 2 {
                    let block = i_str.chars().take(j).collect::<String>();

                    // test if block nicely fits the string
                    if i_str.len() % block.len() != 0 {
                        continue;
                    }

                    let mut is_repeating = true;
                    for k in 0..i_str.len() / block.len() {
                        if i_str[k * block.len()..(k + 1) * block.len()] != block {
                            is_repeating = false;
                            break;
                        }
                    }

                    if is_repeating {
                        sum += i;
                        break;
                    }
                }
            }
        }

        sum
    }
}
