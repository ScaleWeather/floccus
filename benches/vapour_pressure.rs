use criterion::{Criterion, black_box, criterion_group, criterion_main};
use floccus::vapour_pressure;

pub fn vapour_pressure_benchmark(c: &mut Criterion) {
    c.bench_function("tetens 1", |b| {
        b.iter(|| vapour_pressure::tetens1(black_box(300.0)))
    });
    c.bench_function("buck 1", |b| {
        b.iter(|| vapour_pressure::buck1(black_box(300.0), black_box(101325.0)))
    });
}

criterion_group!(benches, vapour_pressure_benchmark);
criterion_main!(benches);
