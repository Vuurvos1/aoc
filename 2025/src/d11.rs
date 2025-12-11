use std::collections::{HashMap, VecDeque};

use crate::Solution;

pub struct Day11;

impl Solution for Day11 {
    type Part1 = usize;
    type Part2 = usize;

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        // find all paths from you to out

        let devices = input
            .lines()
            .map(|line| {
                let (device, connections) = line.split_once(':').unwrap();
                let connections = connections.split_whitespace().collect::<Vec<_>>();
                (device, connections)
            })
            .collect::<HashMap<&str, Vec<&str>>>();

        let mut all_paths = Vec::new();

        let mut stack = VecDeque::new();
        stack.push_back(("you", vec!["you"]));

        while let Some((device, path)) = stack.pop_back() {
            if device == "out" {
                all_paths.push(path);
                continue;
            }

            if let Some(connections) = devices.get(device) {
                for &next_device in connections {
                    if !path.contains(&next_device) {
                        let mut new_path = path.clone();
                        new_path.push(next_device);
                        stack.push_back((next_device, new_path));
                    }
                }
            }
        }

        all_paths.len()
    }

    fn solve_p2(&self, input: &str) -> Self::Part2 {
        // find all paths from svr to out passing through fft and dac

        let devices = input
            .lines()
            .map(|line| {
                let (device, connections) = line.split_once(':').unwrap();
                let connections = connections.split_whitespace().collect::<Vec<_>>();
                (device, connections)
            })
            .collect::<HashMap<&str, Vec<&str>>>();

        let mut all_paths = Vec::new();

        let mut stack = VecDeque::new();
        stack.push_back(("svr", vec!["svr"]));

        while let Some((device, path)) = stack.pop_back() {
            if device == "out" {
                all_paths.push(path);
                continue;
            }

            if let Some(connections) = devices.get(device) {
                for &next_device in connections {
                    if !path.contains(&next_device) {
                        let mut new_path = path.clone();
                        new_path.push(next_device);
                        stack.push_back((next_device, new_path));
                    }
                }
            }
        }

        let paths_passing_through_fft_and_dac = all_paths
            .iter()
            .filter(|path| path.contains(&"fft") && path.contains(&"dac"))
            .collect::<Vec<_>>();
        paths_passing_through_fft_and_dac.len()
    }
}
