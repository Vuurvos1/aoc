use std::collections::HashSet;

use crate::Solution;

pub struct Day10;

impl Solution for Day10 {
    type Part1 = u32;
    type Part2 = u32;

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        let grid: Vec<Vec<i32>> = input
            .lines()
            .map(|line| line.chars().map(|c| c as i32 - 48).collect())
            .collect();

        let mut start_positions: Vec<(i32, i32)> = Vec::new();
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == 0 {
                    start_positions.push((y as i32, x as i32));
                }
            }
        }

        let directions = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
        let mut sum = 0;

        for start in start_positions {
            let mut visited: HashSet<(i32, i32)> = HashSet::new();
            let mut stack: Vec<(i32, i32)> = Vec::new();

            let mut trail_sum = 0;

            stack.push(start);

            while stack.len() > 0 {
                let pos = stack.pop().unwrap();
                if visited.contains(&pos) {
                    // already visited
                    continue;
                }

                visited.insert(pos);

                let height = grid[pos.0 as usize][pos.1 as usize];
                if height == 9 {
                    trail_sum += 1;
                    continue;
                }

                for dir in &directions {
                    let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
                    if !inbounds(new_pos.0, new_pos.1, grid.len(), grid[0].len()) {
                        continue;
                    }

                    let new_height = grid[new_pos.0 as usize][new_pos.1 as usize];
                    if new_height == height + 1 {
                        stack.push(new_pos);
                    }
                }
            }

            sum += trail_sum;
        }

        sum
    }

    fn solve_p2(&self, input: &str) -> Self::Part2 {
        let grid: Vec<Vec<i32>> = input
            .lines()
            .map(|line| line.chars().map(|c| c as i32 - 48).collect())
            .collect();

        let mut start_positions: Vec<(i32, i32)> = Vec::new();
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == 0 {
                    start_positions.push((y as i32, x as i32));
                }
            }
        }

        let directions = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
        let mut sum = 0;

        for start in start_positions {
            let mut stack: Vec<(i32, i32)> = Vec::new();

            let mut trail_sum = 0;

            stack.push(start);

            while stack.len() > 0 {
                let pos = stack.pop().unwrap();

                let height = grid[pos.0 as usize][pos.1 as usize];
                if height == 9 {
                    trail_sum += 1;
                    continue;
                }

                for dir in &directions {
                    let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
                    if !inbounds(new_pos.0, new_pos.1, grid.len(), grid[0].len()) {
                        continue;
                    }

                    let new_height = grid[new_pos.0 as usize][new_pos.1 as usize];

                    if new_height == height + 1 {
                        stack.push(new_pos);
                    }
                }
            }

            sum += trail_sum;
        }

        sum
    }
}

fn inbounds(x: i32, y: i32, width: usize, height: usize) -> bool {
    x >= 0 && x < width as i32 && y >= 0 && y < height as i32
}
