use std::collections::HashMap;

use crate::Solution;

pub struct Day19;

impl Solution for Day19 {
    type Part1 = usize;
    type Part2 = u64;

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        let (mut patterns, designs) = parse_input(input);

        patterns.sort_by(|a, b| a.len().cmp(&b.len()));

        designs
            .iter()
            .filter(|design| can_create_towel(design, &patterns))
            .count()
    }

    fn solve_p2(&self, input: &str) -> Self::Part2 {
        let (patterns, designs) = parse_input(input);

        let mut cache: HashMap<usize, u64> = HashMap::with_capacity(10000);

        designs
            .iter()
            .map(|design| {
                cache.clear();
                towel_permutations(design, 0, &patterns, &mut cache)
            })
            .sum()
    }
}

fn can_create_towel(towel: &str, patterns: &[&str]) -> bool {
    if towel.is_empty() {
        return true;
    }

    for pattern in patterns {
        // front to back
        if towel.starts_with(pattern) {
            let new_towel = &towel[pattern.len()..];
            if can_create_towel(new_towel, patterns) {
                return true;
            }
        }
    }

    false
}

fn towel_permutations(
    design: &str,
    start_idx: usize,
    patterns: &[&str],
    cache: &mut HashMap<usize, u64>,
) -> u64 {
    if start_idx >= design.len() {
        return 1;
    }

    if let Some(&count) = cache.get(&start_idx) {
        return count;
    }

    let mut count = 0;
    let remaining = &design[start_idx..];

    for pattern in patterns {
        if remaining.starts_with(pattern) {
            count += towel_permutations(design, start_idx + pattern.len(), patterns, cache);
        }
    }

    cache.insert(start_idx, count);
    count
}
fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut parts = input.split("\n\n");
    let patterns: Vec<&str> = parts.next().unwrap().split(", ").collect();

    let designs: Vec<&str> = parts.next().unwrap().lines().collect();
    (patterns, designs)
}
