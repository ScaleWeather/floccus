//! Functions to calculate virtual temperature of air
//!
//! In atmospheric thermodynamics, the virtual temperature of a moist air parcel is the temperature
//! at which a theoretical dry air parcel would have a total pressure and density equal
//! to the moist parcel of air ([Wikipedia](https://en.wikipedia.org/wiki/Virtual_temperature)).

use crate::constants::{DIMLESS_ONE, EPSILON, ZERO_KELVIN};
use crate::errors::InputError;
use crate::{Formula2, Formula3};
use crate::quantities::{
    AtmosphericPressure, DryBulbTemperature, MixingRatio, SpecificHumidity, ThermodynamicQuantity,
    VapourPressure, VirtualTemperature,
};

type FormulaQuantity = VirtualTemperature;

/// Formula for computing virtual temperature from temperature and mixing ratio.
///
/// Valid `temperature` range: 173K - 373K
///
/// Valid `mixing_ratio` range: 0.0000000001 - 0.5
pub struct Definition1;

impl Formula2<FormulaQuantity, DryBulbTemperature, MixingRatio> for Definition1 {
    #[inline(always)]
    fn validate_inputs(
        temperature: DryBulbTemperature,
        mixing_ratio: MixingRatio,
    ) -> Result<(), InputError> {
        temperature.check_range_si(173.0, 354.0)?;
        mixing_ratio.check_range_si(0.000_000_000_1, 0.5)?;

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(
        temperature: DryBulbTemperature,
        mixing_ratio: MixingRatio,
    ) -> VirtualTemperature {
        let result = temperature.0
            * ((mixing_ratio.0 + EPSILON) / (EPSILON * (DIMLESS_ONE + mixing_ratio.0)));

        // this is necessary because result is TemperatureInterval
        let result = ZERO_KELVIN + result;

        VirtualTemperature(result)
    }
}

/// Formula for computing virtual temperature from air temperature, pressure and vapour pressure.
///
/// Valid `temperature` range: 173K - 373K
///
/// Valid `pressure` range: 100Pa - 150000Pa
///
/// Valid `vapour_pressure` range: 0Pa - 10000Pa
pub struct Definition2;

impl Formula3<FormulaQuantity, DryBulbTemperature, AtmosphericPressure, VapourPressure>
    for Definition2
{
    #[inline(always)]
    fn validate_inputs(
        temperature: DryBulbTemperature,
        pressure: AtmosphericPressure,
        vapour_pressure: VapourPressure,
    ) -> Result<(), InputError> {
        temperature.check_range_si(173.0, 354.0)?;
        pressure.check_range_si(100.0, 150_000.0)?;
        vapour_pressure.check_range_si(0.0, 10_000.0)?;

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(
        temperature: DryBulbTemperature,
        pressure: AtmosphericPressure,
        vapour_pressure: VapourPressure,
    ) -> VirtualTemperature {
        let result = temperature.0
            / (DIMLESS_ONE - ((vapour_pressure.0 / pressure.0) * (DIMLESS_ONE - EPSILON)));
        let result = ZERO_KELVIN + result;

        VirtualTemperature(result)
    }
}

///Formula for computing virtual temperature from air temperature and specific humidity.
///
///Valid `temperature` range: 173K - 373K
///
///Valid `specific_humidity` range: 100Pa - 150000Pa
pub struct Definition3;

impl Formula2<FormulaQuantity, DryBulbTemperature, SpecificHumidity> for Definition3 {
    #[inline(always)]
    fn validate_inputs(
        temperature: DryBulbTemperature,
        specific_humidity: SpecificHumidity,
    ) -> Result<(), InputError> {
        temperature.check_range_si(173.0, 354.0)?;
        specific_humidity.check_range_si(0.000_000_001, 2.0)?;

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(
        temperature: DryBulbTemperature,
        specific_humidity: SpecificHumidity,
    ) -> VirtualTemperature {
        let result = temperature.0
            * (DIMLESS_ONE + (specific_humidity.0 * ((DIMLESS_ONE / EPSILON) - DIMLESS_ONE)));
        let result = ZERO_KELVIN + result;

        VirtualTemperature(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::{
        test_with_2args, test_with_3args, testing_traits::ReferenceAtmosphere, Argument,
    };

    use super::*;

    #[test]
    fn definition1() {
        test_with_2args::<FormulaQuantity, DryBulbTemperature, MixingRatio, Definition1>(
            Argument::new([173.0, 354.0]),
            Argument::new([0.000_000_000_1, 0.5]),
            ReferenceAtmosphere::Normal,
            1e-12,
        );
    }

    #[test]
    fn definition2() {
        test_with_3args::<
            FormulaQuantity,
            DryBulbTemperature,
            AtmosphericPressure,
            VapourPressure,
            Definition2,
        >(
            Argument::new([173.0, 354.0]),
            Argument::new([100.0, 150_000.0]),
            Argument::new([0.0, 10_000.0]),
            ReferenceAtmosphere::Normal,
            1e-12,
        );
    }

    #[test]
    fn definition3() {
        test_with_2args::<FormulaQuantity, DryBulbTemperature, SpecificHumidity, Definition3>(
            Argument::new([173.0, 354.0]),
            Argument::new([0.000000001, 2.0]),
            ReferenceAtmosphere::Normal,
            1e-12,
        );
    }
}
