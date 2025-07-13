use std::collections::HashSet;

use crate::Solution;

pub struct Day06;

impl Solution for Day06 {
    type Part1 = usize;
    type Part2 = usize;

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        // y, x grid
        let grid = input
            .split("\n")
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let mut pos: (i32, i32) = (0, 0);
        'outer: for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                if grid[row][col] == '^' {
                    pos = (row as i32, col as i32);
                    break 'outer;
                }
            }
        }

        // println!("{:?}", pos);

        let mut direction: (i32, i32) = (-1, 0); // up
        let mut visited: HashSet<(i32, i32)> = HashSet::new();

        loop {
            if pos.0 < 0
                || pos.0 >= grid.len() as i32 - 1
                || pos.1 < 0
                || pos.1 >= grid[0].len() as i32 - 1
            {
                break;
            }

            pos = (pos.0 + direction.0, pos.1 + direction.1);

            // turn right if # else to forward
            if grid[pos.0 as usize][pos.1 as usize] == '#' {
                pos = (pos.0 - direction.0, pos.1 - direction.1);
                direction = (direction.1, -direction.0);
            }

            visited.insert(pos);
        }

        visited.len()
    }

    fn solve_p2(&self, input: &str) -> Self::Part2 {
        // y, x grid
        let grid = input
            .split("\n")
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let mut start_pos: (i16, i16) = (0, 0);
        'outer: for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                if grid[row][col] == '^' {
                    start_pos = (row as i16, col as i16);
                    break 'outer;
                }
            }
        }

        let mut pos = start_pos.clone();
        let mut blockades = 0;

        let mut direction: (i16, i16) = (-1, 0);
        let mut base_path: HashSet<(i16, i16)> = HashSet::new();
        loop {
            if pos.0 < 0
                || pos.0 >= grid.len() as i16 - 1
                || pos.1 < 0
                || pos.1 >= grid[0].len() as i16 - 1
            {
                break;
            }

            pos = (pos.0 + direction.0, pos.1 + direction.1);

            // turn right if # else to forward
            if grid[pos.0 as usize][pos.1 as usize] == '#' {
                pos = (pos.0 - direction.0, pos.1 - direction.1);
                direction = (direction.1, -direction.0);
            }

            base_path.insert(pos);
        }
        base_path.remove(&start_pos); // check the start position

        for p in base_path.iter() {
            let (y, x) = p;

            pos = start_pos.clone();
            let mut direction: (i16, i16) = (-1, 0);
            // let mut visited: HashSet<(i16, i16, i16, i16)> = HashSet::with_capacity(500);
            let mut visited: HashSet<u64> = HashSet::with_capacity(500);

            loop {
                let next_pos = (pos.0 + direction.0, pos.1 + direction.1);
                if next_pos.0 < 0
                    || next_pos.0 >= grid.len() as i16
                    || next_pos.1 < 0
                    || next_pos.1 >= grid[0].len() as i16
                {
                    // println!("Ouf of bounds");
                    break;
                }

                let key = concat_digits(pos.0, pos.1, direction.0, direction.1);
                // let key = (pos.0, pos.1, direction.0, direction.1);
                if visited.contains(&key) && visited.len() > 1 {
                    blockades += 1;
                    break;
                }

                if grid[next_pos.0 as usize][next_pos.1 as usize] == '#'
                    || *y == next_pos.0 && *x == next_pos.1
                {
                    visited.insert(key); // only insert on collision to reduce expensive insertions

                    // turn right
                    direction = (direction.1, -direction.0);
                } else {
                    // move forward
                    pos = next_pos;
                }
            }
        }

        blockades
    }
}

fn concat_digits(a: i16, b: i16, c: i16, d: i16) -> u64 {
    // I doubt this is faster than just having a 4 tuple set but it's fun
    ((a as u64 & 0xffff) << 48)
        | ((b as u64 & 0xffff) << 32)
        | ((c as u64 & 0xffff) << 16)
        | (d as u64 & 0xffff)
}

#[test]
fn test_concat_digits() {
    assert_eq!(concat_digits(1, 2, 3, 4), 0x0001_0002_0003_0004);
    assert_eq!(concat_digits(0, 0, 0, 0), 0);
    assert_eq!(concat_digits(0, 0, 0, 1), 1);
    assert_eq!(concat_digits(0, 0, 1, 0), 0x0000_0000_0001_0000);
    assert_eq!(concat_digits(0, -1, 6, 7), 0x0000_ffff_0006_0007);
}
