use criterion::{criterion_group, criterion_main, Criterion};
use floccus::{formulas::wet_bulb_temperature, formulas::Formula2};

mod utils;
use utils::ReferenceValues;

pub fn benchmark(c: &mut Criterion) {
    let ref_norm = ReferenceValues::normal();

    let mut group = c.benchmark_group("wet_bulb_temperature");

    group.bench_function("stull1", |b| {
        b.iter(|| wet_bulb_temperature::Stull1::compute(ref_norm.temp, ref_norm.rehu))
    });
    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
