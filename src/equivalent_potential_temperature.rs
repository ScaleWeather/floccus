//!Functions to calculate equivalent potential temperature of air in K.

use crate::compute_macros::{
    generate_compute, generate_ndarray_compute, generate_par_ndarray_compute,
    generate_par_vec_compute, generate_vec_compute,
};
use crate::constants::{C_L, C_P, EPSILON, L_V, R_D, R_V};
use crate::errors::InputError;
use crate::{mixing_ratio, potential_temperature, relative_humidity, vapour_pressure, Float};
#[cfg(feature = "debug")]
use floccus_proc::logerr;
use itertools::izip;
use ndarray::{Array, Dimension, FoldWhile};
use rayon::iter::{ParallelBridge, ParallelIterator};

///Most accuarte formula for computing equivalent potential temperature of unsaturated air from
///temperature, pressure and vapour pressure.
///
///Implementation of this formula assumes no liquid or solid water in the air parcel.
///
///First appeared in Paluch, Ilga (1979). J. Atmos. Sci., 36, 2467-2478
///
///Provided in Emmanuel, Kerry (1994). Atmospheric Convection. Oxford University Press.
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `temperature` range: 253K - 324K\
///Valid `pressure` range: 100Pa - 150000Pa\
///Valid `vapour_pressure` range: 0Pa - 10000Pa
pub struct Paluch1;

impl Paluch1 {
    #[allow(missing_docs)]
    #[inline(always)]
    #[allow(clippy::missing_errors_doc)]
    #[cfg_attr(feature = "debug", logerr)]
    pub fn validate_inputs(
        temperature: Float,
        pressure: Float,
        vapour_pressure: Float,
    ) -> Result<(), InputError> {
        if !(253.0..=324.0).contains(&temperature) {
            return Err(InputError::OutOfRange(String::from("temperature")));
        }

        if !(20000.0..=150_000.0).contains(&pressure) {
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
        let p0 = 100_000.0;

        let mixing_ratio = mixing_ratio::General1::compute_unchecked(pressure, vapour_pressure);
        let saturation_vapour_pressure =
            vapour_pressure::Buck1::compute_unchecked(temperature, pressure);

        let relative_humidity = relative_humidity::General2::compute_unchecked(
            vapour_pressure,
            saturation_vapour_pressure,
        );

        temperature
            * (p0 / pressure).powf(R_D / (C_P + mixing_ratio * C_L))
            * relative_humidity.powf((-mixing_ratio * R_V) / (C_P + mixing_ratio * C_L))
            * ((L_V * mixing_ratio) / (temperature * (C_P + mixing_ratio * C_L))).exp()
    }
}

generate_compute!(Paluch1, temperature, pressure, vapour_pressure);
generate_vec_compute!(Paluch1, temperature, pressure, vapour_pressure);
generate_par_vec_compute!(Paluch1, temperature, pressure, vapour_pressure);
generate_ndarray_compute!(Paluch1, temperature, pressure, vapour_pressure);
generate_par_ndarray_compute!(Paluch1, temperature, pressure, vapour_pressure);

///Formula for computing equivalent potential temperature of unsaturated air from
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
pub struct Bryan1;

impl Bryan1 {
    #[allow(missing_docs)]
    #[inline(always)]
    #[allow(clippy::missing_errors_doc)]
    #[cfg_attr(feature = "debug", logerr)]
    pub fn validate_inputs(
        temperature: Float,
        pressure: Float,
        vapour_pressure: Float,
    ) -> Result<(), InputError> {
        if !(253.0..=324.0).contains(&temperature) {
            return Err(InputError::OutOfRange(String::from("temperature")));
        }

        if !(20000.0..=150_000.0).contains(&pressure) {
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
        let kappa = R_D / C_P;

        let potential_temperature = potential_temperature::DaviesJones1::compute_unchecked(
            temperature,
            pressure,
            vapour_pressure,
        );

        let saturation_vapour_pressure =
            vapour_pressure::Buck3::compute_unchecked(temperature, pressure);
        let relative_humidity = relative_humidity::General2::compute_unchecked(
            vapour_pressure,
            saturation_vapour_pressure,
        );

        let mixing_ratio = mixing_ratio::General1::compute_unchecked(pressure, vapour_pressure);

        potential_temperature
            * relative_humidity.powf((-kappa) * (mixing_ratio / EPSILON))
            * ((L_V * mixing_ratio) / (C_P * temperature)).exp()
    }
}

generate_compute!(Bryan1, temperature, pressure, vapour_pressure);
generate_vec_compute!(Bryan1, temperature, pressure, vapour_pressure);
generate_par_vec_compute!(Bryan1, temperature, pressure, vapour_pressure);
generate_ndarray_compute!(Bryan1, temperature, pressure, vapour_pressure);
generate_par_ndarray_compute!(Bryan1, temperature, pressure, vapour_pressure);

///Approximate formula for computing equivalent potential temperature of unsaturated air from
///temperature, pressure and dewpoint.
///
///Derived by D. Bolton (1980)
///[(doi:10.1175/1520-0493(1980)108<1046:TCOEPT>2.0.CO;2)](https://doi.org/10.1175/1520-0493(1980)108%3C1046:TCOEPT%3E2.0.CO;2)
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `pressure` range: 100Pa - 150000Pa\
///Valid `temperature` range: 253K - 324K\
///Valid `dewpoint` range: 253K - 324K
pub struct Bolton1;

impl Bolton1 {
    #[allow(missing_docs)]
    #[inline(always)]
    #[allow(clippy::missing_errors_doc)]
    #[cfg_attr(feature = "debug", logerr)]
    pub fn validate_inputs(
        pressure: Float,
        temperature: Float,
        dewpoint: Float,
    ) -> Result<(), InputError> {
        if !(20000.0..=150_000.0).contains(&pressure) {
            return Err(InputError::OutOfRange(String::from("pressure")));
        }

        if !(253.0..=324.0).contains(&temperature) {
            return Err(InputError::OutOfRange(String::from("temperature")));
        }

        if !(253.0..=324.0).contains(&dewpoint) {
            return Err(InputError::OutOfRange(String::from("dewpoint")));
        }

        Ok(())
    }

    #[inline(always)]
    #[allow(missing_docs)]
    pub fn compute_unchecked(pressure: Float, temperature: Float, dewpoint: Float) -> Float {
        let kappa = R_D / C_P;

        let vapour_pressure = vapour_pressure::Buck3::compute_unchecked(dewpoint, pressure);
        let mixing_ratio = mixing_ratio::General1::compute_unchecked(pressure, vapour_pressure);

        let lcl_temp =
            (1.0 / ((1.0 / (dewpoint - 56.0)) + ((temperature / dewpoint).ln() / 800.0))) + 56.0;

        let theta_dl = temperature
            * (100_000.0 / (pressure - vapour_pressure)).powf(kappa)
            * (temperature / lcl_temp).powf(0.28 * mixing_ratio);

        theta_dl
            * (((3036.0 / lcl_temp) - 1.78) * mixing_ratio * (1.0 + 0.448 * mixing_ratio)).exp()
    }
}

generate_compute!(Bolton1, pressure, temperature, dewpoint);
generate_vec_compute!(Bolton1, pressure, temperature, dewpoint);
generate_par_vec_compute!(Bolton1, pressure, temperature, dewpoint);
generate_ndarray_compute!(Bolton1, pressure, temperature, dewpoint);

// #[cfg(test)]
// mod tests {
//     use crate::{
//         equivalent_potential_temperature,
//         tests_framework::{self, Argument},
//     };

//     #[test]
//     fn paluch1() {
//         assert!(tests_framework::test_with_3args(
//             &equivalent_potential_temperature::paluch1,
//             Argument {
//                 name: "temperature",
//                 def_val: 300.0,
//                 range: [253.0, 324.0]
//             },
//             Argument {
//                 name: "pressure",
//                 def_val: 101325.0,
//                 range: [20000.0, 150_000.0]
//             },
//             Argument {
//                 name: "vapour_pressure",
//                 def_val: 991.189131,
//                 range: [0.0, 10_000.0]
//             },
//             315.23724970376776
//         ));
//     }

//     #[test]
//     fn bryan1() {
//         assert!(tests_framework::test_with_3args(
//             &equivalent_potential_temperature::bryan1,
//             Argument {
//                 name: "temperature",
//                 def_val: 300.0,
//                 range: [253.0, 324.0]
//             },
//             Argument {
//                 name: "pressure",
//                 def_val: 101325.0,
//                 range: [20000.0, 150_000.0]
//             },
//             Argument {
//                 name: "vapour_pressure",
//                 def_val: 991.189131,
//                 range: [0.0, 10_000.0]
//             },
//             316.52762026634014
//         ));
//     }

//     #[test]
//     fn bolton1() {
//         assert!(tests_framework::test_with_3args(
//             &equivalent_potential_temperature::bolton1,
//             Argument {
//                 name: "pressure",
//                 def_val: 101325.0,
//                 range: [20000.0, 150_000.0]
//             },
//             Argument {
//                 name: "temperature",
//                 def_val: 300.0,
//                 range: [253.0, 324.0]
//             },
//             Argument {
//                 name: "dewpoint",
//                 def_val: 280.0,
//                 range: [253.0, 324.0]
//             },
//             317.3855211897774
//         ));
//     }
// }
