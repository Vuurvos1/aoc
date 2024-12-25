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
    let input: Vec<Vec<Vec<char>>> = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file")
        .trim_end()
        .split("\n\n")
        .map(|line| {
            line.lines()
                .map(|l| l.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut keys: Vec<Vec<u32>> = Vec::new();
    let mut locks: Vec<Vec<u32>> = Vec::new();

    for grid in input {
        let mut heights: Vec<u32> = vec![0; 5];

        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == grid[0][0] {
                    heights[j] += 1;
                }
            }
        }

        let is_lock = grid[0][0] == '#';
        if is_lock {
            locks.push(heights);
        } else {
            keys.push(heights);
        }
    }

    let mut sum = 0;

    // try all keys on all locks to check fit
    for lock in &locks {
        for key in &keys {
            let fits = key.iter().zip(lock.iter()).all(|(l, k)| l >= k);
            if fits {
                sum += 1;
            }
        }
    }
    println!("p1: {}", sum);
}

fn p2() {}
