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

fn next_number(seed: u64) -> u64 {
    let mut seed = seed;
    seed = (seed ^ (seed << 6)) & 0xffffff; // (seed ^ (seed * 64)) % 16777216;
    seed = (seed ^ (seed >> 5)) & 0xffffff; // (seed ^ (seed / 32)) % 16777216;
    seed = (seed ^ (seed << 11)) & 0xffffff; // (seed ^ (seed * 2048)) % 16777216;
    seed
}

fn p1() {
    let seeds = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file")
        .trim_end()
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut sum: u64 = 0;
    for mut seed in seeds {
        for _ in 0..2000 {
            seed = next_number(seed);
        }

        sum += seed;
    }

    println!("p1: {}", sum);
}

fn p2() {
    let seeds = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file")
        .trim_end()
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let to_index = |prev: u64, curr: u64| 9 + curr % 10 - prev % 10;

    let mut results: HashMap<(u64, u64, u64, u64), u64> = HashMap::new();
    let mut seen: HashMap<(u64, u64, u64, u64), u64> = HashMap::new();

    for (index, seed) in seeds.iter().enumerate() {
        let zeroth = *seed;
        let first = next_number(zeroth);
        let second = next_number(first);
        let third = next_number(second);

        let mut a;
        let mut b = to_index(zeroth, first);
        let mut c = to_index(first, second);
        let mut d = to_index(second, third);

        let mut number = third;

        for _ in 3..2000 {
            let prev = number;
            number = next_number(number);

            (a, b, c, d) = (b, c, d, to_index(prev, number));

            let key = (a, b, c, d);

            if !seen.contains_key(&key) || seen.get(&key).unwrap() != &(index as u64) {
                results
                    .entry(key)
                    .and_modify(|e| *e += number % 10)
                    .or_insert(number % 10);

                seen.insert(key, index as u64);
            }
        }
    }

    println!("p2: {:?}", results.values().max().unwrap());
}
