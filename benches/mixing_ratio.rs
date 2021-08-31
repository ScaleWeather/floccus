use criterion::{Criterion, black_box, criterion_group, criterion_main};
use floccus::mixing_ratio;

pub fn mixing_ratio_benchmark(c: &mut Criterion) {
    c.bench_function("general 1", |b| {
        b.iter(|| mixing_ratio::general1(black_box(101325.0), black_box(3500.0)))
    });
    c.bench_function("performance 1", |b| {
        b.iter(|| mixing_ratio::performance1(black_box(300.0), black_box(101325.0)))
    });
    c.bench_function("accuracy 1", |b| {
        b.iter(|| mixing_ratio::accuracy1(black_box(300.0), black_box(101325.0)))
    });
}

criterion_group!(benches, mixing_ratio_benchmark);
criterion_main!(benches);
