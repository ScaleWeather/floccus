use itertools::izip;
use ndarray::{Array, ArrayView, Dimension, FoldWhile};
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};

use crate::{errors::InputError, Float};

#[macro_export]
macro_rules! par_compute_vec {
    ($fn_id:expr,$slice1:expr) => {
        $crate::par_array_compute::compute_vec_1($fn_id, $slice1)
    };
    ($fn_id:expr,$slice1:expr,$slice2:expr) => {
        $crate::par_array_compute::compute_vec_2($fn_id, $slice1, $slice2)
    };
    ($fn_id:expr,$slice1:expr,$slice2:expr,$slice3:expr) => {
        $crate::par_array_compute::compute_vec_3($fn_id, $slice1, $slice2, $slice3)
    };
}

#[macro_export]
macro_rules! par_compute_ndarray {
    ($cmp_fn:expr,$vld_fn:expr,$arr1:expr) => {
        $crate::par_array_compute::compute_ndarray_1($cmp_fn, $vld_fn, $arr1.view())
    };

    ($cmp_fn:expr,$vld_fn:expr,$arr1:expr,$arr2:expr) => {
        $crate::par_array_compute::compute_ndarray_2($cmp_fn, $vld_fn, $arr1.view(), $arr2.view())
    };

    ($cmp_fn:expr,$vld_fn:expr,$arr1:expr,$arr2:expr,$arr3:expr) => {
        $crate::par_array_compute::compute_ndarray_3(
            $cmp_fn,
            $vld_fn,
            $arr1.view(),
            $arr2.view(),
            $arr3.view(),
        )
    };
}

#[doc(hidden)]
#[inline(always)]
pub fn compute_vec_1(
    fn_id: fn(Float) -> Result<Float, InputError>,
    slice1: &[Float],
) -> Result<Vec<Float>, InputError> {
    slice1
        .par_iter()
        .map(|a| fn_id(*a))
        .collect::<Result<Vec<Float>, InputError>>()
}

#[doc(hidden)]
#[inline(always)]
pub fn compute_vec_2(
    fn_id: fn(Float, Float) -> Result<Float, InputError>,
    slice1: &[Float],
    slice2: &[Float],
) -> Result<Vec<Float>, InputError> {
    izip!(slice1, slice2)
        .par_bridge()
        .map(|(&a, &b)| fn_id(a, b))
        .collect::<Result<Vec<Float>, InputError>>()
}

#[doc(hidden)]
#[inline(always)]
pub fn compute_vec_3(
    fn_id: fn(Float, Float, Float) -> Result<Float, InputError>,
    slice1: &[Float],
    slice2: &[Float],
    slice3: &[Float],
) -> Result<Vec<Float>, InputError> {
    izip!(slice1, slice2, slice3)
        .par_bridge()
        .map(|(&a, &b, &c)| fn_id(a, b, c))
        .collect::<Result<Vec<Float>, InputError>>()
}

#[doc(hidden)]
#[inline(always)]
pub fn compute_ndarray_1<D: Dimension>(
    cmp_fn: fn(Float) -> Float,
    vld_fn: fn(Float) -> Result<(), InputError>,
    arr1: ArrayView<'_, Float, D>,
) -> Result<Array<Float, D>, InputError> {
    ndarray::Zip::from(&arr1)
        .fold_while(Ok(()), |_, &a| match vld_fn(a) {
            Ok(_) => FoldWhile::Continue(Ok(())),
            Err(e) => FoldWhile::Done(Err(e)),
        })
        .into_inner()?;

    Ok(ndarray::Zip::from(&arr1).par_map_collect(|&a| cmp_fn(a)))
}

#[doc(hidden)]
#[inline(always)]
pub fn compute_ndarray_2<D: Dimension>(
    cmp_fn: fn(Float, Float) -> Float,
    vld_fn: fn(Float, Float) -> Result<(), InputError>,
    arr1: ArrayView<'_, Float, D>,
    arr2: ArrayView<'_, Float, D>,
) -> Result<Array<Float, D>, InputError> {
    ndarray::Zip::from(&arr1)
        .and(&arr2)
        .fold_while(Ok(()), |_, &a, &b| match vld_fn(a, b) {
            Ok(_) => FoldWhile::Continue(Ok(())),
            Err(e) => FoldWhile::Done(Err(e)),
        })
        .into_inner()?;

    Ok(ndarray::Zip::from(&arr1)
        .and(&arr2)
        .par_map_collect(|&a, &b| cmp_fn(a, b)))
}

#[doc(hidden)]
#[inline(always)]
pub fn compute_ndarray_3<D: Dimension>(
    cmp_fn: fn(Float, Float, Float) -> Float,
    vld_fn: fn(Float, Float, Float) -> Result<(), InputError>,
    arr1: ArrayView<'_, Float, D>,
    arr2: ArrayView<'_, Float, D>,
    arr3: ArrayView<'_, Float, D>,
) -> Result<Array<Float, D>, InputError> {
    ndarray::Zip::from(&arr1)
        .and(&arr2)
        .and(&arr3)
        .fold_while(Ok(()), |_, &a, &b, &c| match vld_fn(a, b, c) {
            Ok(_) => FoldWhile::Continue(Ok(())),
            Err(e) => FoldWhile::Done(Err(e)),
        })
        .into_inner()?;

    Ok(ndarray::Zip::from(&arr1)
        .and(&arr2)
        .and(&arr3)
        .par_map_collect(|&a, &b, &c| cmp_fn(a, b, c)))
}

#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;
    use ndarray::Array2;

    use crate::{
        vapour_pressure::{self},
        vapour_pressure_deficit, Float,
    };

    #[test]
    fn arr_macro_1arg() -> Result<(), crate::errors::InputError> {
        let temp = Array2::from_elem((10, 10), 300.0);
        let result = par_compute_ndarray!(
            vapour_pressure::buck3_simplified_unchecked,
            vapour_pressure::buck3_simplified_validate,
            temp
        )?;

        assert_approx_eq!(Float, result[[5, 5]], 3533.6421536199978, epsilon = 0.01);

        Ok(())
    }

    #[test]
    fn arr_macro_2arg() -> Result<(), crate::errors::InputError> {
        let temp = Array2::from_elem((10, 10), 300.0);
        let pressure = Array2::from_elem((10, 10), 101325.0);

        let result = par_compute_ndarray!(
            vapour_pressure::buck3_unchecked,
            vapour_pressure::buck3_validate,
            temp,
            pressure
        )?;

        assert_approx_eq!(Float, result[[5, 5]], 3548.5041048035896, epsilon = 0.01);

        Ok(())
    }

    #[test]
    fn arr_macro_3arg() -> Result<(), crate::errors::InputError> {
        let temp = Array2::from_elem((10, 10), 300.0);
        let pressure = Array2::from_elem((10, 10), 101325.0);
        let relative_humidity = Array2::from_elem((10, 10), 0.5);

        let result = par_compute_ndarray!(
            vapour_pressure_deficit::general3_unchecked,
            vapour_pressure_deficit::general3_validate,
            temp,
            relative_humidity,
            pressure
        )?;

        assert_approx_eq!(Float, result[[5, 5]], 1774.2520524017948, epsilon = 0.01);

        Ok(())
    }

    #[test]
    fn vec_macro_1arg() {
        let temp = vec![300.0; 100];
        let result = par_compute_vec!(vapour_pressure::buck3_simplified, &temp).unwrap();

        assert_approx_eq!(Float, result[50], 3533.6421536199978, epsilon = 0.01);
    }

    #[test]
    fn vec_macro_2arg() {
        let temp = vec![300.0; 100];
        let pressure = vec![101325.0; 100];
        let result = par_compute_vec!(vapour_pressure::buck3, &temp, &pressure).unwrap();

        assert_approx_eq!(Float, result[50], 3548.5041048035896, epsilon = 0.01);
    }

    #[test]
    fn vec_macro_3arg() {
        let temp = vec![300.0; 100];
        let pressure = vec![101325.0; 100];
        let relative_humidity = vec![0.5; 100];
        let result = par_compute_vec!(
            vapour_pressure_deficit::general3,
            &temp,
            &relative_humidity,
            &pressure
        )
        .unwrap();

        assert_approx_eq!(Float, result[50], 1774.2520524017948, epsilon = 0.01);
    }
}
