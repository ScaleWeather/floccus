use criterion::{Criterion, black_box, criterion_group, criterion_main};
use floccus::potential_temperature;

pub fn potential_temperature_benchmark(c: &mut Criterion) {
    c.bench_function("potential_temperature::davies_jones1", |b| {
        b.iter(|| potential_temperature::davies_jones1(black_box(300.0), black_box(101325.0), black_box(3000.0)))
    });
}

criterion_group!(benches, potential_temperature_benchmark);
criterion_main!(benches);