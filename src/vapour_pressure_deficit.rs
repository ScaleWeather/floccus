//!Functions to calculate vapour pressure deficit in Pa.
//!
//!Vapour-pressure deficit, is the difference (deficit) between
//!the amount of moisture in the air and how much moisture the air can hold
//!when it is saturated ([Wikipedia](https://en.wikipedia.org/wiki/Vapour-pressure_deficit)).

use crate::compute_macros::{
    generate_compute, generate_ndarray_compute, generate_par_ndarray_compute,
    generate_par_vec_compute, generate_vec_compute,
};
use crate::errors::InputError;
use crate::{vapour_pressure, Float};
#[cfg(feature = "debug")]
use floccus_proc::logerr;
use itertools::izip;
use ndarray::{Array, Dimension, FoldWhile};
use rayon::iter::{ParallelBridge, ParallelIterator};

///Formula for computing vapour pressure deficit from vapour pressure and saturation vapour pressure
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `vapour_pressure` range: 0Pa - 10000Pa
///Valid `saturation_vapour_pressure` range: 0Pa - 10000Pa
pub struct General1;

impl General1 {
    #[allow(missing_docs)]
    #[allow(clippy::missing_errors_doc)]
    #[inline(always)]
    #[cfg_attr(feature = "debug", logerr)]
    pub fn validate_inputs(
        vapour_pressure: Float,
        saturation_vapour_pressure: Float,
    ) -> Result<(), InputError> {
        if !(0.0..=50_000.0).contains(&vapour_pressure) {
            return Err(InputError::OutOfRange(String::from("vapour_pressure")));
        }

        if !(0.0..=50_000.0).contains(&saturation_vapour_pressure) {
            return Err(InputError::OutOfRange(String::from(
                "saturation_vapour_pressure",
            )));
        }

        Ok(())
    }

    #[inline(always)]
    #[allow(missing_docs)]
    pub fn compute_unchecked(vapour_pressure: Float, saturation_vapour_pressure: Float) -> Float {
        saturation_vapour_pressure - vapour_pressure
    }
}

generate_compute!(General1, vapour_pressure, saturation_vapour_pressure);
generate_vec_compute!(General1, vapour_pressure, saturation_vapour_pressure);
generate_ndarray_compute!(General1, vapour_pressure, saturation_vapour_pressure);
generate_par_vec_compute!(General1, vapour_pressure, saturation_vapour_pressure);
generate_par_ndarray_compute!(General1, vapour_pressure, saturation_vapour_pressure);

///Formula for computing vapour pressure deficit from temperature, dewpoint and pressure
///using [`buck3`](vapour_pressure::buck3) function for vapour pressure calculation
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `vapour_pressure` range: 0Pa - 10000Pa
///Valid `saturation_vapour_pressure` range: 0Pa - 10000Pa
pub struct General2;

impl General2 {
    #[allow(missing_docs)]
    #[allow(clippy::missing_errors_doc)]
    #[inline(always)]
    #[cfg_attr(feature = "debug", logerr)]
    pub fn validate_inputs(
        temperature: Float,
        dewpoint: Float,
        pressure: Float,
    ) -> Result<(), InputError> {
        if !(253.0..=324.0).contains(&temperature) {
            return Err(InputError::OutOfRange(String::from("temperature")));
        }

        if !(253.0..=324.0).contains(&dewpoint) {
            return Err(InputError::OutOfRange(String::from("dewpoint")));
        }

        if !(100.0..=150_000.0).contains(&pressure) {
            return Err(InputError::OutOfRange(String::from("pressure")));
        }
        Ok(())
    }

    #[inline(always)]
    #[allow(missing_docs)]
    pub fn compute_unchecked(temperature: Float, dewpoint: Float, pressure: Float) -> Float {
        let vapour_pressure = vapour_pressure::Buck3::compute_unchecked(dewpoint, pressure);
        let saturation_vapour_pressure =
            vapour_pressure::Buck3::compute_unchecked(temperature, pressure);

        General1::compute_unchecked(vapour_pressure, saturation_vapour_pressure)
    }
}

generate_compute!(General2, temperature, dewpoint, pressure);
generate_vec_compute!(General2, temperature, dewpoint, pressure);
generate_ndarray_compute!(General2, temperature, dewpoint, pressure);
generate_par_vec_compute!(General2, temperature, dewpoint, pressure);
generate_par_ndarray_compute!(General2, temperature, dewpoint, pressure);

///Formula for computing vapour pressure deficit from temperature, relative humidity and pressure
///using [`buck3`](vapour_pressure::buck3) function for vapour pressure calculation
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `vapour_pressure` range: 0Pa - 10000Pa
///Valid `saturation_vapour_pressure` range: 0Pa - 10000Pa
pub struct General3;

impl General3 {
    #[allow(missing_docs)]
    #[allow(clippy::missing_errors_doc)]
    #[inline(always)]
    #[cfg_attr(feature = "debug", logerr)]
    pub fn validate_inputs(
        temperature: Float,
        relative_humidity: Float,
        pressure: Float,
    ) -> Result<(), InputError> {
        if !(253.0..=319.0).contains(&temperature) {
            return Err(InputError::OutOfRange(String::from("temperature")));
        }

        if !(0.05..=1.0).contains(&relative_humidity) {
            return Err(InputError::OutOfRange(String::from("relative_humidity")));
        }

        if !(10000.0..=150_000.0).contains(&pressure) {
            return Err(InputError::OutOfRange(String::from("pressure")));
        }

        Ok(())
    }

    #[inline(always)]
    #[allow(missing_docs)]
    pub fn compute_unchecked(
        temperature: Float,
        relative_humidity: Float,
        pressure: Float,
    ) -> Float {
        let saturation_vapour_pressure =
            vapour_pressure::Buck3::compute_unchecked(temperature, pressure);
        let vapour_pressure = vapour_pressure::SaturationSpecific1::compute_unchecked(
            saturation_vapour_pressure,
            relative_humidity,
        );

        General1::compute_unchecked(vapour_pressure, saturation_vapour_pressure)
    }
}

generate_compute!(General3, temperature, relative_humidity, pressure);
generate_vec_compute!(General3, temperature, relative_humidity, pressure);
generate_ndarray_compute!(General3, temperature, relative_humidity, pressure);
generate_par_vec_compute!(General3, temperature, relative_humidity, pressure);
generate_par_ndarray_compute!(General3, temperature, relative_humidity, pressure);

// #[cfg(test)]
// mod tests {
//     use crate::{
//         tests_framework::{self, Argument},
//         vapour_pressure_deficit,
//     };

//     #[test]
//     fn general1() {
//         assert!(tests_framework::test_with_2args(
//             &vapour_pressure_deficit::general1,
//             Argument {
//                 name: "vapour_pressure",
//                 def_val: 3000.0,
//                 range: [0.0, 50_000.0]
//             },
//             Argument {
//                 name: "saturation_vapour_pressure",
//                 def_val: 3550.0,
//                 range: [0.0, 50_000.0]
//             },
//             550.0
//         ));
//     }

//     #[test]
//     fn general2() {
//         assert!(tests_framework::test_with_3args(
//             &vapour_pressure_deficit::general2,
//             Argument {
//                 name: "temperature",
//                 def_val: 300.0,
//                 range: [253.0, 324.0]
//             },
//             Argument {
//                 name: "dewpoint",
//                 def_val: 290.0,
//                 range: [253.0, 324.0]
//             },
//             Argument {
//                 name: "pressure",
//                 def_val: 101325.0,
//                 range: [100.0, 150_000.0]
//             },
//             1621.9415403325527
//         ));
//     }

//     #[test]
//     fn general3() {
//         assert!(tests_framework::test_with_3args(
//             &vapour_pressure_deficit::general3,
//             Argument {
//                 name: "temperature",
//                 def_val: 300.0,
//                 range: [253.0, 319.0]
//             },
//             Argument {
//                 name: "relative_humidity",
//                 def_val: 0.5,
//                 range: [0.05, 1.0]
//             },
//             Argument {
//                 name: "pressure",
//                 def_val: 101325.0,
//                 range: [10000.0, 150_000.0]
//             },
//             1774.2520524017948
//         ));
//     }
// }
