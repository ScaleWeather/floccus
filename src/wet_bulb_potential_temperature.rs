//!Functions to calculate dry bulb potential temperature of unsaturated air in K.

use crate::{
    constants::{C_P, R_D, ZERO_CELSIUS},
    error_wrapper::InputError,
};

///Formula for computing wet bulb potential temperature from equivalent potential temperature.
///
///Derived by R. Davies-Jones (2008) [(doi:10.1175/2007MWR2224.1)](https://doi.org/10.1175/2007MWR2224.1)
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `temperature` range: 257K - 377K\
pub fn davies_jones1(equivalent_potential_temperature: f64) -> Result<f64, InputError> {
    if !(257.0..=377.0).contains(&equivalent_potential_temperature) {
        return Err(InputError::OutOfRange(String::from(
            "equivalent_potential_temperature",
        )));
    }

    let lambda = C_P / R_D;

    let result = 45.114 - 51.489 * (ZERO_CELSIUS / equivalent_potential_temperature).powf(lambda);
    Ok(result + ZERO_CELSIUS)
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
