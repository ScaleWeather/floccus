//! Functions to calculate mixing ratio of water vapour in unsaturated air

use crate::formula::{Formula1, Formula2};
use crate::quantities::{
    AtmosphericPressure, DewPointTemperature, MixingRatio, ThermodynamicQuantity, VapourPressure,
};
use crate::{constants::EPSILON, errors::InputError};
use crate::{vapour_pressure, Float};
use float_cmp::approx_eq;

/// Formula for computing mixing ratio of unsaturated air from air pressure and vapour pressure
///
/// Valid `pressure` range: 100Pa - 150000Pa
///
/// Valid `vapour_pressure` range: 0Pa - 10000Pa
///
/// Returns [`InputError::IncorrectArgumentSet`] when inputs are equal and division by 0 would occur.
pub struct Definition1;

impl Formula2<MixingRatio, AtmosphericPressure, VapourPressure> for Definition1 {
    #[inline(always)]
    fn validate_inputs(
        pressure: AtmosphericPressure,
        vapour_pressure: VapourPressure,
    ) -> Result<(), InputError> {
        let pressure_si = pressure.get_si_value();
        let vapour_pressure_si = vapour_pressure.get_si_value();

        if !(100.0..=150_000.0).contains(&pressure_si) {
            return Err(InputError::OutOfRange(String::from("pressure")));
        }

        if !(0.0..=50_000.0).contains(&vapour_pressure_si) {
            return Err(InputError::OutOfRange(String::from("vapour_pressure")));
        }

        if approx_eq!(Float, pressure_si, vapour_pressure_si, ulps = 2) {
            return Err(InputError::IncorrectArgumentSet(String::from(
                "pressure and vapour_pressure cannot be equal",
            )));
        }
        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(
        pressure: AtmosphericPressure,
        vapour_pressure: VapourPressure,
    ) -> MixingRatio {
        MixingRatio(EPSILON * (vapour_pressure.0 / (pressure.0 - vapour_pressure.0)))
    }
}

/// Formula for computing mixing ratio of unsaturated air from dewpoint temperature and pressure.
/// Optimised for performance - uses [`Tetens1`].
///
/// Valid `dewpoint` range: 273K - 353K
///
/// Valid `pressure` range: 100Pa - 150000Pa
pub struct Performance1;

impl Formula2<MixingRatio, DewPointTemperature, AtmosphericPressure> for Performance1 {
    #[inline(always)]
    fn validate_inputs(
        dewpoint: DewPointTemperature,
        pressure: AtmosphericPressure,
    ) -> Result<(), InputError> {
        let dewpoint_si = dewpoint.get_si_value();
        let pressure_si = pressure.get_si_value();

        if !(273.0..=353.0).contains(&dewpoint_si) {
            return Err(InputError::OutOfRange(String::from("dewpoint")));
        }

        if !(100.0..=150_000.0).contains(&pressure_si) {
            return Err(InputError::OutOfRange(String::from("pressure")));
        }

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(
        dewpoint: DewPointTemperature,
        pressure: AtmosphericPressure,
    ) -> MixingRatio {
        let vapour_pressure = vapour_pressure::Tetens1::compute_unchecked(dewpoint);

        Definition1::compute_unchecked(pressure, vapour_pressure)
    }
}

/// Formula for computing mixing ratio of unsaturated air from dewpoint temperature and pressure.
/// Optimised for accuracy - uses [`Buck1`].
///
/// Valid `dewpoint` range: 232K - 324K
///
/// Valid `pressure` range: 100Pa - 150000Pa
pub struct Accuracy1;

impl Formula2<MixingRatio, DewPointTemperature, AtmosphericPressure> for Accuracy1 {
    #[inline(always)]
    fn validate_inputs(
        dewpoint: DewPointTemperature,
        pressure: AtmosphericPressure,
    ) -> Result<(), InputError> {
        let dewpoint_si = dewpoint.get_si_value();
        let pressure_si = pressure.get_si_value();

        if !(232.0..=324.0).contains(&dewpoint_si) {
            return Err(InputError::OutOfRange(String::from("dewpoint")));
        }

        if !(100.0..=150_000.0).contains(&pressure_si) {
            return Err(InputError::OutOfRange(String::from("pressure")));
        }
        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(
        dewpoint: DewPointTemperature,
        pressure: AtmosphericPressure,
    ) -> MixingRatio {
        let vapour_pressure = vapour_pressure::Buck1::compute_unchecked(dewpoint, pressure);

        Definition1::compute_unchecked(pressure, vapour_pressure)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        quantities::MixingRatio,
        tests::{test_with_2args, Argument},
    };

    use super::*;

    #[test]
    fn general1() {
        test_with_2args::<MixingRatio, AtmosphericPressure, VapourPressure, Definition1>(
            Argument {
                name: "pressure",
                def_val: 101325.0,
                range: [100.0, 150_000.0],
            },
            Argument {
                name: "vapour_pressure",
                def_val: 3500.0,
                range: [0.0, 50_000.0],
            },
            0.022253316630823517,
        );
    }

    #[test]
    fn performance1() {
        test_with_2args::<MixingRatio, DewPointTemperature, AtmosphericPressure, Performance1>(
            Argument {
                name: "dewpoint",
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
        test_with_2args::<MixingRatio, DewPointTemperature, AtmosphericPressure, Accuracy1>(
            Argument {
                name: "dewpoint",
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
