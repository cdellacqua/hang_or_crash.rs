use criterion::{criterion_group, criterion_main, Criterion};
use hang_or_crash::CustomSlice;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Demo", |b| {
        b.iter_batched(
            || CustomSlice([0; 1_000_000]),
            |test| {
                assert_eq!(test.0[10], 0);
            },
            criterion::BatchSize::NumIterations(1),
        );
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
