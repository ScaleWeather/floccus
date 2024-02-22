#![warn(clippy::pedantic)]
#![warn(missing_docs)]
#![warn(clippy::cargo)]
#![allow(clippy::excessive_precision)]
#![allow(clippy::must_use_candidate)]

//! Crate providing formulae for air thermodynamic calculations.
//!
//! # How to use
//!
//! To use this crate simply import it with `use` statement and then use desired function from chosen module.
//!
//! ```
//! use floccus::vapour_pressure;
//!  # use float_cmp::assert_approx_eq;
//!
//! //Set temperature and pressure in SI units
//! let temperature = 300.0; //in K
//! let pressure = 101325.0; //in Pa
//!
//! //Compute vapour pressure using Buck (1981) formula
//! let vapour_pressure = vapour_pressure::buck1(temperature, pressure).unwrap();
//! println!("{}", vapour_pressure); // 3550.662 (f32) or 3550.6603579471303 (f64)
//! ```
//!
//! # Naming of modules and functions
//!
//! Because some thermodynamic formulae are empirical there are several ways to compute a value of given quantity.
//! Therefore there are multiple functions available to compute the same parameter, which are grouped into modules.
//!
//! The naming of modules and functions follows this convention:
//!
//! ```
//!  # use floccus::vapour_pressure;
//!  # let temperature = 300.0;
//!  # let pressure = 100000.0;
//!  # let vp =
//! vapour_pressure::buck1(temperature, pressure)
//!  # .unwrap();
//! ```
//!
//! Where the module name (`vapour_pressure`) indicates the computed quantity, function name (`buck1`) indicates the author of formula
//! and the function arguments (`temperature, pressure`) are variables used to compute the quantity.
//!
//! # Double precision
//!
//! By default floccus uses single-precision (32-bit) floating-point variables.
//! If increased accuracy is needed (at the cost of performance) `double_precision` feature can be enabled
//! to use double-precision (64-bit) floating point.
//!
//! # Input checking
//!
//! To prevent any unexpected behaviour, all functions check whether provided inputs are within a reasonable range.
//! Exact limits are specified in the documentation of each function.
//! If the input is out of range the function will return an [`InputError::OutOfRange`](errors::InputError::OutOfRange) with erronous input specified.
//!
//! Each function also has `_unchecked` and `_validate` versions. The `_validate` version only checks the inputs with bounds defined for its "parent" function.
//! The `_unchecked` version performs only the calculation without any input checking. All "parent" functions simply call `_validate` and then `_unchecked`.
//!
//! # Units
//!
//! This crate uses basic SI units in the interface.
//!
//! Units for each quantity are:
//! - Pressure: Pascals (Pa)
//! - Temperature: Kelvins (K)
//! - Mass: kilograms (kg)
//! - Length: meters (m)
//! - Relative humidity: ratio (%/100)
//! - Volume: meters cubed (m^3)
//! - Density: kilograms per meter cubed (kg*m^3)
//! - Mixing ratio: kilograms per kilogram (kg*kg^-1)
//! - Specific humidity: kilograms per kilogram (kg*kg^-1)
//!
//! If the formula uses numbers of very different scales there can be an exception from that rule described in the function documentation.
//!
//! # Debugging
//!
//! If additional information is needed about which function returns the error and why, `debug` feature can be enabled.
//! With that feature when returning the error function will also print the error message to `log` with additional
//! information about the error. This feature potentially is not zero-cost so it is optional.

pub mod constants;
pub mod errors;
pub mod formula;
pub mod quantities;

// pub mod equivalent_potential_temperature;
pub mod mixing_ratio;
// pub mod saturation_mixing_ratio;
// pub mod potential_temperature;
// pub mod relative_humidity;
// pub mod specific_humidity;
pub mod vapour_pressure;
// pub mod saturation_vapour_pressure;
// pub mod vapour_pressure_deficit;
pub mod virtual_temperature;
// pub mod wet_bulb_potential_temperature;
pub mod wet_bulb_temperature;

#[cfg(test)]
mod tests;

#[cfg(not(feature = "double_precision"))]
type Float = f32;

#[cfg(feature = "double_precision")]
type Float = f64;

#[cfg(not(feature = "double_precision"))]
pub(crate) use uom::si::f32 as Storage;

#[cfg(feature = "double_precision")]
pub(crate) use uom::si::f64 as Storage;
