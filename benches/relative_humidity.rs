use criterion::Criterion;
use floccus::{formulas::relative_humidity, Formula2};

// this is the best way to avoid code duplication I could find
include!("./reference_values.rs");

pub fn benchmark(c: &mut Criterion) {
    let ref_norm = ReferenceValues::normal();

    let mut group = c.benchmark_group("relative_humidity");

    group.bench_function("relative_humidity::definition1", |b| {
        b.iter(|| relative_humidity::Definition1::compute(ref_norm.mxrt, ref_norm.smrt))
    });

    group.bench_function("relative_humidity::definition2", |b| {
        b.iter(|| relative_humidity::Definition2::compute(ref_norm.vapr, ref_norm.savp))
    });
    group.finish();
}
