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

// numeric keypad
// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+

// directional keypad
//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+

type Cache = HashMap<(Vec<char>, i32), i64>;
type Keypad = HashMap<char, (i32, i32)>;

fn get_command(keypad: &Keypad, start: char, end: char) -> Vec<Vec<char>> {
    if start == end {
        return vec![vec!['A']];
    }

    let start_pos = keypad.get(&start).unwrap();
    let end_pos = keypad.get(&end).unwrap();
    let empty_pos = keypad.get(&' ').unwrap();

    let dist = (
        end_pos.0 as i32 - start_pos.0 as i32,
        end_pos.1 as i32 - start_pos.1 as i32,
    );

    let horizontal_first = |dist: (i32, i32)| -> Vec<char> {
        let mut path = Vec::new();
        path.extend(
            std::iter::repeat(if dist.1 > 0 { 'v' } else { '^' }).take(dist.1.abs() as usize),
        );
        path.extend(
            std::iter::repeat(if dist.0 > 0 { '>' } else { '<' }).take(dist.0.abs() as usize),
        );
        path.push('A');
        path
    };

    let vertical_first = |dist: (i32, i32)| -> Vec<char> {
        let mut path = Vec::new();
        path.extend(
            std::iter::repeat(if dist.0 > 0 { '>' } else { '<' }).take(dist.0.abs() as usize),
        );
        path.extend(
            std::iter::repeat(if dist.1 > 0 { 'v' } else { '^' }).take(dist.1.abs() as usize),
        );
        path.push('A');
        path
    };

    let mut unique_paths: Vec<Vec<char>> = Vec::new();

    // vertical first
    if dist.0 != 0 && &(start_pos.0 + dist.0, start_pos.1) != empty_pos {
        unique_paths.push(vertical_first(dist));
    }

    // horizontal first
    if dist.1 != 0 && &(start_pos.0, start_pos.1 + dist.1) != empty_pos {
        unique_paths.push(horizontal_first(dist));
    }

    unique_paths
}

fn get_key_presses(keypad: &Keypad, code: Vec<char>, robot: u32, cache: &mut Cache) -> i64 {
    let key = (code.clone(), robot as i32);
    if let Some(cache) = cache.get(&key) {
        return *cache;
    }

    let vec_keypad = vec![
        (' ', (0, 0)), // empty
        ('^', (1, 0)), // Up
        ('A', (2, 0)), // Action
        ('<', (0, 1)), // Left
        ('v', (1, 1)), // Down
        ('>', (2, 1)), // Right
    ];
    let direction_keypad: Keypad = vec_keypad.into_iter().collect();

    let mut current = 'A';
    let mut length: i64 = 0;
    for i in 0..code.len() {
        let moves = get_command(&keypad, current, code[i]);

        if robot == 0 {
            length += moves[0].len() as i64;
        } else {
            // get min moves
            let min = moves
                .iter()
                .map(|m| get_key_presses(&direction_keypad, m.clone(), robot - 1, cache))
                .min()
                .unwrap();
            length += min;
        }

        current = code[i];
    }

    cache.insert(key, length);
    length
}

fn p1() {
    let codes: Vec<Vec<char>> = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file")
        .trim_end()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let vec_keypad = vec![
        ('7', (0, 0)),
        ('8', (1, 0)),
        ('9', (2, 0)),
        ('4', (0, 1)),
        ('5', (1, 1)),
        ('6', (2, 1)),
        ('1', (0, 2)),
        ('2', (1, 2)),
        ('3', (2, 2)),
        (' ', (0, 3)),
        ('0', (1, 3)),
        ('A', (2, 3)),
    ];

    let numeric_keypad: Keypad = vec_keypad.into_iter().collect();
    let mut cache: Cache = HashMap::new();

    let mut sum: i64 = 0;
    for code in codes {
        // part of code parsed WITHOUT last char
        let numberic_part = code[..code.len() - 1]
            .iter()
            .collect::<String>()
            .parse::<i64>()
            .unwrap();
        let key_presses = get_key_presses(&numeric_keypad, code, 2, &mut cache);

        sum += numberic_part * key_presses;
    }

    println!("p1: {}", sum);
}

fn p2() {
    let codes: Vec<Vec<char>> = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file")
        .trim_end()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let vec_keypad = vec![
        ('7', (0, 0)),
        ('8', (1, 0)),
        ('9', (2, 0)),
        ('4', (0, 1)),
        ('5', (1, 1)),
        ('6', (2, 1)),
        ('1', (0, 2)),
        ('2', (1, 2)),
        ('3', (2, 2)),
        (' ', (0, 3)),
        ('0', (1, 3)),
        ('A', (2, 3)),
    ];

    let numeric_keypad: Keypad = vec_keypad.into_iter().collect();
    let mut cache: Cache = HashMap::new();

    let mut sum: i64 = 0;
    for code in codes {
        // part of code parsed WITHOUT last char
        let numberic_part = code[..code.len() - 1]
            .iter()
            .collect::<String>()
            .parse::<i64>()
            .unwrap();
        let key_presses = get_key_presses(&numeric_keypad, code, 25, &mut cache);

        sum += numberic_part * key_presses;
    }

    println!("p2: {}", sum);
}
