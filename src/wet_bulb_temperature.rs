//!Functions to calculate wet bulb temperature of unsaturated air in K.

use crate::{constants::ZERO_CELSIUS, errors::InputError};
use crate::Float;

#[cfg(feature="debug")]
use floccus_proc::logerr;

///Formula for computing wet bulb temperature pressure from dry bulb temperature and relative humidity.
///
///Derived by R. Stull (2011) [(doi:10.1175/JAMC-D-11-0143.1)](https://doi.org/10.1175/JAMC-D-11-0143.1)
///Created with use of gene-expression programming.\
///Result error is within −1K to +0.65K, with mean absolute error of 0.28K
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `temperature` range: 253K - 324K\
///Valid `relative_humidity` range: 0.05 - 0.99
#[cfg_attr(feature = "debug", logerr)]
pub fn stull1(temperature: Float, relative_humidity: Float) -> Result<Float, InputError> {
    if !(253.0..=324.0).contains(&temperature) {
        return Err(InputError::OutOfRange(String::from("temperature")));
    }

    if !(0.05..=0.99).contains(&relative_humidity) {
        return Err(InputError::OutOfRange(String::from("relative_humidity")));
    }

    //convert units
    let temperature = temperature - ZERO_CELSIUS;
    let relative_humidity = relative_humidity * 100.0;

    let result = (temperature * (0.151_977 * (relative_humidity + 8.313_659).sqrt()).atan())
        + (temperature + relative_humidity).atan()
        - (relative_humidity - 1.676_331).atan()
        + (0.003_918_38 * relative_humidity.powf(1.5) * (0.023_101 * relative_humidity).atan())
        - 4.686_035;

    Ok(result + ZERO_CELSIUS)
}

#[cfg(test)]
mod tests {
    use crate::{
        tests_framework::{self, Argument},
        wet_bulb_temperature,
    };

    #[test]
    fn stull1() {
        assert!(tests_framework::test_with_2args(
            &wet_bulb_temperature::stull1,
            Argument {
                name: "temperature",
                def_val: 300.0,
                range: [253.0, 324.0]
            },
            Argument {
                name: "relative_humidity",
                def_val: 0.5,
                range: [0.05, 0.99]
            },
            292.73867410526674
        ));
    }
}
