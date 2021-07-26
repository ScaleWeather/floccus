//!Module containing physical constants

///Temperature in Kelvins of 0 Celsius
pub const ZERO_CELSIUS: f64 = 273.15;

///Gravitational acceleration in m*s^-2
pub const G: f64 = 9.80665;

///Universal gas constant in J*K^-1*mol^-1
pub const R: f64 = 8.31446261815324;

///Molar mass of dry air in kg*mol^-1 (ECMWF, 2020)
pub const M_D: f64 = 0.0289644;

///Molar mass of water vapour in kg*mol^-1
pub const M_V: f64 = 0.0180152833;

///Specific heat capacity of dry air at constant pressure in J*kg^-1*K^-1(ECMWF, 2020)
pub const C_P: f64 = 1004.709;

///Specific heat capacity of dry air at constant volume in J*kg^-1*K^-1 (ECMWF, 2020)
pub const C_V: f64 = 717.6493;

///Specific heat capacity of water vapour at constant pressure in J*kg^-1*K^-1 (ECMWF, 2020)
pub const C_PV: f64 = 1846.1;

///Specific heat capacity of water vapour at constant volume in J*kg^-1*K^-1 (ECMWF, 2020)
pub const C_VV: f64 = 1384.575;

///Latent heat of vapourization of water in J*kg^1  (ECMWF, 2020)
pub const L_V: f64 = 2500800.0;

///Ratio of molar masses of dry air and water vapour
pub const EPSILON: f64 = M_V / M_D;

///Specific gas constant for dry air
pub const R_D: f64 = R / M_D;
