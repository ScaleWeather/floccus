//!Functions to calculate virtual temperature of air in K.
//!
//!In atmospheric thermodynamics, the virtual temperature of a moist air parcel is the temperature
//!at which a theoretical dry air parcel would have a total pressure and density equal
//!to the moist parcel of air ([Wikipedia](https://en.wikipedia.org/wiki/Virtual_temperature)).


use crate::Float;
use crate::{constants::EPSILON, errors::InputError};


use itertools::izip;
use ndarray::{Array, Dimension, FoldWhile};
use rayon::iter::{ParallelBridge, ParallelIterator};

///Formula for computing virtual temperature from temperature and mixing ratio.
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `temperature` range: 173K - 373K\
///Valid `mixing_ratio` range: 0.0000000001 - 0.5
pub struct General1;

impl General1 {
    #[allow(missing_docs)]
    #[inline(always)]
    #[allow(clippy::missing_errors_doc)]
    
    pub fn validate_inputs(temperature: Float, mixing_ratio: Float) -> Result<(), InputError> {
        if !(173.0..=354.0).contains(&temperature) {
            return Err(InputError::OutOfRange(String::from("temperature")));
        }

        if !(0.000_000_000_1..=0.5).contains(&mixing_ratio) {
            return Err(InputError::OutOfRange(String::from("mixing_ratio")));
        }

        Ok(())
    }

    #[inline(always)]
    #[allow(missing_docs)]
    pub fn compute_unchecked(temperature: Float, mixing_ratio: Float) -> Float {
        temperature * ((mixing_ratio + EPSILON) / (EPSILON * (1.0 + mixing_ratio)))
    }
}


///Formula for computing virtual temperature from air temperature, pressure and vapour pressure.
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `temperature` range: 173K - 373K\
///Valid `pressure` range: 100Pa - 150000Pa\
///Valid `vapour_pressure` range: 0Pa - 10000Pa
pub struct General2;

impl General2 {
    #[allow(missing_docs)]
    #[inline(always)]
    #[allow(clippy::missing_errors_doc)]
    
    pub fn validate_inputs(
        temperature: Float,
        pressure: Float,
        vapour_pressure: Float,
    ) -> Result<(), InputError> {
        if !(173.0..=354.0).contains(&temperature) {
            return Err(InputError::OutOfRange(String::from("temperature")));
        }

        if !(100.0..=150_000.0).contains(&pressure) {
            return Err(InputError::OutOfRange(String::from("pressure")));
        }

        if !(0.0..=10_000.0).contains(&vapour_pressure) {
            return Err(InputError::OutOfRange(String::from("vapour_pressure")));
        }
        Ok(())
    }

    #[inline(always)]
    #[allow(missing_docs)]
    pub fn compute_unchecked(temperature: Float, pressure: Float, vapour_pressure: Float) -> Float {
        temperature / (1.0 - ((vapour_pressure / pressure) * (1.0 - EPSILON)))
    }
}


///Formula for computing virtual temperature from air temperature and specific humidity.
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `temperature` range: 173K - 373K\
///Valid `specific_humidity` range: 100Pa - 150000Pa
pub struct General3;

impl General3 {
    #[allow(missing_docs)]
    #[inline(always)]
    #[allow(clippy::missing_errors_doc)]
    
    pub fn validate_inputs(temperature: Float, specific_humidity: Float) -> Result<(), InputError> {
        if !(173.0..=354.0).contains(&temperature) {
            return Err(InputError::OutOfRange(String::from("temperature")));
        }

        if !(0.000_000_001..=2.0).contains(&specific_humidity) {
            return Err(InputError::OutOfRange(String::from("specific_humidity")));
        }

        Ok(())
    }

    #[inline(always)]
    #[allow(missing_docs)]
    pub fn compute_unchecked(temperature: Float, specific_humidity: Float) -> Float {
        temperature * (1.0 + (specific_humidity * ((1.0 / EPSILON) - 1.0)))
    }
}


// #[cfg(test)]
// mod tests {
//     use crate::{
//         tests_framework::{self, Argument},
//         virtual_temperature,
//     };

//     #[test]
//     fn general1() {
//         assert!(tests_framework::test_with_2args(
//             &virtual_temperature::general1,
//             Argument {
//                 name: "temperature",
//                 def_val: 300.0,
//                 range: [173.0, 354.0]
//             },
//             Argument {
//                 name: "mixing_ratio",
//                 def_val: 0.022,
//                 range: [0.000_000_000_1, 0.5]
//             },
//             303.9249219815806
//         ));
//     }

//     #[test]
//     fn general2() {
//         assert!(tests_framework::test_with_3args(
//             &virtual_temperature::general2,
//             Argument {
//                 name: "temperature",
//                 def_val: 300.0,
//                 range: [173.0, 354.0]
//             },
//             Argument {
//                 name: "pressure",
//                 def_val: 101325.0,
//                 range: [100.0, 150_000.0]
//             },
//             Argument {
//                 name: "vapour_pressure",
//                 def_val: 3550.0,
//                 range: [0.0, 10_000.0]
//             },
//             304.0265941965307
//         ));
//     }

//     #[test]
//     fn general3() {
//         assert!(tests_framework::test_with_2args(
//             &virtual_temperature::general3,
//             Argument {
//                 name: "temperature",
//                 def_val: 300.0,
//                 range: [173.0, 354.0]
//             },
//             Argument {
//                 name: "specific_humidity",
//                 def_val: 0.022,
//                 range: [0.000000001, 2.0]
//             },
//             304.0112702651753
//         ));
//     }
// }
