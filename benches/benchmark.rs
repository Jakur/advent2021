use advent2021::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("p12", |b| b.iter(|| p12::solve(black_box(&DAY12))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
