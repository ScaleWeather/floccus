use criterion::Criterion;
use floccus::{formulas::wet_bulb_temperature, Formula2};

// this is the best way to avoid code duplication I could find
include!("./reference_values.rs");

pub fn benchmark(c: &mut Criterion) {
    let ref_norm = ReferenceValues::normal();

    let mut group = c.benchmark_group("wet_bulb_temperature");

    group.bench_function("wet_bulb_temperature::stull1", |b| {
        b.iter(|| wet_bulb_temperature::Stull1::compute(ref_norm.temp, ref_norm.rehu))
    });
    group.finish();
}
