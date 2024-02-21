#![allow(missing_docs)]

use std::marker::PhantomData;

use crate::quantities::{Family, Pressure};

pub trait Unit<F: Family> {}

// pub struct Meter;
// pub struct Kilometer;
// pub struct Millimeter;

// pub struct Kilogram;
// pub struct Gram;

// pub struct Kelvin;
// pub struct Celsius;
// pub struct Fahrenheit;

pub struct Pascal<F: Pressure> {
    _family: PhantomData<F>,
}
pub struct HectoPascal<F: Pressure> {
    _family: PhantomData<F>,
}
pub struct KiloPascal<F: Pressure> {
    _family: PhantomData<F>,
}

// pub struct Percent;
// pub struct Decimal;

// pub struct Second;
// pub struct Minute;
// pub struct Hour;

impl<F: Pressure> Unit<F> for Pascal<F> {}
impl<F: Pressure> Unit<F> for HectoPascal<F> {}
impl<F: Pressure> Unit<F> for KiloPascal<F> {}
