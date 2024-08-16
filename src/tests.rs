mod four_arg;
mod one_arg;
pub(crate) mod reference_values;
pub(crate) mod testing_traits;
mod three_arg;
mod two_arg;

use crate::{Float, InputError};
use float_cmp::assert_approx_eq;
use std::marker::PhantomData;

pub use self::four_arg::test_with_4args;
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

    pub fn quantity_name(&self) -> &str {
        I::type_name_as_str()
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

pub fn check_range_error(result: InputError, expected_name: &str) {
    if let InputError::OutOfRange(name) = result {
        assert_eq!(name, expected_name)
    } else {
        panic!("wrong error type")
    }
}
