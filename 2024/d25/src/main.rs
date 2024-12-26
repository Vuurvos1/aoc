use std::{fs, time::Instant};

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

fn p1() {
    let input: Vec<u64> = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file")
        .trim_end()
        .split("\n\n")
        .map(|line| {
            let mut result = 0;
            for c in line.chars() {
                result <<= 1;
                result |= (c == '#') as u64;
            }
            result
        })
        .collect::<Vec<_>>();

    let mut keys: Vec<u64> = Vec::new();
    let mut locks: Vec<u64> = Vec::new();

    for grid in input {
        let is_key = grid & 1 == 0;
        if is_key {
            locks.push(grid)
        } else {
            keys.push(grid);
        }
    }

    let mut sum = 0;

    // try all keys on all locks to check fit
    for lock in &locks {
        for key in &keys {
            let fits = key & lock == 0;
            if fits {
                sum += 1;
            }
        }
    }
    println!("p1: {}", sum);
}

fn p2() {}
