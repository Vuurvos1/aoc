use criterion::{black_box, criterion_group, criterion_main, Criterion};

// bench cast against to_digit
fn char_to_digit(c: char) -> u32 {
    c.to_digit(10).unwrap()
}

fn char_to_cast(c: char) -> u32 {
    c as u32 - 48
}

fn utility_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("utilities");
    group.bench_function("char_to_digit", |b| {
        b.iter(|| char_to_digit(black_box('5')))
    });
    group.bench_function("char_to_cast", |b| b.iter(|| char_to_cast(black_box('5'))));
    group.finish();
}

criterion_group!(benches, utility_benchmarks);
criterion_main!(benches);
