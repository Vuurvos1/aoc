use std::collections::{HashMap, HashSet};

use crate::Solution;

pub struct Day08;

impl Solution for Day08 {
    type Part1 = usize;
    type Part2 = usize;

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        // y, x grid
        let grid = input
            .split("\n")
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let height = grid.len() as i32;
        let width = grid[0].len() as i32;

        let mut antenna: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
        let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] != '.' {
                    if antenna.contains_key(&grid[y][x]) {
                        antenna
                            .get_mut(&grid[y][x])
                            .unwrap()
                            .push((y as i32, x as i32));
                    } else {
                        antenna.insert(grid[y][x], vec![(y as i32, x as i32)]);
                    }
                }
            }
        }

        for (_k, v) in antenna.iter() {
            // go through all antenna combinations
            for i in 0..v.len() {
                for j in i + 1..v.len() {
                    // lowest y (counting from top)
                    let p1 = if v[i].0 < v[j].0 { v[i] } else { v[j] };
                    // highest y
                    let p2 = if v[i].0 > v[j].0 { v[i] } else { v[j] };
                    let delta = (p2.0 - p1.0, p2.1 - p1.1);

                    let n1 = (p1.0 - delta.0, p1.1 - delta.1);
                    if inbounds(n1, height, width) {
                        antinodes.insert(n1);
                    }

                    let n2 = (p2.0 + delta.0, p2.1 + delta.1);
                    if inbounds(n2, height, width) {
                        antinodes.insert(n2);
                    }
                }
            }
        }

        antinodes.len()
    }

    fn solve_p2(&self, input: &str) -> Self::Part2 {
        // y, x grid
        let grid = input
            .split("\n")
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let height = grid.len() as i32;
        let width = grid[0].len() as i32;

        let mut antenna: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
        let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] != '.' {
                    if antenna.contains_key(&grid[y][x]) {
                        antenna
                            .get_mut(&grid[y][x])
                            .unwrap()
                            .push((y as i32, x as i32));
                    } else {
                        antenna.insert(grid[y][x], vec![(y as i32, x as i32)]);
                    }
                }
            }
        }

        for (_k, v) in antenna.iter() {
            // go through all antenna combinations
            for i in 0..v.len() {
                for j in i + 1..v.len() {
                    // lowest y (counting from top)
                    let p1 = if v[i].0 < v[j].0 { v[i] } else { v[j] };
                    antinodes.insert(p1);
                    // highest y
                    let p2 = if v[i].0 > v[j].0 { v[i] } else { v[j] };
                    antinodes.insert(p2);
                    let delta = (p2.0 - p1.0, p2.1 - p1.1);

                    let mut n1 = (p1.0 - delta.0, p1.1 - delta.1);
                    loop {
                        if !inbounds(n1, height, width) {
                            break;
                        }

                        antinodes.insert(n1);
                        n1 = (n1.0 - delta.0, n1.1 - delta.1);
                    }

                    let mut n2 = (p2.0 + delta.0, p2.1 + delta.1);
                    loop {
                        if !inbounds(n2, height, width) {
                            break;
                        }

                        antinodes.insert(n2);
                        n2 = (n2.0 + delta.0, n2.1 + delta.1);
                    }
                }
            }
        }

        antinodes.len()
    }
}

fn inbounds(pos: (i32, i32), height: i32, width: i32) -> bool {
    pos.0 >= 0 && pos.0 < height && pos.1 >= 0 && pos.1 < width
}
