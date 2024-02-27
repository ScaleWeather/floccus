use super::check_result;
use super::testing_traits::{ReferenceAtmosphere, TestingQuantity};
use super::Argument;
use crate::errors::InputError;
use crate::Formula1;
use crate::Float;
use std::mem::discriminant;

pub fn test_with_1arg<O: TestingQuantity, I1: TestingQuantity, F: Formula1<O, I1>>(
    arg1: Argument<I1>,
    atm: ReferenceAtmosphere,
    eps: Float,
) {
    //the first promise of the crate is that returned value
    //is calculated correctly
    let result = F::compute(arg1.ref_val(atm)).unwrap();
    check_result(result, atm, eps);

    // the second promise of the crate is to never return NaN or Inf
    // here we check several edge cases for that
    // the function can only return a finite number or InputError
    // check for error implicit as Result<> ensures that if value
    // is not Ok() then it is Err()
    // here we don't care about error being correct
    let results = vec![
        F::compute(arg1.ref_val(atm)),
        F::compute(I1::new_si(-9999.0)),
    ];

    for result in results {
        if let Ok(result) = result {
            assert!(result.get_si_value().is_finite());
        }
    }

    //the third promise of the crate is to always return finite f64
    //if all inputs are within the range
    //the only allowed error is InccorectArgumentsSet as it can occur
    //for values within valid range
    for arg1_itr in 0..=100 {
        let arg1_tmp =
            (((arg1.range[1] - arg1.range[0]) / 100.0) * arg1_itr as Float) + arg1.range[0];

        let arg1_tmp = I1::new_si(arg1_tmp);

        let result = F::compute(arg1_tmp);

        match result {
            Ok(r) => assert!(r.get_si_value().is_finite()),
            Err(e) => assert_eq!(
                discriminant(&InputError::IncorrectArgumentSet(String::new())),
                discriminant(&e)
            ),
        }
    }

    //the fourth promise of the crate is to return an error with
    //erronous variable name when input is out of range
    let expected = InputError::OutOfRange(arg1.quantity_name());
    let result = F::compute(I1::new_si(arg1.range[0] - 0.1)).unwrap_err();
    assert_eq!(result, expected);
    let result = F::compute(I1::new_si(arg1.range[1] + 0.1)).unwrap_err();
    assert_eq!(result, expected);
}
