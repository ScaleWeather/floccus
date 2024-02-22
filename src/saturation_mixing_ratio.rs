//! Functions to calculate saturation mixing ratio of unsaturated air
//!  
//! Saturation mixing ration is the value of the mixing ratio of saturated air at the
//! given temperature and pressure ([AMETSOC Glossary](https://glossary.ametsoc.org/wiki/Saturation_mixing_ratio)).

use crate::formula::{Formula1, Formula2};
use crate::quantities::{
    AtmosphericPressure, DryBulbTemperature, SaturationMixingRatio, SaturationVapourPressure,
    ThermodynamicQuantity,
};
use crate::{constants::EPSILON, errors::InputError};
use crate::{saturation_vapour_pressure, Float};
use float_cmp::approx_eq;

type FormulaQuantity = SaturationMixingRatio;

/// Formula for computing mixing ratio of unsaturated air from air pressure and vapour pressure
///
/// Valid `pressure` range: 100Pa - 150000Pa
///
/// Valid `saturation_vapour_pressure` range: 0Pa - 10000Pa
///
/// Returns [`InputError::IncorrectArgumentSet`] when inputs are equal and division by 0 would occur.
pub struct Definition1;

impl Formula2<FormulaQuantity, AtmosphericPressure, SaturationVapourPressure> for Definition1 {
    #[inline(always)]
    fn validate_inputs(
        pressure: AtmosphericPressure,
        saturation_vapour_pressure: SaturationVapourPressure,
    ) -> Result<(), InputError> {
        let pressure_si = pressure.get_si_value();
        let saturation_vapour_pressure_si = saturation_vapour_pressure.get_si_value();

        if !(100.0..=150_000.0).contains(&pressure_si) {
            return Err(InputError::OutOfRange(String::from("pressure")));
        }

        if !(0.0..=50_000.0).contains(&saturation_vapour_pressure_si) {
            return Err(InputError::OutOfRange(String::from(
                "saturation_vapour_pressure",
            )));
        }

        if approx_eq!(Float, pressure_si, saturation_vapour_pressure_si, ulps = 2) {
            return Err(InputError::IncorrectArgumentSet(String::from(
                "pressure and saturation_vapour_pressure cannot be equal",
            )));
        }
        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(
        pressure: AtmosphericPressure,
        saturation_vapour_pressure: SaturationVapourPressure,
    ) -> SaturationMixingRatio {
        SaturationMixingRatio(
            EPSILON * (saturation_vapour_pressure.0 / (pressure.0 - saturation_vapour_pressure.0)),
        )
    }
}

/// Formula for computing mixing ratio of unsaturated air from dewpoint temperature and pressure.
/// Optimised for performance - uses [`Tetens1`].
///
/// Valid `dewpoint` range: 273K - 353K
///
/// Valid `pressure` range: 100Pa - 150000Pa
pub struct Performance1;

impl Formula2<FormulaQuantity, DryBulbTemperature, AtmosphericPressure> for Performance1 {
    #[inline(always)]
    fn validate_inputs(
        temperature: DryBulbTemperature,
        pressure: AtmosphericPressure,
    ) -> Result<(), InputError> {
        let temperature_si = temperature.get_si_value();
        let pressure_si = pressure.get_si_value();

        if !(273.0..=353.0).contains(&temperature_si) {
            return Err(InputError::OutOfRange(String::from("temperature")));
        }

        if !(100.0..=150_000.0).contains(&pressure_si) {
            return Err(InputError::OutOfRange(String::from("pressure")));
        }

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(
        temperature: DryBulbTemperature,
        pressure: AtmosphericPressure,
    ) -> SaturationMixingRatio {
        let saturation_vapour_pressure =
            saturation_vapour_pressure::Tetens1::compute_unchecked(temperature);

        Definition1::compute_unchecked(pressure, saturation_vapour_pressure)
    }
}

/// Formula for computing mixing ratio of unsaturated air from dewpoint temperature and pressure.
/// Optimised for accuracy - uses [`Buck1`].
///
/// Valid `dewpoint` range: 232K - 324K
///
/// Valid `pressure` range: 100Pa - 150000Pa
pub struct Accuracy1;

impl Formula2<FormulaQuantity, DryBulbTemperature, AtmosphericPressure> for Accuracy1 {
    #[inline(always)]
    fn validate_inputs(
        temperature: DryBulbTemperature,
        pressure: AtmosphericPressure,
    ) -> Result<(), InputError> {
        let temperature_si = temperature.get_si_value();
        let pressure_si = pressure.get_si_value();

        if !(232.0..=324.0).contains(&temperature_si) {
            return Err(InputError::OutOfRange(String::from("temperature")));
        }

        if !(100.0..=150_000.0).contains(&pressure_si) {
            return Err(InputError::OutOfRange(String::from("pressure")));
        }
        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(
        temperature: DryBulbTemperature,
        pressure: AtmosphericPressure,
    ) -> SaturationMixingRatio {
        let saturation_vapour_pressure =
            saturation_vapour_pressure::Buck1::compute_unchecked(temperature, pressure);

        Definition1::compute_unchecked(pressure, saturation_vapour_pressure)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        quantities::DryBulbTemperature,
        tests::{test_with_2args, Argument},
    };

    use super::*;

    #[test]
    fn general1() {
        test_with_2args::<
            FormulaQuantity,
            AtmosphericPressure,
            SaturationVapourPressure,
            Definition1,
        >(
            Argument {
                name: "pressure",
                def_val: 101325.0,
                range: [100.0, 150_000.0],
            },
            Argument {
                name: "saturation_vapour_pressure",
                def_val: 3500.0,
                range: [0.0, 50_000.0],
            },
            0.022253316630823517,
        );
    }

    #[test]
    fn performance1() {
        test_with_2args::<FormulaQuantity, DryBulbTemperature, AtmosphericPressure, Performance1>(
            Argument {
                name: "temperature",
                def_val: 300.0,
                range: [273.0, 353.0],
            },
            Argument {
                name: "pressure",
                def_val: 101325.0,
                range: [100.0, 150_000.0],
            },
            0.022477100514593465,
        );
    }

    #[test]
    fn accuracy1() {
        test_with_2args::<FormulaQuantity, DryBulbTemperature, AtmosphericPressure, Accuracy1>(
            Argument {
                name: "temperature",
                def_val: 300.0,
                range: [232.0, 324.0],
            },
            Argument {
                name: "pressure",
                def_val: 101325.0,
                range: [100.0, 150_000.0],
            },
            0.022587116896465847,
        );
    }
}
