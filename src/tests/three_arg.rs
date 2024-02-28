use float_cmp::assert_approx_eq;
use itertools::multiunzip;
use ndarray::Array1;

use super::check_result;
use super::testing_traits::{ReferenceAtmosphere, TestingQuantity};
use super::Argument;
use crate::errors::InputError;
use crate::Float;
use crate::Formula3;
use std::mem::discriminant;

pub fn test_with_3args<
    O: TestingQuantity,
    I1: TestingQuantity,
    I2: TestingQuantity,
    I3: TestingQuantity,
    F: Formula3<O, I1, I2, I3>,
>(
    arg1: Argument<I1>,
    arg2: Argument<I2>,
    arg3: Argument<I3>,
    atm: ReferenceAtmosphere,
    eps: Float,
) {
    //the first promise of the crate is that returned value
    //is calculated correctly
    let ref_result = F::compute(arg1.ref_val(atm), arg2.ref_val(atm), arg3.ref_val(atm)).unwrap();
    check_result(ref_result, atm, eps);

    // the second promise of the crate is to never return NaN or Inf
    // here we check several edge cases for that
    // the function can only return a finite number or InputError
    // check for error implicit as Result<> ensures that if value
    // is not Ok() then it is Err()
    // here we don't care about error being correct
    let results = vec![
        F::compute(arg1.ref_val(atm), arg2.ref_val(atm), arg3.ref_val(atm)),
        F::compute(I1::new_si(-9999.0), arg2.ref_val(atm), arg3.ref_val(atm)),
        F::compute(arg1.ref_val(atm), I2::new_si(-9999.0), arg3.ref_val(atm)),
        F::compute(arg1.ref_val(atm), arg2.ref_val(atm), I3::new_si(-9999.0)),
        F::compute(arg1.ref_val(atm), I2::new_si(-9999.0), arg3.ref_val(atm)),
        F::compute(arg1.ref_val(atm), I2::new_si(-9999.0), I3::new_si(-9999.0)),
        F::compute(I1::new_si(-9999.0), arg2.ref_val(atm), I3::new_si(-9999.0)),
        F::compute(
            I1::new_si(-9999.0),
            I2::new_si(-9999.0),
            I3::new_si(-9999.0),
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
    for arg1_itr in 0..=100 {
        for arg2_itr in 0..=100 {
            for arg3_itr in 0..=100 {
                let arg1_tmp =
                    (((arg1.range[1] - arg1.range[0]) / 100.0) * arg1_itr as Float) + arg1.range[0];
                let arg2_tmp =
                    (((arg2.range[1] - arg2.range[0]) / 100.0) * arg2_itr as Float) + arg2.range[0];
                let arg3_tmp =
                    (((arg3.range[1] - arg3.range[0]) / 100.0) * arg3_itr as Float) + arg3.range[0];

                let arg1_tmp = I1::new_si(arg1_tmp);
                let arg2_tmp = I2::new_si(arg2_tmp);
                let arg3_tmp = I3::new_si(arg3_tmp);

                let result = F::compute(arg1_tmp, arg2_tmp, arg3_tmp);

                match result {
                    Ok(r) => assert!(r.get_si_value().is_finite()),
                    Err(e) => assert_eq!(
                        discriminant(&InputError::IncorrectArgumentSet(String::new())),
                        discriminant(&e)
                    ),
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
    )
    .unwrap_err();
    assert_eq!(result, expected);
    let result = F::compute(
        I1::new_si(arg1.range[1] + 0.1),
        arg2.ref_val(atm),
        arg3.ref_val(atm),
    )
    .unwrap_err();
    assert_eq!(result, expected);

    let expected = InputError::OutOfRange(arg2.quantity_name());
    let result = F::compute(
        arg1.ref_val(atm),
        I2::new_si(arg2.range[0] - 0.1),
        arg3.ref_val(atm),
    )
    .unwrap_err();
    assert_eq!(result, expected);
    let result = F::compute(
        arg1.ref_val(atm),
        I2::new_si(arg2.range[1] + 0.1),
        arg3.ref_val(atm),
    )
    .unwrap_err();
    assert_eq!(result, expected);

    let expected = InputError::OutOfRange(arg3.quantity_name());
    let result = F::compute(
        arg1.ref_val(atm),
        arg2.ref_val(atm),
        I3::new_si(arg3.range[0] - 0.1),
    )
    .unwrap_err();
    assert_eq!(result, expected);
    let result = F::compute(
        arg1.ref_val(atm),
        arg2.ref_val(atm),
        I3::new_si(arg3.range[1] + 0.1),
    )
    .unwrap_err();
    assert_eq!(result, expected);

    let arg_vecs = (-10..=10).map(|i| i as Float / 1000.0).map(|i| {
        (
            I1::new_si(arg1.ref_val(atm).get_si_value() + i),
            I2::new_si(arg2.ref_val(atm).get_si_value() + i),
            I3::new_si(arg3.ref_val(atm).get_si_value() + i),
        )
    });

    let arg_vecs: (Vec<_>, Vec<_>, Vec<_>) = multiunzip(arg_vecs);

    let arg_arrs = (
        Array1::from(arg_vecs.0.clone()),
        Array1::from(arg_vecs.1.clone()),
        Array1::from(arg_vecs.2.clone()),
    );

    let result_vec = F::compute_vec(&arg_vecs.0, &arg_vecs.1, &arg_vecs.2).unwrap();
    assert_approx_eq!(
        Float,
        ref_result.get_si_value(),
        result_vec[10].get_si_value(),
        ulps = 4
    );

    let result_arr = F::compute_ndarray(&arg_arrs.0, &arg_arrs.1, &arg_arrs.2).unwrap();
    assert_approx_eq!(
        Float,
        ref_result.get_si_value(),
        result_arr[10].get_si_value(),
        ulps = 4
    );

    let result_vec = F::compute_vec_parallel(&arg_vecs.0, &arg_vecs.1, &arg_vecs.2).unwrap();
    assert_approx_eq!(
        Float,
        ref_result.get_si_value(),
        result_vec[10].get_si_value(),
        ulps = 4
    );

    let result_arr = F::compute_ndarray_parallel(&arg_arrs.0, &arg_arrs.1, &arg_arrs.2).unwrap();
    assert_approx_eq!(
        Float,
        ref_result.get_si_value(),
        result_arr[10].get_si_value(),
        ulps = 4
    );

    let result_imperial = F::compute(
        arg1.ref_val(atm).imperial(),
        arg2.ref_val(atm).imperial(),
        arg3.ref_val(atm).imperial(),
    )
    .unwrap();

    assert_approx_eq!(
        Float,
        ref_result.get_si_value(),
        result_imperial.get_si_value(),
        epsilon = 1e-12
    );

    testing_logger::setup();
    let _ = F::compute(
        I1::new_si(-9999.0),
        I2::new_si(-9999.0),
        I3::new_si(-9999.0),
    );

    testing_logger::validate(|captured_logs| {
        assert_eq!(captured_logs.len(), 1);
        let body = &captured_logs[0].body;
        assert!(body.contains("Formula"));
        assert!(body.contains("calculating"));
        assert!(body.contains("from"));
        assert!(body.contains("inputs"));
        assert!(body.contains("returned error:"));
        assert_eq!(captured_logs[0].level, log::Level::Error);
    });
}
