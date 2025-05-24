#![allow(missing_docs)]

use crate::{Float, errors::InputError};
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

    #[inline]
    fn check_range_si(&self, lower_bound: Float, upper_bound: Float) -> Result<(), InputError> {
        if !(lower_bound..=upper_bound).contains(&self.get_si_value()) {
            return Err(InputError::OutOfRange(Self::name()));
        }

        Ok(())
    }
}

macro_rules! define_quantity {
    ($quantity:ident, $storage:ident, $uom_module:ident, $si_unit:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
        pub struct $quantity(pub crate::Storage::$storage);

        impl $quantity {
            pub fn get<T>(&self) -> Float
            where
                T: uom::si::$uom_module::Unit + uom::si::$uom_module::Conversion<Float>,
            {
                self.0.get::<T>()
            }

            pub fn new<T>(value: Float) -> Self
            where
                T: uom::si::$uom_module::Unit + uom::si::$uom_module::Conversion<Float>,
            {
                Self(crate::Storage::$storage::new::<T>(value))
            }
        }

        impl ThermodynamicQuantity for $quantity {}

        impl QuantityHelpers for $quantity {
            fn get_si_value(&self) -> Float {
                self.get::<uom::si::$uom_module::$si_unit>()
            }
        }
    };
}

macro_rules! define_temperature {
    ($quantity:ident) => {
        define_quantity!(
            $quantity,
            ThermodynamicTemperature,
            thermodynamic_temperature,
            kelvin
        );
    };
}

macro_rules! define_pressure {
    ($quantity:ident) => {
        define_quantity!($quantity, Pressure, pressure, pascal);
    };
}

macro_rules! define_ratio {
    ($quantity:ident) => {
        define_quantity!($quantity, Ratio, ratio, ratio);
    };
}

define_temperature!(DryBulbTemperature);
define_temperature!(WetBulbTemperature);
define_temperature!(DewPointTemperature);
define_temperature!(VirtualTemperature);
define_temperature!(IsobaricEquivalentTemperature);
define_temperature!(AdiabaticEquivalentTemperature);
define_temperature!(PotentialTemperature);
define_temperature!(EquivalentPotentialTemperature);
define_temperature!(WetBulbPotentialTemperature);

define_pressure!(AtmosphericPressure);
define_pressure!(VapourPressure);
define_pressure!(SaturationVapourPressure);
define_pressure!(VapourPressureDeficit);

define_ratio!(MixingRatio);
define_ratio!(SaturationMixingRatio);
define_ratio!(SpecificHumidity);
define_ratio!(RelativeHumidity);
