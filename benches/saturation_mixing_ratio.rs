use criterion::Criterion;
use floccus::{formulas::saturation_mixing_ratio, Formula2};

// this is the best way to avoid code duplication I could find
include!("./reference_values.rs");

pub fn benchmark(c: &mut Criterion) {
    let ref_norm = ReferenceValues::normal();

    c.bench_function("saturation_mixing_ratio::definition1", |b| {
        b.iter(|| saturation_mixing_ratio::Definition1::compute(ref_norm.pres, ref_norm.savp))
    });

    c.bench_function("saturation_mixing_ratio::definition2", |b| {
        b.iter(|| saturation_mixing_ratio::Definition2::compute(ref_norm.mxrt, ref_norm.rehu))
    });
}
