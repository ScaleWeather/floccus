use criterion::{Criterion, black_box, criterion_group, criterion_main};
use floccus::equivalent_potential_temperature;

pub fn equivalent_potential_temperature_benchmark(c: &mut Criterion) {
    c.bench_function("equivalent_potential_temperature::bryan1", |b| {
        b.iter(|| equivalent_potential_temperature::bryan1(black_box(300.0), black_box(101325.0), black_box(3000.0)))
    });
}

criterion_group!(benches, equivalent_potential_temperature_benchmark);
criterion_main!(benches);