use crate::Solution;

/*
Locks and keys fit into 32 bit integers
The first and last rows are only relevant to check if it is a key or a lock
So a 5x5 grid is 25 bits plus 4 extra bits for new lines
The locks and keys can then be checked for overlap using a simple bitwise AND operation

#####
##.##    11011
.#.##    01011
...## -> 00011 -> 110110_010110_000110_000100_000100
...#.    00010
...#.    00010
.....
*/

pub struct Day25;

impl Solution for Day25 {
    type Part1 = u32;
    type Part2 = u32;

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        let mut slice = input.as_bytes();
        let mut keys = Vec::with_capacity(250);
        let mut locks = Vec::with_capacity(250);

        while !slice.is_empty() {
            let bits = slice[6..35]
                .iter()
                .fold(0, |acc, &c| acc << 1 | (c == b'#') as u32);

            if slice[0] == b'#' {
                locks.push(bits);
            } else {
                keys.push(bits);
            }

            slice = &slice[43.min(slice.len())..];
        }

        let mut sum = 0;
        for key in &keys {
            for lock in &locks {
                sum += (key & lock == 0) as u32;
            }
        }

        sum
    }

    fn solve_p2(&self, _input: &str) -> Self::Part2 {
        0
    }
}
