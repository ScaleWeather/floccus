//! Functions to calculate virtual temperature of air
//!
//! In atmospheric thermodynamics, the virtual temperature of a moist air parcel is the temperature
//! at which a theoretical dry air parcel would have a total pressure and density equal
//! to the moist parcel of air ([Wikipedia](https://en.wikipedia.org/wiki/Virtual_temperature)).

use crate::constants::{DIMLESS_ONE, EPSILON, ZERO_KELVIN};
use crate::errors::InputError;
use crate::formula::{Formula2, Formula3};
use crate::quantities::{
    AtmosphericPressure, DryBulbTemperature, MixingRatio, SpecificHumidity, ThermodynamicQuantity,
    VapourPressure, VirtualTemperature,
};

/// Formula for computing virtual temperature from temperature and mixing ratio.
///
/// Valid `temperature` range: 173K - 373K
///
/// Valid `mixing_ratio` range: 0.0000000001 - 0.5
pub struct Definition1;

impl Formula2<VirtualTemperature, DryBulbTemperature, MixingRatio> for Definition1 {
    #[inline(always)]
    fn validate_inputs(
        temperature: DryBulbTemperature,
        mixing_ratio: MixingRatio,
    ) -> Result<(), InputError> {
        let temperature_si = temperature.get_si_value();
        let mixing_ratio_si = mixing_ratio.get_si_value();

        if !(173.0..=354.0).contains(&temperature_si) {
            return Err(InputError::OutOfRange(String::from("temperature")));
        }

        if !(0.000_000_000_1..=0.5).contains(&mixing_ratio_si) {
            return Err(InputError::OutOfRange(String::from("mixing_ratio")));
        }

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

impl Formula3<VirtualTemperature, DryBulbTemperature, AtmosphericPressure, VapourPressure>
    for Definition2
{
    #[inline(always)]
    fn validate_inputs(
        temperature: DryBulbTemperature,
        pressure: AtmosphericPressure,
        vapour_pressure: VapourPressure,
    ) -> Result<(), InputError> {
        let temperature_si = temperature.get_si_value();
        let pressure_si = pressure.get_si_value();
        let vapour_pressure_si = vapour_pressure.get_si_value();

        if !(173.0..=354.0).contains(&temperature_si) {
            return Err(InputError::OutOfRange(String::from("temperature")));
        }

        if !(100.0..=150_000.0).contains(&pressure_si) {
            return Err(InputError::OutOfRange(String::from("pressure")));
        }

        if !(0.0..=10_000.0).contains(&vapour_pressure_si) {
            return Err(InputError::OutOfRange(String::from("vapour_pressure")));
        }
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

impl Formula2<VirtualTemperature, DryBulbTemperature, SpecificHumidity> for Definition3 {
    #[inline(always)]
    fn validate_inputs(
        temperature: DryBulbTemperature,
        specific_humidity: SpecificHumidity,
    ) -> Result<(), InputError> {
        let temperature_si = temperature.get_si_value();
        let specific_humidity_si = specific_humidity.get_si_value();

        if !(173.0..=354.0).contains(&temperature_si) {
            return Err(InputError::OutOfRange(String::from("temperature")));
        }

        if !(0.000_000_001..=2.0).contains(&specific_humidity_si) {
            return Err(InputError::OutOfRange(String::from("specific_humidity")));
        }

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
    use crate::tests::{test_with_2args, test_with_3args, Argument};

    use super::*;

    #[test]
    fn definition1() {
        test_with_2args::<VirtualTemperature, DryBulbTemperature, MixingRatio, Definition1>(
            Argument {
                name: "temperature",
                def_val: 300.0,
                range: [173.0, 354.0],
            },
            Argument {
                name: "mixing_ratio",
                def_val: 0.022,
                range: [0.000_000_000_1, 0.5],
            },
            303.9249219815806,
        );
    }

    #[test]
    fn definition2() {
        test_with_3args::<
            VirtualTemperature,
            DryBulbTemperature,
            AtmosphericPressure,
            VapourPressure,
            Definition2,
        >(
            Argument {
                name: "temperature",
                def_val: 300.0,
                range: [173.0, 354.0],
            },
            Argument {
                name: "pressure",
                def_val: 101325.0,
                range: [100.0, 150_000.0],
            },
            Argument {
                name: "vapour_pressure",
                def_val: 3550.0,
                range: [0.0, 10_000.0],
            },
            304.0265941965307,
        );
    }

    #[test]
    fn definition3() {
        test_with_2args::<VirtualTemperature, DryBulbTemperature, SpecificHumidity, Definition3>(
            Argument {
                name: "temperature",
                def_val: 300.0,
                range: [173.0, 354.0],
            },
            Argument {
                name: "specific_humidity",
                def_val: 0.022,
                range: [0.000000001, 2.0],
            },
            304.0112702651753,
        );
    }
}
