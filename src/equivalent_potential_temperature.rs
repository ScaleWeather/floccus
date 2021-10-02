//!Functions to calculate equivalent potential temperature of air in K.
use crate::Float;
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
    temperature: Float,
    pressure: Float,
    vapour_pressure: Float,
) -> Result<Float, InputError> {
    if !(253.0..=324.0).contains(&temperature) {
        return Err(InputError::OutOfRange(String::from("temperature")));
    }

    if !(20000.0..=150_000.0).contains(&pressure) {
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
        equivalent_potential_temperature,
    };

    #[test]
    fn bryan1() {
        assert!(tests_framework::test_with_3args(
            &equivalent_potential_temperature::bryan1,
            Argument {
                name: "temperature",
                def_val: 300.0,
                range: [253.0, 324.0]
            },
            Argument {
                name: "pressure",
                def_val: 101325.0,
                range: [20000.0, 150_000.0]
            },
            Argument {
                name: "vapour_pressure",
                def_val: 3000.0,
                range: [0.0, 10_000.0]
            },
            353.37350501059836
        ));
    }
}
