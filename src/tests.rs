pub(crate) mod reference_values;
pub(crate) mod testing_traits;

use crate::errors::InputError;
use crate::formula::{Formula1, Formula2, Formula3};
use crate::Float;
use float_cmp::assert_approx_eq;
use std::marker::PhantomData;
use std::mem::discriminant;

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

    dbg!(result, expected, eps);

    assert_approx_eq!(Float, result, expected, epsilon = eps)
}

pub fn test_with_2args<
    O: TestingQuantity,
    I1: TestingQuantity,
    I2: TestingQuantity,
    F: Formula2<O, I1, I2>,
>(
    arg1: Argument<I1>,
    arg2: Argument<I2>,
    atm: ReferenceAtmosphere,
    eps: Float,
) {
    //the first promise of the crate is that returned value
    //is calculated correctly
    let result = F::compute(arg1.ref_val(atm), arg2.ref_val(atm)).unwrap();
    check_result(result, atm, eps);

    // the second promise of the crate is to never return NaN or Inf
    // here we check several edge cases for that
    // the function can only return a finite number or InputError
    // check for error implicit as Result<> ensures that if value
    // is not Ok() then it is Err()
    // here we don't care about error being correct
    let results = vec![
        F::compute(arg1.ref_val(atm), arg2.ref_val(atm)),
        F::compute(I1::new_si(-9999.0), arg2.ref_val(atm)),
        F::compute(arg1.ref_val(atm), I2::new_si(-9999.0)),
        F::compute(I1::new_si(-9999.0), I2::new_si(-9999.0)),
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
            let arg1_tmp =
                (((arg1.range[1] - arg1.range[0]) / 100.0) * arg1_itr as Float) + arg1.range[0];
            let arg2_tmp =
                (((arg2.range[1] - arg2.range[0]) / 100.0) * arg2_itr as Float) + arg2.range[0];

            let arg1_tmp = I1::new_si(arg1_tmp);
            let arg2_tmp = I2::new_si(arg2_tmp);

            let result = F::compute(arg1_tmp, arg2_tmp);

            match result {
                Ok(r) => assert!(r.get_si_value().is_finite()),
                Err(e) => assert_eq!(
                    discriminant(&InputError::IncorrectArgumentSet(String::new())),
                    discriminant(&e)
                ),
            }
        }
    }

    //the fourth promise of the crate is to return an error with
    //erronous variable name when input is out of range
    let expected = InputError::OutOfRange(arg1.quantity_name());
    let result = F::compute(I1::new_si(arg1.range[0] - 0.1), arg2.ref_val(atm)).unwrap_err();
    assert_eq!(result, expected);
    let result = F::compute(I1::new_si(arg1.range[1] + 0.1), arg2.ref_val(atm)).unwrap_err();
    assert_eq!(result, expected);

    let expected = InputError::OutOfRange(arg2.quantity_name());
    let result = F::compute(arg1.ref_val(atm), I2::new_si(arg2.range[0] - 0.1)).unwrap_err();
    assert_eq!(result, expected);
    let result = F::compute(arg1.ref_val(atm), I2::new_si(arg2.range[1] + 0.1)).unwrap_err();
    assert_eq!(result, expected);
}

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

// pub fn test_with_3args<
//     O: ThermodynamicQuantity,
//     I1: ThermodynamicQuantity + TestingQuantity,
//     I2: ThermodynamicQuantity + TestingQuantity,
//     I3: ThermodynamicQuantity + TestingQuantity,
//     F: Formula3<O, I1, I2, I3>,
// >(
//     arg1: Argument<I1>,
//     arg2: Argument<I2>,
//     arg3: Argument<I3>,
//     expected_result: Float,
// ) {
//     //the first promise of the crate is that returned value
//     //is calculated correctly
//     let result = F::compute(I1::default_si(), I2::default_si(), I3::default_si()).unwrap();
//     assert_approx_eq!(
//         Float,
//         result.get_si_value(),
//         expected_result,
//         epsilon = 0.01
//     );

//     // the second promise of the crate is to never return NaN or Inf
//     // here we check several edge cases for that
//     // the function can only return a finite number or InputError
//     // check for error implicit as Result<> ensures that if value
//     // is not Ok() then it is Err()
//     // here we don't care about error being correct
//     let results = vec![
//         F::compute(I1::default_si(), I2::default_si(), I3::default_si()),
//         F::compute(I1::new_si(-9999.0), I2::default_si(), I3::default_si()),
//         F::compute(I1::default_si(), I2::new_si(-9999.0), I3::default_si()),
//         F::compute(I1::default_si(), I2::default_si(), I3::new_si(-9999.0)),
//         F::compute(I1::default_si(), I2::new_si(-9999.0), I3::default_si()),
//         F::compute(I1::default_si(), I2::new_si(-9999.0), I3::new_si(-9999.0)),
//         F::compute(I1::new_si(-9999.0), I2::default_si(), I3::new_si(-9999.0)),
//         F::compute(
//             I1::new_si(-9999.0),
//             I2::new_si(-9999.0),
//             I3::new_si(-9999.0),
//         ),
//     ];

//     for result in results {
//         if let Ok(result) = result {
//             assert!(result.get_si_value().is_finite());
//         }
//     }

//     //the third promise of the crate is to always return finite f64
//     //if all inputs are within the range
//     //the only allowed error is InccorectArgumentsSet as it can occur
//     //for values within valid range
//     for arg1_itr in 0..=100 {
//         for arg2_itr in 0..=100 {
//             for arg3_itr in 0..=100 {
//                 let arg1_tmp =
//                     (((arg1.range[1] - arg1.range[0]) / 100.0) * arg1_itr as Float) + arg1.range[0];
//                 let arg2_tmp =
//                     (((arg2.range[1] - arg2.range[0]) / 100.0) * arg2_itr as Float) + arg2.range[0];
//                 let arg3_tmp =
//                     (((arg3.range[1] - arg3.range[0]) / 100.0) * arg3_itr as Float) + arg3.range[0];

//                 let arg1_tmp = I1::new_si(arg1_tmp);
//                 let arg2_tmp = I2::new_si(arg2_tmp);
//                 let arg3_tmp = I3::new_si(arg3_tmp);

//                 let result = F::compute(arg1_tmp, arg2_tmp, arg3_tmp);

//                 match result {
//                     Ok(r) => assert!(r.get_si_value().is_finite()),
//                     Err(e) => assert_eq!(
//                         discriminant(&InputError::IncorrectArgumentSet(String::new())),
//                         discriminant(&e)
//                     ),
//                 }
//             }
//         }
//     }

//     //the fourth promise of the crate is to return an error with
//     //erronous variable name when input is out of range
//     let expected = InputError::OutOfRange(arg1.quantity_name());
//     let result = F::compute(
//         I1::new_si(arg1.range[0] - 0.1),
//         I2::default_si(),
//         I3::default_si(),
//     )
//     .unwrap_err();
//     assert_eq!(result, expected);
//     let result = F::compute(
//         I1::new_si(arg1.range[1] + 0.1),
//         I2::default_si(),
//         I3::default_si(),
//     )
//     .unwrap_err();
//     assert_eq!(result, expected);

//     let expected = InputError::OutOfRange(arg2.quantity_name());
//     let result = F::compute(
//         I1::default_si(),
//         I2::new_si(arg2.range[0] - 0.1),
//         I3::default_si(),
//     )
//     .unwrap_err();
//     assert_eq!(result, expected);
//     let result = F::compute(
//         I1::default_si(),
//         I2::new_si(arg2.range[1] + 0.1),
//         I3::default_si(),
//     )
//     .unwrap_err();
//     assert_eq!(result, expected);

//     let expected = InputError::OutOfRange(arg3.quantity_name());
//     let result = F::compute(
//         I1::default_si(),
//         I2::default_si(),
//         I3::new_si(arg3.range[0] - 0.1),
//     )
//     .unwrap_err();
//     assert_eq!(result, expected);
//     let result = F::compute(
//         I1::default_si(),
//         I2::default_si(),
//         I3::new_si(arg3.range[1] + 0.1),
//     )
//     .unwrap_err();
//     assert_eq!(result, expected);
// }
