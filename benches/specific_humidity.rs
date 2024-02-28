use criterion::Criterion;
use floccus::{formulas::specific_humidity, Formula2};

// this is the best way to avoid code duplication I could find
include!("./reference_values.rs");

pub fn benchmark(c: &mut Criterion) {
    let ref_norm = ReferenceValues::normal();

    c.bench_function("specific_humidity::definition1", |b| {
        b.iter(|| specific_humidity::Definition1::compute(ref_norm.vapr, ref_norm.pres))
    });
}
