use criterion::{Criterion, black_box, criterion_group, criterion_main};
use floccus::vapour_pressure_deficit;

pub fn virtual_temperature_benchmark(c: &mut Criterion) {
    c.bench_function("vapour_pressure_deficit::general1", |b| {
        b.iter(|| vapour_pressure_deficit::general1(black_box(3000.0), black_box(3550.0)))
    });

    c.bench_function("vapour_pressure_deficit::general2", |b| {
        b.iter(|| vapour_pressure_deficit::general2(black_box(300.0), black_box(290.0), black_box(101325.0)))
    });

    c.bench_function("vapour_pressure_deficit::general3", |b| {
        b.iter(|| vapour_pressure_deficit::general3(black_box(300.0), black_box(0.5), black_box(101325.0)))
    });
}

criterion_group!(benches, virtual_temperature_benchmark);
criterion_main!(benches);
