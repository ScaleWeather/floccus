//! Functions to calculate mixing ratio of water vapour in unsaturated air
//!
//! Mixing ratio is the ratio of the mass of a variable atmospheric constituent to the mass
//! of dry air ([AMETSOC Glossary](https://glossary.ametsoc.org/wiki/Mixing_ratio)).

use crate::Formula2;
use crate::quantities::{AtmosphericPressure, MixingRatio, ThermodynamicQuantity, VapourPressure};
use crate::Float;
use crate::{constants::EPSILON, errors::InputError};
use float_cmp::approx_eq;

type FormulaQuantity = MixingRatio;

/// Formula for computing mixing ratio of unsaturated air from air pressure and vapour pressure
///
/// Valid `pressure` range: 100Pa - 150000Pa
///
/// Valid `vapour_pressure` range: 0Pa - 10000Pa
///
/// Returns [`InputError::IncorrectArgumentSet`] when inputs are equal and division by 0 would occur.
pub struct Definition1;

impl Formula2<FormulaQuantity, AtmosphericPressure, VapourPressure> for Definition1 {
    #[inline(always)]
    fn validate_inputs(
        pressure: AtmosphericPressure,
        vapour_pressure: VapourPressure,
    ) -> Result<(), InputError> {
        pressure.check_range_si(100.0, 150_000.0)?;
        vapour_pressure.check_range_si(0.0, 50_000.0)?;

        if vapour_pressure.0 > pressure.0 {
            return Err(InputError::OutOfRange(String::from(
                "vapour_pressure cannot be greater than pressure",
            )));
        }

        if approx_eq!(
            Float,
            pressure.get_si_value(),
            vapour_pressure.get_si_value(),
            ulps = 2
        ) {
            return Err(InputError::IncorrectArgumentSet(String::from(
                "pressure and vapour_pressure cannot be equal",
            )));
        }
        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(
        pressure: AtmosphericPressure,
        vapour_pressure: VapourPressure,
    ) -> MixingRatio {
        MixingRatio(EPSILON * (vapour_pressure.0 / (pressure.0 - vapour_pressure.0)))
    }
}

#[cfg(test)]
mod tests {

    use crate::tests::{test_with_2args, testing_traits::ReferenceAtmosphere, Argument};

    use super::*;

    #[test]
    fn general1() {
        test_with_2args::<FormulaQuantity, AtmosphericPressure, VapourPressure, Definition1>(
            Argument::new([100.0, 150_000.0]),
            Argument::new([0.0, 50_000.0]),
            ReferenceAtmosphere::Normal,
            1e-12,
        );
    }
}
