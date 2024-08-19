use super::*;

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

impl IsobaricEquivalentTemperature {
    pub fn new<T>(value: Float) -> Self
    where
        T: uom::si::thermodynamic_temperature::Unit
            + uom::si::thermodynamic_temperature::Conversion<Float>,
    {
        Self(Storage::ThermodynamicTemperature::new::<T>(value))
    }
}

impl AdiabaticEquivalentTemperature {
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
