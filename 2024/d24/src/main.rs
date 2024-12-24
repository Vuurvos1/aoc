use std::{collections::HashMap, fs, time::Instant};

fn main() {
    let now = Instant::now();
    p1();
    let elapsed = now.elapsed();
    println!("p1: {:.2?}", elapsed);

    let now = Instant::now();
    p2();
    let elapsed = now.elapsed();
    println!("p2: {:.2?}", elapsed);
}

#[derive(Debug, Clone)]
struct Instruction {
    op: String,
    a: String,
    b: String,
    out: String,
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

fn p1() {
    let raw_input = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file")
        .trim_end()
        .to_string();
    let input = raw_input.split("\n\n").collect::<Vec<_>>();

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

    println!("p1: {}", sum);
}

fn p2() {
    let raw_input = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file")
        .trim_end()
        .to_string();
    let input = raw_input.split("\n\n").collect::<Vec<_>>();

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

    // sort instructions so ands are first, then ors, then xors, after sort alphabetically
    instructions.sort_by(|a, b| {
        let a_val = match a.op.as_str() {
            "AND" => 0,
            "OR" => 1,
            "XOR" => 2,
            _ => unreachable!(),
        };

        let b_val = match b.op.as_str() {
            "AND" => 0,
            "OR" => 1,
            "XOR" => 2,
            _ => unreachable!(),
        };

        a_val
            .cmp(&b_val)
            .then_with(|| a.a.cmp(&b.a))
            .then_with(|| a.b.cmp(&b.b))
            .then_with(|| a.out.cmp(&b.out))
    });

    let mut mermaid_flowchart = String::from("flowchart TD\n");
    for instruction in instructions.iter() {
        // a to op
        mermaid_flowchart.push_str(&format!(
            "{}{{{}}} --> {}_{}_{}[{}]\n",
            instruction.a,
            instruction.a,
            instruction.a,
            instruction.b,
            instruction.op,
            instruction.op
        ));

        // b to op
        mermaid_flowchart.push_str(&format!(
            "{}{{{}}} --> {}_{}_{}[{}]\n",
            instruction.b,
            instruction.b,
            instruction.a,
            instruction.b,
            instruction.op,
            instruction.op
        ));

        // op to out
        mermaid_flowchart.push_str(&format!(
            "{}_{}_{}[{}] --> {}{{{}}}\n",
            instruction.a,
            instruction.b,
            instruction.op,
            instruction.op,
            instruction.out,
            instruction.out
        ));
    }

    // put into file
    fs::write("./src/flowchart.mmd", mermaid_flowchart)
        .expect("Should have been able to write the file");

    // run mmdc -c ./src/config.schema.json -i ./src/flowchart.mmd -o ./src/output.svg -s 2
    // and look for weirdness in the flowchart
}
