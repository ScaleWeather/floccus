use criterion::{black_box, criterion_group, criterion_main, Criterion};
use floccus::vapour_pressure;

pub fn vapour_pressure_benchmark(c: &mut Criterion) {
    c.bench_function("vapour_pressure::general1", |b| {
        b.iter(|| vapour_pressure::general1(black_box(0.022), black_box(101325.0)))
    });

    c.bench_function("vapour_pressure::tetens1", |b| {
        b.iter(|| vapour_pressure::tetens1(black_box(300.0)))
    });

    c.bench_function("vapour_pressure::buck1", |b| {
        b.iter(|| vapour_pressure::buck1(black_box(300.0), black_box(101325.0)))
    });

    c.bench_function("vapour_pressure::buck2", |b| {
        b.iter(|| vapour_pressure::buck2(black_box(250.0), black_box(101325.0)))
    });

    c.bench_function("vapour_pressure::buck3", |b| {
        b.iter(|| vapour_pressure::buck3(black_box(300.0), black_box(101325.0)))
    });

    c.bench_function("vapour_pressure::buck4", |b| {
        b.iter(|| vapour_pressure::buck4(black_box(250.0), black_box(101325.0)))
    });

    c.bench_function("vapour_pressure::buck3_simplified", |b| {
        b.iter(|| vapour_pressure::buck3_simplified(black_box(300.0)))
    });

    c.bench_function("vapour_pressure::buck4_simplified", |b| {
        b.iter(|| vapour_pressure::buck4_simplified(black_box(250.0)))
    });

    c.bench_function("vapour_pressure::saturation_specific1", |b| {
        b.iter(|| vapour_pressure::saturation_specific1(black_box(3000.0), black_box(0.5)))
    });

    c.bench_function("vapour_pressure::saturation_specific2", |b| {
        b.iter(|| vapour_pressure::saturation_specific2(black_box(3000.0), black_box(0.5)))
    });

    c.bench_function("vapour_pressure::wexler1", |b| {
        b.iter(|| vapour_pressure::wexler1(black_box(300.0)))
    });

    c.bench_function("vapour_pressure::wexler2", |b| {
        b.iter(|| vapour_pressure::wexler2(black_box(250.0)))
    });
}

criterion_group!(benches, vapour_pressure_benchmark);
criterion_main!(benches);
