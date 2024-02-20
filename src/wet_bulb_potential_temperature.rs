//!Functions to calculate wet bulb potential temperature of unsaturated air in K.

use crate::compute_macros::{generate_compute, generate_ndarray_compute, generate_par_ndarray_compute, generate_par_vec_compute, generate_vec_compute};
use ndarray::{Array, Dimension, FoldWhile};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use crate::Float;
use crate::{
    constants::{C_P, R_D, ZERO_CELSIUS},
    errors::InputError,
};
#[cfg(feature = "debug")]
use floccus_proc::logerr;

/// Formula for computing wet bulb potential temperature from equivalent potential temperature.
/// 
/// Derived by R. Davies-Jones (2008) [(doi:10.1175/2007MWR2224.1)](https://doi.org/10.1175/2007MWR2224.1)
/// 
/// # Errors
/// 
/// Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
/// Valid `temperature` range: 257K - 377K\
pub struct DaviesJones1;

impl DaviesJones1 {
    #[allow(missing_docs)]
    #[inline(always)]
    pub fn compute_unchecked(equivalent_potential_temperature: Float) -> Float {
        let lambda = C_P / R_D;
        let result =
            45.114 - 51.489 * (ZERO_CELSIUS / equivalent_potential_temperature).powf(lambda);
        result + ZERO_CELSIUS
    }

    #[allow(missing_docs)]
    #[allow(clippy::missing_errors_doc)]
    #[inline(always)]
    #[cfg_attr(feature = "debug", logerr)]
    pub fn validate_inputs(equivalent_potential_temperature: Float) -> Result<(), InputError> {
        if !(257.0..=377.0).contains(&equivalent_potential_temperature) {
            return Err(InputError::OutOfRange(String::from(
                "equivalent_potential_temperature",
            )));
        }

        Ok(())
    }
}

generate_compute!(DaviesJones1, equivalent_potential_temperature);
generate_vec_compute!(DaviesJones1, equivalent_potential_temperature);
generate_ndarray_compute!(DaviesJones1, equivalent_potential_temperature);
generate_par_vec_compute!(DaviesJones1, equivalent_potential_temperature);
generate_par_ndarray_compute!(DaviesJones1, equivalent_potential_temperature);

#[cfg(test)]
mod tests {
    use crate::{
        tests_framework::{self, Argument},
        wet_bulb_potential_temperature,
    };

    #[test]
    fn davies_jones1() {
        assert!(tests_framework::test_with_1arg(
            &wet_bulb_potential_temperature::DaviesJones1::compute,
            Argument {
                name: "equivalent_potential_temperature",
                def_val: 300.0,
                range: [257.0, 377.0]
            },
            281.17941447108467
        ));
    }
}
