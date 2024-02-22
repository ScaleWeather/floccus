//! Functions to calculate potential temperature of dry air
//!
//! The temperature that an unsaturated parcel of dry air would have if brought
//! adiabatically and reversibly from its initial state to a
//! standard pressure, p0 = 100 kPa ([AMETSOC Glossary](https://glossary.ametsoc.org/wiki/Potential_temperature)).

use crate::formula::Formula3;
use crate::quantities::{
    AtmosphericPressure, DryBulbTemperature, PotentialTemperature, ThermodynamicQuantity,
    VapourPressure,
};
use crate::{
    constants::{C_P, R_D},
    errors::InputError,
};
use crate::{Float, Storage};
use float_cmp::approx_eq;
use uom::si::pressure::pascal;
use uom::si::ratio::ratio;
use uom::si::thermodynamic_temperature::kelvin;

/// Formula for computing potential temperature of dry air from temperature, pressure and vapour pressure.
///
/// Provided in by R. Davies-Jones (2009) [(doi:10.1175/2009MWR2774.1)](https://doi.org/10.1175/2009MWR2774.1)
///
/// Valid `temperature` range: 253K - 324K
///
/// Valid `pressure` range: 100Pa - 150000Pa
///
/// Valid `vapour_pressure` range: 0Pa - 10000Pa
///
/// Returns [`InputError::IncorrectArgumentSet`] when `pressure` and `vapour_pressure` are equal,
/// in which case division by 0 occurs.
///
/// Returns [`InputError::IncorrectArgumentSet`] when `pressure` is lower than `vapour_pressure`,
/// in which case floating-point exponentation of negative number occurs.
pub struct Definition1;

impl Formula3<PotentialTemperature, DryBulbTemperature, AtmosphericPressure, VapourPressure>
    for Definition1
{
    #[inline(always)]
    fn validate_inputs(
        temperature: DryBulbTemperature,
        pressure: AtmosphericPressure,
        vapour_pressure: VapourPressure,
    ) -> Result<(), InputError> {
        let temperature_si = temperature.get_si_value();
        let pressure_si = pressure.get_si_value();
        let vapour_pressure_si = vapour_pressure.get_si_value();

        if !(253.0..=324.0).contains(&temperature_si) {
            return Err(InputError::OutOfRange(String::from("temperature")));
        }

        if !(100.0..=150_000.0).contains(&pressure_si) {
            return Err(InputError::OutOfRange(String::from("pressure")));
        }

        if !(0.0..=10_000.0).contains(&vapour_pressure_si) {
            return Err(InputError::OutOfRange(String::from("vapour_pressure")));
        }

        if approx_eq!(Float, pressure_si, vapour_pressure_si, ulps = 2) {
            return Err(InputError::IncorrectArgumentSet(String::from(
                "pressure and vapour_pressure cannot be equal",
            )));
        }

        if vapour_pressure_si > pressure_si {
            return Err(InputError::IncorrectArgumentSet(String::from(
                "vapour_pressure cannot be higher than pressure",
            )));
        }

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(
        temperature: DryBulbTemperature,
        pressure: AtmosphericPressure,
        vapour_pressure: VapourPressure,
    ) -> PotentialTemperature {
        let temperature = temperature.0.get::<kelvin>();
        let pressure = pressure.0.get::<pascal>();
        let vapour_pressure = vapour_pressure.0.get::<pascal>();

        let kappa = (R_D / C_P).get::<ratio>();
        let result = temperature * (100_000.0 / (pressure - vapour_pressure)).powf(kappa);

        let result = Storage::ThermodynamicTemperature::new::<kelvin>(result);

        PotentialTemperature(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::{test_with_3args, Argument};

    use super::*;
    #[test]
    fn definition1() {
        test_with_3args::<
            PotentialTemperature,
            DryBulbTemperature,
            AtmosphericPressure,
            VapourPressure,
            Definition1,
        >(
            Argument {
                name: "temperature",
                def_val: 300.0,
                range: [253.0, 324.0],
            },
            Argument {
                name: "pressure",
                def_val: 101325.0,
                range: [100.0, 150_000.0],
            },
            Argument {
                name: "vapour_pressure",
                def_val: 3000.0,
                range: [0.0, 10_000.0],
            },
            301.45136519081666,
        );
    }
}
