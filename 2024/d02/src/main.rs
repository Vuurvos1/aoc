use std::fs;

fn main() {
    println!("Hello, world!");
}

fn is_safe_report(v: Vec<i32>) -> bool {
    let mut split = v.iter();
    let mut sign_dir = 0;

    let mut prev = split.next().unwrap();
    for item in split {
        let diff = (item - prev).abs();
        if diff > 3 || diff < 1 {
            // println!("Items are not in safe range");
            return false;
        }

        let sign = (item - prev).signum();
        if sign_dir == 0 {
            sign_dir = sign;
        } else if sign_dir != sign {
            // println!("Items are not ascending or descending");
            return false;
        }

        prev = item;
    }

    true
}

#[test]
fn p1() {
    let contents =
        fs::read_to_string("./src/input.txt").expect("Should have been able to read the file");

    let mut sum = 0;

    for line in contents.lines() {
        let split: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let safe = is_safe_report(split);

        if safe {
            sum += 1;
        }
    }

    println!("Hello, world! {}", sum);
}

#[test]
fn p2() {
    let contents =
        fs::read_to_string("./src/input.txt").expect("Should have been able to read the file");

    let mut sum = 0;

    for line in contents.lines() {
        let split = line.split_whitespace().map(|x| x.parse::<i32>().unwrap());
        let v: Vec<i32> = split.collect();
        for i in 0..v.len() {
            let mut c = v.clone();
            c.remove(i);
            let safe = is_safe_report(c);
            if safe {
                sum += 1;
                break;
            }
        }
    }

    println!("Hello, world! {}", sum);
}
