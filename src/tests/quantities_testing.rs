use uom::si::{
    pressure::{pascal, pound_force_per_square_inch},
    ratio::{per_mille, percent, ratio},
    thermodynamic_temperature::{degree_fahrenheit, kelvin},
};

use crate::quantities::*;

pub(crate) trait TestingQuantity: ThermodynamicQuantity {
    fn default_si(&self) -> Self;
    fn default_imperial(&self) -> Self;
}

impl TestingQuantity for DryBulbTemperature {
    fn default_si(&self) -> Self {
        Self::new::<kelvin>(300.0)
    }
    fn default_imperial(&self) -> Self {
        let value = &self.default_si().0.get::<degree_fahrenheit>();

        Self::new::<degree_fahrenheit>(*value)
    }
}

impl TestingQuantity for DewPointTemperature {
    fn default_si(&self) -> Self {
        Self::new::<kelvin>(290.0)
    }
    fn default_imperial(&self) -> Self {
        let value = &self.default_si().0.get::<degree_fahrenheit>();

        Self::new::<degree_fahrenheit>(*value)
    }
}

impl TestingQuantity for EquivalentPotentialTemperature {
    fn default_si(&self) -> Self {
        Self::new::<kelvin>(300.0)
    }
    fn default_imperial(&self) -> Self {
        let value = &self.default_si().0.get::<degree_fahrenheit>();

        Self::new::<degree_fahrenheit>(*value)
    }
}

impl TestingQuantity for AtmosphericPressure {
    fn default_si(&self) -> Self {
        Self::new::<pascal>(101325.0)
    }
    fn default_imperial(&self) -> Self {
        let value = &self.default_si().0.get::<pound_force_per_square_inch>();

        Self::new::<pound_force_per_square_inch>(*value)
    }
}

impl TestingQuantity for VapourPressure {
    fn default_si(&self) -> Self {
        Self::new::<pascal>(1706.0)
    }
    fn default_imperial(&self) -> Self {
        let value = &self.default_si().0.get::<pound_force_per_square_inch>();

        Self::new::<pound_force_per_square_inch>(*value)
    }
}

impl TestingQuantity for SaturationVapourPressure {
    fn default_si(&self) -> Self {
        Self::new::<pascal>(2339.0)
    }
    fn default_imperial(&self) -> Self {
        let value = &self.default_si().0.get::<pound_force_per_square_inch>();

        Self::new::<pound_force_per_square_inch>(*value)
    }
}

impl TestingQuantity for MixingRatio {
    fn default_si(&self) -> Self {
        Self::new::<ratio>(0.01064)
    }
    fn default_imperial(&self) -> Self {
        let value = &self.default_si().0.get::<per_mille>();

        Self::new::<per_mille>(*value)
    }
}

impl TestingQuantity for SaturationMixingRatio {
    fn default_si(&self) -> Self {
        Self::new::<ratio>(0.01467)
    }
    fn default_imperial(&self) -> Self {
        let value = &self.default_si().0.get::<per_mille>();

        Self::new::<per_mille>(*value)
    }
}

impl TestingQuantity for RelativeHumidity {
    fn default_si(&self) -> Self {
        Self::new::<ratio>(0.5)
    }
    fn default_imperial(&self) -> Self {
        let value = &self.default_si().0.get::<percent>();

        Self::new::<percent>(*value)
    }
}

impl TestingQuantity for SpecificHumidity {
    fn default_si(&self) -> Self {
        Self::new::<ratio>(0.022)
    }
    fn default_imperial(&self) -> Self {
        let value = &self.default_si().0.get::<per_mille>();

        Self::new::<per_mille>(*value)
    }
}
