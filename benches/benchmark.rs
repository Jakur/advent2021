use advent2021::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("p6", |b| b.iter(|| p6::solve(black_box(&DAY6))));
}

fn just_parse(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let num = u32::from_str_radix(line, 2).unwrap();
        sum += num;
    }
    sum
}

fn just_parse_fast(input: &[u8]) -> u32 {
    let mut sum = 0;
    let mut offset = 12;
    while offset <= input.len() - 1 {
        let mut num = 0;
        debug_assert_eq!(input[offset], 10);
        for i in (1..13).rev() {
            debug_assert!(input[offset - i] != 10);
            num = num << 1;
            num |= (input[offset - i] & 1) as u32;
        }
        sum += num;
        offset += 13;
    }
    sum
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
