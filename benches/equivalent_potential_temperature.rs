use criterion::{criterion_group, criterion_main, Criterion};
use floccus::{formulas::equivalent_potential_temperature, formulas::Formula4};

mod utils;
use utils::ReferenceValues;

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
            equivalent_potential_temperature::Kerry1::compute(
                ref_norm.temp,
                ref_norm.pres,
                ref_norm.mxrt,
                ref_norm.rehu,
            )
        })
    });
    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
