use std::collections::{HashMap, VecDeque};

use crate::Solution;

pub struct Day20;

impl Solution for Day20 {
    type Part1 = u32;
    type Part2 = u32;

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let mut finish_pos = (0, 0);
        'outer: for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == 'E' {
                    finish_pos = (x as i32, y as i32);
                    break 'outer;
                }
            }
        }

        cheat(grid, finish_pos, 2)
    }

    fn solve_p2(&self, input: &str) -> Self::Part2 {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let mut finish_pos = (0, 0);
        'outer: for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == 'E' {
                    finish_pos = (x as i32, y as i32);
                    break 'outer;
                }
            }
        }

        cheat(grid, finish_pos, 20)
    }
}

fn inbounds<T>(x: i32, y: i32, grid: &Vec<Vec<T>>) -> bool {
    let height = grid.len();
    let width = grid[0].len();
    x >= 0 && x < width as i32 && y >= 0 && y < height as i32
}

fn manhattan_distance(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

/**
 * Returns a map of distances from the start position to every other position
 */
fn get_distances_map(grid: &Vec<Vec<char>>, start_pos: (i32, i32)) -> HashMap<(i32, i32), i32> {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    // let mut distances = vec![vec![u32::MAX; grid[0].len()]; grid.len()];
    // distances[start_pos.1 as usize][start_pos.0 as usize] = 0;
    let mut distances: HashMap<(i32, i32), i32> = HashMap::new();
    distances.insert(start_pos, 0);

    let mut stack: VecDeque<((i32, i32), i32)> = VecDeque::new();
    stack.push_back((start_pos, 0));

    while let Some((pos, distance)) = stack.pop_front() {
        for (dx, dy) in directions {
            let new_pos = (pos.0 + dx, pos.1 + dy);
            if inbounds(new_pos.1, new_pos.0, &grid)
                && grid[new_pos.1 as usize][new_pos.0 as usize] != '#'
            {
                let new_distance = distance + 1;
                let score = *distances.get(&new_pos).unwrap_or(&i32::MAX);
                if score > new_distance {
                    stack.push_back((new_pos, new_distance));
                    distances.insert(new_pos, new_distance);
                }
            }
        }
    }

    distances
}

fn cheat(grid: Vec<Vec<char>>, finish_pos: (i32, i32), cheat_amount: i32) -> u32 {
    let mut cheats: u32 = 0;

    // generate a distance map
    let distances = get_distances_map(&grid, finish_pos);
    let walkable = distances.keys().collect::<Vec<_>>();
    for i in 0..walkable.len() {
        for j in 0..walkable.len() {
            if i == j {
                continue;
            }

            let p1 = walkable[i];
            let p2 = walkable[j];

            let distance = manhattan_distance(*p1, *p2);
            if distance <= cheat_amount
                && distances.get(walkable[i]).unwrap()
                    - distances.get(walkable[j]).unwrap()
                    - distance
                    >= 100
            {
                cheats += 1;
            }
        }
    }

    // get shortest path
    // for every point on path, do a manhattan distance check with the cheat amount
    // if distance on point has a difference of 100 or more, add to sum

    cheats
}
