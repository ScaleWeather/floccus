#![allow(clippy::needless_range_loop)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]

//! Formulae to calculate partial vapour pressure of the unsaturated air.

use crate::constants::DIMLESS_ONE;
use crate::formula::{Formula1, Formula2};
use crate::quantities::{
    AtmosphericPressure, DewPointTemperature, RelativeHumidity, SaturationVapourPressure,
    SpecificHumidity, ThermodynamicQuantity, VapourPressure,
};
use crate::Float;
use crate::Storage::Pressure;
use crate::{constants::EPSILON, errors::InputError};

use uom::si::pressure::{hectopascal, kilopascal, pascal};
use uom::si::thermodynamic_temperature::{degree_celsius, kelvin};

type FormulaQuantity = VapourPressure;

/// Formula for computing vapour pressure from specific humidity and pressure.
/// This function is theoretical not empirical.
///
/// Provided by [Rogers & Yau (1989)](https://www.elsevier.com/books/a-short-course-in-cloud-physics/yau/978-0-08-057094-5).
///
/// Valid `specific humidity` range: 0.00001 - 2.0
///
/// Valid `atmospheric pressure` range: 100Pa - 150000Pa
pub struct Definition1;

impl Formula2<FormulaQuantity, SpecificHumidity, AtmosphericPressure> for Definition1 {
    #[inline(always)]
    fn validate_inputs(
        specific_humidity: SpecificHumidity,
        pressure: AtmosphericPressure,
    ) -> Result<(), InputError> {
        specific_humidity.check_range_si(0.00001, 2.0)?;
        pressure.check_range_si(100.0, 150_000.0)?;

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(
        specific_humidity: SpecificHumidity,
        pressure: AtmosphericPressure,
    ) -> VapourPressure {
        let specific_humidity = specific_humidity.0;
        let pressure = pressure.0;

        let result = -((pressure * specific_humidity)
            / ((specific_humidity * (EPSILON - DIMLESS_ONE)) - EPSILON));

        VapourPressure(result)
    }
}

/// Formula for computing vapour pressure from saturation vapour pressure and relative humidity.
///
/// Valid `saturation_vapour_pressure` range: 0Pa - 50000Pa
///
/// Valid `relative_humidity` range: 0.0 - 2.0
pub struct Definition2;

impl Formula2<FormulaQuantity, SaturationVapourPressure, RelativeHumidity> for Definition2 {
    #[inline(always)]
    fn validate_inputs(
        saturation_vapour_pressure: SaturationVapourPressure,
        relative_humidity: RelativeHumidity,
    ) -> Result<(), InputError> {
        relative_humidity.check_range_si(0.0, 2.0)?;
        saturation_vapour_pressure.check_range_si(0.0, 50_000.0)?;

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(
        saturation_vapour_pressure: SaturationVapourPressure,
        relative_humidity: RelativeHumidity,
    ) -> VapourPressure {
        let result = saturation_vapour_pressure.0 * relative_humidity.0;

        VapourPressure(result)
    }
}

/// Formula for computing vapour pressure from dewpoint temperature and pressure.
/// Should be used for air over water when accuracy is desired.
///
/// Derived by A. L. Buck (1981) [(doi: 10.1175/1520-0450(1981)020<1527:nefcvp>2.0.co;2)](https://doi.org/10.1175/1520-0450(1981)020%3C1527:NEFCVP%3E2.0.CO;2).
///
/// Valid `dewpoint temperature` range: 232K - 324K
///
/// Valid `atmospheric pressure` range: 100Pa - 150000Pa
pub struct Buck1;

impl Formula2<FormulaQuantity, DewPointTemperature, AtmosphericPressure> for Buck1 {
    #[inline(always)]
    fn validate_inputs(
        dewpoint: DewPointTemperature,
        pressure: AtmosphericPressure,
    ) -> Result<(), InputError> {
        dewpoint.check_range_si(232.0, 324.0)?;
        pressure.check_range_si(100.0, 150_000.0)?;

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(
        dewpoint: DewPointTemperature,
        pressure: AtmosphericPressure,
    ) -> VapourPressure {
        let dewpoint = dewpoint.0.get::<degree_celsius>();
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

        VapourPressure(result)
    }
}

/// Formula for computing vapour pressure from dewpoint temperature and pressure.
/// Should be used for air over ice when accuracy is desired.
///
/// Derived by A. L. Buck (1981) [(doi: 10.1175/1520-0450(1981)020<1527:nefcvp>2.0.co;2)](https://doi.org/10.1175/1520-0450(1981)020%3C1527:NEFCVP%3E2.0.CO;2).
///
/// Valid `dewpoint` range: 193K - 274K
///
/// Valid `pressure` range: 100Pa - 150000Pa
pub struct Buck2;

impl Formula2<FormulaQuantity, DewPointTemperature, AtmosphericPressure> for Buck2 {
    #[inline(always)]
    fn validate_inputs(
        dewpoint: DewPointTemperature,
        pressure: AtmosphericPressure,
    ) -> Result<(), InputError> {
        dewpoint.check_range_si(193.0, 274.0)?;
        pressure.check_range_si(100.0, 150_000.0)?;

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(
        dewpoint: DewPointTemperature,
        pressure: AtmosphericPressure,
    ) -> VapourPressure {
        let dewpoint = dewpoint.0.get::<degree_celsius>();
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

        VapourPressure(result)
    }
}

/// Formula for computing vapour pressure from dewpoint temperature and pressure.
/// Should be used for air over water for general use.
///
/// Derived by A. L. Buck (1981) [(doi: 10.1175/1520-0450(1981)020<1527:nefcvp>2.0.co;2)](https://doi.org/10.1175/1520-0450(1981)020%3C1527:NEFCVP%3E2.0.CO;2).
///
/// Valid `dewpoint` range: 253K - 324K
///
/// Valid `pressure` range: 100Pa - 150000Pa
pub struct Buck3;

impl Formula2<FormulaQuantity, DewPointTemperature, AtmosphericPressure> for Buck3 {
    #[inline(always)]
    fn validate_inputs(
        dewpoint: DewPointTemperature,
        pressure: AtmosphericPressure,
    ) -> Result<(), InputError> {
        dewpoint.check_range_si(253.0, 324.0)?;
        pressure.check_range_si(100.0, 150_000.0)?;

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(
        dewpoint: DewPointTemperature,
        pressure: AtmosphericPressure,
    ) -> VapourPressure {
        let dewpoint = dewpoint.0.get::<degree_celsius>();
        let pressure = pressure.0.get::<hectopascal>();

        let lower_a = 6.1121;
        let lower_b = 17.502;
        let lower_c = 240.97;

        let upper_a = 0.000_7;
        let upper_b = 0.000_003_46;

        let lower_e = lower_a * ((lower_b * dewpoint) / (dewpoint + lower_c)).exp();
        let lower_f = 1.0 + upper_a + (pressure * upper_b);

        let result = Pressure::new::<hectopascal>(lower_e * lower_f);

        VapourPressure(result)
    }
}

/// Formula for computing vapour pressure from dewpoint temperature.
/// Simplified version of [`buck3`]. Very popular in meteorological sources.
///
/// Derived by A. L. Buck (1981) [(doi: 10.1175/1520-0450(1981)020<1527:nefcvp>2.0.co;2)](https://doi.org/10.1175/1520-0450(1981)020%3C1527:NEFCVP%3E2.0.CO;2).
///
/// Valid `dewpoint` range: 253K - 324K
pub struct Buck3Simplified;

impl Formula1<FormulaQuantity, DewPointTemperature> for Buck3Simplified {
    #[inline(always)]
    fn validate_inputs(dewpoint: DewPointTemperature) -> Result<(), InputError> {
        dewpoint.check_range_si(253.0, 324.0)?;

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(dewpoint: DewPointTemperature) -> VapourPressure {
        let dewpoint = dewpoint.0.get::<degree_celsius>();

        let lower_a = 6.1121;
        let lower_b = 17.502;
        let lower_c = 240.97;

        let lower_e = lower_a * ((lower_b * dewpoint) / (dewpoint + lower_c)).exp();

        let result = Pressure::new::<hectopascal>(lower_e);

        VapourPressure(result)
    }
}

/// Formula for computing vapour pressure from dewpoint temperature and pressure.
/// Should be used for air over ice for general use.
///
/// Derived by A. L. Buck (1981) [(doi: 10.1175/1520-0450(1981)020<1527:nefcvp>2.0.co;2)](https://doi.org/10.1175/1520-0450(1981)020%3C1527:NEFCVP%3E2.0.CO;2).
///
/// Valid `dewpoint` range: 223K - 274K
///
/// Valid `pressure` range: 100Pa - 150000Pa
pub struct Buck4;

impl Formula2<FormulaQuantity, DewPointTemperature, AtmosphericPressure> for Buck4 {
    #[inline(always)]
    fn validate_inputs(
        dewpoint: DewPointTemperature,
        pressure: AtmosphericPressure,
    ) -> Result<(), InputError> {
        dewpoint.check_range_si(223.0, 274.0)?;
        pressure.check_range_si(100.0, 150_000.0)?;

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(
        dewpoint: DewPointTemperature,
        pressure: AtmosphericPressure,
    ) -> VapourPressure {
        let dewpoint = dewpoint.0.get::<degree_celsius>();
        let pressure = pressure.0.get::<hectopascal>();

        let lower_a = 6.1115;
        let lower_b = 22.452;
        let lower_c = 272.55;

        let upper_a = 0.000_3;
        let upper_b = 0.000_004_18;

        let lower_e = lower_a * ((lower_b * dewpoint) / (dewpoint + lower_c)).exp();
        let lower_f = 1.0 + upper_a + (pressure * upper_b);

        let result = Pressure::new::<hectopascal>(lower_e * lower_f);

        VapourPressure(result)
    }
}

/// Formula for computing vapour pressure from dewpoint temperature.
/// Simplified version of [`buck4`], analogical to [`buck3_simplified`].
///
/// Derived by A. L. Buck (1981) [(doi: 10.1175/1520-0450(1981)020<1527:nefcvp>2.0.co;2)](https://doi.org/10.1175/1520-0450(1981)020%3C1527:NEFCVP%3E2.0.CO;2).
///
/// Valid `dewpoint` range: 223K - 274K
pub struct Buck4Simplified;

impl Formula1<FormulaQuantity, DewPointTemperature> for Buck4Simplified {
    #[inline(always)]
    fn validate_inputs(dewpoint: DewPointTemperature) -> Result<(), InputError> {
        dewpoint.check_range_si(223.0, 274.0)?;

        Ok(())
    }

    #[inline(always)]

    fn compute_unchecked(dewpoint: DewPointTemperature) -> VapourPressure {
        let dewpoint = dewpoint.0.get::<degree_celsius>();

        let lower_a = 6.1115;
        let lower_b = 22.452;
        let lower_c = 272.55;

        let lower_e = lower_a * ((lower_b * dewpoint) / (dewpoint + lower_c)).exp();

        let result = Pressure::new::<hectopascal>(lower_e);

        VapourPressure(result)
    }
}

/// Formula for computing vapour pressure over water from dewpoint temperature.
/// Should be used for temperatures above 273K.
///
/// Derived by O. Tetens (1930).
///
/// Valid `dewpoint` range: 273K - 353K
pub struct Tetens1;

impl Formula1<FormulaQuantity, DewPointTemperature> for Tetens1 {
    #[inline(always)]
    fn validate_inputs(dewpoint: DewPointTemperature) -> Result<(), InputError> {
        dewpoint.check_range_si(273.0, 353.0)?;

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(dewpoint: DewPointTemperature) -> VapourPressure {
        let dewpoint = dewpoint.0.get::<degree_celsius>();

        let lower_a = 0.61078;
        let lower_b = 17.27;
        let lower_c = 237.3;

        let result = lower_a * ((lower_b * dewpoint) / (dewpoint + lower_c)).exp();

        let result = Pressure::new::<kilopascal>(result);

        VapourPressure(result)
    }
}

/// Formula for computing vapour pressure over water from dewpoint temperature.
/// Should be used when accuracy is required as it is
/// computationally expensive.
///
/// Derived by A. Wexler (1976) [(doi: 10.6028/jres.080A.071)](https://dx.doi.org/10.6028%2Fjres.080A.071).
///
/// Valid `dewpoint` range: 273K - 374K
pub struct Wexler1;

impl Formula1<FormulaQuantity, DewPointTemperature> for Wexler1 {
    #[inline(always)]
    fn validate_inputs(dewpoint: DewPointTemperature) -> Result<(), InputError> {
        dewpoint.check_range_si(273.0, 374.0)?;

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(dewpoint: DewPointTemperature) -> VapourPressure {
        let dewpoint = dewpoint.get_si_value();

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

        VapourPressure(result)
    }
}

/// Formula for computing vapour over ice pressure from dewpoint temperature.
/// Should be used when accuracy is required as it is
/// computationally expensive.
///
/// Derived by A. Wexler (1977) [(doi: 10.6028/jres.081A.003)](https://dx.doi.org/10.6028%2Fjres.081A.003).
///
/// Valid `dewpoint` range: 173K - 274K
pub struct Wexler2;

impl Formula1<FormulaQuantity, DewPointTemperature> for Wexler2 {
    #[inline(always)]
    fn validate_inputs(dewpoint: DewPointTemperature) -> Result<(), InputError> {
        dewpoint.check_range_si(173.0, 274.0)?;

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(dewpoint: DewPointTemperature) -> VapourPressure {
        let dewpoint = dewpoint.0.get::<kelvin>();

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

        VapourPressure(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        quantities::{
            AtmosphericPressure, RelativeHumidity, SaturationVapourPressure, SpecificHumidity,
        },
        tests::{test_with_2args, testing_traits::ReferenceAtmosphere, Argument},
    };

    use super::*;

    #[test]
    fn definition1() {
        test_with_2args::<FormulaQuantity, SpecificHumidity, AtmosphericPressure, Definition1>(
            Argument::new([0.00001, 2.0]),
            Argument::new([100.0, 150_000.0]),
            ReferenceAtmosphere::Normal,
            0.00001,
        );
    }

    #[test]
    fn definition2() {
        test_with_2args::<FormulaQuantity, SaturationVapourPressure, RelativeHumidity, Definition2>(
            Argument::new([0.0, 50_000.0]),
            Argument::new([0.0, 2.0]),
            ReferenceAtmosphere::Normal,
            0.00001,
        );
    }

    // #[test]
    // fn buck1() {
    //     test_with_2args::<FormulaQuantity, DewPointTemperature, AtmosphericPressure, Buck1>(
    //         Argument::new([232.0, 324.0]),
    //         Argument::new([100.0, 150_000.0]),
    //         1927.0852679081806,
    //     );
    // }

    // #[test]
    // fn buck2() {
    //     test_with_2args::<FormulaQuantity, DewPointTemperature, AtmosphericPressure, Buck2>(
    //         Argument::new([193.0, 274.0]),
    //         Argument::new([100.0, 150_000.0]),
    //         76.38781790372722,
    //     );
    // }

    // #[test]
    // fn buck3() {
    //     test_with_2args::<FormulaQuantity, DewPointTemperature, AtmosphericPressure, Buck3>(
    //         Argument::new([253.0, 324.0]),
    //         Argument::new([100.0, 150_000.0]),
    //         3548.5041048035896,
    //     );
    // }

    // #[test]
    // fn buck4() {
    //     test_with_2args::<FormulaQuantity, DewPointTemperature, AtmosphericPressure, Buck4>(
    //         Argument::new([223.0, 274.0]),
    //         Argument::new([100.0, 150_000.0]),
    //         76.38685471836712,
    //     );
    // }

    // #[test]
    // fn buck3_simplified() {
    //     test_with_1arg::<FormulaQuantity, DewPointTemperature, Buck3Simplified>(
    //         Argument::new([253.0, 324.0]),
    //         3533.6421536199978,
    //     );
    // }

    // #[test]
    // fn buck4_simplified() {
    //     test_with_1arg::<FormulaQuantity, DewPointTemperature, Buck4Simplified>(
    //         Argument::new([223.0, 274.0]),
    //         76.04197508519536,
    //     );
    // }

    // #[test]
    // fn tetens1() {
    //     test_with_1arg::<FormulaQuantity, DewPointTemperature, Tetens1>(
    //         Argument::new([273.0, 353.0]),
    //         3533.969137160892,
    //     );
    // }

    // #[test]
    // fn wexler1() {
    //     test_with_1arg::<FormulaQuantity, DewPointTemperature, Wexler1>(
    //         Argument::new([273.0, 374.0]),
    //         3535.4235919263083,
    //     );
    // }

    // #[test]
    // fn wexler2() {
    //     test_with_1arg::<FormulaQuantity, DewPointTemperature, Wexler2>(
    //         Argument::new([173.0, 274.0]),
    //         76.04351136780438,
    //     );
    // }
}
