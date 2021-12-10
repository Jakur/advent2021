use advent2021::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("p9", |b| b.iter(|| p9::solve(black_box(&DAY9))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
