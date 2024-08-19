//! Functions to calculate adiabatic potential temperature of unsaturated air
//!
//! Adiabatic equivalent temperature (also known as pseudoequivalent temperature) is a temperature that
//! an air parcel would have after undergoing the following process: dry-adiabatic expansion until saturated;
//! pseudoadiabatic expansion until all moisture is precipitated out; dry- adiabatic compression to the initial
//!  pressure. This is the equivalent temperature as read from a thermodynamic chart and is always greater than
//!  the isobaric equivalent temperature.
//! ([AMETSOC Glossary](https://glossary.ametsoc.org/wiki/Equivalent_temperature)).

use uom::si::pressure::pascal;
use uom::si::ratio::ratio;
use uom::si::thermodynamic_temperature::kelvin;

use crate::constants::{C_P, KAPPA, L_V, REF_PRES, ZERO_KELVIN};
use crate::errors::InputError;
use crate::formulas::Formula2;
use crate::quantities::{
    AdiabaticEquivalentTemperature, AtmosphericPressure, DryBulbTemperature,
    EquivalentPotentialTemperature, MixingRatio, QuantityHelpers,
};

type FormulaQuantity = AdiabaticEquivalentTemperature;

/// Formula for computing adiabatic equivalent temperature from dry-bulb temperature and mixing ratio
///
/// Valid `temperature` range: 253K - 324K
///
/// Valid `mixing_ratio` range: 0.000_000_1 - 2.0
pub struct Definition1;

impl Formula2<FormulaQuantity, DryBulbTemperature, MixingRatio> for Definition1 {
    #[inline(always)]
    fn validate_inputs(
        temperature: DryBulbTemperature,
        mixing_ratio: MixingRatio,
    ) -> Result<(), InputError> {
        temperature.check_range_si(253.0, 324.0)?;
        mixing_ratio.check_range_si(0.000_000_1, 2.0)?;

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(
        temperature: DryBulbTemperature,
        mixing_ratio: MixingRatio,
    ) -> FormulaQuantity {
        let temp_ae = temperature.0 * ((L_V * mixing_ratio.0) / (C_P * temperature.0)).exp();

        AdiabaticEquivalentTemperature(temp_ae + ZERO_KELVIN)
    }
}

/// Formula for computing adiabatic equivalent temperature from
/// atmospheric pressure and equivalent potential temperature.
///
/// Cited in [Davies-Jones (2008)](https://doi.org/10.1175/2007MWR2224.1)
///
/// Valid `equivalent_potential_temperature` range: 173K - 373K
///
/// Valid `pressure` range: 100Pa - 150000Pa
pub struct Definition2;

impl Formula2<FormulaQuantity, EquivalentPotentialTemperature, AtmosphericPressure>
    for Definition2
{
    #[inline(always)]
    fn validate_inputs(
        equivalent_potential_temperature: EquivalentPotentialTemperature,
        pressure: AtmosphericPressure,
    ) -> Result<(), InputError> {
        equivalent_potential_temperature.check_range_si(173., 373.)?;
        pressure.check_range_si(100., 150_000.)?;

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(
        equivalent_potential_temperature: EquivalentPotentialTemperature,
        pressure: AtmosphericPressure,
    ) -> FormulaQuantity {
        let p = pressure.get::<pascal>();
        let p0 = REF_PRES.get::<pascal>();
        let kappa = KAPPA.get::<ratio>();
        let theta_e = equivalent_potential_temperature.get::<kelvin>();

        let pi = (p / p0).powf(kappa);
        let temp_ae = theta_e * pi;

        AdiabaticEquivalentTemperature::new::<kelvin>(temp_ae)
    }
}

#[cfg(test)]
mod tests {

    use crate::tests::{test_with_2args, testing_traits::ReferenceAtmosphere, Argument};

    use super::*;

    #[test]
    fn definition1_norm() {
        test_with_2args::<FormulaQuantity, DryBulbTemperature, MixingRatio, Definition1>(
            Argument::new([253., 324.]),
            Argument::new([0.000_000_1, 2.0]),
            ReferenceAtmosphere::Normal,
            1e-12,
        );
    }

    #[test]
    fn definition2_norm() {
        test_with_2args::<
            FormulaQuantity,
            EquivalentPotentialTemperature,
            AtmosphericPressure,
            Definition2,
        >(
            Argument::new([173., 373.]),
            Argument::new([100., 150_000.]),
            ReferenceAtmosphere::Normal,
            1.,
        );
    }

    #[test]
    fn definition1_freez() {
        test_with_2args::<FormulaQuantity, DryBulbTemperature, MixingRatio, Definition1>(
            Argument::new([253., 324.]),
            Argument::new([0.000_000_1, 2.0]),
            ReferenceAtmosphere::Freezing,
            1e-12,
        );
    }

    #[test]
    fn definition2_freez() {
        test_with_2args::<
            FormulaQuantity,
            EquivalentPotentialTemperature,
            AtmosphericPressure,
            Definition2,
        >(
            Argument::new([173., 373.]),
            Argument::new([100., 150_000.]),
            ReferenceAtmosphere::Freezing,
            1e-1,
        );
    }
}
