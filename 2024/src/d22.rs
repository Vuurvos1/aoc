use std::collections::{HashMap, HashSet};

use crate::Solution;

struct SecretGenerator {
    seed: i64,
}

impl SecretGenerator {
    fn new(seed: i64) -> Self {
        SecretGenerator { seed }
    }
}

impl Iterator for SecretGenerator {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        self.seed = (self.seed ^ (self.seed << 6)) & 0xffffff; // (seed ^ (seed * 64)) % 16777216;
        self.seed = (self.seed ^ (self.seed >> 5)) & 0xffffff; // (seed ^ (seed / 32)) % 16777216;
        self.seed = (self.seed ^ (self.seed << 11)) & 0xffffff; // (seed ^ (seed * 2048)) % 16777216;
        Some(self.seed)
    }
}

pub struct Day22;

impl Solution for Day22 {
    type Part1 = i64;
    type Part2 = i64;

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        let seeds = input
            .lines()
            .map(|line| line.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        let sum = seeds
            .into_iter()
            .map(|seed| SecretGenerator::new(seed).take(2000).last().unwrap())
            .sum();

        sum
    }

    fn solve_p2(&self, input: &str) -> Self::Part2 {
        let seeds = input
            .lines()
            .map(|line| line.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        let mut results: HashMap<i64, i64> = HashMap::with_capacity(4000);

        for (_buyer_index, &seed) in seeds.iter().enumerate() {
            let mut generator = SecretGenerator::new(seed);

            let mut seen_sequences: HashSet<i64> = HashSet::with_capacity(4000);

            let mut tmp = [0i64; 4];
            let mut prev_price = seed % 10;

            // sliding window

            // fill initial window
            for i in 0..4 {
                let curr_price = generator.next().unwrap() % 10;
                tmp[i] = curr_price - prev_price;
                prev_price = curr_price;
            }
            let mut sequence = (tmp[0], tmp[1], tmp[2], tmp[3]); // the window
            results.insert(to_index(sequence), prev_price);

            // process the rest
            for _ in 4..2000 {
                let curr_price = generator.next().unwrap() % 10;
                let change = curr_price - prev_price;

                // shift window
                sequence = (sequence.1, sequence.2, sequence.3, change);

                let index = to_index(sequence);
                if !seen_sequences.contains(&index) {
                    seen_sequences.insert(index);
                    *results.entry(index).or_insert(0) += curr_price;
                }

                prev_price = curr_price;
            }
        }

        *results.values().max().unwrap()
    }
}

fn to_index(seq: (i64, i64, i64, i64)) -> i64 {
    6859 * (seq.0 + 10) + 361 * (seq.1 + 10) + 19 * (seq.2 + 10) + (seq.3 + 10)
}
