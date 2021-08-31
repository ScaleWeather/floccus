//!Functions to calculate virtual temperature of air.
//!
//!In atmospheric thermodynamics, the virtual temperature of a moist air parcel is the temperature
//!at which a theoretical dry air parcel would have a total pressure and density equal
//!to the moist parcel of air ([Wikipedia](https://en.wikipedia.org/wiki/Virtual_temperature)).

use crate::{constants::EPSILON, error_wrapper::InputError, mixing_ratio};

///Formula for computing virtual temperature from temperature and mixing ratio.
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `temperature` range: 100Pa - 150000Pa\
///Valid `mixing_ratio` range: 0Pa - 10000Pa
pub fn general1(temperature: f64, mixing_ratio: f64) -> Result<f64, InputError> {
    let result = temperature * ((mixing_ratio + EPSILON) / (EPSILON * (1.0 + mixing_ratio)));

    Ok(result)
}

pub fn performance1(temperature: f64, dewpoint: f64, pressure: f64) -> Result<f64, InputError> {
    let mixing_ratio = mixing_ratio::performance1(dewpoint, pressure)?;
    let result = general1(temperature, mixing_ratio)?;

    Ok(result)
}

pub fn accuracy1(temperature: f64, dewpoint: f64, pressure: f64) -> Result<f64, InputError> {
    let mixing_ratio = mixing_ratio::accuracy1(dewpoint, pressure)?;
    let result = general1(temperature, mixing_ratio)?;

    Ok(result)
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
}
