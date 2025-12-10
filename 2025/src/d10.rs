use regex::Regex;
use std::collections::{HashSet, VecDeque};

use z3::{ast::Int, Optimize, SatResult};

use crate::Solution;

pub struct Day10;

// NOTE: you need to have z3 installed to run this

struct Machine {
    // u16
    lights: u16,
    // u16
    buttons: Vec<u16>,
    // u16
    joltages: Vec<u16>,
}

impl Solution for Day10 {
    type Part1 = u64;
    type Part2 = u64;

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        let re =
            Regex::new(r"\[([.#]+)\]\s+((?:\([0-9,]+\)\s*)+)\{([0-9,]+)\}").expect("Invalid regex");
        let button_re = Regex::new(r"\(([0-9,]+)\)").expect("Invalid regex");

        let machines: Vec<Machine> = input
            .lines()
            .filter_map(|line| {
                let caps = re.captures(line)?;

                // Parse lights as bitmap: [.##.] -> 0b0110
                let lights_str = caps.get(1)?.as_str();
                let lights = lights_str
                    .chars()
                    .enumerate()
                    .filter(|(_, c)| *c == '#')
                    .fold(0, |acc, (i, _)| acc | (1 << i));

                // Parse buttons as Vec of bitmaps
                let buttons_str = caps.get(2)?.as_str();
                let buttons: Vec<u16> = button_re
                    .captures_iter(buttons_str)
                    .map(|cap| {
                        cap.get(1)
                            .unwrap()
                            .as_str()
                            .split(',')
                            .map(|n| n.parse::<u16>().unwrap())
                            .fold(0, |acc, pos| acc | (1 << pos))
                    })
                    .collect();

                // Parse joltages as array
                let joltages_str = caps.get(3)?.as_str();
                let joltages: Vec<u16> = joltages_str
                    .split(',')
                    .map(|n| n.parse::<u16>().unwrap())
                    .collect();

                Some(Machine {
                    lights,
                    buttons,
                    joltages,
                })
            })
            .collect();

        let mut sum = 0;
        for machine in &machines {
            let shortest_sequence = find_shortest_sequence(machine.lights, &machine.buttons);
            sum += shortest_sequence.unwrap_or(0) as u64;
        }

        sum
    }

    fn solve_p2(&self, input: &str) -> Self::Part2 {
        let re =
            Regex::new(r"\[([.#]+)\]\s+((?:\([0-9,]+\)\s*)+)\{([0-9,]+)\}").expect("Invalid regex");
        let button_re = Regex::new(r"\(([0-9,]+)\)").expect("Invalid regex");

        let machines: Vec<Machine> = input
            .lines()
            .filter_map(|line| {
                let caps = re.captures(line)?;

                // Parse lights as bitmap: [.##.] -> 0b0110
                let lights_str = caps.get(1)?.as_str();
                let lights = lights_str
                    .chars()
                    .enumerate()
                    .filter(|(_, c)| *c == '#')
                    .fold(0, |acc, (i, _)| acc | (1 << i));

                // Parse buttons as Vec of bitmaps
                let buttons_str = caps.get(2)?.as_str();
                let buttons: Vec<u16> = button_re
                    .captures_iter(buttons_str)
                    .map(|cap| {
                        cap.get(1)
                            .unwrap()
                            .as_str()
                            .split(',')
                            .map(|n| n.parse::<u16>().unwrap())
                            .fold(0, |acc, pos| acc | (1 << pos))
                    })
                    .collect();

                // Parse joltages as array
                let joltages_str = caps.get(3)?.as_str();
                let joltages: Vec<u16> = joltages_str
                    .split(',')
                    .map(|n| n.parse::<u16>().unwrap())
                    .collect();

                Some(Machine {
                    lights,
                    buttons,
                    joltages,
                })
            })
            .collect();

        let mut sum = 0;
        for machine in &machines {
            let shortest_sequence = solve_joltages(&machine.buttons, &machine.joltages);
            sum += shortest_sequence.unwrap_or(0) as u64;
        }

        sum
    }
}

fn find_shortest_sequence(target: u16, buttons: &[u16]) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    // start with all lights out
    queue.push_back((0, 0));
    visited.insert(0);

    while let Some((current, depth)) = queue.pop_front() {
        if current == target {
            return Some(depth);
        }

        for button in buttons {
            let new_state = current ^ button;

            if visited.contains(&new_state) {
                continue;
            }

            visited.insert(new_state);
            queue.push_back((new_state, depth + 1));
        }
    }

    None
}

fn solve_joltages(buttons: &[u16], joltages: &[u16]) -> Option<usize> {
    let num_positions = joltages.len();
    let num_buttons = buttons.len();

    // convert bitmask back to positions
    let button_positions: Vec<Vec<usize>> = buttons
        .iter()
        .map(|&button_mask| {
            (0..num_positions)
                .filter(|&pos| button_mask & (1 << pos) != 0)
                .collect()
        })
        .collect();

    let opt = Optimize::new();

    let button_presses: Vec<Int> = (0..num_buttons)
        .map(|i| Int::fresh_const(&format!("button_{}", i)))
        .collect();

    // constraint 1: non negative
    for press in &button_presses {
        opt.assert(&press.ge(0));
    }

    // constraint 2
    for (pos, &target) in joltages.iter().enumerate() {
        let mut terms = Vec::new();

        for (btn_idx, positions) in button_positions.iter().enumerate() {
            if positions.contains(&pos) {
                terms.push(&button_presses[btn_idx]);
            }
        }

        if !terms.is_empty() {
            let sum = Int::add(&terms);
            opt.assert(&sum.eq(Int::from_u64(target as u64)));
        } else if target > 0 {
            return None; // Impossible
        }
    }

    // OBJECTIVE: Minimize total presses
    let total_presses = Int::add(&button_presses.iter().collect::<Vec<_>>());
    opt.minimize(&total_presses);

    // Solve
    match opt.check(&[]) {
        SatResult::Sat => opt
            .get_model()
            .and_then(|model| model.eval(&total_presses, true))
            .and_then(|val| val.as_u64())
            .map(|v| v as usize),
        _ => None,
    }
}
