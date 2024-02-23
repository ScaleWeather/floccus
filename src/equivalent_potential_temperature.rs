//! Functions to calculate equivalent potential temperature of air
//!
//! Equivalent potential eemperature is a thermodynamic quantity, with its natural logarithm proportional
//! to the entropy of moist air, that is conserved in a reversible moist
//! adiabatic process ([AMETSOC Glossary](https://glossary.ametsoc.org/wiki/Equivalent_potential_temperature)).

use uom::si::available_energy::joule_per_kilogram;
use uom::si::pressure::pascal;
use uom::si::ratio::ratio;
use uom::si::specific_heat_capacity::joule_per_kilogram_kelvin;
use uom::si::thermodynamic_temperature::kelvin;

use crate::constants::{C_L, C_P, EPSILON, KAPPA, L_V, R_D, R_V};
use crate::errors::InputError;
use crate::formula::{Formula2, Formula3};
use crate::quantities::{
    AtmosphericPressure, DewPointTemperature, DryBulbTemperature, EquivalentPotentialTemperature,
    ThermodynamicQuantity, VapourPressure,
};
use crate::{
    mixing_ratio, potential_temperature, relative_humidity, saturation_vapour_pressure,
    vapour_pressure,
};

type FormulaQuantity = EquivalentPotentialTemperature;

/// Most accuarte formula for computing equivalent potential temperature of unsaturated air from
/// temperature, pressure and vapour pressure.
///
/// Implementation of this formula assumes no liquid or solid water in the air parcel.
///
/// First appeared in Paluch, Ilga (1979). J. Atmos. Sci., 36, 2467-2478
///
/// Provided in Emmanuel, Kerry (1994). Atmospheric Convection. Oxford University Press.
///
/// Valid `temperature` range: 253K - 324K
///
/// Valid `pressure` range: 100Pa - 150000Pa
///
/// Valid `vapour_pressure` range: 0Pa - 10000Pa
pub struct Paluch1;

impl Formula3<FormulaQuantity, DryBulbTemperature, AtmosphericPressure, VapourPressure>
    for Paluch1
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

        if !(20000.0..=150_000.0).contains(&pressure_si) {
            return Err(InputError::OutOfRange(String::from("pressure")));
        }

        if !(0.0..=10_000.0).contains(&vapour_pressure_si) {
            return Err(InputError::OutOfRange(String::from("vapour_pressure")));
        }

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(
        temperature: DryBulbTemperature,
        pressure: AtmosphericPressure,
        vapour_pressure: VapourPressure,
    ) -> EquivalentPotentialTemperature {
        let mixing_ratio = mixing_ratio::Definition1::compute_unchecked(pressure, vapour_pressure);
        let saturation_vapour_pressure =
            saturation_vapour_pressure::Buck1::compute_unchecked(temperature, pressure);

        let relative_humidity = relative_humidity::Definition2::compute_unchecked(
            vapour_pressure,
            saturation_vapour_pressure,
        );

        let temperature = temperature.0.get::<kelvin>();
        let pressure = pressure.0.get::<pascal>();
        let mixing_ratio = mixing_ratio.0.get::<ratio>();
        let relative_humidity = relative_humidity.0.get::<ratio>();

        let p0 = 100_000.0;
        let r_d = R_D.get::<joule_per_kilogram_kelvin>();
        let r_v = R_V.get::<joule_per_kilogram_kelvin>();
        let l_v = L_V.get::<joule_per_kilogram>();
        let c_p = C_P.get::<joule_per_kilogram_kelvin>();
        let c_l = C_L.get::<joule_per_kilogram_kelvin>();

        let result = temperature
            * (p0 / pressure).powf(r_d / (c_p + mixing_ratio * c_l))
            * relative_humidity.powf((-mixing_ratio * r_v) / (c_p + mixing_ratio * c_l))
            * ((l_v * mixing_ratio) / (temperature * (c_p + mixing_ratio * c_l))).exp();

        EquivalentPotentialTemperature::new::<kelvin>(result)
    }
}

/// Formula for computing equivalent potential temperature of unsaturated air from
/// temperature, pressure and vapour pressure.
///
/// Derived by G. H. Bryan (2008) [(doi:10.1175/2008MWR2593.1)](https://doi.org/10.1175/2008MWR2593.1)
///
/// Valid `temperature` range: 253K - 324K
///
/// Valid `pressure` range: 100Pa - 150000Pa
///
/// Valid `vapour_pressure` range: 0Pa - 10000Pa
pub struct Bryan1;

impl Formula3<FormulaQuantity, DryBulbTemperature, AtmosphericPressure, VapourPressure> for Bryan1 {
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

        if !(20000.0..=150_000.0).contains(&pressure_si) {
            return Err(InputError::OutOfRange(String::from("pressure")));
        }

        if !(0.0..=10_000.0).contains(&vapour_pressure_si) {
            return Err(InputError::OutOfRange(String::from("vapour_pressure")));
        }

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(
        temperature: DryBulbTemperature,
        pressure: AtmosphericPressure,
        vapour_pressure: VapourPressure,
    ) -> EquivalentPotentialTemperature {
        let potential_temperature = potential_temperature::Definition1::compute_unchecked(
            temperature,
            pressure,
            vapour_pressure,
        );

        let saturation_vapour_pressure =
            saturation_vapour_pressure::Buck3::compute_unchecked(temperature, pressure);

        let relative_humidity = relative_humidity::Definition2::compute_unchecked(
            vapour_pressure,
            saturation_vapour_pressure,
        );

        let mixing_ratio = mixing_ratio::Definition1::compute_unchecked(pressure, vapour_pressure);

        let temperature = temperature.0.get::<kelvin>();
        let mixing_ratio = mixing_ratio.0.get::<ratio>();
        let relative_humidity = relative_humidity.0.get::<ratio>();
        let potential_temperature = potential_temperature.0.get::<kelvin>();

        let kappa = KAPPA.get::<ratio>();
        let l_v = L_V.get::<joule_per_kilogram>();
        let c_p = C_P.get::<joule_per_kilogram_kelvin>();
        let epsilon = EPSILON.get::<ratio>();

        let result = potential_temperature
            * relative_humidity.powf((-kappa) * (mixing_ratio / epsilon))
            * ((l_v * mixing_ratio) / (c_p * temperature)).exp();

        EquivalentPotentialTemperature::new::<kelvin>(result)
    }
}

/// Approximate formula for computing equivalent potential temperature of unsaturated air from
/// temperature, pressure and dewpoint.
///
/// Derived by D. Bolton (1980)
/// [(doi:10.1175/1520-0493(1980)108<1046:TCOEPT>2.0.CO;2)](https://doi.org/10.1175/1520-0493(1980)108%3C1046:TCOEPT%3E2.0.CO;2)
///
/// Valid `pressure` range: 100Pa - 150000Pa
///
/// Valid `temperature` range: 253K - 324K
///
/// Valid `dewpoint` range: 253K - 324K
pub struct Bolton1;

impl Formula3<FormulaQuantity, AtmosphericPressure, DryBulbTemperature, DewPointTemperature>
    for Bolton1
{
    #[inline(always)]
    fn validate_inputs(
        pressure: AtmosphericPressure,
        temperature: DryBulbTemperature,
        dewpoint: DewPointTemperature,
    ) -> Result<(), InputError> {
        let pressure_si = pressure.get_si_value();
        let temperature_si = temperature.get_si_value();
        let dewpoint_si = dewpoint.get_si_value();

        if !(20000.0..=150_000.0).contains(&pressure_si) {
            return Err(InputError::OutOfRange(String::from("pressure")));
        }

        if !(253.0..=324.0).contains(&temperature_si) {
            return Err(InputError::OutOfRange(String::from("temperature")));
        }

        if !(253.0..=324.0).contains(&dewpoint_si) {
            return Err(InputError::OutOfRange(String::from("dewpoint")));
        }

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(
        pressure: AtmosphericPressure,
        temperature: DryBulbTemperature,
        dewpoint: DewPointTemperature,
    ) -> EquivalentPotentialTemperature {
        let vapour_pressure = vapour_pressure::Buck3::compute_unchecked(dewpoint, pressure);
        let mixing_ratio = mixing_ratio::Definition1::compute_unchecked(pressure, vapour_pressure);

        let pressure = pressure.0.get::<pascal>();
        let temperature = temperature.0.get::<kelvin>();
        let dewpoint = dewpoint.0.get::<kelvin>();
        let mixing_ratio = mixing_ratio.0.get::<ratio>();
        let vapour_pressure = vapour_pressure.0.get::<pascal>();

        let kappa = KAPPA.get::<ratio>();

        let lcl_temp =
            (1.0 / ((1.0 / (dewpoint - 56.0)) + ((temperature / dewpoint).ln() / 800.0))) + 56.0;

        let theta_dl = temperature
            * (100_000.0 / (pressure - vapour_pressure)).powf(kappa)
            * (temperature / lcl_temp).powf(0.28 * mixing_ratio);

        let result = theta_dl
            * (((3036.0 / lcl_temp) - 1.78) * mixing_ratio * (1.0 + 0.448 * mixing_ratio)).exp();

        EquivalentPotentialTemperature::new::<kelvin>(result)
    }
}

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use crate::{
        quantities::{AtmosphericPressure, DryBulbTemperature, VapourPressure},
        tests::{test_with_3args, Argument},
    };

    use super::*;

    #[test]
    fn paluch1() {
        test_with_3args::<
            FormulaQuantity,
            DryBulbTemperature,
            AtmosphericPressure,
            VapourPressure,
            Paluch1,
        >(
            Argument {
                name: "temperature",
                def_val: 300.0,
                range: [253.0, 324.0],
                _quantity: PhantomData,
            },
            Argument {
                name: "pressure",
                def_val: 101325.0,
                range: [20000.0, 150_000.0],
                _quantity: PhantomData,
            },
            Argument {
                name: "vapour_pressure",
                def_val: 991.189131,
                range: [0.0, 10_000.0],
                _quantity: PhantomData,
            },
            315.23724970376776,
        );
    }

    #[test]
    fn bryan1() {
        test_with_3args::<
            FormulaQuantity,
            DryBulbTemperature,
            AtmosphericPressure,
            VapourPressure,
            Bryan1,
        >(
            Argument {
                name: "temperature",
                def_val: 300.0,
                range: [253.0, 324.0],
                _quantity: PhantomData,
            },
            Argument {
                name: "pressure",
                def_val: 101325.0,
                range: [20000.0, 150_000.0],
                _quantity: PhantomData,
            },
            Argument {
                name: "vapour_pressure",
                def_val: 991.189131,
                range: [0.0, 10_000.0],
                _quantity: PhantomData,
            },
            316.52762026634014,
        );
    }

    #[test]
    fn bolton1() {
        test_with_3args::<
            FormulaQuantity,
            AtmosphericPressure,
            DryBulbTemperature,
            DewPointTemperature,
            Bolton1,
        >(
            Argument {
                name: "pressure",
                def_val: 101325.0,
                range: [20000.0, 150_000.0],
                _quantity: PhantomData,
            },
            Argument {
                name: "temperature",
                def_val: 300.0,
                range: [253.0, 324.0],
                _quantity: PhantomData,
            },
            Argument {
                name: "dewpoint",
                def_val: 280.0,
                range: [253.0, 324.0],
                _quantity: PhantomData,
            },
            317.3855211897774,
        );
    }
}
