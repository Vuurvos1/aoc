use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use crate::Solution;

pub struct Day16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    x: i16,
    y: i16,
    dir_x: i16,
    dir_y: i16,
}

impl State {
    fn new(x: i16, y: i16, dir_x: i16, dir_y: i16) -> Self {
        Self { x, y, dir_x, dir_y }
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.x, self.y, self.dir_x, self.dir_y).cmp(&(other.x, other.y, other.dir_x, other.dir_y))
    }
}

impl Solution for Day16 {
    type Part1 = u64;
    type Part2 = usize;

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        let (grid, start_pos) = parse_input(input);

        let mut distances: HashMap<u64, u64> = HashMap::new();
        let mut heap: BinaryHeap<Reverse<(u64, State)>> = BinaryHeap::new();
        let mut visited: HashSet<u64> = HashSet::new();

        let start_state = State::new(start_pos.0, start_pos.1, 0, 1); // facing east initially
        let pack_64 = unsafe { std::mem::transmute(start_state) };

        distances.insert(pack_64, 0);
        heap.push(Reverse((0, start_state)));

        while let Some(Reverse((score, state))) = heap.pop() {
            let pack_64 = unsafe { std::mem::transmute(state) };

            if visited.contains(&pack_64) {
                continue;
            }
            visited.insert(pack_64);

            // arrived at the end
            if grid[state.x as usize][state.y as usize] == b'E' {
                return score;
            }

            let next_moves = [
                // Move forward
                (
                    score + 1,
                    (
                        state.x + state.dir_x,
                        state.y + state.dir_y,
                        state.dir_x,
                        state.dir_y,
                    ),
                ),
                // Rotate clockwise
                (score + 1000, (state.x, state.y, state.dir_y, -state.dir_x)),
                // Rotate counterclockwise
                (score + 1000, (state.x, state.y, -state.dir_y, state.dir_x)),
            ];

            for (new_score, new_state) in next_moves {
                // Check bounds and walls
                if !inbounds(new_state.0, new_state.1, &grid)
                    || grid[new_state.0 as usize][new_state.1 as usize] == b'#'
                {
                    continue;
                }

                let new_state = State::new(new_state.0, new_state.1, new_state.2, new_state.3);
                let pack_64 = unsafe { std::mem::transmute(new_state) };

                // Skip if already visited
                if visited.contains(&pack_64) {
                    continue;
                }

                // Only add to heap if this is a better path (or first time seeing this state)
                if let Some(&existing_distance) = distances.get(&pack_64) {
                    if new_score >= existing_distance {
                        continue;
                    }
                }

                distances.insert(pack_64, new_score);
                heap.push(Reverse((new_score, new_state)));
            }
        }

        0
    }

    fn solve_p2(&self, input: &str) -> Self::Part2 {
        let (grid, start_pos) = parse_input(input);

        let p1_score = self.solve_p1(input);

        let mut cache: HashMap<u64, u64> = HashMap::new();
        let mut stack: VecDeque<(State, u64, Vec<(i16, i16)>)> = VecDeque::new();
        let mut path_tiles: HashSet<(i16, i16)> = HashSet::new();

        stack.push_back((State::new(start_pos.0, start_pos.1, 0, 1), 0, Vec::new()));

        while let Some((state, score, mut path)) = stack.pop_front() {
            path.push((state.x, state.y));

            if score > p1_score {
                continue;
            }

            let pack_64 = unsafe { std::mem::transmute(state) };

            // update cache score if current score is lower
            if let Some(&old_score) = cache.get(&pack_64) {
                if score > old_score {
                    continue;
                }
            }
            cache.insert(pack_64, score);

            if grid[state.x as usize][state.y as usize] == b'E' {
                path_tiles.extend(path);
                continue;
            }

            // move forward for 1 cost
            let new_pos = (state.x + state.dir_x, state.y + state.dir_y);
            if inbounds(new_pos.1, new_pos.0, &grid)
                && grid[new_pos.0 as usize][new_pos.1 as usize] != b'#'
            {
                let new_score = score + 1;
                if new_score > p1_score {
                    continue;
                }

                stack.push_back((
                    State::new(new_pos.0, new_pos.1, state.dir_x, state.dir_y),
                    new_score,
                    path.clone(),
                ));
            }

            // rotate clockwise or counterclockwise for 1000 cost
            let new_score = score + 1000;
            if new_score > p1_score {
                continue;
            }

            let new_dir = (state.dir_y, -state.dir_x);
            stack.push_back((
                State::new(state.x, state.y, new_dir.0, new_dir.1),
                new_score,
                path.clone(),
            ));
            let new_dir = (-state.dir_y, state.dir_x);
            stack.push_back((
                State::new(state.x, state.y, new_dir.0, new_dir.1),
                new_score,
                path,
            ));
        }

        path_tiles.len()
    }
}

fn inbounds<T>(x: i16, y: i16, grid: &Vec<Vec<T>>) -> bool {
    let height = grid.len();
    let width = grid[0].len();
    x >= 0 && x < width as i16 && y >= 0 && y < height as i16
}

fn parse_input(input: &str) -> (Vec<Vec<u8>>, (i16, i16)) {
    let grid = input
        .lines()
        .map(|line| line.bytes().collect())
        .collect::<Vec<Vec<_>>>();

    let mut start_pos = (0, 0);
    'outer: for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == b'S' {
                start_pos = (y as i16, x as i16);
                break 'outer;
            }
        }
    }

    (grid, start_pos)
}
