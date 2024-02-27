//! Functions to calculate relative humidity

use crate::errors::InputError;
use crate::Formula2;
use crate::quantities::{
    MixingRatio, RelativeHumidity, SaturationMixingRatio, SaturationVapourPressure,
    ThermodynamicQuantity, VapourPressure,
};

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
        mixing_ratio.check_range_si(0.00001, 10.0)?;
        saturation_mixing_ratio.check_range_si(0.00001, 10.0)?;

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
        vapour_pressure.check_range_si(0.0, 50_000.0)?;
        saturation_vapour_pressure.check_range_si(0.1, 50_000.0)?;

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

#[cfg(test)]
mod tests {
    use crate::tests::{test_with_2args, testing_traits::ReferenceAtmosphere, Argument};

    use super::*;

    #[test]
    fn definition1() {
        test_with_2args::<FormulaQuantity, MixingRatio, SaturationMixingRatio, Definition1>(
            Argument::new([0.00001, 10.0]),
            Argument::new([0.00001, 10.0]),
            ReferenceAtmosphere::Normal,
            1e-12,
        );
    }

    #[test]
    fn definition2() {
        test_with_2args::<FormulaQuantity, VapourPressure, SaturationVapourPressure, Definition2>(
            Argument::new([0.0, 50_000.0]),
            Argument::new([0.1, 50_000.0]),
            ReferenceAtmosphere::Normal,
            1e-12,
        );
    }
}
