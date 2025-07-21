use std::collections::HashSet;

use crate::Solution;

pub struct Day06;

impl Solution for Day06 {
    type Part1 = usize;
    type Part2 = usize;

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        // y, x grid
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let mut pos = find_start_pos(&grid);

        let mut direction: (i32, i32) = (-1, 0); // up
        let mut visited: HashSet<(i32, i32)> = HashSet::new();

        loop {
            if pos.0 < 0
                || pos.0 >= grid.len() as i32 - 1
                || pos.1 < 0
                || pos.1 >= grid[0].len() as i32 - 1
            {
                break;
            }

            pos = (pos.0 + direction.0, pos.1 + direction.1);

            // turn right if # else to forward
            if grid[pos.0 as usize][pos.1 as usize] == '#' {
                pos = (pos.0 - direction.0, pos.1 - direction.1);
                direction = (direction.1, -direction.0);
            }

            visited.insert(pos);
        }

        visited.len()
    }

    fn solve_p2(&self, input: &str) -> Self::Part2 {
        // y, x grid
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let start_pos = find_start_pos(&grid);
        let mut pos = start_pos.clone();

        let mut blockades = 0;

        let mut direction: (i32, i32) = (-1, 0);
        let mut base_path: HashSet<(i32, i32)> = HashSet::new();
        loop {
            if pos.0 < 0
                || pos.0 >= grid.len() as i32 - 1
                || pos.1 < 0
                || pos.1 >= grid[0].len() as i32 - 1
            {
                break;
            }

            pos = (pos.0 + direction.0, pos.1 + direction.1);

            // turn right if # else to forward
            if grid[pos.0 as usize][pos.1 as usize] == '#' {
                pos = (pos.0 - direction.0, pos.1 - direction.1);
                direction = (direction.1, -direction.0);
            }

            base_path.insert(pos);
        }
        base_path.remove(&start_pos);

        for p in base_path.iter() {
            if has_loop(&grid, start_pos, *p) {
                blockades += 1;
            }
        }

        blockades
    }
}

fn has_loop(grid: &[Vec<char>], start_pos: (i32, i32), obstacle: (i32, i32)) -> bool {
    let mut pos = start_pos;
    let mut direction = (-1, 0);
    let mut visited = HashSet::with_capacity(500);

    loop {
        let next_pos = (pos.0 + direction.0, pos.1 + direction.1);

        // out of bounds
        if next_pos.0 < 0
            || next_pos.0 >= grid.len() as i32
            || next_pos.1 < 0
            || next_pos.1 >= grid[0].len() as i32
        {
            return false;
        }

        if grid[next_pos.0 as usize][next_pos.1 as usize] == '#' || next_pos == obstacle {
            let state = (pos.0, pos.1, direction.0, direction.1);
            if visited.contains(&state) {
                return true;
            }

            visited.insert(state);

            direction = (direction.1, -direction.0);
        } else {
            pos = next_pos;
        }
    }
}

fn find_start_pos(grid: &Vec<Vec<char>>) -> (i32, i32) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == '^' {
                return (y as i32, x as i32);
            }
        }
    }

    panic!("Start position not found");
}
