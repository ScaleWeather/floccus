use crate::error_wrapper::InputError;

fn test_with_2args(
    tested_function: &dyn Fn(f64, f64) -> Result<f64, InputError>,
    arg1_def: f64,
    arg1_name: &str,
    arg1_rng: [f64; 2],
    arg2_def: f64,
    arg2_rng: [f64; 2],
    arg2_name: &str,
    expected_result: f64,
) -> bool {
    //the first promise of the crate is that returned value
    //is calculated correctly
    let result = tested_function(arg1_def, arg2_def).unwrap();
    assert_eq!(result, expected_result);

    //the second promise of the crate is to never return NaN or Inf
    //here we check several edge cases for that
    //the function can only return a finite number or InputError
    //check for error implicit as Result<> ensures that if value
    //is not Ok() then it is Err()
    //here we don't care about error being correct
    let mut results = vec![];
    results.push(tested_function(arg1_def, 0.0));
    results.push(tested_function(0.0, arg2_def));
    results.push(tested_function(0.0, 0.0));

    for result in results {
        if result.is_ok() {
            assert!(result.unwrap().is_finite())
        }
    }

    //the third promise of the crate is to always return finite f64
    //if all inputs are within the range
    //the check for returned value being Ok(f64) is implicit
    for arg1_itr in 0..=100 {
        for arg2_itr in 0..=100 {
            let arg1 = ((arg1_rng[1] - arg1_rng[0]) / 100.0) * arg1_itr as f64;
            let arg2 = ((arg2_rng[1] - arg2_rng[0]) / 100.0) * arg2_itr as f64;

            let result = tested_function(arg1, arg2).unwrap();
            assert!(result.is_finite());
        }
    }

    //the fourth promise of the crate is to return an error with
    //erronous variable name when input is out of range
    let expected = InputError::OutOfRange(String::from(arg1_name));
    let result = tested_function(arg1_rng[0] - 0.1, arg2_def).unwrap_err();
    assert_eq!(result, expected);
    let result = tested_function(arg1_rng[1] + 0.1, arg2_def).unwrap_err();
    assert_eq!(result, expected);

    let expected = InputError::OutOfRange(String::from(arg2_name));
    let result = tested_function(arg1_def, arg2_rng[0] - 0.1).unwrap_err();
    assert_eq!(result, expected);
    let result = tested_function(arg1_def, arg2_rng[1] + 0.1).unwrap_err();
    assert_eq!(result, expected);
    

    true
}