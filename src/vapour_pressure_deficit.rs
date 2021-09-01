//!Functions to calculate vapour pressure deficit in Pa.
//!
//!Vapour-pressure deficit, is the difference (deficit) between
//!the amount of moisture in the air and how much moisture the air can hold
//!when it is saturated ([Wikipedia](https://en.wikipedia.org/wiki/Vapour-pressure_deficit)).

use crate::{error_wrapper::InputError, vapour_pressure};

///Formula for computing vapour pressure deficit from vapour pressure and saturation vapour pressure
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `vapour_pressure` range: 0Pa - 10000Pa
///Valid `saturation_vapour_pressure` range: 0Pa - 10000Pa
pub fn general1(vapour_pressure: f64, saturation_vapour_pressure: f64) -> Result<f64, InputError> {
    if !(0.0..=10_000.0).contains(&vapour_pressure) {
        return Err(InputError::OutOfRange(String::from("vapour_pressure")));
    }

    if !(0.0..=10_000.0).contains(&saturation_vapour_pressure) {
        return Err(InputError::OutOfRange(String::from(
            "saturation_vapour_pressure",
        )));
    }

    Ok(saturation_vapour_pressure - vapour_pressure)
}

///Formula for computing vapour pressure deficit from temperature, dewpoint and pressure
///using [`buck3`](vapour_pressure::buck3) function for vapour pressure calculation
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `vapour_pressure` range: 0Pa - 10000Pa
///Valid `saturation_vapour_pressure` range: 0Pa - 10000Pa
pub fn general2(temperature: f64, dewpoint: f64, pressure: f64) -> Result<f64, InputError> {
    if !(0.0..=10_000.0).contains(&temperature) {
        return Err(InputError::OutOfRange(String::from("temperature")));
    }

    if !(0.0..=10_000.0).contains(&dewpoint) {
        return Err(InputError::OutOfRange(String::from("dewpoint")));
    }

    if !(0.0..=10_000.0).contains(&pressure) {
        return Err(InputError::OutOfRange(String::from("pressure")));
    }

    let vapour_pressure = vapour_pressure::buck3(dewpoint, pressure)?;
    let saturation_vapour_pressure = vapour_pressure::buck3(temperature, pressure)?;

    let result = general1(vapour_pressure, saturation_vapour_pressure)?;

    Ok(result)
}

///Formula for computing vapour pressure deficit from temperature, relative humidity and pressure
///using [`buck3`](vapour_pressure::buck3) function for vapour pressure calculation
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `vapour_pressure` range: 0Pa - 10000Pa
///Valid `saturation_vapour_pressure` range: 0Pa - 10000Pa
pub fn general3(temperature: f64, relative_humidity: f64, pressure: f64) -> Result<f64, InputError> {
    if !(0.0..=10_000.0).contains(&temperature) {
        return Err(InputError::OutOfRange(String::from("temperature")));
    }

    if !(0.0..=10_000.0).contains(&relative_humidity) {
        return Err(InputError::OutOfRange(String::from("dewpoint")));
    }

    if !(0.0..=10_000.0).contains(&pressure) {
        return Err(InputError::OutOfRange(String::from("pressure")));
    }

    let saturation_vapour_pressure = vapour_pressure::buck3(temperature, pressure)?;
    let vapour_pressure = vapour_pressure::saturation_specific1(saturation_vapour_pressure, relative_humidity)?;

    let result = general1(vapour_pressure, saturation_vapour_pressure)?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::{error_wrapper::InputError, vapour_pressure_deficit};
    use float_cmp::assert_approx_eq;

    #[test]
    fn general1() {
        let result = vapour_pressure_deficit::general1(3250.0, 3550.0).unwrap();
        let expected = 300.0;
        assert_approx_eq!(f64, expected, result, ulps = 2);

        for vapour_pressure in 0..=1000 {
            for saturation_vapour_pressure in 0..=1000 {
                let result = vapour_pressure_deficit::general1(
                    vapour_pressure as f64 * 10.0,
                    saturation_vapour_pressure as f64 * 10.0,
                )
                .unwrap();
                assert!(result.is_finite());
            }
        }

        for &vapour_pressure in [-0.1, 10_000.1].iter() {
            let result = vapour_pressure_deficit::general1(vapour_pressure, 3550.0).unwrap_err();
            let expected = InputError::OutOfRange(String::from("vapour_pressure"));
            assert_eq!(result, expected);
        }

        for &saturation_vapour_pressure in [-0.1, 10_000.1].iter() {
            let result =
                vapour_pressure_deficit::general1(3250.0, saturation_vapour_pressure).unwrap_err();
            let expected = InputError::OutOfRange(String::from("saturation_vapour_pressure"));
            assert_eq!(result, expected);
        }
    }
}
