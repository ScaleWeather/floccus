use criterion::{criterion_group, criterion_main};

mod vapour_pressure;

criterion_group!(benches, vapour_pressure::benchmark);
criterion_main!(benches);
