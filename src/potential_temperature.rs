//!Functions to calculate potential temperature of dry air in K.


use crate::Float;
use crate::{
    constants::{C_P, R_D},
    errors::InputError,
};
use float_cmp::approx_eq;


use itertools::izip;
use ndarray::{Array, Dimension, FoldWhile};
use rayon::iter::{ParallelBridge, ParallelIterator};

///Formula for computing potential temperature of dry air from temperature, pressure and vapour pressure.
///
///Provided by R. Davies-Jones (2009) [(doi:10.1175/2009MWR2774.1)](https://doi.org/10.1175/2009MWR2774.1)
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `temperature` range: 253K - 324K\
///Valid `pressure` range: 100Pa - 150000Pa\
///Valid `vapour_pressure` range: 0Pa - 10000Pa
///
///Returns [`InputError::IncorrectArgumentSet`] when `pressure` and `vapour_pressure` are equal,
///in which case division by 0 occurs.
///
///Returns [`InputError::IncorrectArgumentSet`] when `pressure` is lower than `vapour_pressure`,
///in which case floating-point exponentation of negative number occurs.
pub struct DaviesJones1;

impl DaviesJones1 {
    #[allow(missing_docs)]
    #[inline(always)]
    #[allow(clippy::missing_errors_doc)]
    
    pub fn validate_inputs(
        temperature: Float,
        pressure: Float,
        vapour_pressure: Float,
    ) -> Result<(), InputError> {
        if !(253.0..=324.0).contains(&temperature) {
            return Err(InputError::OutOfRange(String::from("temperature")));
        }

        if !(100.0..=150_000.0).contains(&pressure) {
            return Err(InputError::OutOfRange(String::from("pressure")));
        }

        if !(0.0..=10_000.0).contains(&vapour_pressure) {
            return Err(InputError::OutOfRange(String::from("vapour_pressure")));
        }

        if approx_eq!(Float, pressure, vapour_pressure, ulps = 2) {
            return Err(InputError::IncorrectArgumentSet(String::from(
                "pressure and vapour_pressure cannot be equal",
            )));
        }

        if vapour_pressure > pressure {
            return Err(InputError::IncorrectArgumentSet(String::from(
                "vapour_pressure cannot be higher than pressure",
            )));
        }

        Ok(())
    }

    #[inline(always)]
    #[allow(missing_docs)]
    pub fn compute_unchecked(temperature: Float, pressure: Float, vapour_pressure: Float) -> Float {
        let kappa = R_D / C_P;
        temperature * (100_000.0 / (pressure - vapour_pressure)).powf(kappa)
    }
}


// #[cfg(test)]
// mod tests {
//     use crate::{
//         potential_temperature,
//         tests_framework::{self, Argument},
//     };

//     #[test]
//     fn davies_jones1() {
//         assert!(tests_framework::test_with_3args(
//             &potential_temperature::davies_jones1,
//             Argument {
//                 name: "temperature",
//                 def_val: 300.0,
//                 range: [253.0, 324.0]
//             },
//             Argument {
//                 name: "pressure",
//                 def_val: 101325.0,
//                 range: [100.0, 150_000.0]
//             },
//             Argument {
//                 name: "vapour_pressure",
//                 def_val: 3000.0,
//                 range: [0.0, 10_000.0]
//             },
//             301.45136519081666
//         ));
//     }
// }
