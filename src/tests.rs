mod four_arg;
mod one_arg;
pub(crate) mod reference_values;
pub(crate) mod testing_traits;
mod three_arg;
mod two_arg;

use crate::Float;
use float_cmp::assert_approx_eq;
use std::marker::PhantomData;

pub use self::one_arg::test_with_1arg;
pub use self::three_arg::test_with_3args;
pub use self::two_arg::test_with_2args;

use self::testing_traits::{ReferenceAtmosphere, TestingQuantity};

#[derive(Copy, Clone, Debug)]
pub struct Argument<I: TestingQuantity> {
    pub range: [Float; 2],
    _quantity: PhantomData<I>,
}

impl<I: TestingQuantity> Argument<I> {
    pub fn new(range: [Float; 2]) -> Self {
        Self {
            range,
            _quantity: PhantomData,
        }
    }

    pub fn quantity_name(&self) -> String {
        I::type_name_as_str().to_string()
    }

    pub fn ref_val(&self, atm: ReferenceAtmosphere) -> I {
        I::ref_val_si(atm)
    }
}

fn check_result<T: TestingQuantity>(result: T, atm: ReferenceAtmosphere, eps: Float) {
    let expected = T::ref_val_si(atm).get_si_value();
    let result = result.get_si_value();

    assert_approx_eq!(Float, result, expected, epsilon = eps)
}
