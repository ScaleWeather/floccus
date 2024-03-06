#![allow(unused)]
use criterion::Criterion;
use floccus::{formulas::vapour_pressure, Formula1, Formula2};

// this is the best way to avoid code duplication I could find
include!("./reference_values.rs");

pub fn benchmark(c: &mut Criterion) {
    let ref_norm = ReferenceValues::normal();
    let ref_freeze = ReferenceValues::freeze();

    let mut group = c.benchmark_group("vapour_pressure");


    group.bench_function("vapour_pressure::definition1", |b| {
        b.iter(|| vapour_pressure::Definition1::compute(ref_norm.sphu, ref_norm.pres))
    });

    group.bench_function("vapour_pressure::definition2", |b| {
        b.iter(|| vapour_pressure::Definition2::compute(ref_norm.savp, ref_norm.rehu))
    });

    group.bench_function("vapour_pressure::tetens1", |b| {
        b.iter(|| vapour_pressure::Tetens1::compute(ref_norm.dwpt))
    });

    group.bench_function("vapour_pressure::buck1", |b| {
        b.iter(|| vapour_pressure::Buck1::compute(ref_norm.dwpt, ref_norm.pres))
    });

    group.bench_function("vapour_pressure::buck2", |b| {
        b.iter(|| vapour_pressure::Buck2::compute(ref_freeze.dwpt, ref_freeze.pres))
    });

    group.bench_function("vapour_pressure::buck3", |b| {
        b.iter(|| vapour_pressure::Buck3::compute(ref_norm.dwpt, ref_norm.pres))
    });

    group.bench_function("vapour_pressure::buck4", |b| {
        b.iter(|| vapour_pressure::Buck4::compute(ref_freeze.dwpt, ref_freeze.pres))
    });

    group.bench_function("vapour_pressure::buck3_simplified", |b| {
        b.iter(|| vapour_pressure::Buck3Simplified::compute(ref_norm.dwpt))
    });

    group.bench_function("vapour_pressure::buck4_simplified", |b| {
        b.iter(|| vapour_pressure::Buck4Simplified::compute(ref_freeze.dwpt))
    });

    group.bench_function("vapour_pressure::wexler1", |b| {
        b.iter(|| vapour_pressure::Wexler1::compute(ref_norm.dwpt))
    });

    group.bench_function("vapour_pressure::wexler2", |b| {
        b.iter(|| vapour_pressure::Wexler2::compute(ref_freeze.dwpt))
    });
    group.finish();
}
