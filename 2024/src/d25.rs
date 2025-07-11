use crate::Solution;

pub struct Day25;

impl Solution for Day25 {
    type Part1 = usize;
    type Part2 = u32;

    fn day(&self) -> u32 {
        25
    }

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        let input = input
            .split("\n\n")
            .map(|line| {
                let mut result = 0;
                for c in line.chars() {
                    result <<= 1;
                    result |= (c == '#') as u64;
                }
                result
            })
            .collect::<Vec<_>>();

        let mut keys: Vec<u64> = Vec::new();
        let mut locks: Vec<u64> = Vec::new();

        for grid in input {
            let is_key = grid & 1 == 0;
            if is_key {
                locks.push(grid)
            } else {
                keys.push(grid);
            }
        }

        let mut sum = 0;

        // try all keys on all locks to check fit
        for lock in &locks {
            for key in &keys {
                let fits = key & lock == 0;
                if fits {
                    sum += 1;
                }
            }
        }

        sum
    }

    fn solve_p2(&self, _input: &str) -> Self::Part2 {
        0
    }
}
