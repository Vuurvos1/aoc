use std::collections::HashMap;

use crate::Solution;

#[derive(Debug, Clone)]
struct Instruction {
    op: String,
    a: String,
    b: String,
    out: String,
}

pub struct Day24;

impl Solution for Day24 {
    type Part1 = u64;
    type Part2 = u32;

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        let input = input.split("\n\n").collect::<Vec<_>>();

        let mut memory: HashMap<String, bool> = HashMap::new();
        let mut instructions: Vec<Instruction> = Vec::new();

        for line in input[0].lines() {
            let s = line.split(": ").collect::<Vec<_>>();
            memory.insert(s[0].to_string(), s[1] == "1");
        }

        for line in input[1].lines() {
            let s = line.split(" ").collect::<Vec<_>>();
            instructions.push(Instruction {
                a: s[0].to_string(),
                op: s[1].to_string(),
                b: s[2].to_string(),
                out: s[4].to_string(),
            });
        }

        while instructions.len() > 0 {
            let mut did_something = false;

            // do all possible gates, and remove if done
            for i in 0..instructions.len() {
                if i >= instructions.len() {
                    break;
                }

                let instruction = &instructions[i];
                if memory.contains_key(&instruction.a) && memory.contains_key(&instruction.b) {
                    do_instruction(&mut memory, instruction);
                    instructions.remove(i);
                    did_something = true;
                }
            }

            if !did_something {
                break;
            }
        }

        // get all memory addresses starting with a "z", sort them and bit shift them together
        let mut z_addresses = memory
            .iter()
            .filter(|(key, _)| key.starts_with("z"))
            .map(|(key, _)| key)
            .collect::<Vec<_>>();
        z_addresses.sort();

        let mut sum: u64 = 0;
        for key in z_addresses.iter().rev() {
            let v = memory.get(*key).unwrap();
            // shift one onto sum based on bool
            sum = (sum << 1) | (*v as u64)
        }

        sum
    }

    fn solve_p2(&self, _input: &str) -> Self::Part2 {
        0
    }
}

fn do_instruction(memory: &mut HashMap<String, bool>, instruction: &Instruction) {
    let a = memory.get(&instruction.a).unwrap();
    let b = memory.get(&instruction.b).unwrap();

    let out = match instruction.op.as_str() {
        "AND" => a & b,
        "OR" => a | b,
        "XOR" => a ^ b,
        _ => unreachable!(),
    };

    memory.insert(instruction.out.clone(), out);
}
