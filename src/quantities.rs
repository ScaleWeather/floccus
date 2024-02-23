#![allow(missing_docs)]

use uom::si::{pressure::pascal, ratio::ratio, thermodynamic_temperature::kelvin};

use crate::{Float, Storage};
use std::fmt::Debug;

pub trait ThermodynamicQuantity:
    Debug + Clone + Copy + PartialEq + PartialOrd + Default + Send + Sync
{
    fn get_si_value(&self) -> Float;
    fn new_si(value: Float) -> Self;
    fn name(&self) -> &'static str;
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

impl DryBulbTemperature {
    pub fn new<T>(value: Float) -> Self
    where
        T: uom::si::thermodynamic_temperature::Unit
            + uom::si::thermodynamic_temperature::Conversion<Float>,
    {
        Self(Storage::ThermodynamicTemperature::new::<T>(value))
    }
}

impl WetBulbTemperature {
    pub fn new<T>(value: Float) -> Self
    where
        T: uom::si::thermodynamic_temperature::Unit
            + uom::si::thermodynamic_temperature::Conversion<Float>,
    {
        Self(Storage::ThermodynamicTemperature::new::<T>(value))
    }
}

impl DewPointTemperature {
    pub fn new<T>(value: Float) -> Self
    where
        T: uom::si::thermodynamic_temperature::Unit
            + uom::si::thermodynamic_temperature::Conversion<Float>,
    {
        Self(Storage::ThermodynamicTemperature::new::<T>(value))
    }
}

impl VirtualTemperature {
    pub fn new<T>(value: Float) -> Self
    where
        T: uom::si::thermodynamic_temperature::Unit
            + uom::si::thermodynamic_temperature::Conversion<Float>,
    {
        Self(Storage::ThermodynamicTemperature::new::<T>(value))
    }
}

impl PotentialTemperature {
    pub fn new<T>(value: Float) -> Self
    where
        T: uom::si::thermodynamic_temperature::Unit
            + uom::si::thermodynamic_temperature::Conversion<Float>,
    {
        Self(Storage::ThermodynamicTemperature::new::<T>(value))
    }
}

impl EquivalentPotentialTemperature {
    pub fn new<T>(value: Float) -> Self
    where
        T: uom::si::thermodynamic_temperature::Unit
            + uom::si::thermodynamic_temperature::Conversion<Float>,
    {
        Self(Storage::ThermodynamicTemperature::new::<T>(value))
    }
}

impl WetBulbPotentialTemperature {
    pub fn new<T>(value: Float) -> Self
    where
        T: uom::si::thermodynamic_temperature::Unit
            + uom::si::thermodynamic_temperature::Conversion<Float>,
    {
        Self(Storage::ThermodynamicTemperature::new::<T>(value))
    }
}

impl AtmosphericPressure {
    pub fn new<T>(value: Float) -> Self
    where
        T: uom::si::pressure::Unit + uom::si::pressure::Conversion<Float>,
    {
        Self(Storage::Pressure::new::<T>(value))
    }
}

impl VapourPressure {
    pub fn new<T>(value: Float) -> Self
    where
        T: uom::si::pressure::Unit + uom::si::pressure::Conversion<Float>,
    {
        Self(Storage::Pressure::new::<T>(value))
    }
}

impl SaturationVapourPressure {
    pub fn new<T>(value: Float) -> Self
    where
        T: uom::si::pressure::Unit + uom::si::pressure::Conversion<Float>,
    {
        Self(Storage::Pressure::new::<T>(value))
    }
}

impl VapourPressureDeficit {
    pub fn new<T>(value: Float) -> Self
    where
        T: uom::si::pressure::Unit + uom::si::pressure::Conversion<Float>,
    {
        Self(Storage::Pressure::new::<T>(value))
    }
}

impl MixingRatio {
    pub fn new<T>(value: Float) -> Self
    where
        T: uom::si::ratio::Unit + uom::si::ratio::Conversion<Float>,
    {
        Self(Storage::Ratio::new::<T>(value))
    }
}

impl SaturationMixingRatio {
    pub fn new<T>(value: Float) -> Self
    where
        T: uom::si::ratio::Unit + uom::si::ratio::Conversion<Float>,
    {
        Self(Storage::Ratio::new::<T>(value))
    }
}

impl SpecificHumidity {
    pub fn new<T>(value: Float) -> Self
    where
        T: uom::si::ratio::Unit + uom::si::ratio::Conversion<Float>,
    {
        Self(Storage::Ratio::new::<T>(value))
    }
}

impl RelativeHumidity {
    pub fn new<T>(value: Float) -> Self
    where
        T: uom::si::ratio::Unit + uom::si::ratio::Conversion<Float>,
    {
        Self(Storage::Ratio::new::<T>(value))
    }
}

impl ThermodynamicQuantity for DryBulbTemperature {
    fn get_si_value(&self) -> Float {
        self.0.get::<kelvin>()
    }

    fn new_si(value: Float) -> Self {
        Self::new::<kelvin>(value)
    }

    fn name(&self) -> &'static str {
        "DryBulbTemperature"
    }
}
impl ThermodynamicQuantity for WetBulbTemperature {
    fn get_si_value(&self) -> Float {
        self.0.get::<kelvin>()
    }

    fn new_si(value: Float) -> Self {
        Self::new::<kelvin>(value)
    }

    fn name(&self) -> &'static str {
        "WetBulbTemperature"
    }
}
impl ThermodynamicQuantity for DewPointTemperature {
    fn get_si_value(&self) -> Float {
        self.0.get::<kelvin>()
    }

    fn new_si(value: Float) -> Self {
        Self::new::<kelvin>(value)
    }

    fn name(&self) -> &'static str {
        "DewPointTemperature"
    }
}
impl ThermodynamicQuantity for VirtualTemperature {
    fn get_si_value(&self) -> Float {
        self.0.get::<kelvin>()
    }

    fn new_si(value: Float) -> Self {
        Self::new::<kelvin>(value)
    }

    fn name(&self) -> &'static str {
        "VirtualTemperature"
    }
}
impl ThermodynamicQuantity for PotentialTemperature {
    fn get_si_value(&self) -> Float {
        self.0.get::<kelvin>()
    }

    fn new_si(value: Float) -> Self {
        Self::new::<kelvin>(value)
    }

    fn name(&self) -> &'static str {
        "PotentialTemperature"
    }
}
impl ThermodynamicQuantity for EquivalentPotentialTemperature {
    fn get_si_value(&self) -> Float {
        self.0.get::<kelvin>()
    }

    fn new_si(value: Float) -> Self {
        Self::new::<kelvin>(value)
    }

    fn name(&self) -> &'static str {
        "EquivalentPotentialTemperature"
    }
}
impl ThermodynamicQuantity for WetBulbPotentialTemperature {
    fn get_si_value(&self) -> Float {
        self.0.get::<kelvin>()
    }

    fn new_si(value: Float) -> Self {
        Self::new::<kelvin>(value)
    }

    fn name(&self) -> &'static str {
        "WetBulbPotentialTemperature"
    }
}

impl ThermodynamicQuantity for AtmosphericPressure {
    fn get_si_value(&self) -> Float {
        self.0.get::<pascal>()
    }

    fn new_si(value: Float) -> Self {
        Self::new::<pascal>(value)
    }

    fn name(&self) -> &'static str {
        "AtmosphericPressure"
    }
}
impl ThermodynamicQuantity for VapourPressure {
    fn get_si_value(&self) -> Float {
        self.0.get::<pascal>()
    }

    fn new_si(value: Float) -> Self {
        Self::new::<pascal>(value)
    }

    fn name(&self) -> &'static str {
        "VapourPressure"
    }
}
impl ThermodynamicQuantity for SaturationVapourPressure {
    fn get_si_value(&self) -> Float {
        self.0.get::<pascal>()
    }

    fn new_si(value: Float) -> Self {
        Self::new::<pascal>(value)
    }

    fn name(&self) -> &'static str {
        "SaturationVapourPressure"
    }
}

impl ThermodynamicQuantity for VapourPressureDeficit {
    fn get_si_value(&self) -> Float {
        self.0.get::<pascal>()
    }

    fn new_si(value: Float) -> Self {
        Self::new::<pascal>(value)
    }

    fn name(&self) -> &'static str {
        "VapourPressureDeficit"
    }
}

impl ThermodynamicQuantity for MixingRatio {
    fn get_si_value(&self) -> Float {
        self.0.get::<ratio>()
    }

    fn new_si(value: Float) -> Self {
        Self::new::<ratio>(value)
    }

    fn name(&self) -> &'static str {
        "MixingRatio"
    }
}

impl ThermodynamicQuantity for SaturationMixingRatio {
    fn get_si_value(&self) -> Float {
        self.0.get::<ratio>()
    }

    fn new_si(value: Float) -> Self {
        Self::new::<ratio>(value)
    }

    fn name(&self) -> &'static str {
        "SaturationMixingRatio"
    }
}

impl ThermodynamicQuantity for SpecificHumidity {
    fn get_si_value(&self) -> Float {
        self.0.get::<ratio>()
    }

    fn new_si(value: Float) -> Self {
        Self::new::<ratio>(value)
    }

    fn name(&self) -> &'static str {
        "SpecificHumidity"
    }
}
impl ThermodynamicQuantity for RelativeHumidity {
    fn get_si_value(&self) -> Float {
        self.0.get::<ratio>()
    }

    fn new_si(value: Float) -> Self {
        Self::new::<ratio>(value)
    }

    fn name(&self) -> &'static str {
        "RelativeHumidity"
    }
}
