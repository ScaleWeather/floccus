#![allow(clippy::needless_range_loop)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]

//!Functions to calculate partial vapour pressure of the unsaturated air in Pa.
//!
//!To compute saturation vapour pressure input dry-bulb temperature in place of dewpoint temperature.

use crate::Float;
use crate::{
    constants::{EPSILON, ZERO_CELSIUS},
    errors::InputError,
};

#[cfg(feature = "debug")]
use floccus_proc::logerr;

///Formula for computing vapour pressure from specific humidity and pressure.
///This function is theoretical not empirical.
///
///Provided by [Rogers & Yau (1989)](https://www.elsevier.com/books/a-short-course-in-cloud-physics/yau/978-0-08-057094-5).
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `specific_humidity` range: 0.00001 - 2.0\
///Valid `pressure` range: 100Pa - 150000Pa
pub fn general1(specific_humidity: Float, pressure: Float) -> Result<Float, InputError> {
    general1_validate(specific_humidity, pressure)?;
    Ok(general1_unchecked(specific_humidity, pressure))
}

#[allow(missing_docs)]
#[allow(clippy::missing_errors_doc)]
#[cfg_attr(feature = "debug", logerr)]
pub fn general1_validate(specific_humidity: Float, pressure: Float) -> Result<(), InputError> {
    if !(0.00001..=2.0).contains(&specific_humidity) {
        return Err(InputError::OutOfRange(String::from("specific_humidity")));
    }

    if !(100.0..=150_000.0).contains(&pressure) {
        return Err(InputError::OutOfRange(String::from("pressure")));
    }

    Ok(())
}

#[allow(missing_docs)]
pub fn general1_unchecked(specific_humidity: Float, pressure: Float) -> Float {
    -((pressure * specific_humidity) / ((specific_humidity * (EPSILON - 1.0)) - EPSILON))
}

///Formula for computing vapour pressure from dewpoint temperature and pressure.
///Should be used for air over water when accuracy is desired.
///
///Derived by A. L. Buck (1981) [(doi: 10.1175/1520-0450(1981)020<1527:nefcvp>2.0.co;2)](https://doi.org/10.1175/1520-0450(1981)020%3C1527:NEFCVP%3E2.0.CO;2).
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `dewpoint` range: 232K - 324K\
///Valid `pressure` range: 100Pa - 150000Pa
pub fn buck1(dewpoint: Float, pressure: Float) -> Result<Float, InputError> {
    buck1_validate(dewpoint, pressure)?;
    Ok(buck1_unchecked(dewpoint, pressure))
}

#[allow(missing_docs)]
#[allow(clippy::missing_errors_doc)]
#[cfg_attr(feature = "debug", logerr)]
pub fn buck1_validate(dewpoint: Float, pressure: Float) -> Result<(), InputError> {
    if !(232.0..=324.0).contains(&dewpoint) {
        return Err(InputError::OutOfRange(String::from("dewpoint")));
    }

    if !(100.0..=150_000.0).contains(&pressure) {
        return Err(InputError::OutOfRange(String::from("pressure")));
    }

    Ok(())
}

#[allow(missing_docs)]
pub fn buck1_unchecked(dewpoint: Float, pressure: Float) -> Float {
    let dewpoint = dewpoint - ZERO_CELSIUS; //convert to C
    let pressure = pressure / 100.0; //convert to hPa

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

    (lower_e * lower_f) * 100.0 //return in Pa
}

///Formula for computing vapour pressure from dewpoint temperature and pressure.
///Should be used for air over ice when accuracy is desired.
///
///Derived by A. L. Buck (1981) [(doi: 10.1175/1520-0450(1981)020<1527:nefcvp>2.0.co;2)](https://doi.org/10.1175/1520-0450(1981)020%3C1527:NEFCVP%3E2.0.CO;2).
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `dewpoint` range: 193K - 274K\
///Valid `pressure` range: 100Pa - 150000Pa
pub fn buck2(dewpoint: Float, pressure: Float) -> Result<Float, InputError> {
    buck2_validate(dewpoint, pressure)?;
    Ok(buck2_unchecked(dewpoint, pressure))
}

#[allow(missing_docs)]
#[allow(clippy::missing_errors_doc)]
#[cfg_attr(feature = "debug", logerr)]
pub fn buck2_validate(dewpoint: Float, pressure: Float) -> Result<(), InputError> {
    if !(193.0..=274.0).contains(&dewpoint) {
        return Err(InputError::OutOfRange(String::from("dewpoint")));
    }

    if !(100.0..=150_000.0).contains(&pressure) {
        return Err(InputError::OutOfRange(String::from("pressure")));
    }

    Ok(())
}

#[allow(missing_docs)]
pub fn buck2_unchecked(dewpoint: Float, pressure: Float) -> Float {
    let dewpoint = dewpoint - ZERO_CELSIUS; //convert to C
    let pressure = pressure / 100.0; //convert to hPa

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

    (lower_e * lower_f) * 100.0 //return in Pa
}

///Formula for computing vapour pressure from dewpoint temperature and pressure.
///Should be used for air over water for general use.
///
///Derived by A. L. Buck (1981) [(doi: 10.1175/1520-0450(1981)020<1527:nefcvp>2.0.co;2)](https://doi.org/10.1175/1520-0450(1981)020%3C1527:NEFCVP%3E2.0.CO;2).
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `dewpoint` range: 253K - 324K\
///Valid `pressure` range: 100Pa - 150000Pa
pub fn buck3(dewpoint: Float, pressure: Float) -> Result<Float, InputError> {
    buck3_validate(dewpoint, pressure)?;
    Ok(buck3_unchecked(dewpoint, pressure))
}

#[allow(missing_docs)]
#[allow(clippy::missing_errors_doc)]
#[cfg_attr(feature = "debug", logerr)]
pub fn buck3_validate(dewpoint: Float, pressure: Float) -> Result<(), InputError> {
    if !(253.0..=324.0).contains(&dewpoint) {
        return Err(InputError::OutOfRange(String::from("dewpoint")));
    }

    if !(100.0..=150_000.0).contains(&pressure) {
        return Err(InputError::OutOfRange(String::from("pressure")));
    }

    Ok(())
}

#[allow(missing_docs)]
pub fn buck3_unchecked(dewpoint: Float, pressure: Float) -> Float {
    let dewpoint = dewpoint - ZERO_CELSIUS; //convert to C
    let pressure = pressure / 100.0; //convert to hPa

    let lower_a = 6.1121;
    let lower_b = 17.502;
    let lower_c = 240.97;

    let upper_a = 0.000_7;
    let upper_b = 0.000_003_46;

    let lower_e = lower_a * ((lower_b * dewpoint) / (dewpoint + lower_c)).exp();
    let lower_f = 1.0 + upper_a + (pressure * upper_b);

    (lower_e * lower_f) * 100.0 //return in Pa
}

///Formula for computing vapour pressure from dewpoint temperature.
///Simplified version of [`buck3`]. Very popular in meteorological sources.
///
///Derived by A. L. Buck (1981) [(doi: 10.1175/1520-0450(1981)020<1527:nefcvp>2.0.co;2)](https://doi.org/10.1175/1520-0450(1981)020%3C1527:NEFCVP%3E2.0.CO;2).
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `dewpoint` range: 253K - 324K
pub fn buck3_simplified(dewpoint: Float) -> Result<Float, InputError> {
    buck3_simplified_validate(dewpoint)?;
    Ok(buck3_simplified_unchecked(dewpoint))
}

#[allow(missing_docs)]
#[allow(clippy::missing_errors_doc)]
#[cfg_attr(feature = "debug", logerr)]
pub fn buck3_simplified_validate(dewpoint: Float) -> Result<(), InputError> {
    if !(253.0..=324.0).contains(&dewpoint) {
        return Err(InputError::OutOfRange(String::from("dewpoint")));
    }

    Ok(())
}

#[allow(missing_docs)]
pub fn buck3_simplified_unchecked(dewpoint: Float) -> Float {
    let dewpoint = dewpoint - ZERO_CELSIUS; //convert to C

    let lower_a = 6.1121;
    let lower_b = 17.502;
    let lower_c = 240.97;

    let lower_e = lower_a * ((lower_b * dewpoint) / (dewpoint + lower_c)).exp();

    lower_e * 100.0 //return in Pa
}

///Formula for computing vapour pressure from dewpoint temperature and pressure.
///Should be used for air over ice for general use.
///
///Derived by A. L. Buck (1981) [(doi: 10.1175/1520-0450(1981)020<1527:nefcvp>2.0.co;2)](https://doi.org/10.1175/1520-0450(1981)020%3C1527:NEFCVP%3E2.0.CO;2).
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `dewpoint` range: 223K - 274K\
///Valid `pressure` range: 100Pa - 150000Pa
pub fn buck4(dewpoint: Float, pressure: Float) -> Result<Float, InputError> {
    buck4_validate(dewpoint, pressure)?;
    Ok(buck4_unchecked(dewpoint, pressure))
}

#[allow(missing_docs)]
#[allow(clippy::missing_errors_doc)]
#[cfg_attr(feature = "debug", logerr)]
pub fn buck4_validate(dewpoint: Float, pressure: Float) -> Result<(), InputError> {
    if !(223.0..=274.0).contains(&dewpoint) {
        return Err(InputError::OutOfRange(String::from("dewpoint")));
    }

    if !(100.0..=150_000.0).contains(&pressure) {
        return Err(InputError::OutOfRange(String::from("pressure")));
    }

    Ok(())
}

#[allow(missing_docs)]
pub fn buck4_unchecked(dewpoint: Float, pressure: Float) -> Float {
    let dewpoint = dewpoint - ZERO_CELSIUS; //convert to C
    let pressure = pressure / 100.0; //convert to hPa

    let lower_a = 6.1115;
    let lower_b = 22.452;
    let lower_c = 272.55;

    let upper_a = 0.000_3;
    let upper_b = 0.000_004_18;

    let lower_e = lower_a * ((lower_b * dewpoint) / (dewpoint + lower_c)).exp();
    let lower_f = 1.0 + upper_a + (pressure * upper_b);

    (lower_e * lower_f) * 100.0 //return in Pa
}

///Formula for computing vapour pressure from dewpoint temperature.
///Simplified version of [`buck4`], analogical to [`buck3_simplified`].
///
///Derived by A. L. Buck (1981) [(doi: 10.1175/1520-0450(1981)020<1527:nefcvp>2.0.co;2)](https://doi.org/10.1175/1520-0450(1981)020%3C1527:NEFCVP%3E2.0.CO;2).
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `dewpoint` range: 223K - 274K
pub fn buck4_simplified(dewpoint: Float) -> Result<Float, InputError> {
    buck4_simplified_validate(dewpoint)?;
    Ok(buck4_simplified_unchecked(dewpoint))
}

#[allow(missing_docs)]
#[allow(clippy::missing_errors_doc)]
#[cfg_attr(feature = "debug", logerr)]
pub fn buck4_simplified_validate(dewpoint: Float) -> Result<(), InputError> {
    //validate inputs
    if !(223.0..=274.0).contains(&dewpoint) {
        return Err(InputError::OutOfRange(String::from("dewpoint")));
    }

    Ok(())
}

#[allow(missing_docs)]
pub fn buck4_simplified_unchecked(dewpoint: Float) -> Float {
    let dewpoint = dewpoint - ZERO_CELSIUS; //convert to C

    let lower_a = 6.1115;
    let lower_b = 22.452;
    let lower_c = 272.55;

    let lower_e = lower_a * ((lower_b * dewpoint) / (dewpoint + lower_c)).exp();

    lower_e * 100.0 //return in Pa
}

///Formula for computing vapour pressure over water from dewpoint temperature.
///Should be used for temperatures above 273K.
///
///Derived by O. Tetens (1930).
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when input is out of range.\
///Valid `dewpoint` range: 273K - 353K
pub fn tetens1(dewpoint: Float) -> Result<Float, InputError> {
    tetens1_validate(dewpoint)?;
    Ok(tetens1_unchecked(dewpoint))
}

#[allow(missing_docs)]
#[allow(clippy::missing_errors_doc)]
#[cfg_attr(feature = "debug", logerr)]
pub fn tetens1_validate(dewpoint: Float) -> Result<(), InputError> {
    if !(273.0..=353.0).contains(&dewpoint) {
        return Err(InputError::OutOfRange(String::from("dewpoint")));
    }

    Ok(())
}

#[allow(missing_docs)]
pub fn tetens1_unchecked(dewpoint: Float) -> Float {
    let dewpoint = dewpoint - ZERO_CELSIUS; //convert to C

    let lower_a = 0.61078;
    let lower_b = 17.27;
    let lower_c = 237.3;

    let result = lower_a * ((lower_b * dewpoint) / (dewpoint + lower_c)).exp();

    result * 1000.0 //return in Pa
}

///Formula for computing **ONLY** vapour pressure from saturation vapour pressure and relative humidity.
///For saturation vapour pressure use [`saturation_specific2`]
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when input is out of range.\
///Valid `saturation_vapour_pressure` range: 0Pa - 10000Pa\
///Valid `relative_humidity` range: 0.0 - 1.0
pub fn saturation_specific1(
    saturation_vapour_pressure: Float,
    relative_humidity: Float,
) -> Result<Float, InputError> {
    saturation_specific1_validate(saturation_vapour_pressure, relative_humidity)?;
    Ok(saturation_specific1_unchecked(
        saturation_vapour_pressure,
        relative_humidity,
    ))
}

#[allow(missing_docs)]
#[allow(clippy::missing_errors_doc)]
#[cfg_attr(feature = "debug", logerr)]
pub fn saturation_specific1_validate(
    saturation_vapour_pressure: Float,
    relative_humidity: Float,
) -> Result<(), InputError> {
    if !(0.0..=2.0).contains(&relative_humidity) {
        return Err(InputError::OutOfRange(String::from("relative_humidity")));
    }

    if !(0.0..=50_000.0).contains(&saturation_vapour_pressure) {
        return Err(InputError::OutOfRange(String::from(
            "saturation_vapour_pressure",
        )));
    }

    Ok(())
}

#[allow(missing_docs)]
pub fn saturation_specific1_unchecked(
    saturation_vapour_pressure: Float,
    relative_humidity: Float,
) -> Float {
    saturation_vapour_pressure * relative_humidity
}

///Formula for computing **ONLY** saturation vapour pressure from vapour pressure and relative humidity.
///For vapour pressure use [`saturation_specific1`]
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when input is out of range.\
///Valid `vapour_pressure` range: 0Pa - 10000Pa\
///Valid `relative_humidity` range: 0.00001 - 1.0
pub fn saturation_specific2(
    vapour_pressure: Float,
    relative_humidity: Float,
) -> Result<Float, InputError> {
    saturation_specific2_validate(vapour_pressure, relative_humidity)?;
    Ok(saturation_specific2_uchecked(
        vapour_pressure,
        relative_humidity,
    ))
}

#[allow(missing_docs)]
#[allow(clippy::missing_errors_doc)]
#[cfg_attr(feature = "debug", logerr)]
pub fn saturation_specific2_validate(
    vapour_pressure: Float,
    relative_humidity: Float,
) -> Result<(), InputError> {
    if !(0.00001..=2.0).contains(&relative_humidity) {
        return Err(InputError::OutOfRange(String::from("relative_humidity")));
    }

    if !(0.0..=10_000.0).contains(&vapour_pressure) {
        return Err(InputError::OutOfRange(String::from("vapour_pressure")));
    }

    Ok(())
}

#[allow(missing_docs)]
pub fn saturation_specific2_uchecked(vapour_pressure: Float, relative_humidity: Float) -> Float {
    vapour_pressure / relative_humidity
}

///Formula for computing vapour pressure over water from dewpoint temperature.
///Should be used when accuracy is required as it is
///computationally expensive.
///
///Derived by A. Wexler (1976) [(doi: 10.6028/jres.080A.071)](https://dx.doi.org/10.6028%2Fjres.080A.071).
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `dewpoint` range: 273K - 374K
pub fn wexler1(dewpoint: Float) -> Result<Float, InputError> {
    wexler1_validate(dewpoint)?;
    Ok(wexler1_unchecked(dewpoint))
}

#[allow(missing_docs)]
#[allow(clippy::missing_errors_doc)]
#[cfg_attr(feature = "debug", logerr)]
pub fn wexler1_validate(dewpoint: Float) -> Result<(), InputError> {
    if !(273.0..=374.0).contains(&dewpoint) {
        return Err(InputError::OutOfRange(String::from("dewpoint")));
    }

    Ok(())
}

#[allow(missing_docs)]
pub fn wexler1_unchecked(dewpoint: Float) -> Float {
    // constants from the paper
    let g: [Float; 8] = [
        -2991.2729,
        -6017.0128,
        18.87643854,
        -0.028354721,
        0.0000178383,
        -0.00000000084150417,
        0.00000000000044412543,
        2.858487,
    ];

    let mut ln_p = g[7] * dewpoint.ln();

    for i in 0..=6 {
        ln_p += g[i] * dewpoint.powi(i as i32 - 2);
    }

    ln_p.exp()
}

///Formula for computing vapour over ice pressure from dewpoint temperature.
///Should be used when accuracy is required as it is
///computationally expensive.
///
///Derived by A. Wexler (1977) [(doi: 10.6028/jres.081A.003)](https://dx.doi.org/10.6028%2Fjres.081A.003).
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `dewpoint` range: 173K - 274K
pub fn wexler2(dewpoint: Float) -> Result<Float, InputError> {
    wexler2_validate(dewpoint)?;
    Ok(wexler2_unchecked(dewpoint))
}

#[allow(missing_docs)]
#[allow(clippy::missing_errors_doc)]
#[cfg_attr(feature = "debug", logerr)]
pub fn wexler2_validate(dewpoint: Float) -> Result<(), InputError> {
    if !(173.0..=274.0).contains(&dewpoint) {
        return Err(InputError::OutOfRange(String::from("dewpoint")));
    }
    Ok(())
}

#[allow(missing_docs)]
pub fn wexler2_unchecked(dewpoint: Float) -> Float {
    // constants from the paper
    let big_k: [Float; 6] = [
        -5865.3696,
        22.241033,
        0.013749042,
        -0.00003403177,
        0.000000026967687,
        0.6918651,
    ];

    let mut ln_p = big_k[5] * dewpoint.ln();

    for j in 0..=4 {
        ln_p += big_k[j] * dewpoint.powi(j as i32 - 1);
    }

    ln_p.exp()
}

#[cfg(test)]
mod tests {
    use crate::{
        tests_framework::{self, Argument},
        vapour_pressure,
    };

    #[test]
    fn general1() {
        assert!(tests_framework::test_with_2args(
            &vapour_pressure::general1,
            Argument {
                name: "specific_humidity",
                def_val: 0.022,
                range: [0.00001, 2.0]
            },
            Argument {
                name: "pressure",
                def_val: 101325.0,
                range: [100.0, 150_000.0]
            },
            3536.6680935251343
        ));
    }

    #[test]
    fn buck1() {
        assert!(tests_framework::test_with_2args(
            &vapour_pressure::buck1,
            Argument {
                name: "dewpoint",
                def_val: 300.0,
                range: [232.0, 324.0]
            },
            Argument {
                name: "pressure",
                def_val: 101325.0,
                range: [100.0, 150_000.0]
            },
            3550.6603579471303
        ));
    }

    #[test]
    fn buck2() {
        assert!(tests_framework::test_with_2args(
            &vapour_pressure::buck2,
            Argument {
                name: "dewpoint",
                def_val: 250.0,
                range: [193.0, 274.0]
            },
            Argument {
                name: "pressure",
                def_val: 101325.0,
                range: [100.0, 150_000.0]
            },
            76.38781790372722
        ));
    }

    #[test]
    fn buck3() {
        assert!(tests_framework::test_with_2args(
            &vapour_pressure::buck3,
            Argument {
                name: "dewpoint",
                def_val: 300.0,
                range: [253.0, 324.0]
            },
            Argument {
                name: "pressure",
                def_val: 101325.0,
                range: [100.0, 150_000.0]
            },
            3548.5041048035896
        ));
    }

    #[test]
    fn buck4() {
        assert!(tests_framework::test_with_2args(
            &vapour_pressure::buck4,
            Argument {
                name: "dewpoint",
                def_val: 250.0,
                range: [223.0, 274.0]
            },
            Argument {
                name: "pressure",
                def_val: 101325.0,
                range: [100.0, 150_000.0]
            },
            76.38685471836712
        ));
    }

    #[test]
    fn buck3_simplified() {
        assert!(tests_framework::test_with_1arg(
            &vapour_pressure::buck3_simplified,
            Argument {
                name: "dewpoint",
                def_val: 300.0,
                range: [253.0, 324.0]
            },
            3533.6421536199978
        ));
    }

    #[test]
    fn buck4_simplified() {
        assert!(tests_framework::test_with_1arg(
            &vapour_pressure::buck4_simplified,
            Argument {
                name: "dewpoint",
                def_val: 250.0,
                range: [223.0, 274.0]
            },
            76.04197508519536
        ));
    }

    #[test]
    fn tetens1() {
        assert!(tests_framework::test_with_1arg(
            &vapour_pressure::tetens1,
            Argument {
                name: "dewpoint",
                def_val: 300.0,
                range: [273.0, 353.0]
            },
            3533.969137160892
        ));
    }

    #[test]
    fn saturation_specific1() {
        assert!(tests_framework::test_with_2args(
            &vapour_pressure::saturation_specific1,
            Argument {
                name: "saturation_vapour_pressure",
                def_val: 3550.0,
                range: [0.0, 50_000.0]
            },
            Argument {
                name: "relative_humidity",
                def_val: 0.5,
                range: [0.0, 2.0]
            },
            1775.0
        ));
    }

    #[test]
    fn saturation_specific2() {
        assert!(tests_framework::test_with_2args(
            &vapour_pressure::saturation_specific2,
            Argument {
                name: "vapour_pressure",
                def_val: 3000.0,
                range: [0.0, 10_000.0]
            },
            Argument {
                name: "relative_humidity",
                def_val: 0.5,
                range: [0.00001, 2.0]
            },
            6000.0
        ));
    }

    #[test]
    fn wexler1() {
        assert!(tests_framework::test_with_1arg(
            &vapour_pressure::wexler1,
            Argument {
                name: "dewpoint",
                def_val: 300.0,
                range: [273.0, 374.0]
            },
            3535.4235919263083
        ));
    }

    #[test]
    fn wexler2() {
        assert!(tests_framework::test_with_1arg(
            &vapour_pressure::wexler2,
            Argument {
                name: "dewpoint",
                def_val: 250.0,
                range: [173.0, 274.0]
            },
            76.04351136780438
        ));
    }
}
