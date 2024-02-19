//!Functions to calculate virtual temperature of air in K.
//!
//!In atmospheric thermodynamics, the virtual temperature of a moist air parcel is the temperature
//!at which a theoretical dry air parcel would have a total pressure and density equal
//!to the moist parcel of air ([Wikipedia](https://en.wikipedia.org/wiki/Virtual_temperature)).

use crate::Float;
use crate::{constants::EPSILON, errors::InputError};

#[cfg(feature = "debug")]
use floccus_proc::logerr;

///Formula for computing virtual temperature from temperature and mixing ratio.
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `temperature` range: 173K - 373K\
///Valid `mixing_ratio` range: 0.0000000001 - 0.5
pub fn general1(temperature: Float, mixing_ratio: Float) -> Result<Float, InputError> {
    general1_validate(temperature, mixing_ratio)?;
    Ok(general1_unchecked(temperature, mixing_ratio))
}

#[cfg_attr(feature = "debug", logerr)]
pub fn general1_validate(temperature: Float, mixing_ratio: Float) -> Result<(), InputError> {
    if !(173.0..=354.0).contains(&temperature) {
        return Err(InputError::OutOfRange(String::from("temperature")));
    }

    if !(0.000_000_000_1..=0.5).contains(&mixing_ratio) {
        return Err(InputError::OutOfRange(String::from("mixing_ratio")));
    }

    Ok(())
}

pub fn general1_unchecked(temperature: Float, mixing_ratio: Float) -> Float {
    temperature * ((mixing_ratio + EPSILON) / (EPSILON * (1.0 + mixing_ratio)))
}

///Formula for computing virtual temperature from air temperature, pressure and vapour pressure.
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `temperature` range: 173K - 373K\
///Valid `pressure` range: 100Pa - 150000Pa\
///Valid `vapour_pressure` range: 0Pa - 10000Pa
pub fn general2(
    temperature: Float,
    pressure: Float,
    vapour_pressure: Float,
) -> Result<Float, InputError> {
    general2_validate(temperature, pressure, vapour_pressure)?;
    Ok(general2_unchecked(temperature, pressure, vapour_pressure))
}

#[cfg_attr(feature = "debug", logerr)]
pub fn general2_validate(
    temperature: Float,
    pressure: Float,
    vapour_pressure: Float,
) -> Result<(), InputError> {
    if !(173.0..=354.0).contains(&temperature) {
        return Err(InputError::OutOfRange(String::from("temperature")));
    }

    if !(100.0..=150_000.0).contains(&pressure) {
        return Err(InputError::OutOfRange(String::from("pressure")));
    }

    if !(0.0..=10_000.0).contains(&vapour_pressure) {
        return Err(InputError::OutOfRange(String::from("vapour_pressure")));
    }
    Ok(())
}

pub fn general2_unchecked(temperature: Float, pressure: Float, vapour_pressure: Float) -> Float {
    temperature / (1.0 - ((vapour_pressure / pressure) * (1.0 - EPSILON)))
}

///Formula for computing virtual temperature from air temperature and specific humidity.
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `temperature` range: 173K - 373K\
///Valid `specific_humidity` range: 100Pa - 150000Pa
pub fn general3(temperature: Float, specific_humidity: Float) -> Result<Float, InputError> {
    general3_validate(temperature, specific_humidity)?;
    Ok(general3_unchecked(temperature, specific_humidity))
}

#[cfg_attr(feature = "debug", logerr)]
pub fn general3_validate(temperature: Float, specific_humidity: Float) -> Result<(), InputError> {
    if !(173.0..=354.0).contains(&temperature) {
        return Err(InputError::OutOfRange(String::from("temperature")));
    }

    if !(0.000000001..=2.0).contains(&specific_humidity) {
        return Err(InputError::OutOfRange(String::from("specific_humidity")));
    }

    Ok(())
}

pub fn general3_unchecked(temperature: Float, specific_humidity: Float) -> Float {
    temperature * (1.0 + (specific_humidity * ((1.0 / EPSILON) - 1.0)))
}

#[cfg(test)]
mod tests {
    use crate::{
        tests_framework::{self, Argument},
        virtual_temperature,
    };

    #[test]
    fn general1() {
        assert!(tests_framework::test_with_2args(
            &virtual_temperature::general1,
            Argument {
                name: "temperature",
                def_val: 300.0,
                range: [173.0, 354.0]
            },
            Argument {
                name: "mixing_ratio",
                def_val: 0.022,
                range: [0.000_000_000_1, 0.5]
            },
            303.9249219815806
        ));
    }

    #[test]
    fn general2() {
        assert!(tests_framework::test_with_3args(
            &virtual_temperature::general2,
            Argument {
                name: "temperature",
                def_val: 300.0,
                range: [173.0, 354.0]
            },
            Argument {
                name: "pressure",
                def_val: 101325.0,
                range: [100.0, 150_000.0]
            },
            Argument {
                name: "vapour_pressure",
                def_val: 3550.0,
                range: [0.0, 10_000.0]
            },
            304.0265941965307
        ));
    }

    #[test]
    fn general3() {
        assert!(tests_framework::test_with_2args(
            &virtual_temperature::general3,
            Argument {
                name: "temperature",
                def_val: 300.0,
                range: [173.0, 354.0]
            },
            Argument {
                name: "specific_humidity",
                def_val: 0.022,
                range: [0.000000001, 2.0]
            },
            304.0112702651753
        ));
    }
}
