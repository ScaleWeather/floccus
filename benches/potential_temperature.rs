use criterion::Criterion;
use floccus::{formulas::potential_temperature, Formula3};

// this is the best way to avoid code duplication I could find
include!("./reference_values.rs");

pub fn benchmark(c: &mut Criterion) {
    let ref_norm = ReferenceValues::normal();

    let mut group = c.benchmark_group("potential_temperature");

    group.bench_function("potential_temperature::definition1", |b| {
        b.iter(|| {
            potential_temperature::Definition1::compute(ref_norm.temp, ref_norm.pres, ref_norm.vapr)
        })
    });
    group.finish();
}
