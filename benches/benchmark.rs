use advent2021::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day 3", |b| {
        b.iter(|| p3::solve(black_box(&advent2021::DAY3)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
