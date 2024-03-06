#![allow(unused)]
use criterion::Criterion;
use floccus::{formulas::vapour_pressure, Formula1, Formula2};

// this is the best way to avoid code duplication I could find
include!("./reference_values.rs");

pub fn benchmark(c: &mut Criterion) {
    let ref_norm = ReferenceValues::normal();
    let ref_freeze = ReferenceValues::freeze();

    let mut group = c.benchmark_group("vapour_pressure");


    group.bench_function("definition1", |b| {
        b.iter(|| vapour_pressure::Definition1::compute(ref_norm.sphu, ref_norm.pres))
    });

    group.bench_function("definition2", |b| {
        b.iter(|| vapour_pressure::Definition2::compute(ref_norm.savp, ref_norm.rehu))
    });

    group.bench_function("tetens1", |b| {
        b.iter(|| vapour_pressure::Tetens1::compute(ref_norm.dwpt))
    });

    group.bench_function("buck1", |b| {
        b.iter(|| vapour_pressure::Buck1::compute(ref_norm.dwpt, ref_norm.pres))
    });

    group.bench_function("buck2", |b| {
        b.iter(|| vapour_pressure::Buck2::compute(ref_freeze.dwpt, ref_freeze.pres))
    });

    group.bench_function("buck3", |b| {
        b.iter(|| vapour_pressure::Buck3::compute(ref_norm.dwpt, ref_norm.pres))
    });

    group.bench_function("buck4", |b| {
        b.iter(|| vapour_pressure::Buck4::compute(ref_freeze.dwpt, ref_freeze.pres))
    });

    group.bench_function("buck3_simplified", |b| {
        b.iter(|| vapour_pressure::Buck3Simplified::compute(ref_norm.dwpt))
    });

    group.bench_function("buck4_simplified", |b| {
        b.iter(|| vapour_pressure::Buck4Simplified::compute(ref_freeze.dwpt))
    });

    group.bench_function("wexler1", |b| {
        b.iter(|| vapour_pressure::Wexler1::compute(ref_norm.dwpt))
    });

    group.bench_function("wexler2", |b| {
        b.iter(|| vapour_pressure::Wexler2::compute(ref_freeze.dwpt))
    });
    group.finish();
}
