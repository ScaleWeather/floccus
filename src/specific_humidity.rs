//!Functions to calculate specific humidity of air in kg*kg^-1.
//!
//!Specific humidity (or moisture content) is the ratio of the mass
//!of water vapor to the total mass of the air parcel [Wikipedia](https://en.wikipedia.org/wiki/Humidity#Specific_humidity).
//!
//!Specific humidity is approximately equal to mixing ratio.

use crate::{constants::EPSILON, errors::InputError};
use crate::Float;

#[cfg(feature="debug")]
use floccus_proc::logerr;

///Formula for computing specific humidity from vapour pressure and pressure.
///Reverse function of [`vapour_pressure::general1`](crate::vapour_pressure::general1).
///This function is theoretical not empirical.
///
///Provided by [Rogers & Yau (1989)](https://www.elsevier.com/books/a-short-course-in-cloud-physics/yau/978-0-08-057094-5).
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `vapour_pressure` range: 0Pa - 50000OPa\,
///Valid `pressure` range: 100Pa - 150000Pa
#[cfg_attr(feature = "debug", logerr)]
pub fn general1(vapour_pressure: Float, pressure: Float) -> Result<Float, InputError> {
    if !(0.0..=50_000.0).contains(&vapour_pressure) {
        return Err(InputError::OutOfRange(String::from("vapour_pressure")));
    }

    if !(100.0..=150_000.0).contains(&pressure) {
        return Err(InputError::OutOfRange(String::from("pressure")));
    }

    let result = EPSILON * (vapour_pressure / (pressure - (vapour_pressure * (1.0 - EPSILON))));
    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::{
        specific_humidity,
        tests_framework::{self, Argument},
    };

    #[test]
    fn general1() {
        assert!(tests_framework::test_with_2args(
            &specific_humidity::general1,
            Argument {
                name: "vapour_pressure",
                def_val: 3000.0,
                range: [0.0, 50_000.0]
            },
            Argument {
                name: "pressure",
                def_val: 101325.0,
                range: [100.0, 150_000.0]
            },
            0.018623845512674677
        ));
    }
}
