use criterion::{criterion_group, criterion_main};

mod relative_humidity;
mod saturation_vapour_pressure;
mod vapour_pressure;

criterion_group!(
    benches,
    vapour_pressure::benchmark,
    saturation_vapour_pressure::benchmark,
    relative_humidity::benchmark
);
criterion_main!(benches);
