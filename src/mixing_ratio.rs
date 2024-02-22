//!Functions to calculate mixing ratio of air in kg*kg^-1.
//!
//!To calculate saturation mixing ratio input dry-bulb temperature in place of dewpoint
//!or saturation vapour pressure in place of vapour pressure.


use crate::{constants::EPSILON, errors::InputError};
use crate::{vapour_pressure, Float};
use float_cmp::approx_eq;


use itertools::izip;
use ndarray::{Array, Dimension, FoldWhile};
use rayon::iter::{ParallelBridge, ParallelIterator};

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
pub struct General1;

impl General1 {
    #[allow(missing_docs)]
    #[inline(always)]
    #[allow(clippy::missing_errors_doc)]
    
    pub fn validate_inputs(pressure: Float, vapour_pressure: Float) -> Result<(), InputError> {
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
        Ok(())
    }

    #[inline(always)]
    #[allow(missing_docs)]
    pub fn compute_unchecked(pressure: Float, vapour_pressure: Float) -> Float {
        EPSILON * (vapour_pressure / (pressure - vapour_pressure))
    }
}


///Formula for computing mixing ratio of unsaturated air from dewpoint temperature and pressure.
///Optimised for performance.
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `dewpoint` range: 273K - 353K\
///Valid `pressure` range: 100Pa - 150000Pa
pub struct Performance1;

impl Performance1 {
    #[inline(always)]
    #[allow(missing_docs)]
    #[allow(clippy::missing_errors_doc)]
    
    pub fn validate_inputs(dewpoint: Float, pressure: Float) -> Result<(), InputError> {
        //validate inputs
        if !(273.0..=353.0).contains(&dewpoint) {
            return Err(InputError::OutOfRange(String::from("dewpoint")));
        }

        if !(100.0..=150_000.0).contains(&pressure) {
            return Err(InputError::OutOfRange(String::from("pressure")));
        }

        Ok(())
    }

    #[inline(always)]
    #[allow(missing_docs)]
    pub fn compute_unchecked(dewpoint: Float, pressure: Float) -> Float {
        let vapour_pressure = vapour_pressure::Tetens1::compute_unchecked(dewpoint);

        General1::compute_unchecked(pressure, vapour_pressure)
    }
}


///Formula for computing mixing ratio of unsaturated air from dewpoint temperature and pressure.
///Optimised for accuracy.
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `dewpoint` range: 232K - 324K\
///Valid `pressure` range: 100Pa - 150000Pa
pub struct Accuracy1;

impl Accuracy1 {
    #[inline(always)]
    #[allow(missing_docs)]
    #[allow(clippy::missing_errors_doc)]
    
    pub fn validate_inputs(dewpoint: Float, pressure: Float) -> Result<(), InputError> {
        if !(232.0..=324.0).contains(&dewpoint) {
            return Err(InputError::OutOfRange(String::from("dewpoint")));
        }

        if !(100.0..=150_000.0).contains(&pressure) {
            return Err(InputError::OutOfRange(String::from("pressure")));
        }
        Ok(())
    }

    #[inline(always)]
    #[allow(missing_docs)]
    pub fn compute_unchecked(dewpoint: Float, pressure: Float) -> Float {
        let vapour_pressure = vapour_pressure::Buck1::compute_unchecked(dewpoint, pressure);

        General1::compute_unchecked(pressure, vapour_pressure)
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::{
//         mixing_ratio,
//         tests_framework::{self, Argument},
//     };

//     #[test]
//     fn general1() {
//         assert!(tests_framework::test_with_2args(
//             &mixing_ratio::general1,
//             Argument {
//                 name: "pressure",
//                 def_val: 101325.0,
//                 range: [100.0, 150_000.0]
//             },
//             Argument {
//                 name: "vapour_pressure",
//                 def_val: 3500.0,
//                 range: [0.0, 50_000.0]
//             },
//             0.022253316630823517
//         ));
//     }

//     #[test]
//     fn performance1() {
//         assert!(tests_framework::test_with_2args(
//             &mixing_ratio::performance1,
//             Argument {
//                 name: "dewpoint",
//                 def_val: 300.0,
//                 range: [273.0, 353.0]
//             },
//             Argument {
//                 name: "pressure",
//                 def_val: 101325.0,
//                 range: [100.0, 150_000.0]
//             },
//             0.022477100514593465
//         ));
//     }

//     #[test]
//     fn accuracy1() {
//         assert!(tests_framework::test_with_2args(
//             &mixing_ratio::accuracy1,
//             Argument {
//                 name: "dewpoint",
//                 def_val: 300.0,
//                 range: [232.0, 324.0]
//             },
//             Argument {
//                 name: "pressure",
//                 def_val: 101325.0,
//                 range: [100.0, 150_000.0]
//             },
//             0.022587116896465847
//         ));
//     }
// }
