//!Functions to calculate virtual temperature of air.
//!
//!In atmospheric thermodynamics, the virtual temperature of a moist air parcel is the temperature
//!at which a theoretical dry air parcel would have a total pressure and density equal
//!to the moist parcel of air ([Wikipedia](https://en.wikipedia.org/wiki/Virtual_temperature)).

use crate::{constants::EPSILON, error_wrapper::InputError};

///Formula for computing virtual temperature from temperature and mixing ratio.
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `temperature` range: 173K - 373K\
///Valid `mixing_ratio` range: 0.00001 - 0.5
pub fn general1(temperature: f64, mixing_ratio: f64) -> Result<f64, InputError> {
    if !(173.0..=373.0).contains(&temperature) {
        return Err(InputError::OutOfRange(String::from("temperature")));
    }

    if !(0.00001..=0.5).contains(&mixing_ratio) {
        return Err(InputError::OutOfRange(String::from("mixing_ratio")));
    }

    let result = temperature * ((mixing_ratio + EPSILON) / (EPSILON * (1.0 + mixing_ratio)));

    Ok(result)
}

///Formula for computing virtual temperature from air temperature, pressure and vapour pressure.
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `temperature` range: 173K - 373K\
///Valid `pressure` range: 100Pa - 150000Pa\
///Valid `vapour_pressure` range: 0Pa - 10000Pa
pub fn general2(temperature: f64, pressure: f64, vapour_pressure: f64) -> Result<f64, InputError> {
    if !(173.0..=373.0).contains(&temperature) {
        return Err(InputError::OutOfRange(String::from("temperature")));
    }

    if !(100.0..=150_000.0).contains(&pressure) {
        return Err(InputError::OutOfRange(String::from("pressure")));
    }

    if !(0.0..=10_000.0).contains(&vapour_pressure) {
        return Err(InputError::OutOfRange(String::from("vapour_pressure")));
    }

    let result = temperature / (1.0 - ((vapour_pressure / pressure) * (1.0 - EPSILON)));

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::{error_wrapper::InputError, virtual_temperature};
    use float_cmp::assert_approx_eq;

    #[test]
    fn general1() {
        let result = virtual_temperature::general1(300.0, 0.022).unwrap();
        let expected = 303.9249219815806;
        assert_approx_eq!(f64, expected, result, ulps = 2);

        for &temperature in [172.9, 374.1].iter() {
            let result = virtual_temperature::general1(temperature, 0.022).unwrap_err();
            let expected = InputError::OutOfRange(String::from("temperature"));
            assert_eq!(result, expected);
        }

        for &mixing_ratio in [0.000009, 0.51].iter() {
            let result = virtual_temperature::general1(300.0, mixing_ratio).unwrap_err();
            let expected = InputError::OutOfRange(String::from("mixing_ratio"));
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn general2() {
        let result = virtual_temperature::general2(300.0, 101325.0, 3550.0).unwrap();
        let expected = 304.0265941965307;
        assert_approx_eq!(f64, expected, result, ulps = 2);

        for &temperature in [172.9, 374.1].iter() {
            let result = virtual_temperature::general2(temperature, 101325.0, 3550.0).unwrap_err();
            let expected = InputError::OutOfRange(String::from("temperature"));
            assert_eq!(result, expected);
        }

        for &pressure in [99.9, 150_000.1].iter() {
            let result = virtual_temperature::general2(300.0, pressure, 3550.0).unwrap_err();
            let expected = InputError::OutOfRange(String::from("pressure"));
            assert_eq!(result, expected);
        }

        for &vapour_pressure in [-0.1, 10_000.1].iter() {
            let result = virtual_temperature::general2(300.0, 101325.0, vapour_pressure).unwrap_err();
            let expected = InputError::OutOfRange(String::from("vapour_pressure"));
            assert_eq!(result, expected);
        }
    }
}
