macro_rules! generate_compute {
    ($qnt:tt, $arg1:tt) => {
        impl $qnt {
            #[allow(missing_docs)]
            pub fn compute($arg1: Float) -> Result<Float, InputError> {
                Self::validate_inputs($arg1)?;
                Ok(Self::compute_unchecked($arg1))
            }
        }
    };

    ($qnt:tt, $arg1:tt, $arg2:tt) => {
        impl $qnt {
            pub fn compute($arg1: Float, $arg2: Float) -> Result<Float, InputError> {
                Self::validate_inputs($arg1, $b)?;
                Ok(Self::compute_unchecked($arg1, $b))
            }
        }
    };

    ($qnt:tt, $arg1:tt, $arg2:tt, $arg3:tt) => {
        impl $qnt {
            pub fn compute($arg1: Float, $arg2: Float, $arg3: Float) -> Result<Float, InputError> {
                Self::validate_inputs($arg1, $arg2, $arg3)?;
                Ok(Self::compute_unchecked($arg1, $arg2, $arg3))
            }
        }
    };
}

macro_rules! generate_vec_compute {
    ($qnt:tt, $slice1:tt) => {
        impl $qnt {
            #[allow(missing_docs)]
            pub fn compute_vec($slice1: &[Float]) -> Result<Vec<Float>, InputError> {
                $slice1
                    .iter()
                    .map(|&a| Self::compute(a))
                    .collect::<Result<Vec<Float>, InputError>>()
            }
        }
    };

    ($qnt:tt, $slice1:tt, $slice2:tt) => {
        impl $qnt {
            #[allow(missing_docs)]
            pub fn compute_vec(
                $slice1: &[Float],
                $slice2: &[Float],
            ) -> Result<Vec<Float>, InputError> {
                izip!($slice1, $slice2)
                    .map(|(&a, &b)| Self::compute(a, b))
                    .collect::<Result<Vec<Float>, InputError>>()
            }
        }
    };

    ($qnt:tt, $slice1:tt, $slice2:tt, $slice3:tt) => {
        impl $qnt {
            #[allow(missing_docs)]
            pub fn compute_vec(
                $slice1: &[Float],
                $slice2: &[Float],
            ) -> Result<Vec<Float>, InputError> {
                izip!($slice1, $slice2, $slice3)
                    .map(|(&a, &b, &c)| Self::compute(a, b, c))
                    .collect::<Result<Vec<Float>, InputError>>()
            }
        }
    };
}

macro_rules! generate_ndarray_compute {
    ($qnt:tt, $arr1:tt) => {
        impl $qnt {
            #[allow(missing_docs)]
            pub fn compute_ndarray<D: Dimension>(
                $arr1: &Array<Float, D>,
            ) -> Result<Array<Float, D>, InputError> {
                ndarray::Zip::from($arr1)
                    .fold_while(Ok(()), |_, &a| match Self::validate_inputs(a) {
                        Ok(_) => FoldWhile::Continue(Ok(())),
                        Err(e) => FoldWhile::Done(Err(e)),
                    })
                    .into_inner()?;

                Ok(ndarray::Zip::from($arr1).map_collect(|&a| Self::compute_unchecked(a)))
            }
        }
    };

    ($qnt:tt, $arr1:tt, $arr2:tt) => {
        impl $qnt {
            #[allow(missing_docs)]
            pub fn compute_ndarray<D: Dimension>(
                $arr1: &Array<Float, D>,
                $arr2: &Array<Float, D>,
            ) -> Result<Array<Float, D>, InputError> {
                ndarray::Zip::from($arr1)
                    .and($arr2)
                    .fold_while(Ok(()), |_, &a, &b| match Self::validate_inputs(a, b) {
                        Ok(_) => FoldWhile::Continue(Ok(())),
                        Err(e) => FoldWhile::Done(Err(e)),
                    })
                    .into_inner()?;

                Ok(ndarray::Zip::from($arr1)
                    .and($arr2)
                    .map_collect(|&a, &b| Self::compute_unchecked(a, b)))
            }
        }
    };

    ($qnt:tt, $arr1:tt, $arr2:tt, $arr3:tt) => {
        impl $qnt {
            #[allow(missing_docs)]
            pub fn compute_ndarray<D: Dimension>(
                $arr1: &Array<Float, D>,
                $arr2: &Array<Float, D>,
                $arr3: &Array<Float, D>,
            ) -> Result<Array<Float, D>, InputError> {
                ndarray::Zip::from($arr1)
                    .and($arr2)
                    .and($arr3)
                    .fold_while(Ok(()), |_, &a, &b, &c| {
                        match Self::validate_inputs(a, b, c) {
                            Ok(_) => FoldWhile::Continue(Ok(())),
                            Err(e) => FoldWhile::Done(Err(e)),
                        }
                    })
                    .into_inner()?;

                Ok(ndarray::Zip::from($arr1)
                    .and($arr2)
                    .and($arr3)
                    .map_collect(|&a, &b, &c| Self::compute_unchecked(a, b, c)))
            }
        }
    };
}

macro_rules! generate_par_vec_compute {
    ($qnt:tt, $slice1:tt) => {
        impl $qnt {
            #[allow(missing_docs)]
            pub fn compute_vec_parallel($slice1: &[Float]) -> Result<Vec<Float>, InputError> {
                $slice1
                    .par_iter()
                    .map(|&a| Self::compute(a))
                    .collect::<Result<Vec<Float>, InputError>>()
            }
        }
    };

    ($qnt:tt, $slice1:tt, $slice2:tt) => {
        impl $qnt {
            #[allow(missing_docs)]
            pub fn compute_vec_parallel(
                $slice1: &[Float],
                $slice2: &[Float],
            ) -> Result<Vec<Float>, InputError> {
                izip!($slice1, $slice2)
                    .par_bridge()
                    .map(|(&a, &b)| Self::compute(a, b))
                    .collect::<Result<Vec<Float>, InputError>>()
            }
        }
    };

    ($qnt:tt, $slice1:tt, $slice2:tt, $slice3:tt) => {
        impl $qnt {
            #[allow(missing_docs)]
            pub fn compute_vec_parallel(
                $slice1: &[Float],
                $slice2: &[Float],
            ) -> Result<Vec<Float>, InputError> {
                izip!($slice1, $slice2, $slice3)
                    .par_bridge()
                    .map(|(&a, &b, &c)| Self::compute(a, b, c))
                    .collect::<Result<Vec<Float>, InputError>>()
            }
        }
    };
}

macro_rules! generate_par_ndarray_compute {
    ($qnt:tt, $arr1:tt) => {
        impl $qnt {
            #[allow(missing_docs)]
            pub fn compute_ndarray_parallel<D: Dimension>(
                $arr1: &Array<Float, D>,
            ) -> Result<Array<Float, D>, InputError> {
                ndarray::Zip::from($arr1)
                    .fold_while(Ok(()), |_, &a| match Self::validate_inputs(a) {
                        Ok(_) => FoldWhile::Continue(Ok(())),
                        Err(e) => FoldWhile::Done(Err(e)),
                    })
                    .into_inner()?;

                Ok(ndarray::Zip::from($arr1).par_map_collect(|&a| Self::compute_unchecked(a)))
            }
        }
    };

    ($qnt:tt, $arr1:tt, $arr2:tt) => {
        impl $qnt {
            #[allow(missing_docs)]
            pub fn compute_ndarray_parallel<D: Dimension>(
                $arr1: &Array<Float, D>,
                $arr2: &Array<Float, D>,
            ) -> Result<Array<Float, D>, InputError> {
                ndarray::Zip::from($arr1)
                    .and($arr2)
                    .fold_while(Ok(()), |_, &a, &b| match Self::validate_inputs(a, b) {
                        Ok(_) => FoldWhile::Continue(Ok(())),
                        Err(e) => FoldWhile::Done(Err(e)),
                    })
                    .into_inner()?;

                Ok(ndarray::Zip::from($arr1)
                    .and($arr2)
                    .par_map_collect(|&a, &b| Self::compute_unchecked(a, b)))
            }
        }
    };

    ($qnt:tt, $arr1:tt, $arr2:tt, $arr3:tt) => {
        impl $qnt {
            #[allow(missing_docs)]
            pub fn compute_ndarray_parallel<D: Dimension>(
                $arr1: &Array<Float, D>,
                $arr2: &Array<Float, D>,
                $arr3: &Array<Float, D>,
            ) -> Result<Array<Float, D>, InputError> {
                ndarray::Zip::from($arr1)
                    .and($arr2)
                    .and($arr3)
                    .fold_while(Ok(()), |_, &a, &b, &c| {
                        match Self::validate_inputs(a, b, c) {
                            Ok(_) => FoldWhile::Continue(Ok(())),
                            Err(e) => FoldWhile::Done(Err(e)),
                        }
                    })
                    .into_inner()?;

                Ok(ndarray::Zip::from($arr1)
                    .and($arr2)
                    .and($arr3)
                    .par_map_collect(|&a, &b, &c| Self::compute_unchecked(a, b, c)))
            }
        }
    };
}

pub(crate) use generate_compute;
pub(crate) use generate_ndarray_compute;
pub(crate) use generate_par_ndarray_compute;
pub(crate) use generate_par_vec_compute;
pub(crate) use generate_vec_compute;

// #[cfg(test)]
// mod tests {
//     use float_cmp::assert_approx_eq;
//     use ndarray::Array2;

//     use crate::{
//         vapour_pressure::{self},
//         vapour_pressure_deficit, Float,
//     };

//     #[test]
//     fn arr_macro_1arg() -> Result<(), crate::errors::InputError> {
//         let temp = Array2::from_elem((10, 10), 300.0);
//         let result = compute_ndarray!(
//             vapour_pressure::buck3_simplified_unchecked,
//             vapour_pressure::buck3_simplified_validate,
//             temp.view()
//         )?;

//         assert_approx_eq!(Float, result[[5, 5]], 3533.6421536199978, epsilon = 0.01);

//         Ok(())
//     }

//     #[test]
//     fn arr_macro_2arg() -> Result<(), crate::errors::InputError> {
//         let temp = Array2::from_elem((10, 10), 300.0);
//         let pressure = Array2::from_elem((10, 10), 101325.0);

//         let result = compute_ndarray!(
//             vapour_pressure::buck3_unchecked,
//             vapour_pressure::buck3_validate,
//             temp.view(),
//             pressure.view()
//         )?;

//         assert_approx_eq!(Float, result[[5, 5]], 3548.5041048035896, epsilon = 0.01);

//         Ok(())
//     }

//     #[test]
//     fn arr_macro_3arg() -> Result<(), crate::errors::InputError> {
//         let temp = Array2::from_elem((10, 10), 300.0);
//         let pressure = Array2::from_elem((10, 10), 101325.0);
//         let relative_humidity = Array2::from_elem((10, 10), 0.5);

//         let result = compute_ndarray!(
//             vapour_pressure_deficit::general3_unchecked,
//             vapour_pressure_deficit::general3_validate,
//             temp.view(),
//             relative_humidity.view(),
//             pressure.view()
//         )?;

//         assert_approx_eq!(Float, result[[5, 5]], 1774.2520524017948, epsilon = 0.01);

//         Ok(())
//     }

//     #[test]
//     fn vec_macro_1arg() {
//         let temp = vec![300.0; 100];
//         let result = compute_vec!(vapour_pressure::buck3_simplified, &temp).unwrap();

//         assert_approx_eq!(Float, result[50], 3533.6421536199978, epsilon = 0.01);
//     }

//     #[test]
//     fn vec_macro_2arg() {
//         let temp = vec![300.0; 100];
//         let pressure = vec![101325.0; 100];
//         let result = compute_vec!(vapour_pressure::buck3, &temp, &pressure).unwrap();

//         assert_approx_eq!(Float, result[50], 3548.5041048035896, epsilon = 0.01);
//     }

//     #[test]
//     fn vec_macro_3arg() {
//         let temp = vec![300.0; 100];
//         let pressure = vec![101325.0; 100];
//         let relative_humidity = vec![0.5; 100];
//         let result = compute_vec!(
//             vapour_pressure_deficit::general3,
//             &temp,
//             &relative_humidity,
//             &pressure
//         )
//         .unwrap();

//         assert_approx_eq!(Float, result[50], 1774.2520524017948, epsilon = 0.01);
//     }
// }
