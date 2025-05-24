//! Functions to calculate wet bulb temperature of unsaturated air

use uom::si::pressure::pascal;
use uom::si::ratio::{percent, ratio};
use uom::si::thermodynamic_temperature::{degree_celsius, kelvin};

use crate::constants::{EPSILON, KAPPA, LAMBDA, REF_PRES};
use crate::errors::InputError;
use crate::formulas::Formula2;
use crate::quantities::{
    AdiabaticEquivalentTemperature, AtmosphericPressure, DryBulbTemperature, QuantityHelpers,
    RelativeHumidity, WetBulbTemperature,
};

type FormulaQuantity = WetBulbTemperature;

/// Formula for computing wet bulb temperature from dry bulb temperature and relative humidity.
///
/// Derived by R. Stull (2011) [(doi:10.1175/JAMC-D-11-0143.1)](https://doi.org/10.1175/JAMC-D-11-0143.1)
/// Created with use of gene-expression programming.
///
/// Result error is within -1K to +0.65K, with mean absolute error of 0.28K
///
/// Valid `temperature` range: 253K - 324K

/// Valid `relative_humidity` range: 0.05 - 0.99
pub struct Stull1;

impl Formula2<FormulaQuantity, DryBulbTemperature, RelativeHumidity> for Stull1 {
    #[inline(always)]
    fn validate_inputs_internal(
        temperature: DryBulbTemperature,
        relative_humidity: RelativeHumidity,
    ) -> Result<(), InputError> {
        temperature.check_range_si(253.0, 324.0)?;
        relative_humidity.check_range_si(0.05, 0.99)?;

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(
        temperature: DryBulbTemperature,
        relative_humidity: RelativeHumidity,
    ) -> WetBulbTemperature {
        let temperature = temperature.0.get::<degree_celsius>();
        let relative_humidity = relative_humidity.0.get::<percent>();

        let result = (temperature * (0.151_977 * (relative_humidity + 8.313_659).sqrt()).atan())
            + (temperature + relative_humidity).atan()
            - (relative_humidity - 1.676_331).atan()
            + (0.003_918_38 * relative_humidity.powf(1.5) * (0.023_101 * relative_humidity).atan())
            - 4.686_035;

        WetBulbTemperature::new::<degree_celsius>(result)
    }
}

/// Formula for computing wet bulb temperature pressure from adiabatic equivalent temperature,
/// saturation mixing ratio and pressure.
///
/// Derived by R. Davies-Jones (2008) [(doi:10.1175/2007MWR2224.1)](https://doi.org/10.1175/2007MWR2224.1).
///
/// Note that this formula conditionally selects equation used for computation.
/// Thus it has discontinuities along the input range and has some performance overhead.
///
/// Valid `equivalent_temperature` range: 174K - 377K
///
/// Valid `pressure` range: 100Pa - 150000Pa
pub struct DaviesJones1;

impl Formula2<FormulaQuantity, AdiabaticEquivalentTemperature, AtmosphericPressure>
    for DaviesJones1
{
    #[inline(always)]
    fn validate_inputs_internal(
        equivalent_temperature: AdiabaticEquivalentTemperature,
        pressure: AtmosphericPressure,
    ) -> Result<(), InputError> {
        equivalent_temperature.check_range_si(174., 377.)?;
        pressure.check_range_si(100., 150_000.)?;
        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(
        equivalent_temperature: AdiabaticEquivalentTemperature,
        pressure: AtmosphericPressure,
    ) -> WetBulbTemperature {
        let lambda = LAMBDA.get::<ratio>();
        let kappa = KAPPA.get::<ratio>();
        let t_e = equivalent_temperature.get::<kelvin>();
        let p = pressure.get::<pascal>();
        let p0 = REF_PRES.get::<pascal>();
        let pi = (p / p0).powf(kappa);

        let indicator = (273.15 / t_e).powf(lambda);
        let d_pi = (0.1859 * (pi / p0) + 0.6512).powi(-1);

        let k1_pi = || -38.5 * pi.powi(2) + 137.81 * pi - 53.737;
        let k2_pi = || -4.392 * pi.powi(2) + 56.831 * pi - 0.384;

        let eq_4_9 = || k1_pi() - k2_pi() * indicator;
        let eq_4_10 = || (k1_pi() - 1.21) - (k2_pi() - 1.21) * indicator;
        let eq_4_11 = || {
            (k1_pi() - 2.66) - (k2_pi() - 1.21) * indicator + 0.58 * (273.15 / t_e).powf(-lambda)
        };

        let eq_4_8 = || {
            let eps = EPSILON.get::<ratio>();
            let big_a = 2675.0;
            let a = 17.67;
            let b = 243.5;
            let e_s = 6.112 * (a * (t_e - 273.15) / (t_e - 273.15 + b)).exp(); //effectively Tetens
            let r_s = (eps * e_s) / ((p / 100.0) - e_s);
            let d_ln_es_d = a * b / (t_e - 273.15 + b).powi(2);

            t_e - 273.15 - ((big_a * r_s) / (1.0 + (big_a * r_s * d_ln_es_d)))
        };

        let t_w;

        if indicator > d_pi {
            t_w = eq_4_8();
            #[cfg(test)]
            log::info!("eq 4.8");
        } else if indicator >= 1.0 && indicator <= d_pi {
            t_w = eq_4_9();
            #[cfg(test)]
            log::info!("eq 4.9");
        } else if indicator < 1.0 && indicator >= 0.4 {
            t_w = eq_4_10();
            #[cfg(test)]
            log::info!("eq 4.10");
        } else {
            t_w = eq_4_11();
            #[cfg(test)]
            log::info!("eq 4.11");
        }

        WetBulbTemperature::new::<degree_celsius>(t_w)
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;
    use uom::si::pressure::hectopascal;

    use crate::tests::{test_with_2args, testing_traits::ReferenceAtmosphere, Argument};

    use super::*;

    #[test]
    fn stull1() {
        test_with_2args::<FormulaQuantity, DryBulbTemperature, RelativeHumidity, Stull1>(
            Argument::new([253.0, 324.0]),
            Argument::new([0.05, 0.99]),
            ReferenceAtmosphere::Normal,
            1e1,
        );
    }

    #[test]
    fn daviesjones1_norm() {
        test_with_2args::<
            FormulaQuantity,
            AdiabaticEquivalentTemperature,
            AtmosphericPressure,
            DaviesJones1,
        >(
            Argument::new([174.0, 377.0]),
            Argument::new([100., 150_000.]),
            ReferenceAtmosphere::Normal,
            1e-12,
        );
    }

    #[test]
    fn daviesjones1_freez() {
        test_with_2args::<
            FormulaQuantity,
            AdiabaticEquivalentTemperature,
            AtmosphericPressure,
            DaviesJones1,
        >(
            Argument::new([174.0, 377.0]),
            Argument::new([100., 150_000.]),
            ReferenceAtmosphere::Freezing,
            1e-12,
        );
    }

    #[test]
    fn daviesjones1_manual() {
        testing_logger::setup();
        // eq 4.8
        let pressure = AtmosphericPressure::new::<hectopascal>(500.0);
        let t_e = AdiabaticEquivalentTemperature::new::<kelvin>(240.);
        let wbt = DaviesJones1::compute_unchecked(t_e, pressure);
        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 1);
            assert_eq!(captured_logs[0].body, "eq 4.8")
        });
        assert_approx_eq!(f64, 238.88007446399956, wbt.get_si_value());

        // eq 4.9
        let pressure = AtmosphericPressure::new::<hectopascal>(850.0);
        let t_e = AdiabaticEquivalentTemperature::new::<kelvin>(260.);
        let wbt = DaviesJones1::compute_unchecked(t_e, pressure);
        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 1);
            assert_eq!(captured_logs[0].body, "eq 4.9")
        });
        assert_approx_eq!(f64, 256.61913438141625, wbt.get_si_value());

        // eq 4.10
        let pressure = AtmosphericPressure::new::<hectopascal>(1000.0);
        let t_e = AdiabaticEquivalentTemperature::new::<kelvin>(330.);
        let wbt = DaviesJones1::compute_unchecked(t_e, pressure);
        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 1);
            assert_eq!(captured_logs[0].body, "eq 4.10")
        });
        assert_approx_eq!(f64, 291.2797584152462, wbt.get_si_value());

        // eq 4.11
        let pressure = AtmosphericPressure::new::<hectopascal>(1000.0);
        let t_e = AdiabaticEquivalentTemperature::new::<kelvin>(370.);
        let wbt = DaviesJones1::compute_unchecked(t_e, pressure);
        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 1);
            assert_eq!(captured_logs[0].body, "eq 4.11")
        });
        assert_approx_eq!(f64, 300.1638142668842, wbt.get_si_value());
    }
}
