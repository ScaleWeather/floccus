#![allow(missing_docs)]

use std::marker::PhantomData;

pub trait Quantity<F: Family> {}

pub trait Family {}

pub trait Length: Family {}
pub trait Mass: Family {}
pub trait Time: Family {}
pub trait Temperature: Family {}
pub trait Pressure: Family {}
pub trait Ratio: Family {}

pub struct VaporPressure<F: Pressure> {
    _family: PhantomData<F>,
}

impl<F: Pressure> Quantity<F> for VaporPressure<F> {}
