use crate::{utils::inbounds_grid, Solution};

pub struct Day04;

impl Solution for Day04 {
    type Part1 = u32;
    type Part2 = u32;

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let mut reachable_rolls = 0;

        const DIRECTIONS: [(i32, i32); 8] = [
            (-1, -1),
            (-1, 1),
            (1, -1),
            (1, 1),
            (0, -1),
            (0, 1),
            (-1, 0),
            (1, 0),
        ];

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == '.' {
                    continue;
                }

                let mut roll_count = 0;
                for (dx, dy) in DIRECTIONS {
                    let pos = (x as i32 + dx, y as i32 + dy);
                    if !inbounds_grid(pos.0, pos.1, &grid) {
                        continue;
                    }

                    if grid[pos.1 as usize][pos.0 as usize] == '@' {
                        roll_count += 1;
                    }
                }

                if roll_count < 4 {
                    reachable_rolls += 1;
                }
            }
        }

        reachable_rolls
    }

    fn solve_p2(&self, input: &str) -> Self::Part2 {
        let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let mut reachable_rolls = 0;

        const DIRECTIONS: [(i32, i32); 8] = [
            (-1, -1),
            (-1, 1),
            (1, -1),
            (1, 1),
            (0, -1),
            (0, 1),
            (-1, 0),
            (1, 0),
        ];

        let mut can_remove = true;

        while can_remove {
            if !can_remove {
                break;
            }

            let mut to_remove = Vec::new();
            for y in 0..grid.len() {
                for x in 0..grid[y].len() {
                    if grid[y][x] == '.' {
                        continue;
                    }

                    let mut roll_count = 0;
                    for (dx, dy) in DIRECTIONS {
                        let pos = (x as i32 + dx, y as i32 + dy);
                        if !inbounds_grid(pos.0, pos.1, &grid) {
                            continue;
                        }

                        if grid[pos.1 as usize][pos.0 as usize] == '@' {
                            roll_count += 1;
                        }
                    }

                    if roll_count < 4 {
                        reachable_rolls += 1;
                        to_remove.push((y, x));
                    }
                }
            }

            if to_remove.len() > 0 {
                for (y, x) in to_remove {
                    grid[y][x] = '.';
                }
            } else {
                can_remove = false;
            }
        }

        reachable_rolls
    }
}
