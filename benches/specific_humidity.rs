use criterion::{criterion_group, criterion_main, Criterion};
use floccus::{formulas::specific_humidity, Formula2};

mod utils;
use utils::ReferenceValues;

pub fn benchmark(c: &mut Criterion) {
    let ref_norm = ReferenceValues::normal();

    let mut group = c.benchmark_group("specific_humidity");

    group.bench_function("definition1", |b| {
        b.iter(|| specific_humidity::Definition1::compute(ref_norm.vapr, ref_norm.pres))
    });

    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
