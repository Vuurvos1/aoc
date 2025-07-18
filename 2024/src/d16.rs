use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use crate::Solution;

pub struct Day16;

impl Solution for Day16 {
    type Part1 = u64;
    type Part2 = usize;

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        let (grid, start_pos) = parse_input(input);

        let mut distances: HashMap<(i32, i32, i32, i32), u64> = HashMap::new();
        let mut heap: BinaryHeap<Reverse<(u64, (i32, i32), (i32, i32))>> = BinaryHeap::new();
        let mut visited: HashSet<(i32, i32, i32, i32)> = HashSet::new();

        let start_state = (start_pos.0, start_pos.1, 0, 1); // facing east initially
        distances.insert(start_state, 0);
        heap.push(Reverse((0, start_pos, (0, 1))));

        while let Some(Reverse((score, pos, dir))) = heap.pop() {
            let state = (pos.0, pos.1, dir.0, dir.1);

            if visited.contains(&state) {
                continue;
            }
            visited.insert(state);

            // arrived at the end
            if grid[pos.0 as usize][pos.1 as usize] == 'E' {
                return score;
            }

            let next_moves = [
                // Move forward
                (score + 1, (pos.0 + dir.0, pos.1 + dir.1), dir),
                // Rotate clockwise
                (score + 1000, pos, (dir.1, -dir.0)),
                // Rotate counterclockwise
                (score + 1000, pos, (-dir.1, dir.0)),
            ];

            for (new_score, new_pos, new_dir) in next_moves {
                // Check bounds and walls
                if !inbounds(new_pos.1, new_pos.0, &grid)
                    || grid[new_pos.0 as usize][new_pos.1 as usize] == '#'
                {
                    continue;
                }

                let new_state = (new_pos.0, new_pos.1, new_dir.0, new_dir.1);

                // Skip if already visited
                if visited.contains(&new_state) {
                    continue;
                }

                // Only add to heap if this is a better path (or first time seeing this state)
                if let Some(&existing_distance) = distances.get(&new_state) {
                    if new_score >= existing_distance {
                        continue;
                    }
                }

                distances.insert(new_state, new_score);
                heap.push(Reverse((new_score, new_pos, new_dir)));
            }
        }

        0
    }

    fn solve_p2(&self, input: &str) -> Self::Part2 {
        let (grid, start_pos) = parse_input(input);

        let p1_score = self.solve_p1(input);

        let mut cache: HashMap<(i32, i32, i32, i32), u64> = HashMap::new();
        let mut stack: VecDeque<((i32, i32), (i32, i32), u64, Vec<(i32, i32)>)> = VecDeque::new();
        let mut path_tiles: HashSet<(i32, i32)> = HashSet::new();

        stack.push_back((start_pos, (0, 1), 0, Vec::new()));

        while let Some((pos, dir, score, mut path)) = stack.pop_front() {
            path.push(pos);

            if score > p1_score {
                continue;
            }

            let state = (pos.0, pos.1, dir.0, dir.1);

            // update cache score if current score is lower
            if let Some(&old_score) = cache.get(&state) {
                if score > old_score {
                    continue;
                }
            }
            cache.insert(state, score);

            if grid[pos.0 as usize][pos.1 as usize] == 'E' {
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
