use criterion::Criterion;
use floccus::{formulas::wet_bulb_potential_temperature, Formula1};

// this is the best way to avoid code duplication I could find
include!("./reference_values.rs");

pub fn benchmark(c: &mut Criterion) {
    let ref_norm = ReferenceValues::normal();

    let mut group = c.benchmark_group("wet_bulb_potential_temperature");

    group.bench_function("wet_bulb_potential_temperature::DaviesJones1", |b| {
        b.iter(|| wet_bulb_potential_temperature::DaviesJones1::compute(ref_norm.thte))
    });
    group.finish();
}
