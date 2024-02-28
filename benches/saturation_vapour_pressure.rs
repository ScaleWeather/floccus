use criterion::Criterion;
use floccus::{formulas::saturation_vapour_pressure, Formula1, Formula2};

// this is the best way to avoid code duplication I could find
include!("./reference_values.rs");

pub fn benchmark(c: &mut Criterion) {
    let ref_norm = ReferenceValues::normal();
    let ref_freeze = ReferenceValues::freeze();

    c.bench_function("saturation_vapour_pressure::definition1", |b| {
        b.iter(|| saturation_vapour_pressure::Definition1::compute(ref_norm.vapr, ref_norm.rehu))
    });

    c.bench_function("saturation_vapour_pressure::tetens1", |b| {
        b.iter(|| saturation_vapour_pressure::Tetens1::compute(ref_norm.temp))
    });

    c.bench_function("saturation_vapour_pressure::buck1", |b| {
        b.iter(|| saturation_vapour_pressure::Buck1::compute(ref_norm.temp, ref_norm.pres))
    });

    c.bench_function("saturation_vapour_pressure::buck2", |b| {
        b.iter(|| saturation_vapour_pressure::Buck2::compute(ref_freeze.temp, ref_freeze.pres))
    });

    c.bench_function("saturation_vapour_pressure::buck3", |b| {
        b.iter(|| saturation_vapour_pressure::Buck3::compute(ref_norm.temp, ref_norm.pres))
    });

    c.bench_function("saturation_vapour_pressure::buck4", |b| {
        b.iter(|| saturation_vapour_pressure::Buck4::compute(ref_freeze.temp, ref_freeze.pres))
    });

    c.bench_function("saturation_vapour_pressure::buck3_simplified", |b| {
        b.iter(|| saturation_vapour_pressure::Buck3Simplified::compute(ref_norm.temp))
    });

    c.bench_function("saturation_vapour_pressure::buck4_simplified", |b| {
        b.iter(|| saturation_vapour_pressure::Buck4Simplified::compute(ref_freeze.temp))
    });

    c.bench_function("saturation_vapour_pressure::wexler1", |b| {
        b.iter(|| saturation_vapour_pressure::Wexler1::compute(ref_norm.temp))
    });

    c.bench_function("saturation_vapour_pressure::wexler2", |b| {
        b.iter(|| saturation_vapour_pressure::Wexler2::compute(ref_freeze.temp))
    });
}
