//!Crate providing formulae for thermodynamic calculations.
//!
//!# How to use
//!
//!To use this crate simply import it with `use` statement and then use desired function from chosen module.
//!
//!```
//!use floccus::vapour_pressure;
//! # use float_cmp::assert_approx_eq;
//!
//!//Set temperature and pressure in SI units
//!let temperature = 300.0; //in K
//!let pressure = 101325.0; //in Pa
//!
//!//Compute vapour pressure using Buck (1981) formula
//!let vapour_pressure = vapour_pressure::buck1(temperature, pressure);
//!let expected = 3550.6603579471303;
//!
//!assert_approx_eq!(f64, expected, vapour_pressure, ulps = 2);
//!```
//!
//!# Naming of modules and functions
//!
//!Because some thermodynamic formulae are empirical there are several ways to compute a value of given quantity.
//!Therefore there are multiple functions available to compute the same parameter, which are grouped into modules.
//!
//!The naming of modules and functions follows this convention:
//!
//!```
//! # use floccus::vapour_pressure;
//! # let temperature = 300.0;
//! # let pressure = 1000000.0;
//! # let vp =
//!vapour_pressure::buck1(temperature, pressure);
//!```
//!
//!Where the module name (`vapour_pressure`) indicates the computed quantity, function name (`buck1`) indicates the author of formula
//!and the function arguments (`temperature, pressure`) are variables used to compute the quantity.
//!
//!# Units
//!This crate uses basic SI units in the interface.
//!
//!Units for each quantity are:
//!- Pressure: Pascals (Pa)
//!- Temperature: Kelvins (K)
//!- Mass: kilograms (kg)
//!- Length: meters (m)
//!- Volume: meters cubed (m^3)
//!- Density: kilograms per meter cubed (kg*m^3)
//!
//!If the formula uses numbers of very different scales there can be an exception from that rule described in the function documentation.

pub mod vapour_pressure {
    pub fn buck1(temperature: f64, pressure: f64) -> f64 {
        //input in K & Pa; output in ratio of Pa
        let temperature = temperature - 273.15; //convert to C
        let pressure = pressure / 100.0; //convert to hPa

        let a = 6.1121;
        let b = 18.729;
        let c = 257.87;
        let d = 227.3;

        let upper_a = 0.00072;
        let upper_b = 0.0000032;
        let upper_c = 0.00000000059;

        let e = a * (((b - (temperature / d)) * temperature) / (temperature + c)).exp();
        let f = 1.0 + upper_a + (pressure * (upper_b + (upper_c * temperature * temperature)));

        (e * f) * 100.0 //return in Pa
    }

    pub fn tetens1(temperature: f64) -> f64 {
        //input in K & Pa; output in ratio of Pa
        let temperature = temperature - 273.15; //convert to C

        let a = 0.61078;
        let b = 17.27;
        let c = 237.3;

        let result = a * ((b * temperature) / (temperature + c)).exp();

        result * 1000.0 //return in Pa
    }
}

#[cfg(test)]
mod tests {
    use crate::vapour_pressure;
    use float_cmp::assert_approx_eq;

    #[test]
    fn vapour_pressure_buck1() {
        let result = vapour_pressure::buck1(300.0, 101325.0);
        let expected = 3550.6603579471303;
        assert_approx_eq!(f64, expected, result, ulps = 2);
    }

    #[test]
    fn vapour_pressure_tetens1() {
        let result = vapour_pressure::tetens1(300.0);
        let expected = 3533.969137160892;
        assert_approx_eq!(f64, expected, result, ulps = 2);
    }
}
