//! Functions to calculate vapour pressure deficit
//!
//! Vapour-pressure deficit, is the difference (deficit) between
//! the amount of moisture in the air and how much moisture the air can hold
//! when it is saturated ([Wikipedia](https://en.wikipedia.org/wiki/Vapour-pressure_deficit)).

use crate::errors::InputError;
use crate::formula::Formula2;
use crate::quantities::{
    SaturationVapourPressure, ThermodynamicQuantity, VapourPressure, VapourPressureDeficit,
};

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
        vapour_pressure.check_range_si(0.0, 50_000.0)?;
        saturation_vapour_pressure.check_range_si(0.0, 50_000.0)?;

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

#[cfg(test)]
mod tests {
    use crate::tests::{test_with_2args, testing_traits::ReferenceAtmosphere, Argument};

    use super::*;

    #[test]
    fn definition1() {
        test_with_2args::<FormulaQuantity, VapourPressure, SaturationVapourPressure, Definition1>(
            Argument::new([0.0, 50_000.0]),
            Argument::new([0.0, 50_000.0]),
            ReferenceAtmosphere::Normal,
            1e-12,
        );
    }
}
