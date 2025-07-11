use crate::Solution;

pub struct Day02;

impl Solution for Day02 {
    type Part1 = u32;
    type Part2 = u32;

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        let mut sum = 0;

        for line in input.lines() {
            let split: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            let safe = is_safe_report(split);

            if safe {
                sum += 1;
            }
        }

        sum
    }

    fn solve_p2(&self, input: &str) -> Self::Part2 {
        let mut sum = 0;

        for line in input.lines() {
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

        sum
    }
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
