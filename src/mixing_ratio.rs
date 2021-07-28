//!Functions to calculate mixing ratio of fluids

use crate::{constants::EPSILON, error_wrapper::InputError};

fn air1(pressure: f64, vapour_pressure: f64) -> Result<f64, InputError> {
    let result = EPSILON * (vapour_pressure / (pressure - vapour_pressure));
    Ok(result)
}

#[cfg(test)]
mod tests {
    #[test]
    fn mixing_ratio_general1() {}
}
