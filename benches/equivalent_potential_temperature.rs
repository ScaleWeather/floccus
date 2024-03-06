#![allow(unused)]
use criterion::Criterion;
use floccus::{formulas::equivalent_potential_temperature, Formula4};

// this is the best way to avoid code duplication I could find
include!("./reference_values.rs");

pub fn benchmark(c: &mut Criterion) {
    let ref_norm = ReferenceValues::normal();

    let mut group = c.benchmark_group("equivalent_potential_temperature");

    group.bench_function("Bolton1", |b| {
        b.iter(|| {
            equivalent_potential_temperature::Bolton1::compute(
                ref_norm.pres,
                ref_norm.temp,
                ref_norm.dwpt,
                ref_norm.vapr,
            )
        })
    });

    group.bench_function("Bolton2", |b| {
        b.iter(|| {
            equivalent_potential_temperature::Bolton2::compute(
                ref_norm.temp,
                ref_norm.dwpt,
                ref_norm.mxrt,
                ref_norm.thet,
            )
        })
    });

    group.bench_function("Bryan1", |b| {
        b.iter(|| {
            equivalent_potential_temperature::Bryan1::compute(
                ref_norm.temp,
                ref_norm.mxrt,
                ref_norm.rehu,
                ref_norm.thet,
            )
        })
    });

    group.bench_function("Paluch1", |b| {
        b.iter(|| {
            equivalent_potential_temperature::Paluch1::compute(
                ref_norm.temp,
                ref_norm.pres,
                ref_norm.mxrt,
                ref_norm.rehu,
            )
        })
    });
    group.finish();
}
