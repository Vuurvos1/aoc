use regex::Regex;

use crate::Solution;

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

pub struct Day14;

impl Solution for Day14 {
    type Part1 = u64;
    type Part2 = u64;

    fn solve_p1(&self, input: &str) -> Self::Part1 {
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

        let mut end_positions: Vec<(i32, i32)> = Vec::new();
        for robot in input {
            let [px, py, vx, vy] = robot.try_into().unwrap();
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

        let sum: u64 = quadrant_scores.iter().product();

        sum
    }

    fn solve_p2(&self, _input: &str) -> Self::Part2 {
        0
    }
}

fn walk_robot(p: (i32, i32), v: (i32, i32), steps: i32) -> (i32, i32) {
    let x = (p.0 + (v.0 * steps) + steps * WIDTH) % WIDTH;
    let y = (p.1 + (v.1 * steps) + steps * HEIGHT) % HEIGHT;

    (x, y)
}
