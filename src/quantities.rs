#![allow(missing_docs)]

mod accessors;
mod constructors;
mod trait_impls;

use floccus_proc::Name;

use crate::{errors::InputError, Float, Storage};
use std::fmt::Debug;

pub trait ThermodynamicQuantity:
    Debug + Clone + Copy + PartialEq + PartialOrd + Default + Send + Sync
{
}

pub(crate) trait QuantityName {
    fn type_name_as_str() -> &'static str;
}

pub(crate) trait QuantityHelpers: QuantityName + ThermodynamicQuantity {
    fn get_si_value(&self) -> Float;

    fn name(&self) -> &'static str {
        Self::type_name_as_str()
    }

    #[must_use]
    #[inline(always)]
    fn check_range_si(&self, lower_bound: Float, upper_bound: Float) -> Result<(), InputError> {
        if !(lower_bound..=upper_bound).contains(&self.get_si_value()) {
            return Err(InputError::OutOfRange(self.name()));
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Name)]
pub struct DryBulbTemperature(pub Storage::ThermodynamicTemperature);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Name)]
pub struct WetBulbTemperature(pub Storage::ThermodynamicTemperature);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Name)]
pub struct DewPointTemperature(pub Storage::ThermodynamicTemperature);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Name)]
pub struct VirtualTemperature(pub Storage::ThermodynamicTemperature);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Name)]
pub struct IsobaricEquivalentTemperature(pub Storage::ThermodynamicTemperature);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Name)]
pub struct AdiabaticEquivalentTemperature(pub Storage::ThermodynamicTemperature);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Name)]
pub struct PotentialTemperature(pub Storage::ThermodynamicTemperature);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Name)]
pub struct EquivalentPotentialTemperature(pub Storage::ThermodynamicTemperature);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Name)]
pub struct WetBulbPotentialTemperature(pub Storage::ThermodynamicTemperature);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Name)]
pub struct AtmosphericPressure(pub Storage::Pressure);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Name)]
pub struct VapourPressure(pub Storage::Pressure);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Name)]
pub struct SaturationVapourPressure(pub Storage::Pressure);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Name)]
pub struct VapourPressureDeficit(pub Storage::Pressure);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Name)]
pub struct MixingRatio(pub Storage::Ratio);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Name)]
pub struct SaturationMixingRatio(pub Storage::Ratio);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Name)]
pub struct SpecificHumidity(pub Storage::Ratio);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Name)]
pub struct RelativeHumidity(pub Storage::Ratio);
