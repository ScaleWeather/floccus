use criterion::{Criterion, black_box, criterion_group, criterion_main};
use floccus::mixing_ratio;

pub fn mixing_ratio_benchmark(c: &mut Criterion) {
    c.bench_function("air general 1", |b| {
        b.iter(|| mixing_ratio::air_general1(black_box(101325.0), black_box(3500.0)))
    });
    c.bench_function("air performance 1", |b| {
        b.iter(|| mixing_ratio::air_performance1(black_box(300.0), black_box(101325.0)))
    });
    c.bench_function("air accuracy 1", |b| {
        b.iter(|| mixing_ratio::air_accuracy1(black_box(300.0), black_box(101325.0)))
    });
}

criterion_group!(benches, mixing_ratio_benchmark);
criterion_main!(benches);
