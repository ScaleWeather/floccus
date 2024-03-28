use criterion::{criterion_group, criterion_main, Criterion};
use floccus::{formulas::vapour_pressure_deficit, Formula2};

mod utils;
use utils::ReferenceValues;

pub fn benchmark(c: &mut Criterion) {
    let ref_norm = ReferenceValues::normal();

    let mut group = c.benchmark_group("vapour_pressure_deficit");

    group.bench_function("definition1", |b| {
        b.iter(|| vapour_pressure_deficit::Definition1::compute(ref_norm.vapr, ref_norm.savp))
    });
    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
