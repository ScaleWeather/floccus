//!Functions to calculate mixing ratio of fluids

use crate::{constants::EPSILON, error_wrapper::InputError, vapour_pressure};

///Formula for computing mixing ratio of unsaturated air from air pressure and vapour pressure
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid pressure range: 100Pa - 150000Pa\
///Valid vapour pressure range: 0Pa - 10000Pa
pub fn air_general1(pressure: f64, vapour_pressure: f64) -> Result<f64, InputError> {
    //validate inputs
    if !(100.0..=150_000.0).contains(&pressure) {
        return Err(InputError::OutOfRange(String::from("pressure")));
    }

    if !(0.0..=10_000.0).contains(&vapour_pressure) {
        return Err(InputError::OutOfRange(String::from("vapour pressure")));
    }

    let result = EPSILON * (vapour_pressure / (pressure - vapour_pressure));
    Ok(result)
}

///Formula for computing mixing ratio of unsaturated air from air temperature and pressure.
///Optimised by performance
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid temperature range: 273K - 353K\
///Valid pressure range: 100Pa - 150000Pa
pub fn air_performance1(temperature: f64, pressure: f64) -> Result<f64, InputError> {
    //validate inputs
    if !(273.0..=353.0).contains(&temperature) {
        return Err(InputError::OutOfRange(String::from("temperature")));
    }

    if !(100.0..=150_000.0).contains(&pressure) {
        return Err(InputError::OutOfRange(String::from("pressure")));
    }

    let vapour_pressure = vapour_pressure::tetens1(temperature)?;
    let result = air_general1(pressure, vapour_pressure)?;
    Ok(result)
}

///Formula for computing mixing ratio of unsaturated air from air temperature and pressure.
///Optimised by accuracy
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid temperature range: 232K - 324K\
///Valid pressure range: 100Pa - 150000Pa
pub fn air_accuracy1(temperature: f64, pressure: f64) -> Result<f64, InputError> {
    //validate inputs
    if !(232.0..=324.0).contains(&temperature) {
        return Err(InputError::OutOfRange(String::from("temperature")));
    }

    if !(100.0..=150_000.0).contains(&pressure) {
        return Err(InputError::OutOfRange(String::from("pressure")));
    }

    let vapour_pressure = vapour_pressure::buck1(temperature, pressure)?;
    let result = air_general1(pressure, vapour_pressure)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;

    use crate::{error_wrapper::InputError, mixing_ratio};

    #[test]
    fn test_air_general1() {
        let result = mixing_ratio::air_general1(101325.0, 3500.0).unwrap();
        let expected = 0.022253316630823517;
        assert_approx_eq!(f64, expected, result, ulps = 2);

        for &pressure in [99.9f64, 150000.1f64].iter() {
            let result = mixing_ratio::air_general1(pressure, 3500.0).unwrap_err();
            let expected = InputError::OutOfRange(String::from("pressure"));
            assert_eq!(result, expected);
        }

        for &vapour_pressure in [-0.1f64, 10000.1f64].iter() {
            let result = mixing_ratio::air_general1(101325.0, vapour_pressure).unwrap_err();
            let expected = InputError::OutOfRange(String::from("vapour pressure"));
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_air_performance1() {
        let result = mixing_ratio::air_performance1(300.0, 101325.0).unwrap();
        let expected = 0.022477100514593465;
        assert_approx_eq!(f64, expected, result, ulps = 2);

        for &temperature in [272.9f64, 353.1f64].iter() {
            let result = mixing_ratio::air_performance1(temperature, 101325.0).unwrap_err();
            let expected = InputError::OutOfRange(String::from("temperature"));
            assert_eq!(result, expected);
        }

        for &pressure in [99.9f64, 150000.1f64].iter() {
            let result = mixing_ratio::air_performance1(300.0, pressure).unwrap_err();
            let expected = InputError::OutOfRange(String::from("pressure"));
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_air_accuracy1() {
        let result = mixing_ratio::air_accuracy1(300.0, 101325.0).unwrap();
        let expected = 0.022587116896465847;
        assert_approx_eq!(f64, expected, result, ulps = 2);

        for &temperature in [231.9f64, 324.1f64].iter() {
            let result = mixing_ratio::air_accuracy1(temperature, 101325.0).unwrap_err();
            let expected = InputError::OutOfRange(String::from("temperature"));
            assert_eq!(result, expected);
        }

        for &pressure in [99.9f64, 150000.1f64].iter() {
            let result = mixing_ratio::air_accuracy1(300.0, pressure).unwrap_err();
            let expected = InputError::OutOfRange(String::from("pressure"));
            assert_eq!(result, expected);
        }
    }
}
