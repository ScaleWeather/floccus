//!Functions to calculate mixing ratio of fluids

use crate::{constants::EPSILON, error_wrapper::InputError, vapour_pressure};

fn air_general1(pressure: f64, vapour_pressure: f64) -> Result<f64, InputError> {
    let result = EPSILON * (vapour_pressure / (pressure - vapour_pressure));
    Ok(result)
}

fn air_performance1(temperature: f64, pressure: f64) -> Result<f64, InputError> {
    let vapour_pressure = vapour_pressure::tetens1(temperature)?;
    let result = air_general1(pressure, vapour_pressure)?;
    Ok(result)
}

fn air_accuracy1(temperature: f64, pressure: f64) -> Result<f64, InputError> {
    let vapour_pressure = vapour_pressure::buck1(temperature, pressure)?;
    let result = air_general1(pressure, vapour_pressure)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    #[test]
    fn mixing_ratio_general1() {}
}
