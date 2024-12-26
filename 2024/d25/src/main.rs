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
    let input: Vec<Vec<u8>> = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file")
        .trim_end()
        .split("\n\n")
        .map(|line| {
            line.lines()
                .map(|l| {
                    let mut result = 0u8;
                    for &byte in l.as_bytes() {
                        result <<= 1; // Shift 1 left
                        result |= (byte == b'#') as u8; // Add 1 if the byte is `#`
                    }
                    result
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut keys: Vec<Vec<u8>> = Vec::new();
    let mut locks: Vec<Vec<u8>> = Vec::new();

    for grid in input {
        let is_key = grid[0] == 0;
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
            let fits = key.iter().zip(lock.iter()).all(|(l, k)| l & k == 0);
            if fits {
                sum += 1;
            }
        }
    }
    println!("p1: {}", sum);
}

fn p2() {}
