//! Module containing physical constants

use std::marker::PhantomData;

use crate::Storage;

/// Gravitational acceleration of Earth
pub const G : Storage::Acceleration = Storage::Acceleration {
    dimension: PhantomData,
    units: PhantomData,
    value: 9.806_65,
};

/// Universal gas constant
pub const R: Storage::MolarHeatCapacity = Storage::MolarHeatCapacity {
    dimension: PhantomData,
    units: PhantomData,
    value: 8.314_462_618_153_24,
};

/// Molar mass of dry air (ECMWF, 2020)
pub const M_D: Storage::MolarMass = Storage::MolarMass {
    dimension: PhantomData,
    units: PhantomData,
    value: 0.028_964_4,
};

/// Molar mass of water vapour
pub const M_V: Storage::MolarMass = Storage::MolarMass {
    dimension: PhantomData,
    units: PhantomData,
    value: 0.018_015_283_3,
};

/// Specific heat capacity of dry air at constant pressure (ECMWF, 2020)
pub const C_P: Storage::SpecificHeatCapacity = Storage::SpecificHeatCapacity {
    dimension: PhantomData,
    units: PhantomData,
    value: 1004.709,
};

/// Specific heat capacity of dry air at constant volume` (ECMWF, 2020)
pub const C_V: Storage::SpecificHeatCapacity = Storage::SpecificHeatCapacity {
    dimension: PhantomData,
    units: PhantomData,
    value: 717.6493,
};

/// Specific heat capacity of water vapour at constant pressure (ECMWF, 2020)
pub const C_PV: Storage::SpecificHeatCapacity = Storage::SpecificHeatCapacity {
    dimension: PhantomData,
    units: PhantomData,
    value: 1846.1,
};

/// Specific heat capacity of water vapour at constant volume (ECMWF, 2020)
pub const C_VV: Storage::SpecificHeatCapacity = Storage::SpecificHeatCapacity {
    dimension: PhantomData,
    units: PhantomData,
    value: 1384.575,
};

/// Specific heat capacity of liquid water (ECMWF, 2020)
pub const C_L: Storage::SpecificHeatCapacity = Storage::SpecificHeatCapacity {
    dimension: PhantomData,
    units: PhantomData,
    value: 4218.0,
};

/// Specific heat capacity of solid water (ECMWF, 2020)
pub const C_S: Storage::SpecificHeatCapacity = Storage::SpecificHeatCapacity {
    dimension: PhantomData,
    units: PhantomData,
    value: 2106.0,
};

/// Specific latent heat of vapourization of water (ECMWF, 2020)
pub const L_V: Storage::AvailableEnergy = Storage::AvailableEnergy {
    dimension: PhantomData,
    units: PhantomData,
    value: 2_500_800.0,
};

/// Ratio of molar masses of dry air and water vapour
pub const EPSILON: Storage::Ratio = Storage::Ratio {
    dimension: PhantomData,
    units: PhantomData,
    value: M_V.value / M_D.value,
};

/// Specific gas constant for dry air
pub const R_D: Storage::SpecificHeatCapacity = Storage::SpecificHeatCapacity {
    dimension: PhantomData,
    units: PhantomData,
    value: R.value / M_D.value,
};

/// Specific gas constant for water vapour
pub const R_V: Storage::SpecificHeatCapacity = Storage::SpecificHeatCapacity {
    dimension: PhantomData,
    units: PhantomData,
    value: R.value / M_V.value,
};
