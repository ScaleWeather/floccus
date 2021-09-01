use criterion::{Criterion, black_box, criterion_group, criterion_main};
use floccus::wet_bulb_potential_temperature;

pub fn wet_bulb_potential_temperature_benchmark(c: &mut Criterion) {
    c.bench_function("wet_bulb_potential_temperature::davies_jones1", |b| {
        b.iter(|| wet_bulb_potential_temperature::davies_jones1(black_box(300.0)))
    });
}

criterion_group!(benches, wet_bulb_potential_temperature_benchmark);
criterion_main!(benches);