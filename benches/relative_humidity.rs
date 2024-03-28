use criterion::{criterion_group, criterion_main, Criterion};
use floccus::{formulas::relative_humidity, formulas::Formula2};

mod utils;
use utils::ReferenceValues;

pub fn benchmark(c: &mut Criterion) {
    let ref_norm = ReferenceValues::normal();

    let mut group = c.benchmark_group("relative_humidity");

    group.bench_function("definition1", |b| {
        b.iter(|| relative_humidity::Definition1::compute(ref_norm.mxrt, ref_norm.smrt))
    });

    group.bench_function("definition2", |b| {
        b.iter(|| relative_humidity::Definition2::compute(ref_norm.vapr, ref_norm.savp))
    });
    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
