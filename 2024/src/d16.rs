use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

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

        // Use modified Dijkstra to find all shortest paths
        let mut distances: HashMap<u64, u64> = HashMap::new();
        let mut heap: BinaryHeap<Reverse<(u64, State)>> = BinaryHeap::new();
        let mut predecessors: HashMap<u64, Vec<u64>> = HashMap::new();

        let start_state = State::new(start_pos.0, start_pos.1, 0, 1);
        let start_pack = unsafe { std::mem::transmute(start_state) };

        distances.insert(start_pack, 0);
        heap.push(Reverse((0, start_state)));

        let mut end_states = Vec::new();
        let mut best_score = u64::MAX;

        while let Some(Reverse((score, state))) = heap.pop() {
            let pack_64 = unsafe { std::mem::transmute(state) };

            // Skip if we've already processed this state with a better score
            if let Some(&existing_score) = distances.get(&pack_64) {
                if score > existing_score {
                    continue;
                }
            }

            // Check if we reached the end
            if grid[state.x as usize][state.y as usize] == b'E' {
                if score < best_score {
                    best_score = score;
                    end_states.clear();
                    end_states.push(pack_64);
                } else if score == best_score {
                    end_states.push(pack_64);
                }
                continue;
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

            for (new_score, new_state_tuple) in next_moves {
                // Check bounds and walls
                if !inbounds(new_state_tuple.0, new_state_tuple.1, &grid)
                    || grid[new_state_tuple.0 as usize][new_state_tuple.1 as usize] == b'#'
                {
                    continue;
                }

                let new_state = State::new(
                    new_state_tuple.0,
                    new_state_tuple.1,
                    new_state_tuple.2,
                    new_state_tuple.3,
                );
                let new_pack = unsafe { std::mem::transmute(new_state) };

                // Handle shortest path tracking
                match distances.get(&new_pack) {
                    Some(&existing_score) => {
                        if new_score < existing_score {
                            // Found a better path
                            distances.insert(new_pack, new_score);
                            predecessors.insert(new_pack, vec![pack_64]);
                            heap.push(Reverse((new_score, new_state)));
                        } else if new_score == existing_score {
                            // Found an equally good path
                            predecessors.entry(new_pack).or_default().push(pack_64);
                        }
                    }
                    None => {
                        // First time seeing this state
                        distances.insert(new_pack, new_score);
                        predecessors.insert(new_pack, vec![pack_64]);
                        heap.push(Reverse((new_score, new_state)));
                    }
                }
            }
        }

        // Backtrack from all end states to collect all tiles on optimal paths
        let mut path_tiles: HashSet<(i16, i16)> = HashSet::new();
        let mut stack: Vec<u64> = end_states;
        let mut visited: HashSet<u64> = HashSet::new();

        while let Some(state_pack) = stack.pop() {
            if visited.contains(&state_pack) {
                continue;
            }
            visited.insert(state_pack);

            let state: State = unsafe { std::mem::transmute(state_pack) };
            path_tiles.insert((state.x, state.y));

            if let Some(preds) = predecessors.get(&state_pack) {
                stack.extend(preds.iter());
            }
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
