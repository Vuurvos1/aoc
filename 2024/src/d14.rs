use std::{fs::OpenOptions, io::Write};

use regex::Regex;

use crate::Solution;

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;
const LCM: i32 = 10403; // lcm of 101 and 103

pub struct Day14;

impl Solution for Day14 {
    type Part1 = u64;
    type Part2 = i32;

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        let re = Regex::new(r"-?\d+").expect("Invalid regex");
        let input: Vec<_> = input
            .lines()
            .map(|line| {
                let digits: Vec<i32> = re
                    .find_iter(line) // Find all matches
                    .filter_map(|mat| mat.as_str().parse::<i32>().ok()) // Attempt to parse to i32, filter out errors
                    .collect();
                (digits[0], digits[1], digits[2], digits[3])
            })
            .collect();

        let mut end_positions: Vec<(i32, i32)> = Vec::new();
        for robot in input {
            let (px, py, vx, vy) = robot;
            let end_position = walk_robot((px, py), (vx, vy), 100);
            end_positions.push(end_position);
        }

        let mut quadrant_scores = vec![0, 0, 0, 0];
        for pos in end_positions {
            let x = pos.0;
            let y = pos.1;

            if x < WIDTH / 2 && y < HEIGHT / 2 {
                quadrant_scores[0] += 1;
            } else if x > WIDTH / 2 && y < HEIGHT / 2 {
                quadrant_scores[1] += 1;
            } else if x < WIDTH / 2 && y > HEIGHT / 2 {
                quadrant_scores[2] += 1;
            } else if x > WIDTH / 2 && y > HEIGHT / 2 {
                quadrant_scores[3] += 1;
            }
        }

        let sum = quadrant_scores.iter().product();
        sum
    }

    fn solve_p2(&self, input: &str) -> Self::Part2 {
        // for this part I noticed when plotting a bunch of grids that the robots tended to group together horizontally or vertically
        // which seemed to happen at a certian interval for me this was
        // vertical:   y = 101x + 11
        // horizontal: y = 103x + 65
        // then I just plotted a bunch of grids with the intervals till I found the right one
        // Also these 2 lines intersect at x = -27 and y = -2716 and the lcm of 101 and 103 is 10,403
        // so if you add 10403 to the y you should see the christmas tree at that timestamp

        let re = Regex::new(r"-?\d+").expect("Invalid regex");
        let robots: Vec<_> = input
            .lines()
            .map(|line| {
                let digits: Vec<i32> = re
                    .find_iter(line) // Find all matches
                    .filter_map(|mat| mat.as_str().parse::<i32>().ok()) // Attempt to parse to i32, filter out errors
                    .collect();
                (digits[0], digits[1], digits[2], digits[3])
            })
            .collect();

        let (horizontal_offset, vertical_offset) = find_best_density_offset(&robots);
        let intersection = find_intersection(horizontal_offset, vertical_offset);
        intersection
    }
}

fn walk_robot(p: (i32, i32), v: (i32, i32), steps: i32) -> (i32, i32) {
    let x = (p.0 + (v.0 * steps) + steps * WIDTH) % WIDTH;
    let y = (p.1 + (v.1 * steps) + steps * HEIGHT) % HEIGHT;
    (x, y)
}

fn find_best_density_offset(robots: &[(i32, i32, i32, i32)]) -> (i32, i32) {
    let mut best_horizontal_density = 0;
    let mut best_vertical_density = 0;
    let mut best_horizontal_offset = 0;
    let mut best_vertical_offset = 0;

    for offset in 0..HEIGHT {
        let (horizontal_density, vertical_density) = calculate_densities(robots, offset);

        if horizontal_density > best_horizontal_density {
            best_horizontal_density = horizontal_density;
            best_horizontal_offset = offset;
        }

        if offset < WIDTH && vertical_density > best_vertical_density {
            best_vertical_density = vertical_density;
            best_vertical_offset = offset;
        }
    }

    (best_horizontal_offset, best_vertical_offset)
}

fn calculate_densities(robots: &[(i32, i32, i32, i32)], time: i32) -> (i32, i32) {
    let mut horizontal_counts = std::collections::HashMap::new();
    let mut vertical_counts = std::collections::HashMap::new();

    for &(px, py, vx, vy) in robots {
        let pos = walk_robot((px, py), (vx, vy), time);
        *horizontal_counts.entry(pos.1).or_insert(0) += 1; // y-coordinate
        *vertical_counts.entry(pos.0).or_insert(0) += 1; // x-coordinate
    }

    let max_horizontal = *horizontal_counts.values().max().unwrap_or(&0);
    let max_vertical = *vertical_counts.values().max().unwrap_or(&0);

    (max_horizontal, max_vertical)
}

fn find_intersection(offset_x: i32, offset_y: i32) -> i32 {
    // y = 101x + 11
    // y = 103x + 65

    let x = (offset_y - offset_x) / (HEIGHT - WIDTH);
    let y = WIDTH * x + offset_y;

    // make sure y is a positive value
    if y < 0 {
        return y + LCM;
    }

    y
}

fn _points_to_grid_string(points: &Vec<(i32, i32)>) -> String {
    let mut grid = vec![vec!['.'; WIDTH as usize]; HEIGHT as usize];

    for point in points {
        grid[point.1 as usize][point.0 as usize] = '#';
    }

    let mut grid_string = String::new();
    for row in grid {
        grid_string.push_str(&row.iter().collect::<String>());
        grid_string.push('\n');
    }

    grid_string
}

fn _output_grid(input: &str) {
    let re = Regex::new(r"-?\d+").expect("Invalid regex");
    let input: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            let digits: Vec<i32> = re
                .find_iter(line) // Find all matches
                .filter_map(|mat| mat.as_str().parse::<i32>().ok()) // Attempt to parse to i32, filter out errors
                .collect();
            digits
        })
        .collect();

    let _file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("./src/tmp.txt")
        .expect("Could not clear file");

    let mut out_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("./src/tmp.txt")
        .expect("Could not open file");

    for i in (65..15000).step_by(103) {
        let mut end_positions: Vec<(i32, i32)> = Vec::new();
        for robot in input.clone() {
            let [px, py, vx, vy] = robot.try_into().unwrap();
            let end_position = walk_robot((px, py), (vx, vy), i);
            end_positions.push(end_position);
        }

        let grid_string = _points_to_grid_string(&end_positions);

        out_file
            .write_all(format!("- {}\n", i).as_bytes())
            .expect("Could not write to file");

        out_file
            .write_all(grid_string.as_bytes())
            .expect("Could not write to file");

        // append 2 new lines
        out_file
            .write_all("\n\n".as_bytes())
            .expect("Could not write to file");
    }
}
