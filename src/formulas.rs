pub mod equivalent_potential_temperature;
pub mod mixing_ratio;
pub mod potential_temperature;
pub mod relative_humidity;
pub mod saturation_mixing_ratio;
pub mod saturation_vapour_pressure;
pub mod specific_humidity;
pub mod vapour_pressure;
pub mod vapour_pressure_deficit;
pub mod virtual_temperature;
pub mod wet_bulb_potential_temperature;
pub mod wet_bulb_temperature;

mod traits;

pub use traits::{Formula1, Formula2, Formula3, Formula4};
