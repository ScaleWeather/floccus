//! Functions to calculate relative humidity

use crate::errors::InputError;
use crate::formula::Formula2;
use crate::quantities::{MixingRatio, RelativeHumidity};
use crate::{mixing_ratio, vapour_pressure, Float};

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
        mixing_ratio: Float,
        saturation_mixing_ratio: Float,
    ) -> Result<(), InputError> {
        if !(0.00001..=10.0).contains(&mixing_ratio) {
            return Err(InputError::OutOfRange(String::from("mixing_ratio")));
        }

        if !(0.00001..=10.0).contains(&saturation_mixing_ratio) {
            return Err(InputError::OutOfRange(String::from(
                "saturation_mixing_ratio",
            )));
        }

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(mixing_ratio: Float, saturation_mixing_ratio: Float) -> Float {
        mixing_ratio / saturation_mixing_ratio
    }
}

///Formula for computing relative humidity from vapour pressure and saturation vapour pressure.
///Can be used interchangeably with [`general1`].
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `vapour_pressure` range: 0Pa - 10000Pa
///Valid `saturation_vapour_pressure` range: 0Pa - 10000Pa
pub struct Definition2;

impl Definition2 {
    #[allow(missing_docs)]
    #[allow(clippy::missing_errors_doc)]
    #[inline(always)]

    pub fn validate_inputs(
        vapour_pressure: Float,
        saturation_vapour_pressure: Float,
    ) -> Result<(), InputError> {
        if !(0.0..=50_000.0).contains(&vapour_pressure) {
            return Err(InputError::OutOfRange(String::from("vapour_pressure")));
        }

        if !(0.1..=50_000.0).contains(&saturation_vapour_pressure) {
            return Err(InputError::OutOfRange(String::from(
                "saturation_vapour_pressure",
            )));
        }

        Ok(())
    }

    #[inline(always)]
    #[allow(missing_docs)]
    pub fn compute_unchecked(vapour_pressure: Float, saturation_vapour_pressure: Float) -> Float {
        vapour_pressure / saturation_vapour_pressure
    }
}

///Formula for computing relative humidity from temperature and dewpoint using [`tetens1`](vapour_pressure::tetens1)
///function for vapour pressure calculation
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `temperature` range: 273K - 353K
///Valid `dewpoint` range: 273K - 353K
pub struct General3;

impl General3 {
    #[allow(missing_docs)]
    #[allow(clippy::missing_errors_doc)]
    #[inline(always)]

    pub fn validate_inputs(temperature: Float, dewpoint: Float) -> Result<(), InputError> {
        if !(273.0..=353.0).contains(&temperature) {
            return Err(InputError::OutOfRange(String::from("temperature")));
        }

        if !(273.0..=353.0).contains(&dewpoint) {
            return Err(InputError::OutOfRange(String::from("dewpoint")));
        }

        Ok(())
    }

    #[inline(always)]
    #[allow(missing_docs)]
    pub fn compute_unchecked(temperature: Float, dewpoint: Float) -> Float {
        let vapour_pressure = vapour_pressure::Tetens1::compute_unchecked(dewpoint);
        let saturation_vapour_pressure = vapour_pressure::Tetens1::compute_unchecked(temperature);

        General2::compute_unchecked(vapour_pressure, saturation_vapour_pressure)
    }
}

///Formula for computing relative humidity from temperature, dewpoint and pressure using [`buck3`](vapour_pressure::buck3)
///function for vapour pressure calculation
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `temperature` range: 253K - 324K\
///Valid `dewpoint` range: 253K - 324K\
///Valid `pressure` range: 100Pa - 150000Pa
pub struct General4;

impl General4 {
    #[allow(missing_docs)]
    #[inline(always)]
    #[allow(clippy::missing_errors_doc)]

    pub fn validate_inputs(
        temperature: Float,
        dewpoint: Float,
        pressure: Float,
    ) -> Result<(), InputError> {
        if !(253.0..=324.0).contains(&temperature) {
            return Err(InputError::OutOfRange(String::from("temperature")));
        }

        if !(253.0..=324.0).contains(&dewpoint) {
            return Err(InputError::OutOfRange(String::from("dewpoint")));
        }

        if !(100.0..=150_000.0).contains(&pressure) {
            return Err(InputError::OutOfRange(String::from("pressure")));
        }

        Ok(())
    }

    #[inline(always)]
    #[allow(missing_docs)]
    pub fn compute_unchecked(temperature: Float, dewpoint: Float, pressure: Float) -> Float {
        let vapour_pressure = vapour_pressure::Buck3::compute_unchecked(dewpoint, pressure);
        let saturation_vapour_pressure =
            vapour_pressure::Buck3::compute_unchecked(temperature, pressure);

        General2::compute_unchecked(vapour_pressure, saturation_vapour_pressure)
    }
}

///Formula for computing relative humidity from temperature, dewpoint and pressure using [`accuracy1`](mixing_ratio::accuracy1)
///function for mixing ratio calculation
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when one of inputs is out of range.\
///Valid `temperature` range: 232K - 324K\
///Valid `dewpoint` range: 232K - 324K\
///Valid `pressure` range: 100Pa - 150000Pa
pub struct General5;

impl General5 {
    #[allow(missing_docs)]
    #[allow(clippy::missing_errors_doc)]
    #[inline(always)]

    pub fn validate_inputs(
        temperature: Float,
        dewpoint: Float,
        pressure: Float,
    ) -> Result<(), InputError> {
        if !(232.0..=314.0).contains(&temperature) {
            return Err(InputError::OutOfRange(String::from("temperature")));
        }

        if !(232.0..=314.0).contains(&dewpoint) {
            return Err(InputError::OutOfRange(String::from("dewpoint")));
        }

        if !(10000.0..=150_000.0).contains(&pressure) {
            return Err(InputError::OutOfRange(String::from("pressure")));
        }

        Ok(())
    }

    #[inline(always)]
    #[allow(missing_docs)]
    pub fn compute_unchecked(temperature: Float, dewpoint: Float, pressure: Float) -> Float {
        let mixing_ratio = mixing_ratio::Accuracy1::compute_unchecked(dewpoint, pressure);
        let saturation_mixing_ratio =
            mixing_ratio::Accuracy1::compute_unchecked(temperature, pressure);

        General1::compute_unchecked(mixing_ratio, saturation_mixing_ratio)
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::{
//         relative_humidity,
//         tests_framework::{self, Argument},
//     };

//     #[test]
//     fn general1() {
//         assert!(tests_framework::test_with_2args(
//             &relative_humidity::general1,
//             Argument {
//                 name: "mixing_ratio",
//                 def_val: 0.01064,
//                 range: [0.00001, 10.0]
//             },
//             Argument {
//                 name: "saturation_mixing_ratio",
//                 def_val: 0.01467,
//                 range: [0.00001, 10.0]
//             },
//             0.7252897068847989
//         ));
//     }

//     #[test]
//     fn general2() {
//         assert!(tests_framework::test_with_2args(
//             &relative_humidity::general2,
//             Argument {
//                 name: "vapour_pressure",
//                 def_val: 1706.0,
//                 range: [0.0, 50_000.0]
//             },
//             Argument {
//                 name: "saturation_vapour_pressure",
//                 def_val: 2339.0,
//                 range: [0.1, 50_000.0]
//             },
//             0.7293715262932877
//         ));
//     }

//     #[test]
//     fn general3() {
//         assert!(tests_framework::test_with_2args(
//             &relative_humidity::general3,
//             Argument {
//                 name: "temperature",
//                 def_val: 300.0,
//                 range: [273.0, 353.0]
//             },
//             Argument {
//                 name: "dewpoint",
//                 def_val: 290.0,
//                 range: [273.0, 353.0]
//             },
//             0.5431069897660531
//         ));
//     }

//     #[test]
//     fn general4() {
//         assert!(tests_framework::test_with_3args(
//             &relative_humidity::general4,
//             Argument {
//                 name: "temperature",
//                 def_val: 300.0,
//                 range: [253.0, 324.0]
//             },
//             Argument {
//                 name: "dewpoint",
//                 def_val: 290.0,
//                 range: [253.0, 324.0]
//             },
//             Argument {
//                 name: "pressure",
//                 def_val: 101325.0,
//                 range: [100.0, 150_000.0]
//             },
//             0.5429224562155812
//         ));
//     }

//     #[test]
//     fn general5() {
//         assert!(tests_framework::test_with_3args(
//             &relative_humidity::general5,
//             Argument {
//                 name: "temperature",
//                 def_val: 300.0,
//                 range: [232.0, 314.0]
//             },
//             Argument {
//                 name: "dewpoint",
//                 def_val: 290.0,
//                 range: [232.0, 314.0]
//             },
//             Argument {
//                 name: "pressure",
//                 def_val: 101325.0,
//                 range: [10000.0, 150_000.0]
//             },
//             0.5338747953552858
//         ));
//     }
// }
