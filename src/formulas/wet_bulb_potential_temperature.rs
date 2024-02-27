//!Functions to calculate wet bulb potential temperature of unsaturated air in K.

use uom::si::ratio::ratio;
use uom::si::thermodynamic_temperature::{degree_celsius, kelvin};

use crate::Formula1;
use crate::quantities::{
    EquivalentPotentialTemperature, ThermodynamicQuantity, WetBulbPotentialTemperature,
};
use crate::Storage;
use crate::{
    constants::{C_P, R_D},
    errors::InputError,
};

type FormulaQuantity = WetBulbPotentialTemperature;

/// Formula for computing wet bulb potential temperature from equivalent potential temperature.
///
/// Derived by R. Davies-Jones (2008) [(doi:10.1175/2007MWR2224.1)](https://doi.org/10.1175/2007MWR2224.1)
///
/// Valid `temperature` range: 257K - 377K
pub struct DaviesJones1;

impl Formula1<FormulaQuantity, EquivalentPotentialTemperature> for DaviesJones1 {
    #[inline(always)]
    fn validate_inputs(
        equivalent_potential_temperature: EquivalentPotentialTemperature,
    ) -> Result<(), InputError> {
        equivalent_potential_temperature.check_range_si(257.0, 377.0)?;

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(
        equivalent_potential_temperature: EquivalentPotentialTemperature,
    ) -> WetBulbPotentialTemperature {
        let lambda = (C_P / R_D).get::<ratio>();
        let equivalent_potential_temperature = equivalent_potential_temperature.0.get::<kelvin>();
        let result = 45.114 - 51.489 * (273.15 / equivalent_potential_temperature).powf(lambda);

        let result = Storage::ThermodynamicTemperature::new::<degree_celsius>(result);

        WetBulbPotentialTemperature(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        quantities::EquivalentPotentialTemperature,
        tests::{test_with_1arg, testing_traits::ReferenceAtmosphere, Argument},
    };

    use super::*;

    #[test]
    fn davies_jones1() {
        test_with_1arg::<FormulaQuantity, EquivalentPotentialTemperature, DaviesJones1>(
            Argument::new([257.0, 377.0]),
            ReferenceAtmosphere::Normal,
            1e-12,
        );
    }
}
