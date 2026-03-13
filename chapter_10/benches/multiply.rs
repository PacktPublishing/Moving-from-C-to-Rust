use criterion::{criterion_group, criterion_main, Criterion};
use performance_test::{make_random, matrix_multiply};
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    let x = make_random(100, 100);
    let y = make_random(100, 100);
    c.bench_function("multiply 100x100", |b| {
        b.iter(|| matrix_multiply(black_box(x.clone()), black_box(y.clone())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
