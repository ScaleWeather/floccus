//!Functions to calculate mixing ratio of fluids

use crate::{constants::EPSILON, error_wrapper::InputError, vapour_pressure};

///Formula for computing mixing ratio of unsaturated air from air pressure and vapour pressure
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid pressure range: 100Pa - 150000Pa\
///Valid vapour pressure range: 0Pa - 10000Pa
pub fn air_general1(pressure: f64, vapour_pressure: f64) -> Result<f64, InputError> {
    let result = EPSILON * (vapour_pressure / (pressure - vapour_pressure));
    Ok(result)
}

///Formula for computing mixing ratio of unsaturated air from air temperature and pressure.
///Optimised by performance
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid temperature range: 273K - 353K\
///Valid pressure range: 100Pa - 150000Pa
pub fn air_performance1(temperature: f64, pressure: f64) -> Result<f64, InputError> {
    let vapour_pressure = vapour_pressure::tetens1(temperature)?;
    let result = air_general1(pressure, vapour_pressure)?;
    Ok(result)
}

///Formula for computing mixing ratio of unsaturated air from air temperature and pressure.
///Optimised by accuracy
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid temperature range: 232K - 324K\
///Valid pressure range: 100Pa - 150000Pa
pub fn air_accuracy1(temperature: f64, pressure: f64) -> Result<f64, InputError> {
    let vapour_pressure = vapour_pressure::buck1(temperature, pressure)?;
    let result = air_general1(pressure, vapour_pressure)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;

    use crate::mixing_ratio;

    #[test]
    fn test_air_general1() {
        let result = mixing_ratio::air_general1(101325.0, 3500.0).unwrap();
        let expected = 0.022253316630823517;
        assert_approx_eq!(f64, expected, result, ulps = 2);
    }

    #[test]
    fn test_air_performance1() {
        let result = mixing_ratio::air_performance1(300.0, 101325.0).unwrap();
        let expected = 0.022477100514593465;
        assert_approx_eq!(f64, expected, result, ulps = 2);
    }

    #[test]
    fn test_air_accuracy1() {
        let result = mixing_ratio::air_accuracy1(300.0, 101325.0).unwrap();
        let expected = 0.022587116896465847;
        assert_approx_eq!(f64, expected, result, ulps = 2);
    }
}
