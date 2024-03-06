#![allow(unused)]
use criterion::Criterion;
use floccus::{formulas::saturation_mixing_ratio, Formula2};

// this is the best way to avoid code duplication I could find
include!("./reference_values.rs");

pub fn benchmark(c: &mut Criterion) {
    let ref_norm = ReferenceValues::normal();

    let mut group = c.benchmark_group("saturation_mixing_ratio");

    group.bench_function("definition1", |b| {
        b.iter(|| saturation_mixing_ratio::Definition1::compute(ref_norm.pres, ref_norm.savp))
    });

    group.bench_function("definition2", |b| {
        b.iter(|| saturation_mixing_ratio::Definition2::compute(ref_norm.mxrt, ref_norm.rehu))
    });

    group.finish();
}
