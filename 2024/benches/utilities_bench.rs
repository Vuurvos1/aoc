use aoc2024::utils::count_digits;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

// bench cast against to_digit
fn char_to_digit(c: char) -> u32 {
    c.to_digit(10).unwrap()
}

fn char_to_cast(c: char) -> u32 {
    c as u32 - 48
}

// bench digit count functions
fn count_digits_log(n: u64) -> u32 {
    if n == 0 {
        1
    } else {
        (n as f64).log10().floor() as u32 + 1
    }
}

fn utility_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("utilities");
    group.bench_function("char_to_digit", |b| {
        b.iter(|| char_to_digit(black_box('5')))
    });
    group.bench_function("char_to_cast", |b| b.iter(|| char_to_cast(black_box('5'))));

    group.bench_function("count_digits_log", |b| {
        b.iter(|| count_digits_log(black_box(1234567890)))
    });
    group.bench_function("count_digits", |b| {
        b.iter(|| count_digits(black_box(1234567890)))
    });
    group.finish();
}

criterion_group!(benches, utility_benchmarks);
criterion_main!(benches);
