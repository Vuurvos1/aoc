use crate::Solution;

pub struct Day25;

impl Solution for Day25 {
    type Part1 = usize;
    type Part2 = u32;

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        let input = input
            .split("\n\n")
            .map(|line| {
                let mut result = 0;
                let bytes = line.as_bytes();
                for &c in &bytes[6..] {
                    result <<= 1;
                    result |= (c == b'#') as u64;
                }
                result
            })
            .collect::<Vec<_>>();

        let mut keys: Vec<u64> = Vec::with_capacity(input.len() / 2);
        let mut locks: Vec<u64> = Vec::with_capacity(input.len() / 2);

        for grid in input {
            let is_key = grid & 1 == 0;
            if is_key {
                locks.push(grid)
            } else {
                keys.push(grid);
            }
        }

        // try all keys on all locks to check fit
        keys.iter()
            .map(|&key| locks.iter().filter(|&&lock| key & lock == 0).count())
            .sum()
    }

    fn solve_p2(&self, _input: &str) -> Self::Part2 {
        0
    }
}
