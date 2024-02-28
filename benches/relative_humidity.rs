// use criterion::{black_box, criterion_group, criterion_main, Criterion};
// use floccus::relative_humidity;

// pub fn relative_humidity_benchmark(c: &mut Criterion) {
//     c.bench_function("relative_humidity::general1", |b| {
//         b.iter(|| relative_humidity::general1(black_box(0.01064), black_box(0.01467)))
//     });

//     c.bench_function("relative_humidity::general2", |b| {
//         b.iter(|| relative_humidity::general2(black_box(1706.0), black_box(2339.0)))
//     });

//     c.bench_function("relative_humidity::general3", |b| {
//         b.iter(|| relative_humidity::general3(black_box(300.0), black_box(290.0)))
//     });

//     c.bench_function("relative_humidity::general4", |b| {
//         b.iter(|| relative_humidity::general4(black_box(300.0), black_box(290.0), black_box(101325.0)))
//     });

//     c.bench_function("relative_humidity::general5", |b| {
//         b.iter(|| relative_humidity::general5(black_box(300.0), black_box(290.0), black_box(101325.0)))
//     });
// }

// criterion_group!(benches, relative_humidity_benchmark);
// criterion_main!(benches);
