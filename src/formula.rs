#![allow(missing_docs)]

use uom::si::{
    pressure::hectopascal, ratio::ratio, temperature_coefficient, temperature_interval,
    thermodynamic_temperature,
};

use crate::{
    errors::InputError,
    quantities::{DewPointTemperature, DryBulbTemperature, ThermodynamicQuantity, VapourPressure},
    vapour_pressure::Buck3Simplified,
    Storage::{
        Pressure, Ratio, TemperatureCoefficient, TemperatureInterval, ThermodynamicTemperature,
    },
};

pub trait Formula1<O: ThermodynamicQuantity, I1: ThermodynamicQuantity> {
    #[allow(missing_docs)]
    fn compute_unchecked(i1: I1) -> O;

    #[allow(missing_docs)]
    #[allow(clippy::missing_errors_doc)]
    fn validate_inputs(i1: I1) -> Result<(), InputError>;

    #[allow(clippy::missing_errors_doc)]
    #[allow(missing_docs)]
    #[inline]
    fn compute(i1: I1) -> Result<O, InputError> {
        #[cfg(not(feature = "debug"))]
        Self::validate_inputs(i1)?;
        #[cfg(feature = "debug")]
        Self::validate_inputs_loggerr(i1)?;

        Ok(Self::compute_unchecked(i1))
    }

    #[cfg(feature = "debug")]
    #[inline(always)]
    #[allow(missing_docs)]
    #[allow(clippy::missing_errors_doc)]
    fn validate_inputs_loggerr(i1: I1) -> Result<(), InputError> {
        use std::any::type_name;

        Self::validate_inputs(i1).or_else(|err| {
            log::error!(
                "Formula {} calculating {} from inputs {:?} returned error: {}",
                type_name::<Self>(),
                type_name::<O>(),
                i1,
                err
            );
            Err(err)
        })
    }
}

struct BuckTest;

impl Formula1<VapourPressure, DewPointTemperature> for BuckTest {
    #[inline(always)]
    fn validate_inputs(i1: DewPointTemperature) -> Result<(), InputError> {
        let i1_si = i1.0.get::<thermodynamic_temperature::kelvin>();

        if !(253.0..=324.0).contains(&i1_si) {
            return Err(InputError::OutOfRange(String::from("dewpoint")));
        }

        Ok(())
    }

    #[inline(always)]
    fn compute_unchecked(i1: DewPointTemperature) -> VapourPressure {
        let dewpoint = i1.0.get::<thermodynamic_temperature::degree_celsius>();
        let dewpoint = Ratio::new::<ratio>(dewpoint);

        let lower_a = Pressure::new::<hectopascal>(6.1121);
        let lower_b = Ratio::new::<ratio>(17.502);
        let lower_c = Ratio::new::<ratio>(240.97);

        let lower_e = lower_a * ((lower_b * dewpoint) / (dewpoint + lower_c)).exp();

        VapourPressure(lower_e)
    }
}

#[cfg(test)]
mod tests {
    use uom::si::f64::ThermodynamicTemperature;

    use super::*;

    #[test]
    fn test_buck3_simplified() {
        let input = ThermodynamicTemperature::new::<thermodynamic_temperature::kelvin>(300.0);
        let input = DewPointTemperature(input);

        let result = BuckTest::compute(input).unwrap();

        println!("{:?}", result.0);
    }
}
