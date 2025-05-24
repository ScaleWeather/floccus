use criterion::{criterion_group, criterion_main, Criterion};
use floccus::{formulas::potential_temperature, formulas::Formula3};

mod utils;
use utils::ReferenceValues;

pub fn benchmark(c: &mut Criterion) {
    let ref_norm = ReferenceValues::normal();

    let mut group = c.benchmark_group("potential_temperature");

    group.bench_function("definition1", |b| {
        b.iter(|| {
            potential_temperature::Definition1::compute(ref_norm.temp, ref_norm.pres, ref_norm.vapr)
        })
    });
    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
