use criterion::{Criterion, black_box, criterion_group, criterion_main};
use floccus::virtual_temperature;

pub fn virtual_temperature_benchmark(c: &mut Criterion) {
    c.bench_function("virtual_temperature::general1", |b| {
        b.iter(|| virtual_temperature::general1(black_box(300.0), black_box(0.022)))
    });

    c.bench_function("virtual_temperature::general2", |b| {
        b.iter(|| virtual_temperature::general2(black_box(300.0), black_box(101325.0), black_box(3550.0)))
    });
}

criterion_group!(benches, virtual_temperature_benchmark);
criterion_main!(benches);
