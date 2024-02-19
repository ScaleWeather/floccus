//!Functions to calculate dry bulb potential temperature of unsaturated air in K.

use crate::Float;
use crate::{
    constants::{C_P, R_D, ZERO_CELSIUS},
    errors::InputError,
};

#[cfg(feature = "debug")]
use floccus_proc::logerr;

///Formula for computing wet bulb potential temperature from equivalent potential temperature.
///
///Derived by R. Davies-Jones (2008) [(doi:10.1175/2007MWR2224.1)](https://doi.org/10.1175/2007MWR2224.1)
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `temperature` range: 257K - 377K\
pub fn davies_jones1(equivalent_potential_temperature: Float) -> Result<Float, InputError> {
    davies_jones1_validate(equivalent_potential_temperature)?;
    Ok(davies_jones1_unchecked(equivalent_potential_temperature))
}

#[allow(missing_docs)]
#[allow(clippy::missing_errors_doc)]
#[cfg_attr(feature = "debug", logerr)]
pub fn davies_jones1_validate(equivalent_potential_temperature: Float) -> Result<(), InputError> {
    if !(257.0..=377.0).contains(&equivalent_potential_temperature) {
        return Err(InputError::OutOfRange(String::from(
            "equivalent_potential_temperature",
        )));
    }

    Ok(())
}

#[allow(missing_docs)]
pub fn davies_jones1_unchecked(equivalent_potential_temperature: Float) -> Float {
    let lambda = C_P / R_D;
    let result = 45.114 - 51.489 * (ZERO_CELSIUS / equivalent_potential_temperature).powf(lambda);
    result + ZERO_CELSIUS
}

#[cfg(test)]
mod tests {
    use crate::{
        tests_framework::{self, Argument},
        wet_bulb_potential_temperature,
    };

    #[test]
    fn davies_jones1() {
        assert!(tests_framework::test_with_1arg(
            &wet_bulb_potential_temperature::davies_jones1,
            Argument {
                name: "equivalent_potential_temperature",
                def_val: 300.0,
                range: [257.0, 377.0]
            },
            281.17941447108467
        ));
    }
}
