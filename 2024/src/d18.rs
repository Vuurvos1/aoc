use std::collections::{HashSet, VecDeque};

use crate::Solution;

pub struct Day18;

impl Solution for Day18 {
    type Part1 = u32;
    type Part2 = String;

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        let mut grid: [[bool; 71]; 71] = [[false; 71]; 71];
        for line in input.lines().take(1024) {
            let (a, b) = line.split_once(',').unwrap();
            let (x, y): (usize, usize) = (a.parse().unwrap(), b.parse().unwrap());
            grid[x][y] = true;
        }

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let start_pos = (0, 0);
        let finish_pos = (70, 70);

        let mut stack: VecDeque<((i32, i32), u32)> = VecDeque::new();
        let mut visited: HashSet<_> = HashSet::new();
        stack.push_back((start_pos, 0));

        while let Some((pos, score)) = stack.pop_front() {
            if pos == finish_pos {
                return score;
            }

            if !visited.insert(pos) {
                continue;
            }

            for &(dy, dx) in &directions {
                let new_pos = (pos.0 + dx, pos.1 + dy);
                if inbounds(new_pos.0, new_pos.1, 71)
                    && !grid[new_pos.0 as usize][new_pos.1 as usize]
                {
                    stack.push_back((new_pos, score + 1));
                }
            }
        }

        0
    }

    fn solve_p2(&self, input: &str) -> Self::Part2 {
        let input: Vec<(usize, usize)> = input
            .lines()
            .map(|line| {
                let (a, b) = line.split_once(',').unwrap();
                (a.parse().unwrap(), b.parse().unwrap())
            })
            .collect();

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let start_pos = (0, 0);
        let finish_pos = (70, 70);

        let mut cache: HashSet<(i32, i32)> = HashSet::new();
        let mut stack: VecDeque<(i32, i32)> = VecDeque::new();

        let blocked;

        let mut search_min: usize = 1024;
        let mut search_max: usize = input.len();

        loop {
            let mut found = false;
            cache.clear();
            stack.clear();
            stack.push_back(start_pos);

            let search_mid = (search_min + search_max) / 2 + 1;

            let mut grid = [[false; 71]; 71];
            for &(x, y) in input.iter().take(search_mid) {
                grid[x][y] = true;
            }

            while let Some(pos) = stack.pop_front() {
                if pos == finish_pos {
                    found = true;
                    break;
                }

                if cache.contains(&pos) {
                    continue;
                }
                cache.insert(pos);

                for &(dy, dx) in &directions {
                    let new_pos = (pos.0 + dx, pos.1 + dy);
                    if inbounds(new_pos.0, new_pos.1, 71)
                        && !grid[new_pos.0 as usize][new_pos.1 as usize]
                    {
                        stack.push_back(new_pos);
                    }
                }
            }

            if found {
                search_min = search_mid;
            } else {
                search_max = search_mid;
            }

            if search_max - search_min <= 1 {
                blocked = input[search_min];
                break;
            }
        }

        format!("{},{}", blocked.0, blocked.1)
    }
}

fn inbounds(x: i32, y: i32, size: u32) -> bool {
    x >= 0 && x < size as i32 && y >= 0 && y < size as i32
}
