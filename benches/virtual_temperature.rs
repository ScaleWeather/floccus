use criterion::Criterion;
use floccus::{formulas::virtual_temperature, Formula2, Formula3};

// this is the best way to avoid code duplication I could find
include!("./reference_values.rs");

pub fn benchmark(c: &mut Criterion) {
    let ref_norm = ReferenceValues::normal();

    let mut group = c.benchmark_group("virtual_temperature");

    group.bench_function("virtual_temperature::definition1", |b| {
        b.iter(|| virtual_temperature::Definition1::compute(ref_norm.temp, ref_norm.mxrt))
    });

    group.bench_function("virtual_temperature::definition2", |b| {
        b.iter(|| {
            virtual_temperature::Definition2::compute(ref_norm.temp, ref_norm.pres, ref_norm.vapr)
        })
    });

    group.bench_function("virtual_temperature::definition3", |b| {
        b.iter(|| virtual_temperature::Definition3::compute(ref_norm.temp, ref_norm.sphu))
    });
    group.finish();
}
