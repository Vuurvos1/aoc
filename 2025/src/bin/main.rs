use aoc2025::{
    d01,
    d02,
    d03,
    d04,
    // d05, d06, d07, d08, d09, d10, d11, d12
};
use std::env;

struct DayResult {
    p1_result: String,
    p2_result: String,
    p1_time: std::time::Duration,
    p2_time: std::time::Duration,
    total_time: std::time::Duration,
}

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

    let mut total_time = std::time::Duration::new(0, 0);

    for day_num in 1..=25 {
        if let Ok(input) = std::fs::read_to_string(format!(
            "{}/input/d{:02}.txt",
            env!("CARGO_MANIFEST_DIR"),
            day_num
        )) {
            if let Some(result) = execute_day(day_num, &input) {
                print_compact(day_num, &result);
                total_time += result.total_time;
            }
        }
    }

    println!("=== Total time: {:.2?} ===", total_time);
}

fn run_single_day(day_num: u32) {
    let input = std::fs::read_to_string(format!(
        "{}/input/d{:02}.txt",
        env!("CARGO_MANIFEST_DIR"),
        day_num
    ))
    .expect(&format!("Input file not found for day {}", day_num));

    println!("--- Day {} ---", day_num);

    if let Some(result) = execute_day(day_num, &input) {
        print_verbose(&result);
    } else {
        panic!("Day not implemented");
    }
}

fn execute_day(day_num: u32, input: &str) -> Option<DayResult> {
    let input = normalize_input(input);
    let total_start = std::time::Instant::now();

    let (p1_result, p2_result, p1_time, p2_time) = match day_num {
        1 => run_day_solution(d01::Day01, &input),
        2 => run_day_solution(d02::Day02, &input),
        3 => run_day_solution(d03::Day03, &input),
        4 => run_day_solution(d04::Day04, &input),
        // 5 => run_day_solution(d05::Day05, &input),
        // 6 => run_day_solution(d06::Day06, &input),
        // 7 => run_day_solution(d07::Day07, &input),
        // 8 => run_day_solution(d08::Day08, &input),
        // 9 => run_day_solution(d09::Day09, &input),
        // 10 => run_day_solution(d10::Day10, &input),
        // 11 => run_day_solution(d11::Day11, &input),
        // 12 => run_day_solution(d12::Day12, &input),
        _ => return None,
    };

    let total_time = total_start.elapsed();

    Some(DayResult {
        p1_result,
        p2_result,
        p1_time,
        p2_time,
        total_time,
    })
}

fn run_day_solution<S: aoc2025::Solution>(
    solution: S,
    input: &str,
) -> (String, String, std::time::Duration, std::time::Duration) {
    let now = std::time::Instant::now();
    let p1_result = solution.solve_p1(input);
    let p1_time = now.elapsed();

    let now = std::time::Instant::now();
    let p2_result = solution.solve_p2(input);
    let p2_time = now.elapsed();

    (
        p1_result.to_string(),
        p2_result.to_string(),
        p1_time,
        p2_time,
    )
}

fn print_verbose(result: &DayResult) {
    println!("Part 1: {:.2?}", result.p1_time);
    println!("{}", result.p1_result);
    println!("Part 2: {:.2?}", result.p2_time);
    println!("{}", result.p2_result);
}

fn print_compact(day_num: u32, result: &DayResult) {
    println!(
        "Day {}: Total={:.2?} | P1={:.2?} | P2={:.2?}",
        day_num, result.total_time, result.p1_time, result.p2_time
    );
}

fn normalize_input(input: &str) -> String {
    input
        .lines()
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string()
}
