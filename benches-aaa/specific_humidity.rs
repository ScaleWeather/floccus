use criterion::{Criterion, black_box, criterion_group, criterion_main};
use floccus::specific_humidity;

pub fn specific_humidity_benchmark(c: &mut Criterion) {
    c.bench_function("specific_humidity::general1", |b| {
        b.iter(|| specific_humidity::general1(black_box(3000.0), black_box(101325.0)))
    });
}

criterion_group!(benches, specific_humidity_benchmark);
criterion_main!(benches);