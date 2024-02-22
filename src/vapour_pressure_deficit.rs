//! Functions to calculate vapour pressure deficit
//!
//! Vapour-pressure deficit, is the difference (deficit) between
//! the amount of moisture in the air and how much moisture the air can hold
//! when it is saturated ([Wikipedia](https://en.wikipedia.org/wiki/Vapour-pressure_deficit)).

use crate::errors::InputError;
use crate::formula::{Formula2, Formula3};
use crate::quantities::{
    AtmosphericPressure, DewPointTemperature, DryBulbTemperature, RelativeHumidity,
    SaturationVapourPressure, ThermodynamicQuantity, VapourPressure, VapourPressureDeficit,
};
use crate::{saturation_vapour_pressure, vapour_pressure};

type FormulaQuantity = VapourPressureDeficit;

/// Formula for computing vapour pressure deficit from vapour pressure and saturation vapour pressure
///
/// Valid `vapour_pressure` range: 0Pa - 50000Pa
///
/// Valid `saturation_vapour_pressure` range: 0Pa - 50000Pa
pub struct Definition1;

impl Formula2<FormulaQuantity, VapourPressure, SaturationVapourPressure> for Definition1 {
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

        if !(0.0..=50_000.0).contains(&saturation_vapour_pressure_si) {
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
    ) -> VapourPressureDeficit {
        VapourPressureDeficit(saturation_vapour_pressure.0 - vapour_pressure.0)
    }
}

/// Formula for computing vapour pressure deficit from temperature, dewpoint and pressure
/// using [`buck3`](vapour_pressure::buck3) function for vapour pressure calculation
///
/// Valid `temperature` range: 253K - 324K
///
/// Valid `dewpoint` range: 253K - 324K
///
/// Valid `pressure` range: 100Pa - 150000Pa
pub struct General1;

impl Formula3<FormulaQuantity, DryBulbTemperature, DewPointTemperature, AtmosphericPressure>
    for General1
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
    ) -> VapourPressureDeficit {
        let vapour_pressure = vapour_pressure::Buck3::compute_unchecked(dewpoint, pressure);
        let saturation_vapour_pressure =
            saturation_vapour_pressure::Buck3::compute_unchecked(temperature, pressure);

        Definition1::compute_unchecked(vapour_pressure, saturation_vapour_pressure)
    }
}

/// Formula for computing vapour pressure deficit from temperature, relative humidity and pressure
/// using [`buck3`](vapour_pressure::buck3) function for vapour pressure calculation
///
/// Valid `temperature` range: 253K - 319K
///
/// Valid `relative_humidity` range: 0.05 - 2.0
///
/// Valid `pressure` range: 100Pa - 150000Pa
pub struct General2;

impl Formula3<FormulaQuantity, DryBulbTemperature, RelativeHumidity, AtmosphericPressure>
    for General2
{
    #[inline(always)]
    fn validate_inputs(
        temperature: DryBulbTemperature,
        relative_humidity: RelativeHumidity,
        pressure: AtmosphericPressure,
    ) -> Result<(), InputError> {
        let temperature_si = temperature.get_si_value();
        let relative_humidity_si = relative_humidity.get_si_value();
        let pressure_si = pressure.get_si_value();

        if !(253.0..=319.0).contains(&temperature_si) {
            return Err(InputError::OutOfRange(String::from("temperature")));
        }

        if !(0.05..=2.0).contains(&relative_humidity_si) {
            return Err(InputError::OutOfRange(String::from("relative_humidity")));
        }

        if !(10000.0..=150_000.0).contains(&pressure_si) {
            return Err(InputError::OutOfRange(String::from("pressure")));
        }

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(
        temperature: DryBulbTemperature,
        relative_humidity: RelativeHumidity,
        pressure: AtmosphericPressure,
    ) -> VapourPressureDeficit {
        let saturation_vapour_pressure =
            saturation_vapour_pressure::Buck3::compute_unchecked(temperature, pressure);
        let vapour_pressure = vapour_pressure::Definition2::compute_unchecked(
            saturation_vapour_pressure,
            relative_humidity,
        );

        Definition1::compute_unchecked(vapour_pressure, saturation_vapour_pressure)
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::{test_with_2args, test_with_3args, Argument};

    use super::*;

    #[test]
    fn definition1() {
        test_with_2args::<FormulaQuantity, VapourPressure, SaturationVapourPressure, Definition1>(
            Argument {
                name: "vapour_pressure",
                def_val: 3000.0,
                range: [0.0, 50_000.0],
            },
            Argument {
                name: "saturation_vapour_pressure",
                def_val: 3550.0,
                range: [0.0, 50_000.0],
            },
            550.0,
        );
    }

    #[test]
    fn general1() {
        test_with_3args::<
            FormulaQuantity,
            DryBulbTemperature,
            DewPointTemperature,
            AtmosphericPressure,
            General1,
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
            1621.9415403325527,
        );
    }

    #[test]
    fn general2() {
        test_with_3args::<
            FormulaQuantity,
            DryBulbTemperature,
            RelativeHumidity,
            AtmosphericPressure,
            General2,
        >(
            Argument {
                name: "temperature",
                def_val: 300.0,
                range: [253.0, 319.0],
            },
            Argument {
                name: "relative_humidity",
                def_val: 0.5,
                range: [0.05, 2.0],
            },
            Argument {
                name: "pressure",
                def_val: 101325.0,
                range: [10000.0, 150_000.0],
            },
            1774.2520524017948,
        );
    }
}
