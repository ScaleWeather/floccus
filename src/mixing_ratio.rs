//!Functions to calculate mixing ratio of air in kg*kg^-1.
//!
//!To calculate saturation mixing ratio input dry-bulb temperature in place of dewpoint
//!or saturation vapour pressure in place of vapour pressure.

use crate::{constants::EPSILON, errors::InputError, vapour_pressure};
use float_cmp::approx_eq;
use crate::Float;

#[cfg(feature="debug")]
use floccus_proc::logerr;

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
#[cfg_attr(feature = "debug", logerr)]
pub fn general1(pressure: Float, vapour_pressure: Float) -> Result<Float, InputError> {
    //validate inputs
    if !(100.0..=150_000.0).contains(&pressure) {
        return Err(InputError::OutOfRange(String::from("pressure")));
    }

    if !(0.0..=50_000.0).contains(&vapour_pressure) {
        return Err(InputError::OutOfRange(String::from("vapour_pressure")));
    }

    if approx_eq!(Float, pressure, vapour_pressure, ulps = 2) {
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
#[cfg_attr(feature = "debug", logerr)]
pub fn performance1(dewpoint: Float, pressure: Float) -> Result<Float, InputError> {
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
#[cfg_attr(feature = "debug", logerr)]
pub fn accuracy1(dewpoint: Float, pressure: Float) -> Result<Float, InputError> {
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
    use crate::{
        mixing_ratio,
        tests_framework::{self, Argument},
    };

    #[test]
    fn general1() {
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
                range: [0.0, 50_000.0]
            },
            0.022253316630823517
        ));
    }

    #[test]
    fn performance1() {
        assert!(tests_framework::test_with_2args(
            &mixing_ratio::performance1,
            Argument {
                name: "dewpoint",
                def_val: 300.0,
                range: [273.0, 353.0]
            },
            Argument {
                name: "pressure",
                def_val: 101325.0,
                range: [100.0, 150_000.0]
            },
            0.022477100514593465
        ));
    }

    #[test]
    fn accuracy1() {
        assert!(tests_framework::test_with_2args(
            &mixing_ratio::accuracy1,
            Argument {
                name: "dewpoint",
                def_val: 300.0,
                range: [232.0, 324.0]
            },
            Argument {
                name: "pressure",
                def_val: 101325.0,
                range: [100.0, 150_000.0]
            },
            0.022587116896465847
        ));
    }
}
