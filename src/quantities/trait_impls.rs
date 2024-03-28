use super::*;
use uom::si::{pressure::pascal, ratio::ratio, thermodynamic_temperature::kelvin};

impl ThermodynamicQuantity for DryBulbTemperature {}
impl ThermodynamicQuantity for WetBulbTemperature {}
impl ThermodynamicQuantity for DewPointTemperature {}
impl ThermodynamicQuantity for VirtualTemperature {}
impl ThermodynamicQuantity for PotentialTemperature {}
impl ThermodynamicQuantity for EquivalentPotentialTemperature {}
impl ThermodynamicQuantity for WetBulbPotentialTemperature {}
impl ThermodynamicQuantity for AtmosphericPressure {}
impl ThermodynamicQuantity for VapourPressure {}
impl ThermodynamicQuantity for SaturationVapourPressure {}
impl ThermodynamicQuantity for VapourPressureDeficit {}
impl ThermodynamicQuantity for MixingRatio {}
impl ThermodynamicQuantity for SaturationMixingRatio {}
impl ThermodynamicQuantity for SpecificHumidity {}
impl ThermodynamicQuantity for RelativeHumidity {}

impl QuantityHelpers for DryBulbTemperature {
    fn get_si_value(&self) -> Float {
        self.get::<kelvin>()
    }
}
impl QuantityHelpers for WetBulbTemperature {
    fn get_si_value(&self) -> Float {
        self.get::<kelvin>()
    }
}
impl QuantityHelpers for DewPointTemperature {
    fn get_si_value(&self) -> Float {
        self.get::<kelvin>()
    }
}
impl QuantityHelpers for VirtualTemperature {
    fn get_si_value(&self) -> Float {
        self.get::<kelvin>()
    }
}
impl QuantityHelpers for PotentialTemperature {
    fn get_si_value(&self) -> Float {
        self.get::<kelvin>()
    }
}
impl QuantityHelpers for EquivalentPotentialTemperature {
    fn get_si_value(&self) -> Float {
        self.get::<kelvin>()
    }
}
impl QuantityHelpers for WetBulbPotentialTemperature {
    fn get_si_value(&self) -> Float {
        self.get::<kelvin>()
    }
}

impl QuantityHelpers for AtmosphericPressure {
    fn get_si_value(&self) -> Float {
        self.get::<pascal>()
    }
}
impl QuantityHelpers for VapourPressure {
    fn get_si_value(&self) -> Float {
        self.get::<pascal>()
    }
}
impl QuantityHelpers for SaturationVapourPressure {
    fn get_si_value(&self) -> Float {
        self.get::<pascal>()
    }
}

impl QuantityHelpers for VapourPressureDeficit {
    fn get_si_value(&self) -> Float {
        self.get::<pascal>()
    }
}

impl QuantityHelpers for MixingRatio {
    fn get_si_value(&self) -> Float {
        self.get::<ratio>()
    }
}

impl QuantityHelpers for SaturationMixingRatio {
    fn get_si_value(&self) -> Float {
        self.get::<ratio>()
    }
}

impl QuantityHelpers for SpecificHumidity {
    fn get_si_value(&self) -> Float {
        self.get::<ratio>()
    }
}
impl QuantityHelpers for RelativeHumidity {
    fn get_si_value(&self) -> Float {
        self.get::<ratio>()
    }
}
