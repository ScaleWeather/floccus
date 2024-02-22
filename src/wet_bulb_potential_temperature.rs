//!Functions to calculate wet bulb potential temperature of unsaturated air in K.

use uom::si::ratio::ratio;
use uom::si::thermodynamic_temperature::{degree_celsius, kelvin};

use crate::formula::Formula1;
use crate::quantities::{
    EquivalentPotentialTemperature, ThermodynamicQuantity, WetBulbPotentialTemperature,
};
use crate::Storage;
use crate::{
    constants::{C_P, R_D},
    errors::InputError,
};

/// Formula for computing wet bulb potential temperature from equivalent potential temperature.
///
/// Derived by R. Davies-Jones (2008) [(doi:10.1175/2007MWR2224.1)](https://doi.org/10.1175/2007MWR2224.1)
///
/// Valid `temperature` range: 257K - 377K
pub struct DaviesJones1;

impl Formula1<WetBulbPotentialTemperature, EquivalentPotentialTemperature> for DaviesJones1 {
    #[inline(always)]
    fn validate_inputs(
        equivalent_potential_temperature: EquivalentPotentialTemperature,
    ) -> Result<(), InputError> {
        let equivalent_potential_temperature_si = equivalent_potential_temperature.get_si_value();

        if !(257.0..=377.0).contains(&equivalent_potential_temperature_si) {
            return Err(InputError::OutOfRange(String::from(
                "equivalent_potential_temperature",
            )));
        }

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
        quantities::{EquivalentPotentialTemperature, WetBulbPotentialTemperature},
        tests::{test_with_1arg, Argument},
    };

    use super::DaviesJones1;

    #[test]
    fn davies_jones1() {
        test_with_1arg::<WetBulbPotentialTemperature, EquivalentPotentialTemperature, DaviesJones1>(
            Argument {
                name: "equivalent_potential_temperature",
                def_val: 300.0,
                range: [257.0, 377.0],
            },
            281.17941447108467,
        );
    }
}
