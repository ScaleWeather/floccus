#![allow(missing_docs)]

mod accessors;
mod constructors;
mod trait_impls;

use crate::{Float, Storage, errors::InputError};
use std::any::type_name;
use std::fmt::Debug;

pub trait ThermodynamicQuantity:
    Debug + Clone + Copy + PartialEq + PartialOrd + Default + Send + Sync
{
}

pub(crate) trait QuantityHelpers: ThermodynamicQuantity {
    fn get_si_value(&self) -> Float;

    fn name() -> &'static str {
        type_name::<Self>()
    }

    #[must_use]
    #[inline(always)]
    fn check_range_si(&self, lower_bound: Float, upper_bound: Float) -> Result<(), InputError> {
        if !(lower_bound..=upper_bound).contains(&self.get_si_value()) {
            return Err(InputError::OutOfRange(Self::name()));
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct DryBulbTemperature(pub Storage::ThermodynamicTemperature);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct WetBulbTemperature(pub Storage::ThermodynamicTemperature);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct DewPointTemperature(pub Storage::ThermodynamicTemperature);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct VirtualTemperature(pub Storage::ThermodynamicTemperature);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct IsobaricEquivalentTemperature(pub Storage::ThermodynamicTemperature);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct AdiabaticEquivalentTemperature(pub Storage::ThermodynamicTemperature);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct PotentialTemperature(pub Storage::ThermodynamicTemperature);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct EquivalentPotentialTemperature(pub Storage::ThermodynamicTemperature);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct WetBulbPotentialTemperature(pub Storage::ThermodynamicTemperature);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct AtmosphericPressure(pub Storage::Pressure);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct VapourPressure(pub Storage::Pressure);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct SaturationVapourPressure(pub Storage::Pressure);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct VapourPressureDeficit(pub Storage::Pressure);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct MixingRatio(pub Storage::Ratio);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct SaturationMixingRatio(pub Storage::Ratio);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct SpecificHumidity(pub Storage::Ratio);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct RelativeHumidity(pub Storage::Ratio);
