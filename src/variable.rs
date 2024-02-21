#![allow(dead_code)]
#![allow(missing_docs)]

use std::marker::PhantomData;

use crate::{
    quantities::{Family, Pressure, Quantity},
    units::{KiloPascal, Pascal, Unit},
};

pub struct Variable<F: Family, U: Unit<F>, Q: Quantity<F>> {
    pub value: f64,
    _quantity: PhantomData<Q>,
    _unit: PhantomData<U>,
    _family: PhantomData<F>,
}

impl<F: Family, U: Unit<F>, Q: Quantity<F>> Variable<F, U, Q> {
    pub fn new(value: f64) -> Self {
        Variable {
            value,
            _quantity: PhantomData::<Q>,
            _unit: PhantomData::<U>,
            _family: PhantomData::<F>,
        }
    }
}

pub trait UnitFrom<T>: Sized {
    fn from_convert(value: T) -> Self;
}

impl<F: Pressure, Q: Quantity<F>> UnitFrom<Variable<F, Pascal<F>, Q>>
    for Variable<F, KiloPascal<F>, Q>
{
    fn from_convert(value: Variable<F, Pascal<F>, Q>) -> Self {
        todo!()
    }
}
