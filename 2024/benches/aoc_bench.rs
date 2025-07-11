use aoc2024::{d01::Day01, d23::Day23, d25::Day25, Solution};
use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;

fn run_bench<S: Solution>(c: &mut Criterion, day: u32, solution: S) {
    let input = std::fs::read_to_string(format!("inputs/d{:02}.txt", day)).unwrap();
    let mut group = c.benchmark_group(format!("Day {:02}", day));
    group
        .measurement_time(Duration::from_secs(10))
        .sample_size(10);
    group.bench_function("Part 1", |b| b.iter(|| solution.solve_p1(&input)));
    group.bench_function("Part 2", |b| b.iter(|| solution.solve_p2(&input)));
    group.finish();
}

fn all_benchmarks(c: &mut Criterion) {
    run_bench(c, 1, Day01);
    run_bench(c, 23, Day23);
    // run_bench(c, 24, Day24); // solved by hand
    run_bench(c, 25, Day25);
}

criterion_group!(benches, all_benchmarks);
criterion_main!(benches);
