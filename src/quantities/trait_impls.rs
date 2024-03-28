use super::*;
use uom::si::{pressure::pascal, ratio::ratio, thermodynamic_temperature::kelvin};

impl ThermodynamicQuantity for DryBulbTemperature {
    fn get_si_value(&self) -> Float {
        self.get::<kelvin>()
    }
}
impl ThermodynamicQuantity for WetBulbTemperature {
    fn get_si_value(&self) -> Float {
        self.get::<kelvin>()
    }
}
impl ThermodynamicQuantity for DewPointTemperature {
    fn get_si_value(&self) -> Float {
        self.get::<kelvin>()
    }
}
impl ThermodynamicQuantity for VirtualTemperature {
    fn get_si_value(&self) -> Float {
        self.get::<kelvin>()
    }
}
impl ThermodynamicQuantity for PotentialTemperature {
    fn get_si_value(&self) -> Float {
        self.get::<kelvin>()
    }
}
impl ThermodynamicQuantity for EquivalentPotentialTemperature {
    fn get_si_value(&self) -> Float {
        self.get::<kelvin>()
    }
}
impl ThermodynamicQuantity for WetBulbPotentialTemperature {
    fn get_si_value(&self) -> Float {
        self.get::<kelvin>()
    }
}

impl ThermodynamicQuantity for AtmosphericPressure {
    fn get_si_value(&self) -> Float {
        self.get::<pascal>()
    }
}
impl ThermodynamicQuantity for VapourPressure {
    fn get_si_value(&self) -> Float {
        self.get::<pascal>()
    }
}
impl ThermodynamicQuantity for SaturationVapourPressure {
    fn get_si_value(&self) -> Float {
        self.get::<pascal>()
    }
}

impl ThermodynamicQuantity for VapourPressureDeficit {
    fn get_si_value(&self) -> Float {
        self.get::<pascal>()
    }
}

impl ThermodynamicQuantity for MixingRatio {
    fn get_si_value(&self) -> Float {
        self.get::<ratio>()
    }
}

impl ThermodynamicQuantity for SaturationMixingRatio {
    fn get_si_value(&self) -> Float {
        self.get::<ratio>()
    }
}

impl ThermodynamicQuantity for SpecificHumidity {
    fn get_si_value(&self) -> Float {
        self.get::<ratio>()
    }
}
impl ThermodynamicQuantity for RelativeHumidity {
    fn get_si_value(&self) -> Float {
        self.get::<ratio>()
    }
}
