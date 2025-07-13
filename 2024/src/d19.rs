use std::collections::HashMap;

use crate::Solution;

pub struct Day19;

impl Solution for Day19 {
    type Part1 = u32;
    type Part2 = u64;

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        let split_input = input.split("\n\n").collect::<Vec<&str>>();

        let mut patterns = String::from(split_input[0])
            .split(", ")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        patterns.sort_by(|a, b| a.len().cmp(&b.len()));
        let designs = String::from(split_input[1])
            .lines()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        let mut count = 0;
        for design in designs {
            let c = create_towel(design, &patterns);
            if c != 0 {
                count += 1
            }
        }

        count
    }

    fn solve_p2(&self, input: &str) -> Self::Part2 {
        let split_input = input.split("\n\n").collect::<Vec<&str>>();

        let patterns = String::from(split_input[0])
            .split(", ")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let designs = String::from(split_input[1])
            .lines()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        let mut count: u64 = 0;
        let mut cache: HashMap<String, u64> = HashMap::with_capacity(10000);

        for design in designs {
            cache.clear();
            count += towel_permutations(design, &patterns, &mut cache);
        }

        count
    }
}

fn create_towel(towel: String, patterns: &Vec<String>) -> i32 {
    if towel == "" {
        return 1;
    }

    for pattern in patterns {
        // front to back
        if towel.starts_with(pattern.as_str()) {
            let new_towel = towel.replacen(pattern.as_str(), "", 1);
            let c = create_towel(new_towel, patterns);
            if c > 0 {
                return c;
            }
        }
    }

    0
}

fn towel_permutations(
    towel: String,
    patterns: &Vec<String>,
    cache: &mut HashMap<String, u64>,
) -> u64 {
    if towel == "" {
        return 1;
    }

    if let Some(c) = cache.get(&towel) {
        return *c;
    }

    let mut count = 0;
    for pattern in patterns {
        if towel.starts_with(pattern.as_str()) {
            let new_towel = towel.replacen(pattern.as_str(), "", 1);
            count += towel_permutations(new_towel, patterns, cache);
        }
    }

    cache.insert(towel, count);
    count
}
