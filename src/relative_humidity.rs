//!Functions to calculate relative humidity

use crate::{error_wrapper::InputError, mixing_ratio, vapour_pressure};

///Formula for computing relative humidity from mixing ratio and saturation mixing ratio.
///Can be used interchangeably with [`general2`].
///
///By the definition of mixing ratio, this formula is mathematically equivalent of
///formula used in [`general2`].
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `mixing_ratio` range: 0.00001 - 0.5\
///Valid `saturation_mixing_ratio` range: 0.00001 - 0.5
pub fn general1(mixing_ratio: f64, saturation_mixing_ratio: f64) -> Result<f64, InputError> {
    if !(0.00001 ..=0.5).contains(&mixing_ratio) {
        return Err(InputError::OutOfRange(String::from("mixing_ratio")));
    }

    if !(0.00001 ..=0.5).contains(&saturation_mixing_ratio) {
        return Err(InputError::OutOfRange(String::from("saturation_mixing_ratio")));
    }
    
    Ok(mixing_ratio / saturation_mixing_ratio)
}

///Formula for computing relative humidity from vapour pressure and saturation vapour pressure.
///Can be used interchangeably with [`general1`].
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `vapour_pressure` range: 0Pa - 10000Pa
///Valid `saturation_vapour_pressure` range: 0Pa - 10000Pa
pub fn general2(vapour_pressure: f64, saturation_vapour_pressure: f64) -> Result<f64, InputError> {
    if !(0.0..=10_000.0).contains(&vapour_pressure) {
        return Err(InputError::OutOfRange(String::from("vapour_pressure")));
    }

    if !(0.0..=10_000.0).contains(&saturation_vapour_pressure) {
        return Err(InputError::OutOfRange(String::from("saturation_vapour_pressure")));
    }

    Ok(vapour_pressure / saturation_vapour_pressure)
}

///Formula for computing relative humidity from temperature and dewpoint using [`tetens1`](vapour_pressure::tetens1)
///function for vapour pressure calculation
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `temperature` range: 273K - 353K
///Valid `dewpoint` range: 273K - 353K
pub fn general3(temperature: f64, dewpoint: f64) -> Result<f64, InputError> {
    if !(273.0..=353.0).contains(&temperature) {
        return Err(InputError::OutOfRange(String::from("temperature")));
    }

    if !(273.0..=353.0).contains(&temperature) {
        return Err(InputError::OutOfRange(String::from("temperature")));
    }

     let vapour_pressure = vapour_pressure::tetens1(dewpoint)?;
     let saturation_vapour_pressure = vapour_pressure::tetens1(temperature)?;
     let result = general2(vapour_pressure, saturation_vapour_pressure)?;

     Ok(result)
}

///Formula for computing relative humidity from temperature, dewpoint and pressure using [`buck3`](vapour_pressure::buck3)
///function for vapour pressure calculation
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `temperature` range: 253K - 324K\
///Valid `dewpoint` range: 253K - 324K\
///Valid `pressure` range: 100Pa - 150000Pa
pub fn general4(temperature: f64, dewpoint: f64, pressure: f64) -> Result<f64, InputError> {
    if !(253.0..=324.0).contains(&temperature) {
        return Err(InputError::OutOfRange(String::from("temperature")));
    }

    if !(253.0..=324.0).contains(&temperature) {
        return Err(InputError::OutOfRange(String::from("temperature")));
    }

    if !(100.0..=150_000.0).contains(&pressure) {
        return Err(InputError::OutOfRange(String::from("pressure")));
    }

    let vapour_pressure = vapour_pressure::buck3(dewpoint, pressure)?;
    let saturation_vapour_pressure = vapour_pressure::buck3(temperature, pressure)?;
    let result = general2(vapour_pressure, saturation_vapour_pressure)?;

    Ok(result)
}

///Formula for computing relative humidity from temperature, dewpoint and pressure using [`accuracy1`](mixing_ratio::accuracy1)
///function for mixing ratio calculation
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `temperature` range: 232K - 324K\
///Valid `dewpoint` range: 232K - 324K\
///Valid `pressure` range: 100Pa - 150000Pa
pub fn general5(temperature: f64, dewpoint: f64, pressure: f64) -> Result<f64, InputError> {
    if !(232.0..=324.0).contains(&temperature) {
        return Err(InputError::OutOfRange(String::from("temperature")));
    }

    if !(232.0..=324.0).contains(&dewpoint) {
        return Err(InputError::OutOfRange(String::from("dewpoint")));
    }

    if !(100.0..=150_000.0).contains(&pressure) {
        return Err(InputError::OutOfRange(String::from("pressure")));
    }

    let mixing_ratio = mixing_ratio::accuracy1(dewpoint, pressure)?;
    let saturation_mixing_ratio = mixing_ratio::accuracy1(temperature, pressure)?;
    let result = general1(mixing_ratio, saturation_mixing_ratio)?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::{error_wrapper::InputError, relative_humidity};
    use float_cmp::assert_approx_eq;

    #[test]
    fn general1() {
        let result = relative_humidity::general1(0.01064,0.01467).unwrap();
        let expected = 0.7252897068847989;
        assert_approx_eq!(f64, expected, result, ulps = 2);

        for &mixing_ratio in [0.000009, 0.51].iter() {
            let result = relative_humidity::general1(mixing_ratio, 0.01064).unwrap_err();
            let expected = InputError::OutOfRange(String::from("mixing_ratio"));
            assert_eq!(result, expected);
        }

        for &saturation_mixing_ratio in [0.000009, 0.51].iter() {
            let result = relative_humidity::general1(0.01467, saturation_mixing_ratio).unwrap_err();
            let expected = InputError::OutOfRange(String::from("saturation_mixing_ratio"));
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn general2() {
        let result = relative_humidity::general2(1706.0, 2339.0).unwrap();
        let expected = 0.7293715262932877;
        assert_approx_eq!(f64, expected, result, ulps = 2);

        for &vapour_pressure in [-0.1, 10_000.1].iter() {
            let result = relative_humidity::general2(vapour_pressure, 2339.0).unwrap_err();
            let expected = InputError::OutOfRange(String::from("vapour_pressure"));
            assert_eq!(result, expected);
        }

        for &saturation_vapour_pressure in [-0.1, 10_000.1].iter() {
            let result = relative_humidity::general2(1706.0, saturation_vapour_pressure).unwrap_err();
            let expected = InputError::OutOfRange(String::from("saturation_vapour_pressure"));
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn general3() {
        let result = relative_humidity::general3(300.0, 290.0).unwrap();
        let expected = 0.5431069897660531;
        assert_approx_eq!(f64, expected, result, ulps = 2);

        for &temperature in [272.9, 353.1].iter() {
            let result = relative_humidity::general3(temperature, 290.0).unwrap_err();
            let expected = InputError::OutOfRange(String::from("temperature"));
            assert_eq!(result, expected);
        }

        for &dewpoint in [272.9, 353.1].iter() {
            let result = relative_humidity::general3(300.0, dewpoint).unwrap_err();
            let expected = InputError::OutOfRange(String::from("dewpoint"));
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn general4() {
        let result = relative_humidity::general4(300.0, 290.0, 101325.0).unwrap();
        let expected = 0.5429224562155812;
        assert_approx_eq!(f64, expected, result, ulps = 2);

        for &temperature in [252.9, 324.1].iter() {
            let result = relative_humidity::general4(temperature, 290.0, 101325.0).unwrap_err();
            let expected = InputError::OutOfRange(String::from("temperature"));
            assert_eq!(result, expected);
        }

        for &dewpoint in [252.9, 324.1].iter() {
            let result = relative_humidity::general4(300.0, dewpoint, 101325.0).unwrap_err();
            let expected = InputError::OutOfRange(String::from("dewpoint"));
            assert_eq!(result, expected);
        }

        for &pressure in [99.9, 150_000.1].iter() {
            let result = relative_humidity::general4(300.0, 290.0, pressure).unwrap_err();
            let expected = InputError::OutOfRange(String::from("pressure"));
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn general5() {
        let result = relative_humidity::general5(300.0, 290.0, 101325.0).unwrap();
        let expected = 0.5338747953552858;
        assert_approx_eq!(f64, expected, result, ulps = 2);

        for &temperature in [231.9, 324.1].iter() {
            let result = relative_humidity::general5(temperature, 290.0, 101325.0).unwrap_err();
            let expected = InputError::OutOfRange(String::from("temperature"));
            assert_eq!(result, expected);
        }

        for &dewpoint in [231.9, 324.1].iter() {
            let result = relative_humidity::general5(300.0, dewpoint, 101325.0).unwrap_err();
            let expected = InputError::OutOfRange(String::from("dewpoint"));
            assert_eq!(result, expected);
        }

        for &pressure in [99.9, 150_000.1].iter() {
            let result = relative_humidity::general5(300.0, 290.0, pressure).unwrap_err();
            let expected = InputError::OutOfRange(String::from("pressure"));
            assert_eq!(result, expected);
        }
    }
}
