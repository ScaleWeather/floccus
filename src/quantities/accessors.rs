use super::*;

impl DryBulbTemperature {
    pub fn get<T>(&self) -> Float
    where
        T: uom::si::thermodynamic_temperature::Unit
            + uom::si::thermodynamic_temperature::Conversion<Float>,
    {
        self.0.get::<T>()
    }
}

impl WetBulbTemperature {
    pub fn get<T>(&self) -> Float
    where
        T: uom::si::thermodynamic_temperature::Unit
            + uom::si::thermodynamic_temperature::Conversion<Float>,
    {
        self.0.get::<T>()
    }
}

impl DewPointTemperature {
    pub fn get<T>(&self) -> Float
    where
        T: uom::si::thermodynamic_temperature::Unit
            + uom::si::thermodynamic_temperature::Conversion<Float>,
    {
        self.0.get::<T>()
    }
}

impl VirtualTemperature {
    pub fn get<T>(&self) -> Float
    where
        T: uom::si::thermodynamic_temperature::Unit
            + uom::si::thermodynamic_temperature::Conversion<Float>,
    {
        self.0.get::<T>()
    }
}

impl PotentialTemperature {
    pub fn get<T>(&self) -> Float
    where
        T: uom::si::thermodynamic_temperature::Unit
            + uom::si::thermodynamic_temperature::Conversion<Float>,
    {
        self.0.get::<T>()
    }
}

impl IsobaricEquivalentTemperature {
    pub fn get<T>(&self) -> Float
    where
        T: uom::si::thermodynamic_temperature::Unit
            + uom::si::thermodynamic_temperature::Conversion<Float>,
    {
        self.0.get::<T>()
    }
}

impl AdiabaticEquivalentTemperature {
    pub fn get<T>(&self) -> Float
    where
        T: uom::si::thermodynamic_temperature::Unit
            + uom::si::thermodynamic_temperature::Conversion<Float>,
    {
        self.0.get::<T>()
    }
}

impl EquivalentPotentialTemperature {
    pub fn get<T>(&self) -> Float
    where
        T: uom::si::thermodynamic_temperature::Unit
            + uom::si::thermodynamic_temperature::Conversion<Float>,
    {
        self.0.get::<T>()
    }
}

impl WetBulbPotentialTemperature {
    pub fn get<T>(&self) -> Float
    where
        T: uom::si::thermodynamic_temperature::Unit
            + uom::si::thermodynamic_temperature::Conversion<Float>,
    {
        self.0.get::<T>()
    }
}

impl AtmosphericPressure {
    pub fn get<T>(&self) -> Float
    where
        T: uom::si::pressure::Unit + uom::si::pressure::Conversion<Float>,
    {
        self.0.get::<T>()
    }
}

impl VapourPressure {
    pub fn get<T>(&self) -> Float
    where
        T: uom::si::pressure::Unit + uom::si::pressure::Conversion<Float>,
    {
        self.0.get::<T>()
    }
}

impl SaturationVapourPressure {
    pub fn get<T>(&self) -> Float
    where
        T: uom::si::pressure::Unit + uom::si::pressure::Conversion<Float>,
    {
        self.0.get::<T>()
    }
}

impl VapourPressureDeficit {
    pub fn get<T>(&self) -> Float
    where
        T: uom::si::pressure::Unit + uom::si::pressure::Conversion<Float>,
    {
        self.0.get::<T>()
    }
}

impl MixingRatio {
    pub fn get<T>(&self) -> Float
    where
        T: uom::si::ratio::Unit + uom::si::ratio::Conversion<Float>,
    {
        self.0.get::<T>()
    }
}

impl SaturationMixingRatio {
    pub fn get<T>(&self) -> Float
    where
        T: uom::si::ratio::Unit + uom::si::ratio::Conversion<Float>,
    {
        self.0.get::<T>()
    }
}

impl SpecificHumidity {
    pub fn get<T>(&self) -> Float
    where
        T: uom::si::ratio::Unit + uom::si::ratio::Conversion<Float>,
    {
        self.0.get::<T>()
    }
}

impl RelativeHumidity {
    pub fn get<T>(&self) -> Float
    where
        T: uom::si::ratio::Unit + uom::si::ratio::Conversion<Float>,
    {
        self.0.get::<T>()
    }
}
