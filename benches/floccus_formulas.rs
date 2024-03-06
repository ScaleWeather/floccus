use criterion::{criterion_group, criterion_main};

mod equivalent_potential_temperature;
mod mixing_ratio;
mod potential_temperature;
mod relative_humidity;
mod saturation_mixing_ratio;
mod saturation_vapour_pressure;
mod specific_humidity;
mod vapour_pressure;
mod vapour_pressure_deficit;
mod virtual_temperature;
mod wet_bulb_potential_temperature;
mod wet_bulb_temperature;

criterion_group!(
    benches,
    vapour_pressure::benchmark,
    saturation_vapour_pressure::benchmark,
    relative_humidity::benchmark,
    potential_temperature::benchmark,
    equivalent_potential_temperature::benchmark,
    wet_bulb_potential_temperature::benchmark,
    wet_bulb_temperature::benchmark,
    specific_humidity::benchmark,
    mixing_ratio::benchmark,
    saturation_mixing_ratio::benchmark,
    virtual_temperature::benchmark,
    vapour_pressure_deficit::benchmark
);
criterion_main!(benches);
