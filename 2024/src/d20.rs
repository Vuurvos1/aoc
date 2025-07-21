use std::collections::VecDeque;

use crate::Solution;

pub struct Day20;

impl Solution for Day20 {
    type Part1 = u32;
    type Part2 = u32;

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        solve(input, 2)
    }

    fn solve_p2(&self, input: &str) -> Self::Part2 {
        solve(input, 20)
    }
}

fn solve(input: &str, cheat_amount: i32) -> u32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let finish_pos = find_position(&grid, 'E');
    let distances = get_distances_grid(&grid, finish_pos);
    count_cheats(&distances, cheat_amount)
}

fn find_position(grid: &[Vec<char>], target: char) -> (i32, i32) {
    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == target {
                return (x as i32, y as i32);
            }
        }
    }
    panic!("Target '{}' not found", target);
}

fn get_distances_grid(grid: &[Vec<char>], start_pos: (i32, i32)) -> Vec<Vec<i32>> {
    let height = grid.len();
    let width = grid[0].len();
    let mut distances = vec![vec![-1i32; width]; height]; // Use -1 for unreachable

    let mut queue = VecDeque::new();
    distances[start_pos.1 as usize][start_pos.0 as usize] = 0;
    queue.push_back((start_pos, 0));

    while let Some((pos, dist)) = queue.pop_front() {
        for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let new_pos = (pos.0 + dx, pos.1 + dy);
            let x = new_pos.0 as usize;
            let y = new_pos.1 as usize;

            // Check bounds and if it's a valid move and not visited yet
            if y < height && x < width && grid[y][x] != '#' && distances[y][x] == -1 {
                distances[y][x] = dist + 1;
                queue.push_back((new_pos, dist + 1));
            }
        }
    }

    distances
}

fn count_cheats(distances: &[Vec<i32>], cheat_amount: i32) -> u32 {
    let height = distances.len();
    let width = distances[0].len();
    let mut cheats = 0;

    for y1 in 0..height {
        for x1 in 0..width {
            let dist1 = distances[y1][x1];
            if dist1 == -1 {
                continue;
            }

            let (x1, y1) = (x1 as i32, y1 as i32);

            for dx in -cheat_amount..=cheat_amount {
                let remaining = cheat_amount - dx.abs();
                for dy in -remaining..=remaining {
                    let x2 = x1 + dx;
                    let y2 = y1 + dy;

                    if x2 >= 0 && x2 < width as i32 && y2 >= 0 && y2 < height as i32 {
                        let dist2 = distances[y2 as usize][x2 as usize];
                        if dist2 != -1 {
                            // Only consider reachable positions
                            let cheat_distance = dx.abs() + dy.abs();
                            let time_saved = dist1 - dist2 - cheat_distance;

                            if time_saved >= 100 {
                                cheats += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    cheats
}
