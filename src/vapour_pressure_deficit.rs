//!Functions to calculate vapour pressure deficit in Pa.
//!
//!Vapour-pressure deficit, is the difference (deficit) between
//!the amount of moisture in the air and how much moisture the air can hold
//!when it is saturated ([Wikipedia](https://en.wikipedia.org/wiki/Vapour-pressure_deficit)).

use crate::{errors::InputError, vapour_pressure};
use crate::Float;

#[cfg(feature="debug")]
use floccus_proc::logerr;

///Formula for computing vapour pressure deficit from vapour pressure and saturation vapour pressure
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `vapour_pressure` range: 0Pa - 10000Pa
///Valid `saturation_vapour_pressure` range: 0Pa - 10000Pa
#[cfg_attr(feature = "debug", logerr)]
pub fn general1(vapour_pressure: Float, saturation_vapour_pressure: Float) -> Result<Float, InputError> {
    if !(0.0..=50_000.0).contains(&vapour_pressure) {
        return Err(InputError::OutOfRange(String::from("vapour_pressure")));
    }

    if !(0.0..=50_000.0).contains(&saturation_vapour_pressure) {
        return Err(InputError::OutOfRange(String::from(
            "saturation_vapour_pressure",
        )));
    }

    Ok(saturation_vapour_pressure - vapour_pressure)
}

///Formula for computing vapour pressure deficit from temperature, dewpoint and pressure
///using [`buck3`](vapour_pressure::buck3) function for vapour pressure calculation
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `vapour_pressure` range: 0Pa - 10000Pa
///Valid `saturation_vapour_pressure` range: 0Pa - 10000Pa
#[cfg_attr(feature = "debug", logerr)]
pub fn general2(temperature: Float, dewpoint: Float, pressure: Float) -> Result<Float, InputError> {
    if !(253.0..=324.0).contains(&temperature) {
        return Err(InputError::OutOfRange(String::from("temperature")));
    }

    if !(253.0..=324.0).contains(&dewpoint) {
        return Err(InputError::OutOfRange(String::from("dewpoint")));
    }

    if !(100.0..=150_000.0).contains(&pressure) {
        return Err(InputError::OutOfRange(String::from("pressure")));
    }

    let vapour_pressure = vapour_pressure::buck3(dewpoint, pressure)?;
    let saturation_vapour_pressure = vapour_pressure::buck3(temperature, pressure)?;

    let result = general1(vapour_pressure, saturation_vapour_pressure)?;

    Ok(result)
}

///Formula for computing vapour pressure deficit from temperature, relative humidity and pressure
///using [`buck3`](vapour_pressure::buck3) function for vapour pressure calculation
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `vapour_pressure` range: 0Pa - 10000Pa
///Valid `saturation_vapour_pressure` range: 0Pa - 10000Pa
#[cfg_attr(feature = "debug", logerr)]
pub fn general3(
    temperature: Float,
    relative_humidity: Float,
    pressure: Float,
) -> Result<Float, InputError> {
    if !(253.0..=319.0).contains(&temperature) {
        return Err(InputError::OutOfRange(String::from("temperature")));
    }

    if !(0.05..=1.0).contains(&relative_humidity) {
        return Err(InputError::OutOfRange(String::from("relative_humidity")));
    }

    if !(10000.0..=150_000.0).contains(&pressure) {
        return Err(InputError::OutOfRange(String::from("pressure")));
    }

    let saturation_vapour_pressure = vapour_pressure::buck3(temperature, pressure)?;
    let vapour_pressure =
        vapour_pressure::saturation_specific1(saturation_vapour_pressure, relative_humidity)?;

    let result = general1(vapour_pressure, saturation_vapour_pressure)?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::{
        tests_framework::{self, Argument},
        vapour_pressure_deficit,
    };

    #[test]
    fn general1() {
        assert!(tests_framework::test_with_2args(
            &vapour_pressure_deficit::general1,
            Argument {
                name: "vapour_pressure",
                def_val: 3000.0,
                range: [0.0, 50_000.0]
            },
            Argument {
                name: "saturation_vapour_pressure",
                def_val: 3550.0,
                range: [0.0, 50_000.0]
            },
            550.0
        ));
    }

    #[test]
    fn general2() {
        assert!(tests_framework::test_with_3args(
            &vapour_pressure_deficit::general2,
            Argument {
                name: "temperature",
                def_val: 300.0,
                range: [253.0, 324.0]
            },
            Argument {
                name: "dewpoint",
                def_val: 290.0,
                range: [253.0, 324.0]
            },
            Argument {
                name: "pressure",
                def_val: 101325.0,
                range: [100.0, 150_000.0]
            },
            1621.9415403325527
        ));
    }

    #[test]
    fn general3() {
        assert!(tests_framework::test_with_3args(
            &vapour_pressure_deficit::general3,
            Argument {
                name: "temperature",
                def_val: 300.0,
                range: [253.0, 319.0]
            },
            Argument {
                name: "relative_humidity",
                def_val: 0.5,
                range: [0.05, 1.0]
            },
            Argument {
                name: "pressure",
                def_val: 101325.0,
                range: [10000.0, 150_000.0]
            },
            1774.2520524017948
        ));
    }
}
