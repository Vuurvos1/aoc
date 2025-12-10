use regex::Regex;

use crate::Solution;

pub struct Day10;

struct Machine {
    lights: u32,
    buttons: Vec<u32>,
    joltages: Vec<u32>,
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
                    .fold(0u32, |acc, (i, _)| acc | (1 << i));

                // Parse buttons as Vec of bitmaps
                let buttons_str = caps.get(2)?.as_str();
                let buttons: Vec<u32> = button_re
                    .captures_iter(buttons_str)
                    .map(|cap| {
                        cap.get(1)
                            .unwrap()
                            .as_str()
                            .split(',')
                            .map(|n| n.parse::<u32>().unwrap())
                            .fold(0u32, |acc, pos| acc | (1 << pos))
                    })
                    .collect();

                // Parse joltages as array
                let joltages_str = caps.get(3)?.as_str();
                let joltages: Vec<u32> = joltages_str
                    .split(',')
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect();

                Some(Machine {
                    lights,
                    buttons,
                    joltages,
                })
            })
            .collect();

        for machine in &machines {
            // Print lights as binary with # and .
            let lights_str: String = (0..8)
                .map(|i| {
                    if machine.lights & (1 << i) != 0 {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect();
            println!("lights: [{}]", lights_str);

            // Print each button as binary with # and .
            for (i, &button) in machine.buttons.iter().enumerate() {
                let button_str: String = (0..8)
                    .map(|i| if button & (1 << i) != 0 { '#' } else { '.' })
                    .collect();
                println!("button {}: ({})", i, button_str);
            }

            println!("joltages: {:?}", machine.joltages);
            println!();
        }

        0
    }

    fn solve_p2(&self, input: &str) -> Self::Part2 {
        0
    }
}
