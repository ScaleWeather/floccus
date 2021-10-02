//!Module containing physical constants

use crate::Float;

///Temperature of 0 Celsius in `K`
pub const ZERO_CELSIUS: Float = 273.15;

///Gravitational acceleration in `m s^-2`
pub const G: Float = 9.80665;

///Universal gas constant in `J K^-1 mol^-1`
pub const R: Float = 8.314_462_618_153_24;

///Molar mass of dry air in `kg mol^-1` (ECMWF, 2020)
pub const M_D: Float = 0.028_964_4;

///Molar mass of water vapour in `kg mol^-1`
pub const M_V: Float = 0.018_015_283_3;

///Specific heat capacity of dry air at constant pressure in `J kg^-1 K^-1` (ECMWF, 2020)
pub const C_P: Float = 1004.709;

///Specific heat capacity of dry air at constant volume in `J kg^-1 K^-1` (ECMWF, 2020)
pub const C_V: Float = 717.6493;

///Specific heat capacity of water vapour at constant pressure in `J kg^-1 K^-1` (ECMWF, 2020)
pub const C_PV: Float = 1846.1;

///Specific heat capacity of water vapour at constant volume in `J kg^-1 K^-1` (ECMWF, 2020)
pub const C_VV: Float = 1384.575;

///Mass latent heat of vapourization of water in `J kg^1`  (ECMWF, 2020)
pub const L_V: Float = 2_500_800.0;

///Ratio of molar masses of dry air and water vapour in `no unit`
pub const EPSILON: Float = M_V / M_D;

///Specific gas constant for dry air in `J kg^-1 K^-1`
pub const R_D: Float = R / M_D;
