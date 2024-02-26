//! Functions to calculate wet bulb temperature of unsaturated air

use uom::si::ratio::percent;
use uom::si::thermodynamic_temperature::degree_celsius;

use crate::errors::InputError;
use crate::formula::Formula2;
use crate::quantities::{
    DryBulbTemperature, RelativeHumidity, ThermodynamicQuantity, WetBulbTemperature,
};
use crate::Storage;

type FormulaQuantity = WetBulbTemperature;

/// Formula for computing wet bulb temperature pressure from dry bulb temperature and relative humidity.
///
/// Derived by R. Stull (2011) [(doi:10.1175/JAMC-D-11-0143.1)](https://doi.org/10.1175/JAMC-D-11-0143.1)
/// Created with use of gene-expression programming.
///
/// Result error is within -1K to +0.65K, with mean absolute error of 0.28K
///
/// Valid `temperature` range: 253K - 324K

/// Valid `relative_humidity` range: 0.05 - 0.99
pub struct Stull1;

impl Formula2<FormulaQuantity, DryBulbTemperature, RelativeHumidity> for Stull1 {
    #[inline(always)]
    fn validate_inputs(
        temperature: DryBulbTemperature,
        relative_humidity: RelativeHumidity,
    ) -> Result<(), InputError> {
        temperature.check_range_si(253.0, 324.0)?;
        relative_humidity.check_range_si(0.05, 0.99)?;

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(
        temperature: DryBulbTemperature,
        relative_humidity: RelativeHumidity,
    ) -> WetBulbTemperature {
        let temperature = temperature.0.get::<degree_celsius>();
        let relative_humidity = relative_humidity.0.get::<percent>();

        let result = (temperature * (0.151_977 * (relative_humidity + 8.313_659).sqrt()).atan())
            + (temperature + relative_humidity).atan()
            - (relative_humidity - 1.676_331).atan()
            + (0.003_918_38 * relative_humidity.powf(1.5) * (0.023_101 * relative_humidity).atan())
            - 4.686_035;

        let result = Storage::ThermodynamicTemperature::new::<degree_celsius>(result);

        WetBulbTemperature(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::{test_with_2args, testing_traits::ReferenceAtmosphere, Argument};

    use super::*;

    #[test]
    fn stull1() {
        test_with_2args::<FormulaQuantity, DryBulbTemperature, RelativeHumidity, Stull1>(
            Argument::new([253.0, 324.0]),
            Argument::new([0.05, 0.99]),
            ReferenceAtmosphere::Normal,
            1e-12,
        );
    }
}
