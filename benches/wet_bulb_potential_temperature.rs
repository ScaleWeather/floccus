use criterion::{criterion_group, criterion_main, Criterion};
use floccus::{formulas::wet_bulb_potential_temperature, formulas::Formula1};

mod utils;
use utils::ReferenceValues;

pub fn benchmark(c: &mut Criterion) {
    let ref_norm = ReferenceValues::normal();

    let mut group = c.benchmark_group("wet_bulb_potential_temperature");

    group.bench_function("DaviesJones1", |b| {
        b.iter(|| wet_bulb_potential_temperature::DaviesJones1::compute(ref_norm.thte))
    });
    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
