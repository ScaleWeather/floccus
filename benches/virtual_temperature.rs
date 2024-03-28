use criterion::{criterion_group, criterion_main, Criterion};
use floccus::{formulas::virtual_temperature, Formula2, Formula3};

mod utils;
use utils::ReferenceValues;

pub fn benchmark(c: &mut Criterion) {
    let ref_norm = ReferenceValues::normal();

    let mut group = c.benchmark_group("virtual_temperature");

    group.bench_function("definition1", |b| {
        b.iter(|| virtual_temperature::Definition1::compute(ref_norm.temp, ref_norm.mxrt))
    });

    group.bench_function("definition2", |b| {
        b.iter(|| {
            virtual_temperature::Definition2::compute(ref_norm.temp, ref_norm.pres, ref_norm.vapr)
        })
    });

    group.bench_function("definition3", |b| {
        b.iter(|| virtual_temperature::Definition3::compute(ref_norm.temp, ref_norm.sphu))
    });
    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
