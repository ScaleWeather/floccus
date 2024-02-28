//! Functions to calculate specific humidity of air
//!
//! Specific humidity (or moisture content) is the ratio of the mass
//! of water vapor to the total mass of the air parcel [Wikipedia](https://en.wikipedia.org/wiki/Humidity#Specific_humidity).
//!
//! Specific humidity is approximately equal to mixing ratio.

use crate::constants::DIMLESS_ONE;
use crate::Formula2;
use crate::quantities::{
    AtmosphericPressure, SpecificHumidity, ThermodynamicQuantity, VapourPressure,
};
use crate::{constants::EPSILON, errors::InputError};

type FormulaQuantity = SpecificHumidity;

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

impl Formula2<FormulaQuantity, VapourPressure, AtmosphericPressure> for Definition1 {
    #[inline(always)]
    fn validate_inputs(
        vapour_pressure: VapourPressure,
        pressure: AtmosphericPressure,
    ) -> Result<(), InputError> {
        vapour_pressure.check_range_si(0.0, 50_000.0)?;
        pressure.check_range_si(100.0, 150_000.0)?;

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
    use crate::tests::{test_with_2args, testing_traits::ReferenceAtmosphere, Argument};

    use super::*;

    #[test]
    fn definition1() {
        test_with_2args::<FormulaQuantity, VapourPressure, AtmosphericPressure, Definition1>(
            Argument::new([0.0, 50_000.0]),
            Argument::new([100.0, 150_000.0]),
            ReferenceAtmosphere::Normal,
            1e-12,
        );
    }
}
