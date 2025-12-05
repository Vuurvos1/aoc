use crate::Solution;

pub struct Day05;

impl Solution for Day05 {
    type Part1 = u64;
    type Part2 = u64;

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        let input = input.split("\n\n").collect::<Vec<&str>>();
        let ranges: Vec<(u64, u64)> = input[0]
            .lines()
            .map(|line| {
                line.split("-")
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>()
            })
            .map(|x| (x[0], x[1]))
            .collect();
        let ingredients: Vec<u64> = input[1]
            .lines()
            .map(|line| line.parse::<u64>().unwrap())
            .collect();

        let mut count = 0;

        for ingredient in ingredients {
            for range in &ranges {
                if ingredient >= range.0 && ingredient <= range.1 {
                    count += 1;
                    break;
                }
            }
        }

        count
    }

    fn solve_p2(&self, input: &str) -> Self::Part2 {
        let input = input.split("\n\n").collect::<Vec<&str>>();

        let mut ranges: Vec<(u64, u64)> = input[0]
            .lines()
            .map(|line| {
                line.split("-")
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>()
            })
            .map(|x| (x[0], x[1]))
            .collect::<Vec<(u64, u64)>>();
        ranges.sort_by_key(|r| r.0);

        let mut non_overlapping: Vec<(u64, u64)> = Vec::with_capacity(ranges.len());
        for range in ranges {
            if non_overlapping.is_empty() {
                non_overlapping.push(range);
                continue;
            }

            let last_idx = non_overlapping.len() - 1;
            let last = non_overlapping[last_idx];

            if range.0 <= last.1 + 1 {
                // update last index with extended range
                non_overlapping[last_idx] = (last.0, range.1.max(last.1));
            } else {
                // no overlap
                non_overlapping.push(range);
            }
        }

        // sum up all range differences
        non_overlapping
            .iter()
            .fold(0, |acc, range| acc + range.1 - range.0 + 1)
    }
}
