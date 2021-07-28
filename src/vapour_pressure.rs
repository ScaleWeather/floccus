//!Functions to calculate partial vapour pressure and saturation vapour pressure in the unsaturated air

use crate::{constants::ZERO_CELSIUS, error_wrapper::InputError};

///Formula for computing vapour pressure from air temperature and pressure.
///Most accurate in temperature range from 233K to 323K.
///
///Derived by A. L. Buck (1981) [(doi: 10.1175/1520-0450(1981)020<1527:nefcvp>2.0.co;2)](https://doi.org/10.1175/1520-0450(1981)020%3C1527:NEFCVP%3E2.0.CO;2).
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid temperature range: 232K - 324K\
///Valid pressure range: 100Pa - 150000Pa
pub fn buck1(temperature: f64, pressure: f64) -> Result<f64, InputError> {
    //validate inputs
    if !(232.0..=324.0).contains(&temperature) {
        return Err(InputError::OutOfRange(String::from("temperature")));
    }

    if !(100.0..=150_000.0).contains(&pressure) {
        return Err(InputError::OutOfRange(String::from("pressure")));
    }

    let temperature = temperature - ZERO_CELSIUS; //convert to C
    let pressure = pressure / 100.0; //convert to hPa

    let lower_a = 6.1121;
    let lower_b = 18.729;
    let lower_c = 257.87;
    let lower_d = 227.3;

    let upper_a = 0.000_72;
    let upper_b = 0.000_003_2;
    let upper_c = 0.000_000_000_59;

    let lower_e = lower_a
        * (((lower_b - (temperature / lower_d)) * temperature) / (temperature + lower_c)).exp();
    let lower_f = 1.0 + upper_a + (pressure * (upper_b + (upper_c * temperature * temperature)));

    Ok((lower_e * lower_f) * 100.0) //return in Pa
}

///Formula for computing vapour pressure from air temperature over water.
///Should be used for temperatures above 273K.
///
///Derived by O. Tetens (1930).
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when input is out of range.\
///Valid temperature range: 273K - 353K
pub fn tetens1(temperature: f64) -> Result<f64, InputError> {
    //validate inputs
    if !(273.0..=354.0).contains(&temperature) {
        return Err(InputError::OutOfRange(String::from("temperature")));
    }

    let temperature = temperature - 273.15; //convert to C

    let lower_a = 0.61078;
    let lower_b = 17.27;
    let lower_c = 237.3;

    let result = lower_a * ((lower_b * temperature) / (temperature + lower_c)).exp();

    Ok(result * 1000.0) //return in Pa
}

#[cfg(test)]
mod tests {
    use crate::{error_wrapper::InputError, vapour_pressure};
    use float_cmp::assert_approx_eq;

    #[test]
    fn test_buck1() {
        let result = vapour_pressure::buck1(300.0, 101325.0).unwrap();
        let expected = 3550.6603579471303;
        assert_approx_eq!(f64, expected, result, ulps = 2);

        for &temperature in [231.9f64, 324.1f64].iter() {
            let result = vapour_pressure::buck1(temperature, 101325.0).unwrap_err();
            let expected = InputError::OutOfRange(String::from("temperature"));
            assert_eq!(result, expected);
        }

        for &pressure in [99.9f64, 150000.1f64].iter() {
            let result = vapour_pressure::buck1(300.0, pressure).unwrap_err();
            let expected = InputError::OutOfRange(String::from("pressure"));
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_tetens1() {
        let result = vapour_pressure::tetens1(300.0).unwrap();
        let expected = 3533.969137160892;
        assert_approx_eq!(f64, expected, result, ulps = 2);

        for &temperature in [272.9f64, 354.1f64].iter() {
            let result = vapour_pressure::tetens1(temperature).unwrap_err();
            let expected = InputError::OutOfRange(String::from("temperature"));
            assert_eq!(result, expected);
        }
    }
}
