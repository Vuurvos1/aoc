use std::{collections::HashSet, fs, time::Instant};

use crate::Solution;

pub struct Day12;

impl Solution for Day12 {
    type Part1 = u32;
    type Part2 = u32;

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let directions = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut sum: u32 = 0;

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if visited.contains(&(y, x)) {
                    continue;
                }

                let mut stack: Vec<(usize, usize)> = Vec::new();
                let mut plot: HashSet<(usize, usize)> = HashSet::new();
                stack.push((y, x));
                plot.insert((y, x));

                while stack.len() > 0 {
                    let pos = stack.pop().unwrap();
                    if visited.contains(&pos) {
                        // already visited
                        continue;
                    }
                    visited.insert(pos);

                    for dir in &directions {
                        let new_pos = (pos.0 as i32 + dir.0, pos.1 as i32 + dir.1);
                        if !inbounds(new_pos.0, new_pos.1, &grid) {
                            continue;
                        }

                        if grid[new_pos.0 as usize][new_pos.1 as usize] == grid[y][x] {
                            stack.push((new_pos.0 as usize, new_pos.1 as usize));
                            plot.insert((new_pos.0 as usize, new_pos.1 as usize));
                        }
                    }
                }

                let perim = get_plot_perimeter(&plot, &grid);
                sum += perim * plot.len() as u32;
            }
        }

        sum
    }

    fn solve_p2(&self, input: &str) -> Self::Part2 {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let directions = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut sum: u32 = 0;

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if visited.contains(&(y, x)) {
                    continue;
                }

                let mut stack: Vec<(usize, usize)> = Vec::new();
                let mut plot: HashSet<(usize, usize)> = HashSet::new();
                stack.push((y, x));
                plot.insert((y, x));

                while stack.len() > 0 {
                    let pos = stack.pop().unwrap();
                    if visited.contains(&pos) {
                        // already visited
                        continue;
                    }
                    visited.insert(pos);

                    for dir in &directions {
                        let new_pos = (pos.0 as i32 + dir.0, pos.1 as i32 + dir.1);
                        if !inbounds(new_pos.0, new_pos.1, &grid) {
                            continue;
                        }

                        if grid[new_pos.0 as usize][new_pos.1 as usize] == grid[y][x] {
                            stack.push((new_pos.0 as usize, new_pos.1 as usize));
                            plot.insert((new_pos.0 as usize, new_pos.1 as usize));
                        }
                    }
                }

                let perim = get_long_edges(&plot, &grid);
                sum += perim * plot.len() as u32;
            }
        }

        sum
    }
}

fn inbounds<T>(x: i32, y: i32, grid: &Vec<Vec<T>>) -> bool {
    let height = grid.len();
    let width = grid[0].len();
    x >= 0 && x < width as i32 && y >= 0 && y < height as i32
}

fn get_plot_perimeter(plots: &HashSet<(usize, usize)>, grid: &Vec<Vec<char>>) -> u32 {
    let mut edges = 0;
    let directions = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

    for p in plots {
        for d in &directions {
            let new_pos = (p.0 as i32 + d.0, p.1 as i32 + d.1);
            if !inbounds(new_pos.0, new_pos.1, grid) {
                edges += 1;
                continue;
            }
            if grid[new_pos.0 as usize][new_pos.1 as usize] != grid[p.0][p.1] {
                edges += 1;
            }
        }
    }
    return edges;
}

fn get_long_edges(plots: &HashSet<(usize, usize)>, grid: &Vec<Vec<char>>) -> u32 {
    let cardinal_directions = vec![
        ((1, 0), (0, 1)),
        ((0, 1), (-1, 0)),
        ((-1, 0), (0, -1)),
        ((0, -1), (1, 0)),
    ];

    // count the corners of the shape
    let mut corners = 0;
    for cell in plots {
        let cell_self = grid[cell.0][cell.1];
        // check for every cardinal direction
        for (dir1, dir2) in &cardinal_directions {
            let new_pos1 = (cell.0 as i32 + dir1.0, cell.1 as i32 + dir1.1);
            let new_pos2 = (cell.0 as i32 + dir2.0, cell.1 as i32 + dir2.1);

            // outside corners
            let p1_edge = !inbounds(new_pos1.0, new_pos1.1, &grid)
                || grid[new_pos1.0 as usize][new_pos1.1 as usize] != cell_self;
            let p2_edge = !inbounds(new_pos2.0, new_pos2.1, &grid)
                || grid[new_pos2.0 as usize][new_pos2.1 as usize] != cell_self;

            if p1_edge && p2_edge {
                corners += 1;
                continue;
            }

            // inside corners
            let corner_y = cell.0 as i32 + dir1.0 + dir2.0;
            let corner_x = cell.1 as i32 + dir1.1 + dir2.1;
            if !inbounds(corner_x, corner_y, &grid) {
                continue;
            }

            let corner_cell = grid[corner_y as usize][corner_x as usize];
            if !p1_edge && !p2_edge && corner_cell != cell_self {
                corners += 1;
                continue;
            }
        }
    }

    return corners;
}
