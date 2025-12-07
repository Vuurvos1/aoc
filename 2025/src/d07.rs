use std::collections::{HashMap, HashSet};

use crate::{utils::inbounds_grid, Solution};

pub struct Day07;

impl Solution for Day07 {
    type Part1 = u64;
    type Part2 = u64;

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        let grid = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let start_pos = grid[0]
            .iter()
            .position(|&c| c == 'S')
            .map(|x| (x as i32, 0))
            .expect("Start position 'S' not found in first row");

        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut beams: Vec<(i32, i32)> = Vec::new();
        beams.push(start_pos);
        let mut splitters: u64 = 0;

        while let Some((x, mut y)) = beams.pop() {
            while inbounds_grid(x, y, &grid) && grid[y as usize][x as usize] != '^' {
                y += 1;
            }

            if !inbounds_grid(x, y, &grid) {
                continue;
            }

            if visited.contains(&(x, y)) {
                continue;
            }

            visited.insert((x, y));
            beams.push((x + 1, y + 1)); // right splitter
            beams.push((x - 1, y + 1)); // left splitter
            splitters += 1;
        }

        splitters
    }

    fn solve_p2(&self, input: &str) -> Self::Part2 {
        let grid = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let start_pos = grid[0]
            .iter()
            .position(|&c| c == 'S')
            .map(|x| (x as i32, 0))
            .expect("Start position 'S' not found in first row");

        // keep track of visited splitters and the number of possible paths from that splitter
        let mut cache: HashMap<(i32, i32), u64> = HashMap::new();

        fn count_paths(
            x: i32,
            y: i32,
            grid: &Vec<Vec<char>>,
            cache: &mut HashMap<(i32, i32), u64>,
        ) -> u64 {
            // completed path
            if y >= grid.len() as i32 {
                return 1;
            }

            // Out of bounds on sides or top = dead end
            if !inbounds_grid(x, y, grid) {
                return 0;
            }

            let cell = grid[y as usize][x as usize];

            if cell == '^' {
                if let Some(&cached) = cache.get(&(x, y)) {
                    return cached;
                }

                let left_paths = count_paths(x - 1, y, grid, cache);
                let right_paths = count_paths(x + 1, y, grid, cache);
                let total = left_paths + right_paths;

                cache.insert((x, y), total);
                return total;
            }

            count_paths(x, y + 1, grid, cache)
        }

        count_paths(start_pos.0, start_pos.1, &grid, &mut cache)
    }
}
