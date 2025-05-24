//!Functions to calculate wet bulb potential temperature of unsaturated air in K.

use uom::si::ratio::ratio;
use uom::si::thermodynamic_temperature::{degree_celsius, kelvin};

use crate::constants::LAMBDA;
use crate::errors::InputError;
use crate::formulas::Formula1;
use crate::quantities::{
    EquivalentPotentialTemperature, QuantityHelpers, WetBulbPotentialTemperature,
};
use crate::Storage;

type FormulaQuantity = WetBulbPotentialTemperature;

/// Formula for computing wet bulb potential temperature from equivalent potential temperature.
///
/// Derived by R. Davies-Jones (2008) [(doi:10.1175/2007MWR2224.1)](https://doi.org/10.1175/2007MWR2224.1).
/// This formula is a part of three formuale covering a wide range of theta_e inputs.
/// The other two formulas have not been (yet) implemented.
///
/// Valid `equivalent_potential_temperature` range: 257K - 377K
pub struct DaviesJones1;

impl Formula1<FormulaQuantity, EquivalentPotentialTemperature> for DaviesJones1 {
    #[inline(always)]
    fn validate_inputs_internal(
        equivalent_potential_temperature: EquivalentPotentialTemperature,
    ) -> Result<(), InputError> {
        equivalent_potential_temperature.check_range_si(257.0, 377.0)?;

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(
        equivalent_potential_temperature: EquivalentPotentialTemperature,
    ) -> WetBulbPotentialTemperature {
        let lambda = LAMBDA.get::<ratio>();
        let equivalent_potential_temperature = equivalent_potential_temperature.0.get::<kelvin>();
        let result = 45.114 - 51.489 * (273.15 / equivalent_potential_temperature).powf(lambda);

        let result = Storage::ThermodynamicTemperature::new::<degree_celsius>(result);

        WetBulbPotentialTemperature(result)
    }
}

/// Formula for computing wet bulb potential temperature from equivalent potential temperature.
///
/// Derived by R. Davies-Jones (2008) [(doi:10.1175/2007MWR2224.1)](https://doi.org/10.1175/2007MWR2224.1).
/// This is a very accurate rational-function approximation of [DaviesJones1] and two other (unimplemented)
/// formulas.  
///
/// Valid `equivalent_potential_temperature` range: 174K - 377K
pub struct DaviesJones2;

impl Formula1<FormulaQuantity, EquivalentPotentialTemperature> for DaviesJones2 {
    #[inline(always)]
    fn validate_inputs_internal(
        equivalent_potential_temperature: EquivalentPotentialTemperature,
    ) -> Result<(), InputError> {
        equivalent_potential_temperature.check_range_si(174.0, 377.0)?;

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(
        equivalent_potential_temperature: EquivalentPotentialTemperature,
    ) -> WetBulbPotentialTemperature {
        let theta_e = equivalent_potential_temperature.get::<kelvin>();

        let x = theta_e / 273.15;
        let a0 = 7.101574;
        let a1 = -20.68208;
        let a2 = 16.11182;
        let a3 = 2.574631;
        let a4 = -5.205688;
        let b1 = -3.552497;
        let b2 = 3.781782;
        let b3 = -0.6899655;
        let b4 = -0.5929340;

        let exponent = (a0 + a1 * x + a2 * x.powi(2) + a3 * x.powi(3) + a4 * x.powi(4))
            / (1. + b1 * x + b2 * x.powi(2) + b3 * x.powi(3) + b4 * x.powi(4));
        let theta_w = theta_e - 273.15 - exponent.exp();

        WetBulbPotentialTemperature::new::<degree_celsius>(theta_w)
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
            1e-2,
        );
    }

    #[test]
    fn davies_jones2() {
        test_with_1arg::<FormulaQuantity, EquivalentPotentialTemperature, DaviesJones2>(
            Argument::new([174.0, 377.0]),
            ReferenceAtmosphere::Normal,
            1e-1,
        );
    }
}
