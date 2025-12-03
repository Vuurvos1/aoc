use crate::Solution;

pub struct Day01;

impl Solution for Day01 {
    type Part1 = u32;
    type Part2 = i32;

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        let mut dial = 50;
        let mut count = 0;

        for line in input.lines() {
            let mut chars = line.chars();
            let direction = chars.next().unwrap();
            let amount: i32 = chars.as_str().parse().unwrap();

            if direction == 'R' {
                dial += amount;
            } else {
                dial -= amount;
            }

            dial = dial % 100;

            if dial == 0 {
                count += 1;
            }
        }

        count
    }

    fn solve_p2(&self, input: &str) -> Self::Part2 {
        let mut dial = 50;
        let mut count = 0;

        for line in input.lines() {
            let (dir, num) = line.split_at(1);
            let delta = num.parse::<i32>().unwrap() * if dir == "R" { 1 } else { -1 };

            dial += delta;

            if dial <= 0 && delta != dial {
                count += 1;
            }
            count += dial.abs() / 100;
            dial = dial.rem_euclid(100);
        }

        count
    }
}

#[test]
fn p2_test1() {
    // end on 0
    let input = "R50";
    let p = Day01.solve_p2(input);
    assert_eq!(p, 1);
}

#[test]
fn p2_test2() {
    // pass 0
    let input = "R51";
    let p = Day01.solve_p2(input);
    assert_eq!(p, 1);
}

#[test]
fn p2_test3() {
    // pass 0 x times
    let input = "R200";
    let p = Day01.solve_p2(input);
    assert_eq!(p, 2);
}
