use aoc2025::{
    d01::Day01,
    d02::Day02,
    d03::Day03,
    //  d04::Day04, d05::Day05, d06::Day06, d07::Day07, d08::Day08,
    // d09::Day09, d10::Day10, d11::Day11, d12::Day12,
    Solution,
};
use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;

fn run_bench<S: Solution>(c: &mut Criterion, day: u32, solution: S) {
    let input = std::fs::read_to_string(format!("input/d{:02}.txt", day)).unwrap();
    let input = normalize_input(&input);
    let mut group = c.benchmark_group(format!("d{:02}", day));
    group
        .warm_up_time(Duration::from_secs(3))
        .measurement_time(Duration::from_secs(10))
        .sample_size(10);
    group.bench_function("Part 1", |b| b.iter(|| solution.solve_p1(&input)));
    group.bench_function("Part 2", |b| b.iter(|| solution.solve_p2(&input)));
    group.finish();
}

fn all_benchmarks(c: &mut Criterion) {
    run_bench(c, 1, Day01);
    run_bench(c, 2, Day02);
    run_bench(c, 3, Day03);
    // run_bench(c, 4, Day04);
    // run_bench(c, 5, Day05);
    // run_bench(c, 6, Day06);
    // run_bench(c, 7, Day07);
    // run_bench(c, 8, Day08);
    // run_bench(c, 9, Day09);
    // run_bench(c, 10, Day10);
    // run_bench(c, 11, Day11);
    // run_bench(c, 12, Day12);
}

criterion_group!(benches, all_benchmarks);
criterion_main!(benches);

fn normalize_input(input: &str) -> String {
    input
        .lines()
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string()
}
