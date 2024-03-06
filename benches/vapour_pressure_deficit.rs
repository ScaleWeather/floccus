#![allow(unused)]
use criterion::Criterion;
use floccus::{formulas::vapour_pressure_deficit, Formula2};

// this is the best way to avoid code duplication I could find
include!("./reference_values.rs");

pub fn benchmark(c: &mut Criterion) {
    let ref_norm = ReferenceValues::normal();

    let mut group = c.benchmark_group("vapour_pressure_deficit");

    group.bench_function("vapour_pressure_deficit::definition1", |b| {
        b.iter(|| vapour_pressure_deficit::Definition1::compute(ref_norm.vapr, ref_norm.savp))
    });
    group.finish();
}
