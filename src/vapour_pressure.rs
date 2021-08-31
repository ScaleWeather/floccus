//!Functions to calculate partial vapour pressure of the unsaturated air.
//!
//!To compute saturation vapour pressure input dry-bulb temperature in place of dewpoint temperature.

use crate::{constants::ZERO_CELSIUS, error_wrapper::InputError};

///Formula for computing vapour pressure from dewpoint temperature and pressure.
///Should be used for air over water when accuracy is desired.
///
///Derived by A. L. Buck (1981) [(doi: 10.1175/1520-0450(1981)020<1527:nefcvp>2.0.co;2)](https://doi.org/10.1175/1520-0450(1981)020%3C1527:NEFCVP%3E2.0.CO;2).
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `dewpoint` range: 232K - 324K\
///Valid `pressure` range: 100Pa - 150000Pa
pub fn buck1(dewpoint: f64, pressure: f64) -> Result<f64, InputError> {
    //validate inputs
    if !(232.0..=324.0).contains(&dewpoint) {
        return Err(InputError::OutOfRange(String::from("dewpoint")));
    }

    if !(100.0..=150_000.0).contains(&pressure) {
        return Err(InputError::OutOfRange(String::from("pressure")));
    }

    let dewpoint = dewpoint - ZERO_CELSIUS; //convert to C
    let pressure = pressure / 100.0; //convert to hPa

    let lower_a = 6.1121;
    let lower_b = 18.729;
    let lower_c = 257.87;
    let lower_d = 227.3;

    let upper_a = 0.000_72;
    let upper_b = 0.000_003_2;
    let upper_c = 0.000_000_000_59;

    let lower_e =
        lower_a * (((lower_b - (dewpoint / lower_d)) * dewpoint) / (dewpoint + lower_c)).exp();
    let lower_f = 1.0 + upper_a + (pressure * (upper_b + (upper_c * dewpoint * dewpoint)));

    Ok((lower_e * lower_f) * 100.0) //return in Pa
}

///Formula for computing vapour pressure from dewpoint temperature and pressure.
///Should be used for air over ice when accuracy is desired.
///
///Derived by A. L. Buck (1981) [(doi: 10.1175/1520-0450(1981)020<1527:nefcvp>2.0.co;2)](https://doi.org/10.1175/1520-0450(1981)020%3C1527:NEFCVP%3E2.0.CO;2).
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `dewpoint` range: 193K - 274K\
///Valid `pressure` range: 100Pa - 150000Pa
pub fn buck2(dewpoint: f64, pressure: f64) -> Result<f64, InputError> {
    //validate inputs
    if !(193.0..=274.0).contains(&dewpoint) {
        return Err(InputError::OutOfRange(String::from("dewpoint")));
    }

    if !(100.0..=150_000.0).contains(&pressure) {
        return Err(InputError::OutOfRange(String::from("pressure")));
    }

    let dewpoint = dewpoint - ZERO_CELSIUS; //convert to C
    let pressure = pressure / 100.0; //convert to hPa

    let lower_a = 6.1115;
    let lower_b = 23.036;
    let lower_c = 279.82;
    let lower_d = 333.7;

    let upper_a = 0.000_22;
    let upper_b = 0.000_003_83;
    let upper_c = 0.000_000_000_64;

    let lower_e =
        lower_a * (((lower_b - (dewpoint / lower_d)) * dewpoint) / (dewpoint + lower_c)).exp();
    let lower_f = 1.0 + upper_a + (pressure * (upper_b + (upper_c * dewpoint * dewpoint)));

    Ok((lower_e * lower_f) * 100.0) //return in Pa
}

///Formula for computing vapour pressure from dewpoint temperature and pressure.
///Should be used for air over water for general use.
///
///Derived by A. L. Buck (1981) [(doi: 10.1175/1520-0450(1981)020<1527:nefcvp>2.0.co;2)](https://doi.org/10.1175/1520-0450(1981)020%3C1527:NEFCVP%3E2.0.CO;2).
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `dewpoint` range: 253K - 324K\
///Valid `pressure` range: 100Pa - 150000Pa
pub fn buck3(dewpoint: f64, pressure: f64) -> Result<f64, InputError> {
    //validate inputs
    if !(253.0..=324.0).contains(&dewpoint) {
        return Err(InputError::OutOfRange(String::from("dewpoint")));
    }

    if !(100.0..=150_000.0).contains(&pressure) {
        return Err(InputError::OutOfRange(String::from("pressure")));
    }

    let dewpoint = dewpoint - ZERO_CELSIUS; //convert to C
    let pressure = pressure / 100.0; //convert to hPa

    let lower_a = 6.1121;
    let lower_b = 17.502;
    let lower_c = 240.97;

    let upper_a = 0.000_7;
    let upper_b = 0.000_003_46;

    let lower_e = lower_a * ((lower_b * dewpoint) / (dewpoint + lower_c)).exp();
    let lower_f = 1.0 + upper_a + (pressure * upper_b);

    Ok((lower_e * lower_f) * 100.0) //return in Pa
}

///Formula for computing vapour pressure from dewpoint temperature and pressure.
///Should be used for air over ice for general use.
///
///Derived by A. L. Buck (1981) [(doi: 10.1175/1520-0450(1981)020<1527:nefcvp>2.0.co;2)](https://doi.org/10.1175/1520-0450(1981)020%3C1527:NEFCVP%3E2.0.CO;2).
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `dewpoint` range: 223K - 274K\
///Valid `pressure` range: 100Pa - 150000Pa
pub fn buck4(dewpoint: f64, pressure: f64) -> Result<f64, InputError> {
    //validate inputs
    if !(223.0..=274.0).contains(&dewpoint) {
        return Err(InputError::OutOfRange(String::from("dewpoint")));
    }

    if !(100.0..=150_000.0).contains(&pressure) {
        return Err(InputError::OutOfRange(String::from("pressure")));
    }

    let dewpoint = dewpoint - ZERO_CELSIUS; //convert to C
    let pressure = pressure / 100.0; //convert to hPa

    let lower_a = 6.1115;
    let lower_b = 22.452;
    let lower_c = 272.55;

    let upper_a = 0.000_3;
    let upper_b = 0.000_004_18;

    let lower_e = lower_a * ((lower_b * dewpoint) / (dewpoint + lower_c)).exp();
    let lower_f = 1.0 + upper_a + (pressure * upper_b);

    Ok((lower_e * lower_f) * 100.0) //return in Pa
}

///Formula for computing vapour pressure over water from dewpoint temperature.
///Should be used for temperatures above 273K.
///
///Derived by O. Tetens (1930).
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when input is out of range.\
///Valid `dewpoint` range: 273K - 353K
pub fn tetens1(dewpoint: f64) -> Result<f64, InputError> {
    //validate inputs
    if !(273.0..=353.0).contains(&dewpoint) {
        return Err(InputError::OutOfRange(String::from("dewpoint")));
    }

    let dewpoint = dewpoint - ZERO_CELSIUS; //convert to C

    let lower_a = 0.61078;
    let lower_b = 17.27;
    let lower_c = 237.3;

    let result = lower_a * ((lower_b * dewpoint) / (dewpoint + lower_c)).exp();

    Ok(result * 1000.0) //return in Pa
}

///Formula for computing **ONLY** vapour pressure from saturation vapour pressure and relative humidity.
///For saturation vapour pressure use [`saturation_specific2`]
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when input is out of range.\
///Valid `saturation_vapour_pressure` range: 0Pa - 10000Pa\
///Valid `relative_humidity` range: 0.0 - 1.0
pub fn saturation_specific1(
    saturation_vapour_pressure: f64,
    relative_humidity: f64,
) -> Result<f64, InputError> {
    if !(0.0..=1.0).contains(&relative_humidity) {
        return Err(InputError::OutOfRange(String::from("relative_humidity")));
    }

    if !(0.0..=10_000.0).contains(&saturation_vapour_pressure) {
        return Err(InputError::OutOfRange(String::from(
            "saturation_vapour_pressure",
        )));
    }

    Ok(saturation_vapour_pressure * relative_humidity)
}

///Formula for computing **ONLY** saturation vapour pressure from vapour pressure and relative humidity.
///For vapour pressure use [`saturation_specific1`]
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when input is out of range.\
///Valid `vapour_pressure` range: 0Pa - 10000Pa\
///Valid `relative_humidity` range: 0.0 - 1.0
pub fn saturation_specific2(
    vapour_pressure: f64,
    relative_humidity: f64,
) -> Result<f64, InputError> {
    if !(0.00001..=1.0).contains(&relative_humidity) {
        return Err(InputError::OutOfRange(String::from("relative_humidity")));
    }

    if !(0.0..=10_000.0).contains(&vapour_pressure) {
        return Err(InputError::OutOfRange(String::from("vapour_pressure")));
    }

    Ok(vapour_pressure / relative_humidity)
}

#[cfg(test)]
mod tests {
    use crate::{error_wrapper::InputError, vapour_pressure};
    use float_cmp::assert_approx_eq;

    #[test]
    fn buck1() {
        let result = vapour_pressure::buck1(300.0, 101325.0).unwrap();
        let expected = 3550.6603579471303;
        assert_approx_eq!(f64, expected, result, ulps = 2);

        for &dewpoint in [231.9, 324.1].iter() {
            let result = vapour_pressure::buck1(dewpoint, 101325.0).unwrap_err();
            let expected = InputError::OutOfRange(String::from("dewpoint"));
            assert_eq!(result, expected);
        }

        for &pressure in [99.9, 150000.1].iter() {
            let result = vapour_pressure::buck1(300.0, pressure).unwrap_err();
            let expected = InputError::OutOfRange(String::from("pressure"));
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn buck2() {
        let result = vapour_pressure::buck2(250.0, 101325.0).unwrap();
        let expected = 76.38781790372722;
        assert_approx_eq!(f64, expected, result, ulps = 2);

        for &dewpoint in [192.9, 274.1].iter() {
            let result = vapour_pressure::buck2(dewpoint, 101325.0).unwrap_err();
            let expected = InputError::OutOfRange(String::from("dewpoint"));
            assert_eq!(result, expected);
        }

        for &pressure in [99.9, 150000.1].iter() {
            let result = vapour_pressure::buck2(250.0, pressure).unwrap_err();
            let expected = InputError::OutOfRange(String::from("pressure"));
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn buck3() {
        let result = vapour_pressure::buck3(300.0, 101325.0).unwrap();
        let expected = 3548.5041048035896;
        assert_approx_eq!(f64, expected, result, ulps = 2);

        for &dewpoint in [252.9, 324.1].iter() {
            let result = vapour_pressure::buck3(dewpoint, 101325.0).unwrap_err();
            let expected = InputError::OutOfRange(String::from("dewpoint"));
            assert_eq!(result, expected);
        }

        for &pressure in [99.9, 150000.1].iter() {
            let result = vapour_pressure::buck3(300.0, pressure).unwrap_err();
            let expected = InputError::OutOfRange(String::from("pressure"));
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn buck4() {
        let result = vapour_pressure::buck4(250.0, 101325.0).unwrap();
        let expected = 76.38685471836712;
        assert_approx_eq!(f64, expected, result, ulps = 2);

        for &dewpoint in [222.9, 274.1].iter() {
            let result = vapour_pressure::buck4(dewpoint, 101325.0).unwrap_err();
            let expected = InputError::OutOfRange(String::from("dewpoint"));
            assert_eq!(result, expected);
        }

        for &pressure in [99.9, 150000.1].iter() {
            let result = vapour_pressure::buck4(250.0, pressure).unwrap_err();
            let expected = InputError::OutOfRange(String::from("pressure"));
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn tetens1() {
        let result = vapour_pressure::tetens1(300.0).unwrap();
        let expected = 3533.969137160892;
        assert_approx_eq!(f64, expected, result, ulps = 2);

        for &dewpoint in [272.9, 353.1].iter() {
            let result = vapour_pressure::tetens1(dewpoint).unwrap_err();
            let expected = InputError::OutOfRange(String::from("dewpoint"));
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn saturation_specific1() {
        let result = vapour_pressure::saturation_specific1(3550.0, 0.4).unwrap();
        let expected = 1420.0;
        assert_approx_eq!(f64, expected, result, ulps = 2);

        for saturation_vapour_pressure in 0..=10_000 {
            for relative_humidity in 0..=100 {
                let result = vapour_pressure::saturation_specific1(
                    saturation_vapour_pressure as f64,
                    relative_humidity as f64 / 100.0,
                )
                .unwrap();
                assert!(result.is_finite());
            }
        }

        for &saturation_vapour_pressure in [-0.1, 10_000.1].iter() {
            let result =
                vapour_pressure::saturation_specific1(saturation_vapour_pressure, 0.4).unwrap_err();
            let expected = InputError::OutOfRange(String::from("saturation_vapour_pressure"));
            assert_eq!(result, expected);
        }

        for &relative_humidity in [-0.1, 1.1].iter() {
            let result =
                vapour_pressure::saturation_specific1(3550.0, relative_humidity).unwrap_err();
            let expected = InputError::OutOfRange(String::from("relative_humidity"));
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn saturation_specific2() {
        let result = vapour_pressure::saturation_specific2(3000.0, 0.4).unwrap();
        let expected = 7500.0;
        assert_approx_eq!(f64, expected, result, ulps = 2);

        for vapour_pressure in 0..=10_000 {
            for relative_humidity in 1..=100 {
                let result = vapour_pressure::saturation_specific2(
                    vapour_pressure as f64,
                    relative_humidity as f64 / 100.0,
                )
                .unwrap();
                assert!(result.is_finite());
            }
        }

        for &vapour_pressure in [-0.1, 10_000.1].iter() {
            let result = vapour_pressure::saturation_specific2(vapour_pressure, 0.4).unwrap_err();
            let expected = InputError::OutOfRange(String::from("vapour_pressure"));
            assert_eq!(result, expected);
        }

        for &relative_humidity in [-0.1, 1.1].iter() {
            let result =
                vapour_pressure::saturation_specific2(3000.0, relative_humidity).unwrap_err();
            let expected = InputError::OutOfRange(String::from("relative_humidity"));
            assert_eq!(result, expected);
        }
    }
}
