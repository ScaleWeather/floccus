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
//!let vapour_pressure = vapour_pressure::buck1(temperature, pressure).unwrap();
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
//! # let pressure = 100000.0;
//! # let vp =
//!vapour_pressure::buck1(temperature, pressure)
//! # .unwrap();
//!```
//!
//!Where the module name (`vapour_pressure`) indicates the computed quantity, function name (`buck1`) indicates the author of formula
//!and the function arguments (`temperature, pressure`) are variables used to compute the quantity.
//!
//!# Input checking
//!
//!To prevent any unexpected behaviour, all functions check whether provided inputs are within a reasonable range.
//!Exact limits are specified in the documentation of each function.
//!If the input is out of range the function will return an [`InputError::OutOfRange`](error_wrapper::InputError::OutOfRange) with erronous input specified.
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

pub mod constants;
pub mod error_wrapper;
pub mod vapour_pressure;
