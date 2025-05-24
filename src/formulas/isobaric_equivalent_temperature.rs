//! Functions to calculate isobaric potential temperature of unsaturated air
//!
//!
//! Isobaric equivalent temperature is a temperature that an air parcel would have if all water
//! vapor were condensed at constant pressure and the enthalpy released from the vapor used to heat the air.
//! This process is physically impossible in the atmosphere
//! ([AMETSOC Glossary](https://glossary.ametsoc.org/wiki/Equivalent_temperature)).

use crate::constants::{C_P, DIMLESS_ONE, L_V, ZERO_KELVIN};
use crate::errors::InputError;
use crate::formulas::Formula2;
use crate::quantities::{
    DryBulbTemperature, IsobaricEquivalentTemperature, MixingRatio, QuantityHelpers,
};

type FormulaQuantity = IsobaricEquivalentTemperature;

/// Formula for computing isobaric equivalent temperature from dry-bulb temperature and mixing ratio
///
/// Valid `temperature` range: 253K - 324K
///
/// Valid `mixing_ratio` range: 0.000_000_1 - 2.0
///
pub struct Definition1;

impl Formula2<FormulaQuantity, DryBulbTemperature, MixingRatio> for Definition1 {
    #[inline]
    fn validate_inputs_internal(
        temperature: DryBulbTemperature,
        mixing_ratio: MixingRatio,
    ) -> Result<(), InputError> {
        temperature.check_range_si(253.0, 324.0)?;
        mixing_ratio.check_range_si(0.000_000_1, 2.0)?;

        Ok(())
    }

    #[inline]
    fn compute_unchecked(
        temperature: DryBulbTemperature,
        mixing_ratio: MixingRatio,
    ) -> FormulaQuantity {
        let temp_ie =
            temperature.0 * (DIMLESS_ONE + ((L_V * mixing_ratio.0) / (C_P * temperature.0)));

        IsobaricEquivalentTemperature(ZERO_KELVIN + temp_ie)
    }
}

#[cfg(test)]
mod tests {

    use crate::tests::{test_with_2args, testing_traits::ReferenceAtmosphere, Argument};

    use super::*;

    #[test]
    fn definition1() {
        test_with_2args::<FormulaQuantity, DryBulbTemperature, MixingRatio, Definition1>(
            Argument::new([253., 324.]),
            Argument::new([0.000_000_1, 2.0]),
            ReferenceAtmosphere::Normal,
            1e-12,
        );
    }
}
