use aoc2024::{d01, d23, d25};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 || (args.len() == 2 && args[1] == "all") {
        run_all_days();
    } else {
        let day_to_run = &args[1];
        let day_num: u32 = day_to_run
            .parse()
            .expect("Please provide a valid day number");
        run_single_day(day_num);
    }
}

fn run_all_days() {
    println!("=== Running All Days ===");

    let implemented_days = vec![1, 23, 25];
    let mut total_time = std::time::Duration::new(0, 0);

    for day_num in implemented_days {
        if let Ok(input) = std::fs::read_to_string(format!("./inputs/d{:02}.txt", day_num)) {
            let day_start = std::time::Instant::now();

            match day_num {
                1 => run_day_compact(d01::Day01, &input, day_num),
                23 => run_day_compact(d23::Day23, &input, day_num),
                25 => run_day_compact(d25::Day25, &input, day_num),
                _ => continue,
            }

            let day_elapsed = day_start.elapsed();
            total_time += day_elapsed;
        } else {
            println!("Day {}: Input file not found", day_num);
        }
    }

    println!("=== Total time: {:.2?} ===", total_time);
}

fn run_single_day(day_num: u32) {
    let input = std::fs::read_to_string(format!("./inputs/d{:02}.txt", day_num)).unwrap();

    println!("--- Day {} ---", day_num);

    match day_num {
        1 => run_day(d01::Day01, &input),
        23 => run_day(d23::Day23, &input),
        // 24 => run_day(d24::Day24, &input), // solved by hand
        25 => run_day(d25::Day25, &input),
        _ => panic!("Day not implemented"),
    }
}

fn run_day<S: aoc2024::Solution>(solution: S, input: &str) {
    let now = std::time::Instant::now();
    let p1_result = solution.solve_p1(input);
    let elapsed = now.elapsed();
    println!("Part 1: {:.2?}", elapsed);
    println!("{}", p1_result);

    let now = std::time::Instant::now();
    let p2_result = solution.solve_p2(input);
    let elapsed = now.elapsed();
    println!("Part 2: {:.2?}", elapsed);
    println!("{}", p2_result);
}

fn run_day_compact<S: aoc2024::Solution>(solution: S, input: &str, day_num: u32) {
    let day_start = std::time::Instant::now();

    let now = std::time::Instant::now();
    let _p1_result = solution.solve_p1(input);
    let p1_elapsed = now.elapsed();

    let now = std::time::Instant::now();
    let _p2_result = solution.solve_p2(input);
    let p2_elapsed = now.elapsed();

    let day_elapsed = day_start.elapsed();

    println!(
        "Day {}: Total={:.2?} | P1={:.2?} | P2={:.2?}",
        day_num, day_elapsed, p1_elapsed, p2_elapsed
    );
}
