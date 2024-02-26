//! Functions to calculate saturation mixing ratio of unsaturated air
//!  
//! Saturation mixing ration is the value of the mixing ratio of saturated air at the
//! given temperature and pressure ([AMETSOC Glossary](https://glossary.ametsoc.org/wiki/Saturation_mixing_ratio)).

use crate::formula::Formula2;
use crate::quantities::{
    AtmosphericPressure, MixingRatio, RelativeHumidity, SaturationMixingRatio,
    SaturationVapourPressure, ThermodynamicQuantity,
};
use crate::Float;
use crate::{constants::EPSILON, errors::InputError};
use float_cmp::approx_eq;

type FormulaQuantity = SaturationMixingRatio;

/// Formula for computing saturation mixing ratio of unsaturated air from air pressure and vapour pressure
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
        pressure.check_range_si(100.0, 150_000.0)?;
        saturation_vapour_pressure.check_range_si(0.0, 50_000.0)?;

        if saturation_vapour_pressure.0 > pressure.0 {
            return Err(InputError::OutOfRange(String::from(
                "saturation_vapour_pressure cannot be greater than pressure",
            )));
        }

        if approx_eq!(
            Float,
            pressure.get_si_value(),
            saturation_vapour_pressure.get_si_value(),
            ulps = 2
        ) {
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

/// Formula for computing saturation mixing ratio of unsaturated air from
/// mixing ratio and relative humidity.
///
/// Valid `mixing_ratio` range: 0.000_000_000_1 - 1.0
///
/// Valid `relative_humditity` range: 0.000_000_000_1 - 2.0
pub struct Definition2;

impl Formula2<FormulaQuantity, MixingRatio, RelativeHumidity> for Definition2 {
    #[inline(always)]
    fn validate_inputs(
        mixing_ratio: MixingRatio,
        relative_humidity: RelativeHumidity,
    ) -> Result<(), InputError> {
        mixing_ratio.check_range_si(0.000_000_000_1, 1.0)?;
        relative_humidity.check_range_si(0.000_000_000_1, 2.0)?;

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(
        mixing_ratio: MixingRatio,
        relative_humidity: RelativeHumidity,
    ) -> SaturationMixingRatio {
        SaturationMixingRatio(mixing_ratio.0 / relative_humidity.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::{test_with_2args, testing_traits::ReferenceAtmosphere, Argument};

    use super::*;

    #[test]
    fn definition1() {
        test_with_2args::<
            FormulaQuantity,
            AtmosphericPressure,
            SaturationVapourPressure,
            Definition1,
        >(
            Argument::new([100.0, 150_000.0]),
            Argument::new([0.0, 50_000.0]),
            ReferenceAtmosphere::Normal,
            1e-2,
        );
    }

    #[test]
    fn definition2() {
        test_with_2args::<FormulaQuantity, MixingRatio, RelativeHumidity, Definition2>(
            Argument::new([0.000_000_000_1, 1.0]),
            Argument::new([0.000_000_000_1, 2.0]),
            ReferenceAtmosphere::Normal,
            1e-12,
        );
    }
}
