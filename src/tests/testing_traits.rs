use uom::si::{
    pressure::{pascal, pound_force_per_square_inch},
    ratio::{per_mille, percent, ratio},
    thermodynamic_temperature::{degree_fahrenheit, kelvin},
};

use super::reference_values::*;
use crate::{quantities::*, Float};

#[derive(Copy, Clone, Debug)]
pub(crate) enum ReferenceAtmosphere {
    Normal,
    Freezing,
}

pub(crate) trait TestingQuantity: ThermodynamicQuantity {
    fn new_si(value: Float) -> Self;
    fn imperial(&self) -> Self;
    fn ref_val_si(atm: ReferenceAtmosphere) -> Self;
}

impl TestingQuantity for DryBulbTemperature {
    fn new_si(value: Float) -> Self {
        Self::new::<kelvin>(value)
    }

    fn imperial(&self) -> Self {
        let value = self.0.get::<degree_fahrenheit>();

        Self::new::<degree_fahrenheit>(value)
    }

    fn ref_val_si(atm: ReferenceAtmosphere) -> Self {
        match atm {
            ReferenceAtmosphere::Normal => Self::new::<kelvin>(TEMP_NORM),
            ReferenceAtmosphere::Freezing => Self::new::<kelvin>(TEMP_FREEZ),
        }
    }
}

impl TestingQuantity for DewPointTemperature {
    fn new_si(value: Float) -> Self {
        Self::new::<kelvin>(value)
    }

    fn imperial(&self) -> Self {
        let value = self.0.get::<degree_fahrenheit>();

        Self::new::<degree_fahrenheit>(value)
    }

    fn ref_val_si(atm: ReferenceAtmosphere) -> Self {
        match atm {
            ReferenceAtmosphere::Normal => Self::new::<kelvin>(DWPT_NORM),
            ReferenceAtmosphere::Freezing => Self::new::<kelvin>(DWPT_FREEZ),
        }
    }
}

impl TestingQuantity for EquivalentPotentialTemperature {
    fn new_si(value: Float) -> Self {
        Self::new::<kelvin>(value)
    }

    fn imperial(&self) -> Self {
        let value = self.0.get::<degree_fahrenheit>();

        Self::new::<degree_fahrenheit>(value)
    }

    fn ref_val_si(atm: ReferenceAtmosphere) -> Self {
        match atm {
            ReferenceAtmosphere::Normal => Self::new::<kelvin>(THETAE_NORM),
            ReferenceAtmosphere::Freezing => Self::new::<kelvin>(THETAE_FREEZ),
        }
    }
}

impl TestingQuantity for AtmosphericPressure {
    fn new_si(value: Float) -> Self {
        Self::new::<pascal>(value)
    }

    fn imperial(&self) -> Self {
        let value = self.0.get::<pound_force_per_square_inch>();

        Self::new::<pound_force_per_square_inch>(value)
    }

    fn ref_val_si(atm: ReferenceAtmosphere) -> Self {
        match atm {
            ReferenceAtmosphere::Normal => Self::new::<pascal>(PRES_NORM),
            ReferenceAtmosphere::Freezing => Self::new::<pascal>(PRES_FREEZ),
        }
    }
}

impl TestingQuantity for VapourPressure {
    fn new_si(value: Float) -> Self {
        Self::new::<pascal>(value)
    }

    fn imperial(&self) -> Self {
        let value = self.0.get::<pound_force_per_square_inch>();

        Self::new::<pound_force_per_square_inch>(value)
    }

    fn ref_val_si(atm: ReferenceAtmosphere) -> Self {
        match atm {
            ReferenceAtmosphere::Normal => Self::new::<pascal>(VP_NORM),
            ReferenceAtmosphere::Freezing => Self::new::<pascal>(VP_FREEZ),
        }
    }
}

impl TestingQuantity for SaturationVapourPressure {
    fn ref_val_si(atm: ReferenceAtmosphere) -> Self {
        match atm {
            ReferenceAtmosphere::Normal => Self::new::<pascal>(SVP_NORM),
            ReferenceAtmosphere::Freezing => Self::new::<pascal>(SVP_FREEZ),
        }
    }

    fn new_si(value: Float) -> Self {
        Self::new::<pascal>(value)
    }

    fn imperial(&self) -> Self {
        let value = self.0.get::<pound_force_per_square_inch>();

        Self::new::<pound_force_per_square_inch>(value)
    }
}

impl TestingQuantity for MixingRatio {
    fn ref_val_si(atm: ReferenceAtmosphere) -> Self {
        match atm {
            ReferenceAtmosphere::Normal => Self::new::<ratio>(MR_NORM),
            ReferenceAtmosphere::Freezing => Self::new::<ratio>(MR_FREEZ),
        }
    }

    fn new_si(value: Float) -> Self {
        Self::new::<ratio>(value)
    }

    fn imperial(&self) -> Self {
        let value = self.0.get::<per_mille>();

        Self::new::<per_mille>(value)
    }
}

impl TestingQuantity for SaturationMixingRatio {
    fn new_si(value: Float) -> Self {
        Self::new::<ratio>(value)
    }

    fn imperial(&self) -> Self {
        let value = self.0.get::<per_mille>();

        Self::new::<per_mille>(value)
    }

    fn ref_val_si(atm: ReferenceAtmosphere) -> Self {
        match atm {
            ReferenceAtmosphere::Normal => Self::new::<ratio>(SMR_NROM),
            ReferenceAtmosphere::Freezing => Self::new::<ratio>(SMR_FREEZ),
        }
    }
}

impl TestingQuantity for RelativeHumidity {
    fn new_si(value: Float) -> Self {
        Self::new::<ratio>(value)
    }

    fn imperial(&self) -> Self {
        let value = self.0.get::<percent>();

        Self::new::<percent>(value)
    }

    fn ref_val_si(atm: ReferenceAtmosphere) -> Self {
        match atm {
            ReferenceAtmosphere::Normal => Self::new::<ratio>(RH_NORM),
            ReferenceAtmosphere::Freezing => Self::new::<ratio>(RH_FREEZ),
        }
    }
}

impl TestingQuantity for SpecificHumidity {
    fn new_si(value: Float) -> Self {
        Self::new::<ratio>(value)
    }

    fn imperial(&self) -> Self {
        let value = self.0.get::<per_mille>();

        Self::new::<per_mille>(value)
    }

    fn ref_val_si(atm: ReferenceAtmosphere) -> Self {
        match atm {
            ReferenceAtmosphere::Normal => Self::new::<ratio>(SH_NORM),
            ReferenceAtmosphere::Freezing => Self::new::<ratio>(SH_FREEZ),
        }
    }
}
