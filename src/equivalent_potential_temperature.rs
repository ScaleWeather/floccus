//!Functions to calculate equivalent potential temperature of air in K.

use crate::{
    constants::{C_P, EPSILON, L_V, R_D},
    error_wrapper::InputError,
    mixing_ratio, potential_temperature, relative_humidity, vapour_pressure,
};

///Formula for computing equivalent potential temperature of dry air from
///temperature, pressure and vapour pressure.
///
///Derived by G. H. Bryan (2008) [(doi:10.1175/2008MWR2593.1)](https://doi.org/10.1175/2008MWR2593.1)
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `temperature` range: 253K - 324K\
///Valid `pressure` range: 100Pa - 150000Pa\
///Valid `vapour_pressure` range: 0Pa - 10000Pa
pub fn bryan1(
    temperature: f64,
    pressure: f64,
    vapour_pressure: f64,
) -> Result<f64, InputError> {
    if !(253.0..=324.0).contains(&temperature) {
        return Err(InputError::OutOfRange(String::from("temperature")));
    }

    if !(100.0..=150_000.0).contains(&pressure) {
        return Err(InputError::OutOfRange(String::from("pressure")));
    }

    if !(0.0..=10_000.0).contains(&vapour_pressure) {
        return Err(InputError::OutOfRange(String::from("vapour_pressure")));
    }

    let kappa = R_D / C_P;

    let potential_temperature =
        potential_temperature::davies_jones1(temperature, pressure, vapour_pressure)?;

    let saturation_vapour_pressure = vapour_pressure::buck3(temperature, pressure)?;
    let relative_humidity =
        relative_humidity::general2(vapour_pressure, saturation_vapour_pressure)?;

    let mixing_ratio = mixing_ratio::general1(pressure, vapour_pressure)?;

    let result = potential_temperature
        * relative_humidity.powf((-kappa) * (mixing_ratio / EPSILON))
        * ((L_V * mixing_ratio) / (C_P * temperature)).exp();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::{
        tests_framework::{self, Argument},
        wet_bulb_temperature,
    };

    #[test]
    fn stull1() {
        assert!(tests_framework::test_with_2args(
            &wet_bulb_temperature::stull1,
            Argument {
                name: "temperature",
                def_val: 300.0,
                range: [253.0, 324.0]
            },
            Argument {
                name: "relative_humidity",
                def_val: 0.5,
                range: [0.05, 0.99]
            },
            292.73867410526674
        ));
    }
}
