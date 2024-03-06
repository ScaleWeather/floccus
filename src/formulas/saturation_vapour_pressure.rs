//! Formulae to calculate saturation vapour pressure of the unsaturated air
//!
//! The vapor pressure of a system, at a given temperature, for which the vapor
//! of a substance is in equilibrium with a plane surface of that substance's pure
//! liquid or solid phase; that is, the vapor pressure of a system that has attained
//! saturation but not supersaturation ([AMETSOC Glossary](https://glossary.ametsoc.org/wiki/Saturation_vapor_pressure)).

use crate::errors::InputError;
use crate::{Formula1, Formula2};
use crate::quantities::{
    AtmosphericPressure, DryBulbTemperature, RelativeHumidity, SaturationVapourPressure,
    ThermodynamicQuantity, VapourPressure,
};
use crate::Float;
use crate::Storage::Pressure;

use uom::si::pressure::{hectopascal, kilopascal, pascal};
use uom::si::thermodynamic_temperature::{degree_celsius, kelvin};

type FormulaQuantity = SaturationVapourPressure;

/// Formula for computing saturation vapour pressure from vapour pressure and relative humidity.
///
/// Valid `vapour_pressure` range: 0Pa - 50000Pa
///
/// Valid `relative_humidity` range: 0.00001 - 2.0
pub struct Definition1;

impl Formula2<FormulaQuantity, VapourPressure, RelativeHumidity> for Definition1 {
    #[inline(always)]
    fn validate_inputs(
        vapour_pressure: VapourPressure,
        relative_humidity: RelativeHumidity,
    ) -> Result<(), InputError> {
        vapour_pressure.check_range_si(0.0, 50_000.0)?;
        relative_humidity.check_range_si(0.00001, 2.0)?;

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(
        vapour_pressure: VapourPressure,
        relative_humidity: RelativeHumidity,
    ) -> SaturationVapourPressure {
        SaturationVapourPressure(vapour_pressure.0 / relative_humidity.0)
    }
}

/// Formula for saturation computing saturation vapour pressure from dewpoint temperature and pressure.
/// Should be used for air over water when accuracy is desired.
///
/// Derived by A. L. Buck (1981) [(doi: 10.1175/1520-0450(1981)020<1527:nefcvp>2.0.co;2)](https://doi.org/10.1175/1520-0450(1981)020%3C1527:NEFCVP%3E2.0.CO;2).
///
/// Valid `dewpoint temperature` range: 232K - 324K
///
/// Valid `atmospheric pressure` range: 100Pa - 150000Pa
pub struct Buck1;

impl Formula2<FormulaQuantity, DryBulbTemperature, AtmosphericPressure> for Buck1 {
    #[inline(always)]
    fn validate_inputs(
        temperature: DryBulbTemperature,
        pressure: AtmosphericPressure,
    ) -> Result<(), InputError> {
        temperature.check_range_si(232.0, 324.0)?;
        pressure.check_range_si(100.0, 150_000.0)?;

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(
        temperature: DryBulbTemperature,
        pressure: AtmosphericPressure,
    ) -> SaturationVapourPressure {
        let dewpoint = temperature.0.get::<degree_celsius>();
        let pressure = pressure.0.get::<hectopascal>();

        let lower_a = 6.1121;
        let lower_b = 18.729;
        let lower_c = 257.87;
        let lower_d = 227.3;

        let upper_a = 0.000_72;
        let upper_b = 0.000_003_2;
        let upper_c = 0.000_000_000_59;

        let lower_e =
            lower_a * (((lower_b - (dewpoint / lower_d)) * dewpoint) / (dewpoint + lower_c)).exp();
        let lower_f = 1.0 + upper_a + (pressure * (upper_b + (upper_c * dewpoint * dewpoint)));

        let result = Pressure::new::<hectopascal>(lower_e * lower_f);

        SaturationVapourPressure(result)
    }
}

/// Formula for computing saturation vapour pressure from dewpoint temperature and pressure.
/// Should be used for air over ice when accuracy is desired.
///
/// Derived by A. L. Buck (1981) [(doi: 10.1175/1520-0450(1981)020<1527:nefcvp>2.0.co;2)](https://doi.org/10.1175/1520-0450(1981)020%3C1527:NEFCVP%3E2.0.CO;2).
///
/// Valid `dewpoint` range: 193K - 274K
///
/// Valid `pressure` range: 100Pa - 150000Pa
pub struct Buck2;

impl Formula2<FormulaQuantity, DryBulbTemperature, AtmosphericPressure> for Buck2 {
    #[inline(always)]
    fn validate_inputs(
        temperature: DryBulbTemperature,
        pressure: AtmosphericPressure,
    ) -> Result<(), InputError> {
        temperature.check_range_si(193.0, 274.0)?;
        pressure.check_range_si(100.0, 150_000.0)?;

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(
        temperature: DryBulbTemperature,
        pressure: AtmosphericPressure,
    ) -> SaturationVapourPressure {
        let dewpoint = temperature.0.get::<degree_celsius>();
        let pressure = pressure.0.get::<hectopascal>();

        let lower_a = 6.1115;
        let lower_b = 23.036;
        let lower_c = 279.82;
        let lower_d = 333.7;

        let upper_a = 0.000_22;
        let upper_b = 0.000_003_83;
        let upper_c = 0.000_000_000_64;

        let lower_e =
            lower_a * (((lower_b - (dewpoint / lower_d)) * dewpoint) / (dewpoint + lower_c)).exp();
        let lower_f = 1.0 + upper_a + (pressure * (upper_b + (upper_c * dewpoint * dewpoint)));

        let result = Pressure::new::<hectopascal>(lower_e * lower_f);

        SaturationVapourPressure(result)
    }
}

/// Formula for computing saturation vapour pressure from dewpoint temperature and pressure.
/// Should be used for air over water for general use.
///
/// Derived by A. L. Buck (1981) [(doi: 10.1175/1520-0450(1981)020<1527:nefcvp>2.0.co;2)](https://doi.org/10.1175/1520-0450(1981)020%3C1527:NEFCVP%3E2.0.CO;2).
///
/// Valid `dewpoint` range: 253K - 324K
///
/// Valid `pressure` range: 100Pa - 150000Pa
pub struct Buck3;

impl Formula2<FormulaQuantity, DryBulbTemperature, AtmosphericPressure> for Buck3 {
    #[inline(always)]
    fn validate_inputs(
        temperature: DryBulbTemperature,
        pressure: AtmosphericPressure,
    ) -> Result<(), InputError> {
        temperature.check_range_si(253.0, 324.0)?;
        pressure.check_range_si(100.0, 150_000.0)?;

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(
        temperature: DryBulbTemperature,
        pressure: AtmosphericPressure,
    ) -> SaturationVapourPressure {
        let dewpoint = temperature.0.get::<degree_celsius>();
        let pressure = pressure.0.get::<hectopascal>();

        let lower_a = 6.1121;
        let lower_b = 17.502;
        let lower_c = 240.97;

        let upper_a = 0.000_7;
        let upper_b = 0.000_003_46;

        let lower_e = lower_a * ((lower_b * dewpoint) / (dewpoint + lower_c)).exp();
        let lower_f = 1.0 + upper_a + (pressure * upper_b);

        let result = Pressure::new::<hectopascal>(lower_e * lower_f);

        SaturationVapourPressure(result)
    }
}

/// Formula for computing saturation vapour pressure from dewpoint temperature.
/// Simplified version of [`buck3`]. Very popular in meteorological sources.
///
/// Derived by A. L. Buck (1981) [(doi: 10.1175/1520-0450(1981)020<1527:nefcvp>2.0.co;2)](https://doi.org/10.1175/1520-0450(1981)020%3C1527:NEFCVP%3E2.0.CO;2).
///
/// Valid `dewpoint` range: 253K - 324K
pub struct Buck3Simplified;

impl Formula1<FormulaQuantity, DryBulbTemperature> for Buck3Simplified {
    #[inline(always)]
    fn validate_inputs(temperature: DryBulbTemperature) -> Result<(), InputError> {
        temperature.check_range_si(253.0, 324.0)?;

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(temperature: DryBulbTemperature) -> SaturationVapourPressure {
        let dewpoint = temperature.0.get::<degree_celsius>();

        let lower_a = 6.1121;
        let lower_b = 17.502;
        let lower_c = 240.97;

        let lower_e = lower_a * ((lower_b * dewpoint) / (dewpoint + lower_c)).exp();

        let result = Pressure::new::<hectopascal>(lower_e);

        SaturationVapourPressure(result)
    }
}

/// Formula for computing vapour saturation pressure from dewpoint temperature and pressure.
/// Should be used for air over ice for general use.
///
/// Derived by A. L. Buck (1981) [(doi: 10.1175/1520-0450(1981)020<1527:nefcvp>2.0.co;2)](https://doi.org/10.1175/1520-0450(1981)020%3C1527:NEFCVP%3E2.0.CO;2).
///
/// Valid `dewpoint` range: 223K - 274K
///
/// Valid `pressure` range: 100Pa - 150000Pa
pub struct Buck4;

impl Formula2<FormulaQuantity, DryBulbTemperature, AtmosphericPressure> for Buck4 {
    #[inline(always)]
    fn validate_inputs(
        temperature: DryBulbTemperature,
        pressure: AtmosphericPressure,
    ) -> Result<(), InputError> {
        temperature.check_range_si(223.0, 274.0)?;
        pressure.check_range_si(100.0, 150_000.0)?;

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(
        temperature: DryBulbTemperature,
        pressure: AtmosphericPressure,
    ) -> SaturationVapourPressure {
        let dewpoint = temperature.0.get::<degree_celsius>();
        let pressure = pressure.0.get::<hectopascal>();

        let lower_a = 6.1115;
        let lower_b = 22.452;
        let lower_c = 272.55;

        let upper_a = 0.000_3;
        let upper_b = 0.000_004_18;

        let lower_e = lower_a * ((lower_b * dewpoint) / (dewpoint + lower_c)).exp();
        let lower_f = 1.0 + upper_a + (pressure * upper_b);

        let result = Pressure::new::<hectopascal>(lower_e * lower_f);

        SaturationVapourPressure(result)
    }
}

/// Formula for computing saturation vapour pressure from dewpoint temperature.
/// Simplified version of [`buck4`], analogical to [`buck3_simplified`].
///
/// Derived by A. L. Buck (1981) [(doi: 10.1175/1520-0450(1981)020<1527:nefcvp>2.0.co;2)](https://doi.org/10.1175/1520-0450(1981)020%3C1527:NEFCVP%3E2.0.CO;2).
///
/// Valid `dewpoint` range: 223K - 274K
pub struct Buck4Simplified;

impl Formula1<FormulaQuantity, DryBulbTemperature> for Buck4Simplified {
    #[inline(always)]
    fn validate_inputs(temperature: DryBulbTemperature) -> Result<(), InputError> {
        temperature.check_range_si(223.0, 274.0)?;

        Ok(())
    }

    #[inline(always)]

    fn compute_unchecked(temperature: DryBulbTemperature) -> SaturationVapourPressure {
        let dewpoint = temperature.0.get::<degree_celsius>();

        let lower_a = 6.1115;
        let lower_b = 22.452;
        let lower_c = 272.55;

        let lower_e = lower_a * ((lower_b * dewpoint) / (dewpoint + lower_c)).exp();

        let result = Pressure::new::<hectopascal>(lower_e);

        SaturationVapourPressure(result)
    }
}

/// Formula for computing saturation vapour pressure over water from dewpoint temperature.
/// Should be used for temperatures above 273K.
///
/// Derived by O. Tetens (1930).
///
/// Valid `dewpoint` range: 273K - 353K
pub struct Tetens1;

impl Formula1<FormulaQuantity, DryBulbTemperature> for Tetens1 {
    #[inline(always)]
    fn validate_inputs(temperature: DryBulbTemperature) -> Result<(), InputError> {
        temperature.check_range_si(273.0, 353.0)?;

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(temperature: DryBulbTemperature) -> SaturationVapourPressure {
        let dewpoint = temperature.0.get::<degree_celsius>();

        let lower_a = 0.61078;
        let lower_b = 17.27;
        let lower_c = 237.3;

        let result = lower_a * ((lower_b * dewpoint) / (dewpoint + lower_c)).exp();

        let result = Pressure::new::<kilopascal>(result);

        SaturationVapourPressure(result)
    }
}

/// Formula for computing saturation vapour pressure over water from dewpoint temperature.
/// Should be used when accuracy is required as it is
/// computationally expensive.
///
/// Derived by A. Wexler (1976) [(doi: 10.6028/jres.080A.071)](https://dx.doi.org/10.6028%2Fjres.080A.071).
///
/// Valid `dewpoint` range: 273K - 374K
pub struct Wexler1;

impl Formula1<FormulaQuantity, DryBulbTemperature> for Wexler1 {
    #[inline(always)]
    fn validate_inputs(temperature: DryBulbTemperature) -> Result<(), InputError> {
        temperature.check_range_si(273.0, 374.0)?;

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(temperature: DryBulbTemperature) -> SaturationVapourPressure {
        let dewpoint = temperature.get_si_value();

        // constants from the paper
        let g: [Float; 8] = [
            -2991.2729,
            -6017.0128,
            18.876_438_54,
            -0.028_354_721,
            0.000_017_838_3,
            -0.000_000_000_841_504_17,
            0.000_000_000_000_444_125_43,
            2.858_487,
        ];

        let mut ln_p = g[7] * dewpoint.ln();

        for i in 0..=6 {
            ln_p += g[i] * dewpoint.powi(i as i32 - 2);
        }

        let result = Pressure::new::<pascal>(ln_p.exp());

        SaturationVapourPressure(result)
    }
}

/// Formula for computing saturation vapour over ice pressure from dewpoint temperature.
/// Should be used when accuracy is required as it is
/// computationally expensive.
///
/// Derived by A. Wexler (1977) [(doi: 10.6028/jres.081A.003)](https://dx.doi.org/10.6028%2Fjres.081A.003).
///
/// Valid `dewpoint` range: 173K - 274K
pub struct Wexler2;

impl Formula1<FormulaQuantity, DryBulbTemperature> for Wexler2 {
    #[inline(always)]
    fn validate_inputs(temperature: DryBulbTemperature) -> Result<(), InputError> {
        temperature.check_range_si(173.0, 274.0)?;

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(temperature: DryBulbTemperature) -> SaturationVapourPressure {
        let dewpoint = temperature.0.get::<kelvin>();

        // constants from the paper
        let big_k: [Float; 6] = [
            -5865.3696,
            22.241_033,
            0.013_749_042,
            -0.000_034_031_77,
            0.000_000_026_967_687,
            0.691_865_1,
        ];

        let mut ln_p = big_k[5] * dewpoint.ln();

        for j in 0..=4 {
            ln_p += big_k[j] * dewpoint.powi(j as i32 - 1);
        }

        let result = Pressure::new::<pascal>(ln_p.exp());

        SaturationVapourPressure(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        quantities::{AtmosphericPressure, DryBulbTemperature, RelativeHumidity, VapourPressure},
        tests::{test_with_1arg, test_with_2args, testing_traits::ReferenceAtmosphere, Argument},
    };

    use super::*;

    #[test]
    fn definition1() {
        test_with_2args::<FormulaQuantity, VapourPressure, RelativeHumidity, Definition1>(
            Argument::new([0.0, 50_000.0]),
            Argument::new([0.00001, 2.0]),
            ReferenceAtmosphere::Normal,
            1e-12,
        );
    }

    #[test]
    fn buck1() {
        test_with_2args::<FormulaQuantity, DryBulbTemperature, AtmosphericPressure, Buck1>(
            Argument::new([232.0, 324.0]),
            Argument::new([100.0, 150_000.0]),
            ReferenceAtmosphere::Normal,
            1e2,
        );
    }

    #[test]
    fn buck2() {
        test_with_2args::<FormulaQuantity, DryBulbTemperature, AtmosphericPressure, Buck2>(
            Argument::new([193.0, 274.0]),
            Argument::new([100.0, 150_000.0]),
            ReferenceAtmosphere::Freezing,
            1e0,
        );
    }

    #[test]
    fn buck3() {
        test_with_2args::<FormulaQuantity, DryBulbTemperature, AtmosphericPressure, Buck3>(
            Argument::new([253.0, 324.0]),
            Argument::new([100.0, 150_000.0]),
            ReferenceAtmosphere::Normal,
            1e2,
        );
    }

    #[test]
    fn buck4() {
        test_with_2args::<FormulaQuantity, DryBulbTemperature, AtmosphericPressure, Buck4>(
            Argument::new([223.0, 274.0]),
            Argument::new([100.0, 150_000.0]),
            ReferenceAtmosphere::Freezing,
            1e0,
        );
    }

    #[test]
    fn buck3_simplified() {
        test_with_1arg::<FormulaQuantity, DryBulbTemperature, Buck3Simplified>(
            Argument::new([253.0, 324.0]),
            ReferenceAtmosphere::Normal,
            1e1,
        );
    }

    #[test]
    fn buck4_simplified() {
        test_with_1arg::<FormulaQuantity, DryBulbTemperature, Buck4Simplified>(
            Argument::new([223.0, 274.0]),
            ReferenceAtmosphere::Freezing,
            1e-1,
        );
    }

    #[test]
    fn tetens1() {
        test_with_1arg::<FormulaQuantity, DryBulbTemperature, Tetens1>(
            Argument::new([273.0, 353.0]),
            ReferenceAtmosphere::Normal,
            1e1,
        );
    }

    #[test]
    fn wexler1() {
        test_with_1arg::<FormulaQuantity, DryBulbTemperature, Wexler1>(
            Argument::new([273.0, 374.0]),
            ReferenceAtmosphere::Normal,
            1e-12,
        );
    }

    #[test]
    fn wexler2() {
        test_with_1arg::<FormulaQuantity, DryBulbTemperature, Wexler2>(
            Argument::new([173.0, 274.0]),
            ReferenceAtmosphere::Freezing,
            1e-12,
        );
    }
}
