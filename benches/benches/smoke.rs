//! Каркас: проверяет, что пайплайн `cargo xtask bench` работает.
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

fn smoke(c: &mut Criterion) {
    c.bench_function("smoke/sum_1k", |b| {
        b.iter(|| (0..black_box(1000u32)).sum::<u32>())
    });
}

criterion_group!(benches, smoke);
criterion_main!(benches);
