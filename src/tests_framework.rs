use crate::errors::InputError;
use crate::Float;
use float_cmp::assert_approx_eq;
use std::mem::discriminant;

#[derive(Copy, Clone, Debug, Default)]
pub struct Argument {
    pub name: &'static str,
    pub def_val: Float,
    pub range: [Float; 2],
}

//due to a bug [https://github.com/rust-lang/rust/issues/46379]
//cargo flags those functions as a dead code even though
//they are used in tests
#[allow(dead_code)]
//this function should work as a reference for other test functions below
pub fn test_with_2args(
    tested_function: &dyn Fn(Float, Float) -> Result<Float, InputError>,
    arg1: Argument,
    arg2: Argument,
    expected_result: Float,
) -> bool {
    //the first promise of the crate is that returned value
    //is calculated correctly
    let result = tested_function(arg1.def_val, arg2.def_val).unwrap();
    assert_approx_eq!(Float, result, expected_result, epsilon = 0.01);

    //the second promise of the crate is to never return NaN or Inf
    //here we check several edge cases for that
    //the function can only return a finite number or InputError
    //check for error implicit as Result<> ensures that if value
    //is not Ok() then it is Err()
    //here we don't care about error being correct
    let results = vec![
        tested_function(0.0, arg2.def_val),
        tested_function(arg1.def_val, 0.0),
        tested_function(0.0, 0.0),
    ];

    for result in results {
        if result.is_ok() {
            assert!(result.unwrap().is_finite());
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

            let result = tested_function(arg1_tmp, arg2_tmp);

            if result.is_err() {
                assert!(
                    discriminant(&InputError::IncorrectArgumentSet(String::from("")))
                        == discriminant(&result.unwrap_err())
                );
            } else {
                assert!(result.unwrap().is_finite());
            }
        }
    }

    //the fourth promise of the crate is to return an error with
    //erronous variable name when input is out of range
    let expected = InputError::OutOfRange(String::from(arg1.name));
    let result = tested_function(arg1.range[0] - 0.1, arg2.def_val).unwrap_err();
    assert_eq!(result, expected);
    let result = tested_function(arg1.range[1] + 0.1, arg2.def_val).unwrap_err();
    assert_eq!(result, expected);

    let expected = InputError::OutOfRange(String::from(arg2.name));
    let result = tested_function(arg1.def_val, arg2.range[0] - 0.1).unwrap_err();
    assert_eq!(result, expected);
    let result = tested_function(arg1.def_val, arg2.range[1] + 0.1).unwrap_err();
    assert_eq!(result, expected);

    true
}

#[allow(dead_code)]
pub fn test_with_1arg(
    tested_function: &dyn Fn(Float) -> Result<Float, InputError>,
    arg1: Argument,
    expected_result: Float,
) -> bool {
    let result = tested_function(arg1.def_val).unwrap();
    assert_approx_eq!(Float, result, expected_result, epsilon = 0.01);

    let results = vec![tested_function(0.0)];

    for result in results {
        if result.is_ok() {
            assert!(result.unwrap().is_finite());
        }
    }

    for arg1_itr in 0..=100 {
        let arg1_tmp =
            (((arg1.range[1] - arg1.range[0]) / 100.0) * arg1_itr as Float) + arg1.range[0];

        let result = tested_function(arg1_tmp);

        if result.is_err() {
            assert!(
                discriminant(&InputError::IncorrectArgumentSet(String::from("")))
                    == discriminant(&result.unwrap_err())
            );
        } else {
            assert!(result.unwrap().is_finite());
        }
    }

    let expected = InputError::OutOfRange(String::from(arg1.name));
    let result = tested_function(arg1.range[0] - 0.1).unwrap_err();
    assert_eq!(result, expected);
    let result = tested_function(arg1.range[1] + 0.1).unwrap_err();
    assert_eq!(result, expected);

    true
}

#[allow(dead_code)]
pub fn test_with_3args(
    tested_function: &dyn Fn(Float, Float, Float) -> Result<Float, InputError>,
    arg1: Argument,
    arg2: Argument,
    arg3: Argument,
    expected_result: Float,
) -> bool {
    let result = tested_function(arg1.def_val, arg2.def_val, arg3.def_val).unwrap();
    assert_approx_eq!(Float, result, expected_result, epsilon = 0.01);

    let results = vec![
        tested_function(0.0, arg2.def_val, arg3.def_val),
        tested_function(arg1.def_val, 0.0, arg3.def_val),
        tested_function(arg1.def_val, arg2.def_val, 0.0),
        tested_function(0.0, 0.0, 0.0),
    ];

    for result in results {
        if result.is_ok() {
            assert!(result.unwrap().is_finite());
        }
    }

    for arg1_itr in 0..=100 {
        for arg2_itr in 0..=100 {
            for arg3_itr in 0..=100 {
                let arg1_tmp =
                    (((arg1.range[1] - arg1.range[0]) / 100.0) * arg1_itr as Float) + arg1.range[0];
                let arg2_tmp =
                    (((arg2.range[1] - arg2.range[0]) / 100.0) * arg2_itr as Float) + arg2.range[0];
                let arg3_tmp =
                    (((arg3.range[1] - arg3.range[0]) / 100.0) * arg3_itr as Float) + arg3.range[0];

                let result = tested_function(arg1_tmp, arg2_tmp, arg3_tmp);

                if result.is_err() {
                    assert!(
                        discriminant(&InputError::IncorrectArgumentSet(String::from("")))
                            == discriminant(&result.unwrap_err())
                    );
                } else {
                    println!("{} {} {} {:?}", arg1_tmp, arg2_tmp, arg3_tmp, result);
                    assert!(result.unwrap().is_finite());
                }
            }
        }
    }

    let expected = InputError::OutOfRange(String::from(arg1.name));
    let result = tested_function(arg1.range[0] - 0.1, arg2.def_val, arg3.def_val).unwrap_err();
    assert_eq!(result, expected);
    let result = tested_function(arg1.range[1] + 0.1, arg2.def_val, arg3.def_val).unwrap_err();
    assert_eq!(result, expected);

    let expected = InputError::OutOfRange(String::from(arg2.name));
    let result = tested_function(arg1.def_val, arg2.range[0] - 0.1, arg3.def_val).unwrap_err();
    assert_eq!(result, expected);
    let result = tested_function(arg1.def_val, arg2.range[1] + 0.1, arg3.def_val).unwrap_err();
    assert_eq!(result, expected);

    let expected = InputError::OutOfRange(String::from(arg3.name));
    let result = tested_function(arg1.def_val, arg2.def_val, arg3.range[0] - 0.1).unwrap_err();
    assert_eq!(result, expected);
    let result = tested_function(arg1.def_val, arg2.def_val, arg3.range[1] + 0.1).unwrap_err();
    assert_eq!(result, expected);

    true
}
