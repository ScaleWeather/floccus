//! Functions to calculate equivalent potential temperature of air
//!
//! Equivalent potential eemperature is a thermodynamic quantity, with its natural logarithm proportional
//! to the entropy of moist air, that is conserved in a reversible moist
//! adiabatic process ([AMETSOC Glossary](https://glossary.ametsoc.org/wiki/Equivalent_potential_temperature)).

use float_cmp::approx_eq;
use uom::si::available_energy::joule_per_kilogram;
use uom::si::pressure::pascal;
use uom::si::ratio::ratio;
use uom::si::specific_heat_capacity::joule_per_kilogram_kelvin;
use uom::si::thermodynamic_temperature::kelvin;

use crate::constants::{C_L, C_P, EPSILON, KAPPA, L_V, R_D, R_V};
use crate::errors::InputError;
use crate::quantities::{
    AtmosphericPressure, DewPointTemperature, DryBulbTemperature, EquivalentPotentialTemperature,
    MixingRatio, PotentialTemperature, RelativeHumidity, ThermodynamicQuantity, VapourPressure,
};
use crate::{formulas::mixing_ratio, Float};
use crate::{Formula2, Formula4};

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
/// Valid `mixing_ratio` range: 0.000_000_1 - 2.0
///
/// Valid `relative_humidity` range: 0.000_000_1 - 2.0
pub struct Paluch1;

impl
    Formula4<
        FormulaQuantity,
        DryBulbTemperature,
        AtmosphericPressure,
        MixingRatio,
        RelativeHumidity,
    > for Paluch1
{
    #[inline(always)]
    fn validate_inputs(
        temperature: DryBulbTemperature,
        pressure: AtmosphericPressure,
        mixing_ratio: MixingRatio,
        relative_humidity: RelativeHumidity,
    ) -> Result<(), InputError> {
        temperature.check_range_si(253.0, 324.0)?;
        pressure.check_range_si(100.0, 150_000.0)?;
        mixing_ratio.check_range_si(0.000_000_1, 2.0)?;
        relative_humidity.check_range_si(0.000_000_1, 2.0)?;

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(
        temperature: DryBulbTemperature,
        pressure: AtmosphericPressure,
        mixing_ratio: MixingRatio,
        relative_humidity: RelativeHumidity,
    ) -> EquivalentPotentialTemperature {
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
/// Valid `mixing_ratio` range: 0.000_000_1 - 2.0
///
/// Valid `relative_humidity` range: 0.000_000_1 - 2.0
///
/// Valid `potential_temperature` range: 253K - 324K
pub struct Bryan1;

impl
    Formula4<
        FormulaQuantity,
        DryBulbTemperature,
        MixingRatio,
        RelativeHumidity,
        PotentialTemperature,
    > for Bryan1
{
    #[inline(always)]
    fn validate_inputs(
        temperature: DryBulbTemperature,
        mixing_ratio: MixingRatio,
        relative_humidity: RelativeHumidity,
        potential_temperature: PotentialTemperature,
    ) -> Result<(), InputError> {
        temperature.check_range_si(253.0, 324.0)?;
        mixing_ratio.check_range_si(0.000_000_1, 2.0)?;
        relative_humidity.check_range_si(0.000_000_1, 2.0)?;
        potential_temperature.check_range_si(253.0, 324.0)?;

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(
        temperature: DryBulbTemperature,
        mixing_ratio: MixingRatio,
        relative_humidity: RelativeHumidity,
        potential_temperature: PotentialTemperature,
    ) -> EquivalentPotentialTemperature {
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
///
/// Valid `vapour_pressure` range: 0Pa - 50000Pa
pub struct Bolton1;

impl
    Formula4<
        FormulaQuantity,
        AtmosphericPressure,
        DryBulbTemperature,
        DewPointTemperature,
        VapourPressure,
    > for Bolton1
{
    #[inline(always)]
    fn validate_inputs(
        pressure: AtmosphericPressure,
        temperature: DryBulbTemperature,
        dewpoint: DewPointTemperature,
        vapour_pressure: VapourPressure,
    ) -> Result<(), InputError> {
        pressure.check_range_si(100.0, 150_000.0)?;
        temperature.check_range_si(253.0, 324.0)?;
        dewpoint.check_range_si(253.0, 324.0)?;
        vapour_pressure.check_range_si(0.0, 50_000.0)?;

        if approx_eq!(
            Float,
            pressure.get_si_value(),
            vapour_pressure.get_si_value(),
            ulps = 2
        ) {
            return Err(InputError::IncorrectArgumentSet(
                "pressure must be greater than vapour pressure".to_string(),
            ));
        }

        if vapour_pressure.0 > pressure.0 {
            return Err(InputError::IncorrectArgumentSet(
                "pressure must be greater than vapour pressure".to_string(),
            ));
        }

        if dewpoint.0 > temperature.0 {
            return Err(InputError::IncorrectArgumentSet(
                "dewpoint must be less than temperature".to_string(),
            ));
        }

        let mixing_ratio = mixing_ratio::Definition1::compute_unchecked(pressure, vapour_pressure);

        mixing_ratio.check_range_si(0.000_000_1, 2.0).or_else(|_| {
            Err(InputError::IncorrectArgumentSet(
                "pressure and vapour_pressure must give mixing_ratio less than 2 so cannot be close to each other".to_string(),
            ))
        }
        )?;

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(
        pressure: AtmosphericPressure,
        temperature: DryBulbTemperature,
        dewpoint: DewPointTemperature,
        vapour_pressure: VapourPressure,
    ) -> EquivalentPotentialTemperature {
        let mixing_ratio = mixing_ratio::Definition1::compute_unchecked(pressure, vapour_pressure);

        let pressure = pressure.0.get::<pascal>();
        let temperature = temperature.0.get::<kelvin>();
        let dewpoint = dewpoint.0.get::<kelvin>();
        let mixing_ratio = mixing_ratio.0.get::<ratio>();
        let vapour_pressure = vapour_pressure.0.get::<pascal>();

        let kappa = KAPPA.get::<ratio>();

        // technically LCL and Theta-DL should be extracted to separate functions

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
    use crate::{
        quantities::{AtmosphericPressure, DryBulbTemperature, VapourPressure},
        tests::{test_with_4args, testing_traits::ReferenceAtmosphere, Argument},
    };

    use super::*;

    #[test]
    fn paluch1() {
        test_with_4args::<
            FormulaQuantity,
            DryBulbTemperature,
            AtmosphericPressure,
            MixingRatio,
            RelativeHumidity,
            Paluch1,
        >(
            Argument::new([253.0, 324.0]),
            Argument::new([100.0, 150_000.0]),
            Argument::new([0.000_000_1, 2.0]),
            Argument::new([0.000_000_1, 2.0]),
            ReferenceAtmosphere::Normal,
            1e-12,
        );
    }

    #[test]
    fn bryan1() {
        test_with_4args::<
            FormulaQuantity,
            DryBulbTemperature,
            MixingRatio,
            RelativeHumidity,
            PotentialTemperature,
            Bryan1,
        >(
            Argument::new([253.0, 324.0]),
            Argument::new([0.000_000_1, 2.0]),
            Argument::new([0.000_000_1, 2.0]),
            Argument::new([253.0, 324.0]),
            ReferenceAtmosphere::Normal,
            1e1,
        );
    }

    #[test]
    fn bolton1() {
        test_with_4args::<
            FormulaQuantity,
            AtmosphericPressure,
            DryBulbTemperature,
            DewPointTemperature,
            VapourPressure,
            Bolton1,
        >(
            Argument::new([100.0, 150_000.0]),
            Argument::new([253.0, 324.0]),
            Argument::new([253.0, 324.0]),
            Argument::new([0.0, 50_000.0]),
            ReferenceAtmosphere::Normal,
            1e1,
        );
    }
}
