//!Functions to calculate wet bulb temperature of unsaturated air.

use crate::{constants::ZERO_CELSIUS, error_wrapper::InputError};

///Formula for computing wet bulb temperature pressure from dry bulb temperature and relative humidity.
///
///Derived by R. Stull (2011) [(doi:10.1175/JAMC-D-11-0143.1)](https://doi.org/10.1175/JAMC-D-11-0143.1)
///Created with use of gene-expression programming.\
///Result error is within −1K to +0.65K, with mean absolute error of 0.28K
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid temperature range: 253K - 324K\
///Valid relative_humidity range: 0.05 - 0.99
pub fn stull1(temperature: f64, relative_humidity: f64) -> Result<f64, InputError> {
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
    use crate::{error_wrapper::InputError, wet_bulb_temperature};
    use float_cmp::assert_approx_eq;

    #[test]
    fn stull1() {
        let result = wet_bulb_temperature::stull1(300.0, 0.5).unwrap();
        let expected = 292.73867410526674;
        assert_approx_eq!(f64, expected, result, ulps = 2);

        for &temperature in [252.9f64, 324.1f64].iter() {
            let result = wet_bulb_temperature::stull1(temperature, 0.5).unwrap_err();
            let expected = InputError::OutOfRange(String::from("temperature"));
            assert_eq!(result, expected);
        }

        for &relative_humidity in [0.04f64, 1.0f64].iter() {
            let result = wet_bulb_temperature::stull1(300.0, relative_humidity).unwrap_err();
            let expected = InputError::OutOfRange(String::from("relative_humidity"));
            assert_eq!(result, expected);
        }
    }
}
