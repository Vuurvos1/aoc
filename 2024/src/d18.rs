use std::collections::{HashMap, HashSet, VecDeque};

use crate::Solution;

pub struct Day18;

impl Solution for Day18 {
    type Part1 = u32;
    type Part2 = String;

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        let grid: HashSet<(i32, i32)> = input
            .lines()
            .take(1024)
            .map(|line| {
                let bits = line
                    .split(',')
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<i32>>();
                (bits[0], bits[1])
            })
            .collect();

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let start_pos = (0, 0);
        let finish_pos = (70, 70);

        let mut cache: HashMap<(i32, i32), u32> = HashMap::new();
        let mut stack: VecDeque<((i32, i32), u32)> = VecDeque::new();
        let mut min: u32 = u32::MAX;
        stack.push_back((start_pos, 0));

        while let Some((pos, score)) = stack.pop_front() {
            if pos == finish_pos {
                min = min.min(score);
                break;
            }

            if let Some(&old_score) = cache.get(&pos) {
                if score >= old_score {
                    continue;
                }
            }
            cache.insert((pos.0, pos.1), score);

            for &(dy, dx) in &directions {
                let new_pos = (pos.0 + dx, pos.1 + dy);
                if inbounds(new_pos.1, new_pos.0, 71) && !grid.contains(&(new_pos.0, new_pos.1)) {
                    stack.push_back((new_pos, score + 1));
                }
            }
        }

        min
    }

    fn solve_p2(&self, input: &str) -> Self::Part2 {
        let input: Vec<(i32, i32)> = input
            .lines()
            .map(|line| {
                let bits = line
                    .split(',')
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<i32>>();
                (bits[0], bits[1])
            })
            .collect();

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let start_pos = (0, 0);
        let finish_pos = (70, 70);

        let mut cache: HashSet<(i32, i32)> = HashSet::new();
        let mut stack: VecDeque<(i32, i32)> = VecDeque::new();

        let mut blocked: (i32, i32) = (0, 0);
        let mut block_index = 0;

        let mut search_min: usize = 1024;
        let mut search_max: usize = input.len();

        loop {
            let mut found = false;
            cache.clear();
            stack.clear();
            stack.push_back(start_pos);

            let search_mid = (search_min + search_max) / 2 + 1;
            let map_bytes: HashSet<(i32, i32)> = input.iter().take(search_mid).cloned().collect();

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
                    if inbounds(new_pos.1, new_pos.0, 71)
                        && !map_bytes.contains(&(new_pos.0, new_pos.1))
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
                block_index = search_min;
                break;
            }
        }

        // println!("Hello, world! {},{} {}", blocked.0, blocked.1, block_index);
        format!("{},{}", blocked.0, blocked.1)
    }
}

fn inbounds(x: i32, y: i32, size: u32) -> bool {
    x >= 0 && x < size as i32 && y >= 0 && y < size as i32
}
