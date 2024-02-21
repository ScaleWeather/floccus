#![allow(missing_docs)]

use crate::Storage;
use std::fmt::Debug;

pub trait ThermodynamicQuantity: Debug + Clone + Copy + PartialEq + PartialOrd + Default {}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct DryBulbTemperature(pub Storage::ThermodynamicTemperature);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct WetBulbTemperature(pub Storage::TemperatureInterval);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct DewPointTemperature(pub Storage::ThermodynamicTemperature);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct VirtualTemperature(pub Storage::TemperatureInterval);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct PotentialTemperature(pub Storage::TemperatureInterval);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct EquivalentPotentialTemperature(pub Storage::TemperatureInterval);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct WetBulbPotentialTemperature(pub Storage::TemperatureInterval);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct VapourPressure(pub Storage::Pressure);

impl ThermodynamicQuantity for DryBulbTemperature {}
impl ThermodynamicQuantity for WetBulbTemperature {}
impl ThermodynamicQuantity for DewPointTemperature {}
impl ThermodynamicQuantity for VirtualTemperature {}
impl ThermodynamicQuantity for PotentialTemperature {}
impl ThermodynamicQuantity for EquivalentPotentialTemperature {}
impl ThermodynamicQuantity for WetBulbPotentialTemperature {}

impl ThermodynamicQuantity for VapourPressure {}
