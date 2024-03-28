use criterion::{criterion_group, criterion_main, Criterion};
use floccus::{formulas::mixing_ratio, formulas::Formula2};

mod utils;
use utils::ReferenceValues;

pub fn benchmark(c: &mut Criterion) {
    let ref_norm = ReferenceValues::normal();

    let mut group = c.benchmark_group("mixing_ratio");

    group.bench_function("definition1", |b| {
        b.iter(|| mixing_ratio::Definition1::compute(ref_norm.pres, ref_norm.vapr))
    });
    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
