use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use crate::Solution;

pub struct Day16;

impl Solution for Day16 {
    type Part1 = u64;
    type Part2 = usize;

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        let (grid, start_pos) = parse_input(input);

        let mut cache: HashMap<(i32, i32, i32, i32), u64> = HashMap::new();
        let mut heap: BinaryHeap<Reverse<(u64, (i32, i32), (i32, i32))>> = BinaryHeap::new();
        let mut min: u64 = u64::MAX;
        heap.push(Reverse((0, start_pos, (0, 1))));

        while let Some(Reverse((score, pos, dir))) = heap.pop() {
            // update cache score if current score is lower
            if cache.contains_key(&(pos.0, pos.1, dir.0, dir.1)) {
                let old_score = cache.get(&(pos.0, pos.1, dir.0, dir.1)).unwrap();
                if score > *old_score {
                    continue;
                }
            }
            cache.insert((pos.0, pos.1, dir.0, dir.1), score);

            if score > min {
                continue;
            }

            if grid[pos.0 as usize][pos.1 as usize] == 'E' {
                min = min.min(score);
                continue;
            }

            // move forward for 1 cost
            let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
            if inbounds(new_pos.1, new_pos.0, &grid)
                && grid[new_pos.0 as usize][new_pos.1 as usize] != '#'
            {
                heap.push(Reverse((score + 1, new_pos, dir)));
            }

            // rotate clockwise or counterclockwise for 1000 cost
            let new_dir = (dir.1, -dir.0);
            heap.push(Reverse((score + 1000, pos, new_dir)));
            let new_dir = (-dir.1, dir.0);
            heap.push(Reverse((score + 1000, pos, new_dir)));
        }

        min
    }

    fn solve_p2(&self, input: &str) -> Self::Part2 {
        let (grid, start_pos) = parse_input(input);

        let p1_score = self.solve_p1(input);

        let mut cache: HashMap<(i32, i32, i32, i32), u64> = HashMap::new();
        let mut stack: VecDeque<((i32, i32), (i32, i32), u64, Vec<(i32, i32)>)> = VecDeque::new();
        let mut path_tiles: HashSet<(i32, i32)> = HashSet::new();
        let mut min: u64 = u64::MAX;
        stack.push_back((start_pos, (0, 1), 0, Vec::new()));

        while stack.len() > 0 {
            let (pos, dir, score, mut path) = stack.pop_front().unwrap();
            path.push(pos);

            if score > p1_score {
                continue;
            }

            // update cache score if current score is lower
            if cache.contains_key(&(pos.0, pos.1, dir.0, dir.1)) {
                let old_score = cache.get(&(pos.0, pos.1, dir.0, dir.1)).unwrap();
                if score > *old_score {
                    continue;
                }
            }
            cache.insert((pos.0, pos.1, dir.0, dir.1), score);

            if grid[pos.0 as usize][pos.1 as usize] == 'E' {
                min = min.min(score);
                path_tiles.extend(path);
                continue;
            }

            // move forward for 1 cost
            let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
            if inbounds(new_pos.1, new_pos.0, &grid)
                && grid[new_pos.0 as usize][new_pos.1 as usize] != '#'
            {
                stack.push_back((new_pos, dir, score + 1, path.clone()));
            }

            // rotate clockwise or counterclockwise for 1000 cost
            let new_dir = (dir.1, -dir.0);
            stack.push_back((pos, new_dir, score + 1000, path.clone()));
            let new_dir = (-dir.1, dir.0);
            stack.push_back((pos, new_dir, score + 1000, path));
        }

        path_tiles.len()
    }
}

fn inbounds<T>(x: i32, y: i32, grid: &Vec<Vec<T>>) -> bool {
    let height = grid.len();
    let width = grid[0].len();
    x >= 0 && x < width as i32 && y >= 0 && y < height as i32
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, (i32, i32)) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut start_pos = (0, 0);
    'outer: for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'S' {
                start_pos = (y as i32, x as i32);
                break 'outer;
            }
        }
    }

    (grid, start_pos)
}
