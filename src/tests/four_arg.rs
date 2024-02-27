use super::check_result;
use super::testing_traits::{ReferenceAtmosphere, TestingQuantity};
use super::Argument;
use crate::errors::InputError;
use crate::Float;
use crate::Formula4;
use std::mem::discriminant;

pub fn test_with_4args<
    O: TestingQuantity,
    I1: TestingQuantity,
    I2: TestingQuantity,
    I3: TestingQuantity,
    I4: TestingQuantity,
    F: Formula4<O, I1, I2, I3, I4>,
>(
    arg1: Argument<I1>,
    arg2: Argument<I2>,
    arg3: Argument<I3>,
    arg4: Argument<I4>,
    atm: ReferenceAtmosphere,
    eps: Float,
) {
    let result = F::compute(
        arg1.ref_val(atm),
        arg2.ref_val(atm),
        arg3.ref_val(atm),
        arg4.ref_val(atm),
    )
    .unwrap();
    check_result(result, atm, eps);

    let results = vec![
        F::compute(
            arg1.ref_val(atm),
            arg2.ref_val(atm),
            arg3.ref_val(atm),
            arg4.ref_val(atm),
        ),
        F::compute(
            I1::new_si(-9999.0),
            arg2.ref_val(atm),
            arg3.ref_val(atm),
            arg4.ref_val(atm),
        ),
        F::compute(
            arg1.ref_val(atm),
            I2::new_si(-9999.0),
            arg3.ref_val(atm),
            arg4.ref_val(atm),
        ),
        F::compute(
            arg1.ref_val(atm),
            arg2.ref_val(atm),
            I3::new_si(-9999.0),
            arg4.ref_val(atm),
        ),
        F::compute(
            arg1.ref_val(atm),
            arg2.ref_val(atm),
            arg3.ref_val(atm),
            I4::new_si(-9999.0),
        ),
        F::compute(
            arg1.ref_val(atm),
            I2::new_si(-9999.0),
            I3::new_si(-9999.0),
            arg4.ref_val(atm),
        ),
        F::compute(
            I1::new_si(-9999.0),
            I2::new_si(-9999.0),
            arg3.ref_val(atm),
            arg4.ref_val(atm),
        ),
        F::compute(
            I1::new_si(-9999.0),
            arg2.ref_val(atm),
            I3::new_si(-9999.0),
            arg4.ref_val(atm),
        ),
        F::compute(
            I1::new_si(-9999.0),
            arg2.ref_val(atm),
            arg3.ref_val(atm),
            I4::new_si(-9999.0),
        ),
        F::compute(
            I1::new_si(-9999.0),
            I2::new_si(-9999.0),
            I3::new_si(-9999.0),
            I4::new_si(-9999.0),
        ),
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
    for arg1_itr in 0..=20 {
        for arg2_itr in 0..=20 {
            for arg3_itr in 0..=20 {
                for arg4_itr in 0..=20 {
                    let arg1_tmp = (((arg1.range[1] - arg1.range[0]) / 20.0) * arg1_itr as Float)
                        + arg1.range[0];
                    let arg2_tmp = (((arg2.range[1] - arg2.range[0]) / 20.0) * arg2_itr as Float)
                        + arg2.range[0];
                    let arg3_tmp = (((arg3.range[1] - arg3.range[0]) / 20.0) * arg3_itr as Float)
                        + arg3.range[0];
                    let arg4_tmp = (((arg4.range[1] - arg4.range[0]) / 20.0) * arg4_itr as Float)
                        + arg4.range[0];

                    let arg1_tmp = I1::new_si(arg1_tmp);
                    let arg2_tmp = I2::new_si(arg2_tmp);
                    let arg3_tmp = I3::new_si(arg3_tmp);
                    let arg4_tmp = I4::new_si(arg4_tmp);

                    let result = F::compute(arg1_tmp, arg2_tmp, arg3_tmp, arg4_tmp);

                    match result.clone() {
                        Ok(r) => assert!(r.get_si_value().is_finite()),

                        Err(e) => assert_eq!(
                            discriminant(&InputError::IncorrectArgumentSet(String::new())),
                            discriminant(&e)
                        ),
                    }
                }
            }
        }
    }

    //the fourth promise of the crate is to return an error with
    //erronous variable name when input is out of range
    let expected = InputError::OutOfRange(arg1.quantity_name());
    let result = F::compute(
        I1::new_si(arg1.range[0] - 0.1),
        arg2.ref_val(atm),
        arg3.ref_val(atm),
        arg4.ref_val(atm),
    )
    .unwrap_err();
    assert_eq!(result, expected);
    let result = F::compute(
        I1::new_si(arg1.range[1] + 0.1),
        arg2.ref_val(atm),
        arg3.ref_val(atm),
        arg4.ref_val(atm),
    )
    .unwrap_err();
    assert_eq!(result, expected);

    let expected = InputError::OutOfRange(arg2.quantity_name());
    let result = F::compute(
        arg1.ref_val(atm),
        I2::new_si(arg2.range[0] - 0.1),
        arg3.ref_val(atm),
        arg4.ref_val(atm),
    )
    .unwrap_err();
    assert_eq!(result, expected);
    let result = F::compute(
        arg1.ref_val(atm),
        I2::new_si(arg2.range[1] + 0.1),
        arg3.ref_val(atm),
        arg4.ref_val(atm),
    )
    .unwrap_err();
    assert_eq!(result, expected);

    let expected = InputError::OutOfRange(arg3.quantity_name());
    let result = F::compute(
        arg1.ref_val(atm),
        arg2.ref_val(atm),
        I3::new_si(arg3.range[0] - 0.1),
        arg4.ref_val(atm),
    )
    .unwrap_err();
    assert_eq!(result, expected);
    let result = F::compute(
        arg1.ref_val(atm),
        arg2.ref_val(atm),
        I3::new_si(arg3.range[1] + 0.1),
        arg4.ref_val(atm),
    )
    .unwrap_err();
    assert_eq!(result, expected);

    let expected = InputError::OutOfRange(arg4.quantity_name());
    let result = F::compute(
        arg1.ref_val(atm),
        arg2.ref_val(atm),
        arg3.ref_val(atm),
        I4::new_si(arg4.range[0] - 0.1),
    )
    .unwrap_err();
    assert_eq!(result, expected);
    let result = F::compute(
        arg1.ref_val(atm),
        arg2.ref_val(atm),
        arg3.ref_val(atm),
        I4::new_si(arg4.range[1] + 0.1),
    )
    .unwrap_err();
    assert_eq!(result, expected);
}
