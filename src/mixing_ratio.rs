//!Functions to calculate mixing ratio of air in kg*kg^-1.
//!
//!To calculate saturation mixing ratio input dry-bulb temperature in place of dewpoint
//!or saturation vapour pressure in place of vapour pressure.

use crate::{constants::EPSILON, error_wrapper::InputError, vapour_pressure};
use float_cmp::approx_eq;

///Formula for computing mixing ratio of unsaturated air from air pressure and vapour pressure
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `pressure` range: 100Pa - 150000Pa\
///Valid `vapour_pressure` range: 0Pa - 10000Pa
///
///Returns [`InputError::IncorrectArgumentSet`] when inputs are equal, in which
///case division by 0 occurs.
pub fn general1(pressure: f64, vapour_pressure: f64) -> Result<f64, InputError> {
    //validate inputs
    if !(100.0..=150_000.0).contains(&pressure) {
        return Err(InputError::OutOfRange(String::from("pressure")));
    }

    if !(0.0..=10_000.0).contains(&vapour_pressure) {
        return Err(InputError::OutOfRange(String::from("vapour_pressure")));
    }

    if approx_eq!(f64, pressure, vapour_pressure, ulps = 2) {
        return Err(InputError::IncorrectArgumentSet(String::from(
            "pressure and vapour_pressure cannot be equal",
        )));
    }

    let result = EPSILON * (vapour_pressure / (pressure - vapour_pressure));
    Ok(result)
}

///Formula for computing mixing ratio of unsaturated air from dewpoint temperature and pressure.
///Optimised by performance.
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `dewpoint` range: 273K - 353K\
///Valid `pressure` range: 100Pa - 150000Pa
pub fn performance1(dewpoint: f64, pressure: f64) -> Result<f64, InputError> {
    //validate inputs
    if !(273.0..=353.0).contains(&dewpoint) {
        return Err(InputError::OutOfRange(String::from("dewpoint")));
    }

    if !(100.0..=150_000.0).contains(&pressure) {
        return Err(InputError::OutOfRange(String::from("pressure")));
    }

    let vapour_pressure = vapour_pressure::tetens1(dewpoint)?;
    let result = general1(pressure, vapour_pressure)?;
    Ok(result)
}

///Formula for computing mixing ratio of unsaturated air from dewpoint temperature and pressure.
///Optimised by accuracy.
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `dewpoint` range: 232K - 324K\
///Valid `pressure` range: 100Pa - 150000Pa
pub fn accuracy1(dewpoint: f64, pressure: f64) -> Result<f64, InputError> {
    //validate inputs
    if !(232.0..=324.0).contains(&dewpoint) {
        return Err(InputError::OutOfRange(String::from("dewpoint")));
    }

    if !(100.0..=150_000.0).contains(&pressure) {
        return Err(InputError::OutOfRange(String::from("pressure")));
    }

    let vapour_pressure = vapour_pressure::buck1(dewpoint, pressure)?;
    let result = general1(pressure, vapour_pressure)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;

    use crate::{
        error_wrapper::InputError,
        mixing_ratio,
        tests_framework::{self, Argument},
    };

    #[test]
    fn general1() {
        let result = mixing_ratio::general1(101325.0, 3500.0).unwrap();
        let expected = 0.022253316630823517;
        assert_approx_eq!(f64, expected, result, ulps = 2);

        for &pressure in [99.9, 150000.1].iter() {
            let result = mixing_ratio::general1(pressure, 3500.0).unwrap_err();
            let expected = InputError::OutOfRange(String::from("pressure"));
            assert_eq!(result, expected);
        }

        for &vapour_pressure in [-0.1, 10000.1].iter() {
            let result = mixing_ratio::general1(101325.0, vapour_pressure).unwrap_err();
            let expected = InputError::OutOfRange(String::from("vapour_pressure"));
            assert_eq!(result, expected);
        }

        assert!(tests_framework::test_with_2args(
            &mixing_ratio::general1,
            Argument {
                name: "pressure",
                def_val: 101325.0,
                range: [100.0, 150_000.0]
            },
            Argument {
                name: "vapour_pressure",
                def_val: 3500.0,
                range: [0.0, 10_000.0]
            },
            0.022253316630823517
        ));
    }

    #[test]
    fn performance1() {
        let result = mixing_ratio::performance1(300.0, 101325.0).unwrap();
        let expected = 0.022477100514593465;
        assert_approx_eq!(f64, expected, result, ulps = 2);

        for &dewpoint in [272.9, 353.1].iter() {
            let result = mixing_ratio::performance1(dewpoint, 101325.0).unwrap_err();
            let expected = InputError::OutOfRange(String::from("dewpoint"));
            assert_eq!(result, expected);
        }

        for &pressure in [99.9, 150000.1].iter() {
            let result = mixing_ratio::performance1(300.0, pressure).unwrap_err();
            let expected = InputError::OutOfRange(String::from("pressure"));
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn accuracy1() {
        let result = mixing_ratio::accuracy1(300.0, 101325.0).unwrap();
        let expected = 0.022587116896465847;
        assert_approx_eq!(f64, expected, result, ulps = 2);

        for &dewpoint in [231.9, 324.1].iter() {
            let result = mixing_ratio::accuracy1(dewpoint, 101325.0).unwrap_err();
            let expected = InputError::OutOfRange(String::from("dewpoint"));
            assert_eq!(result, expected);
        }

        for &pressure in [99.9, 150000.1].iter() {
            let result = mixing_ratio::accuracy1(300.0, pressure).unwrap_err();
            let expected = InputError::OutOfRange(String::from("pressure"));
            assert_eq!(result, expected);
        }
    }
}
