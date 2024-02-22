//! Functions to calculate specific humidity of air
//!
//! Specific humidity (or moisture content) is the ratio of the mass
//! of water vapor to the total mass of the air parcel [Wikipedia](https://en.wikipedia.org/wiki/Humidity#Specific_humidity).
//!
//! Specific humidity is approximately equal to mixing ratio.

use crate::constants::DIMLESS_ONE;
use crate::formula::Formula2;
use crate::quantities::{
    AtmosphericPressure, SpecificHumidity, ThermodynamicQuantity, VapourPressure,
};
use crate::{constants::EPSILON, errors::InputError};

/// Formula for computing specific humidity from vapour pressure and pressure.
/// Reverse function of [`vapour_pressure::general1`](crate::vapour_pressure::general1).
/// This function is theoretical not empirical.
///
/// Provided by [Rogers & Yau (1989)](https://www.elsevier.com/books/a-short-course-in-cloud-physics/yau/978-0-08-057094-5).
///
/// Valid `vapour_pressure` range: 0Pa - 50000OPa
///
/// Valid `pressure` range: 100Pa - 150000Pa
pub struct Definition1;

impl Formula2<SpecificHumidity, VapourPressure, AtmosphericPressure> for Definition1 {
    #[inline(always)]
    fn validate_inputs(
        vapour_pressure: VapourPressure,
        pressure: AtmosphericPressure,
    ) -> Result<(), InputError> {
        let vapour_pressure_si = vapour_pressure.get_si_value();
        let pressure_si = pressure.get_si_value();

        if !(0.0..=50_000.0).contains(&vapour_pressure_si) {
            return Err(InputError::OutOfRange(String::from("vapour_pressure")));
        }

        if !(100.0..=150_000.0).contains(&pressure_si) {
            return Err(InputError::OutOfRange(String::from("pressure")));
        }

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(
        vapour_pressure: VapourPressure,
        pressure: AtmosphericPressure,
    ) -> SpecificHumidity {
        let result = EPSILON
            * (vapour_pressure.0 / (pressure.0 - (vapour_pressure.0 * (DIMLESS_ONE - EPSILON))));

        SpecificHumidity(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::{test_with_2args, Argument};

    use super::*;

    #[test]
    fn definition1() {
        test_with_2args::<SpecificHumidity, VapourPressure, AtmosphericPressure, Definition1>(
            Argument {
                name: "vapour_pressure",
                def_val: 3000.0,
                range: [0.0, 50_000.0],
            },
            Argument {
                name: "pressure",
                def_val: 101325.0,
                range: [100.0, 150_000.0],
            },
            0.018623845512674677,
        );
    }
}
