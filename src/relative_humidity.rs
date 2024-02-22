//! Functions to calculate relative humidity

use crate::errors::InputError;
use crate::formula::{Formula1, Formula2, Formula3};
use crate::quantities::{
    AtmosphericPressure, DewPointTemperature, DryBulbTemperature, MixingRatio, RelativeHumidity,
    SaturationMixingRatio, SaturationVapourPressure, ThermodynamicQuantity, VapourPressure,
};
use crate::{mixing_ratio, saturation_mixing_ratio, saturation_vapour_pressure, vapour_pressure};

type FormulaQuantity = RelativeHumidity;

/// Formula for computing relative humidity from mixing ratio and saturation mixing ratio.
/// Can be used interchangeably with [`general2`].
///
/// By the definition of mixing ratio, this formula is mathematically equivalent of
/// formula used in [`general2`].
///
/// Valid `mixing_ratio` range: 0.00001 - 10.0
///
/// Valid `saturation_mixing_ratio` range: 0.00001 - 10.0
pub struct Definition1;

impl Formula2<FormulaQuantity, MixingRatio, SaturationMixingRatio> for Definition1 {
    #[inline(always)]
    fn validate_inputs(
        mixing_ratio: MixingRatio,
        saturation_mixing_ratio: SaturationMixingRatio,
    ) -> Result<(), InputError> {
        let mixing_ratio_si = mixing_ratio.get_si_value();
        let saturation_mixing_ratio_si = saturation_mixing_ratio.get_si_value();

        if !(0.00001..=10.0).contains(&mixing_ratio_si) {
            return Err(InputError::OutOfRange(String::from("mixing_ratio")));
        }

        if !(0.00001..=10.0).contains(&saturation_mixing_ratio_si) {
            return Err(InputError::OutOfRange(String::from(
                "saturation_mixing_ratio",
            )));
        }

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(
        mixing_ratio: MixingRatio,
        saturation_mixing_ratio: SaturationMixingRatio,
    ) -> RelativeHumidity {
        RelativeHumidity(mixing_ratio.0 / saturation_mixing_ratio.0)
    }
}

/// Formula for computing relative humidity from vapour pressure and saturation vapour pressure.
/// Can be used interchangeably with [`general1`].
///
/// Valid `vapour_pressure` range: 0Pa - 50000Pa
///
/// Valid `saturation_vapour_pressure` range: 0Pa - 50000Pa
pub struct Definition2;

impl Formula2<FormulaQuantity, VapourPressure, SaturationVapourPressure> for Definition2 {
    #[inline(always)]
    fn validate_inputs(
        vapour_pressure: VapourPressure,
        saturation_vapour_pressure: SaturationVapourPressure,
    ) -> Result<(), InputError> {
        let vapour_pressure_si = vapour_pressure.get_si_value();
        let saturation_vapour_pressure_si = saturation_vapour_pressure.get_si_value();

        if !(0.0..=50_000.0).contains(&vapour_pressure_si) {
            return Err(InputError::OutOfRange(String::from("vapour_pressure")));
        }

        if !(0.1..=50_000.0).contains(&saturation_vapour_pressure_si) {
            return Err(InputError::OutOfRange(String::from(
                "saturation_vapour_pressure",
            )));
        }

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(
        vapour_pressure: VapourPressure,
        saturation_vapour_pressure: SaturationVapourPressure,
    ) -> RelativeHumidity {
        RelativeHumidity(vapour_pressure.0 / saturation_vapour_pressure.0)
    }
}

/// Formula for computing relative humidity from temperature and dewpoint using [`tetens1`](vapour_pressure::tetens1)
/// function for vapour pressure calculation
///
/// Valid `temperature` range: 273K - 353K
///
/// Valid `dewpoint` range: 273K - 353K
pub struct General1;

impl Formula2<FormulaQuantity, DryBulbTemperature, DewPointTemperature> for General1 {
    #[inline(always)]
    fn validate_inputs(
        temperature: DryBulbTemperature,
        dewpoint: DewPointTemperature,
    ) -> Result<(), InputError> {
        let temperature_si = temperature.get_si_value();
        let dewpoint_si = dewpoint.get_si_value();

        if !(273.0..=353.0).contains(&temperature_si) {
            return Err(InputError::OutOfRange(String::from("temperature")));
        }

        if !(273.0..=353.0).contains(&dewpoint_si) {
            return Err(InputError::OutOfRange(String::from("dewpoint")));
        }

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(
        temperature: DryBulbTemperature,
        dewpoint: DewPointTemperature,
    ) -> RelativeHumidity {
        let vapour_pressure = vapour_pressure::Tetens1::compute_unchecked(dewpoint);
        let saturation_vapour_pressure =
            saturation_vapour_pressure::Tetens1::compute_unchecked(temperature);

        Definition2::compute_unchecked(vapour_pressure, saturation_vapour_pressure)
    }
}

/// Formula for computing relative humidity from temperature, dewpoint and pressure using [`buck3`](vapour_pressure::buck3)
/// function for vapour pressure calculation
///
/// Valid `temperature` range: 253K - 324K
///
/// Valid `dewpoint` range: 253K - 324K
///
/// Valid `pressure` range: 100Pa - 150000Pa
pub struct General2;

impl Formula3<FormulaQuantity, DryBulbTemperature, DewPointTemperature, AtmosphericPressure>
    for General2
{
    #[inline(always)]
    fn validate_inputs(
        temperature: DryBulbTemperature,
        dewpoint: DewPointTemperature,
        pressure: AtmosphericPressure,
    ) -> Result<(), InputError> {
        let temperature_si = temperature.get_si_value();
        let dewpoint_si = dewpoint.get_si_value();
        let pressure_si = pressure.get_si_value();

        if !(253.0..=324.0).contains(&temperature_si) {
            return Err(InputError::OutOfRange(String::from("temperature")));
        }

        if !(253.0..=324.0).contains(&dewpoint_si) {
            return Err(InputError::OutOfRange(String::from("dewpoint")));
        }

        if !(100.0..=150_000.0).contains(&pressure_si) {
            return Err(InputError::OutOfRange(String::from("pressure")));
        }

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(
        temperature: DryBulbTemperature,
        dewpoint: DewPointTemperature,
        pressure: AtmosphericPressure,
    ) -> RelativeHumidity {
        let vapour_pressure = vapour_pressure::Buck3::compute_unchecked(dewpoint, pressure);
        let saturation_vapour_pressure =
            saturation_vapour_pressure::Buck3::compute_unchecked(temperature, pressure);

        Definition2::compute_unchecked(vapour_pressure, saturation_vapour_pressure)
    }
}

/// Formula for computing relative humidity from temperature, dewpoint and pressure using [`accuracy1`](mixing_ratio::accuracy1)
/// function for mixing ratio calculation
///
/// Valid `temperature` range: 232K - 324K
///
/// Valid `dewpoint` range: 232K - 324K
///
/// Valid `pressure` range: 100Pa - 150000Pa
pub struct General3;

impl Formula3<FormulaQuantity, DryBulbTemperature, DewPointTemperature, AtmosphericPressure>
    for General3
{
    #[inline(always)]
    fn validate_inputs(
        temperature: DryBulbTemperature,
        dewpoint: DewPointTemperature,
        pressure: AtmosphericPressure,
    ) -> Result<(), InputError> {
        let temperature_si = temperature.get_si_value();
        let dewpoint_si = dewpoint.get_si_value();
        let pressure_si = pressure.get_si_value();

        if !(232.0..=314.0).contains(&temperature_si) {
            return Err(InputError::OutOfRange(String::from("temperature")));
        }

        if !(232.0..=314.0).contains(&dewpoint_si) {
            return Err(InputError::OutOfRange(String::from("dewpoint")));
        }

        if !(10000.0..=150_000.0).contains(&pressure_si) {
            return Err(InputError::OutOfRange(String::from("pressure")));
        }

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(
        temperature: DryBulbTemperature,
        dewpoint: DewPointTemperature,
        pressure: AtmosphericPressure,
    ) -> FormulaQuantity {
        let mixing_ratio = mixing_ratio::Accuracy1::compute_unchecked(dewpoint, pressure);
        let saturation_mixing_ratio =
            saturation_mixing_ratio::Accuracy1::compute_unchecked(temperature, pressure);

        Definition1::compute_unchecked(mixing_ratio, saturation_mixing_ratio)
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::{test_with_2args, test_with_3args, Argument};

    use super::*;

    #[test]
    fn definition1() {
        test_with_2args::<FormulaQuantity, MixingRatio, SaturationMixingRatio, Definition1>(
            Argument {
                name: "mixing_ratio",
                def_val: 0.01064,
                range: [0.00001, 10.0],
            },
            Argument {
                name: "saturation_mixing_ratio",
                def_val: 0.01467,
                range: [0.00001, 10.0],
            },
            0.7252897068847989,
        );
    }

    #[test]
    fn definition2() {
        test_with_2args::<FormulaQuantity, VapourPressure, SaturationVapourPressure, Definition2>(
            Argument {
                name: "vapour_pressure",
                def_val: 1706.0,
                range: [0.0, 50_000.0],
            },
            Argument {
                name: "saturation_vapour_pressure",
                def_val: 2339.0,
                range: [0.1, 50_000.0],
            },
            0.7293715262932877,
        );
    }

    #[test]
    fn general1() {
        test_with_2args::<FormulaQuantity, DryBulbTemperature, DewPointTemperature, General1>(
            Argument {
                name: "temperature",
                def_val: 300.0,
                range: [273.0, 353.0],
            },
            Argument {
                name: "dewpoint",
                def_val: 290.0,
                range: [273.0, 353.0],
            },
            0.5431069897660531,
        );
    }

    #[test]
    fn general2() {
        test_with_3args::<
            FormulaQuantity,
            DryBulbTemperature,
            DewPointTemperature,
            AtmosphericPressure,
            General2,
        >(
            Argument {
                name: "temperature",
                def_val: 300.0,
                range: [253.0, 324.0],
            },
            Argument {
                name: "dewpoint",
                def_val: 290.0,
                range: [253.0, 324.0],
            },
            Argument {
                name: "pressure",
                def_val: 101325.0,
                range: [100.0, 150_000.0],
            },
            0.5429224562155812,
        );
    }

    #[test]
    fn general3() {
        test_with_3args::<
            FormulaQuantity,
            DryBulbTemperature,
            DewPointTemperature,
            AtmosphericPressure,
            General3,
        >(
            Argument {
                name: "temperature",
                def_val: 300.0,
                range: [232.0, 314.0],
            },
            Argument {
                name: "dewpoint",
                def_val: 290.0,
                range: [232.0, 314.0],
            },
            Argument {
                name: "pressure",
                def_val: 101325.0,
                range: [10000.0, 150_000.0],
            },
            0.5338747953552858,
        );
    }
}
