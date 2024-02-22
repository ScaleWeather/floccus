#![allow(missing_docs)]

use crate::{errors::InputError, quantities::ThermodynamicQuantity};
use ndarray::{Array, Dimension, FoldWhile, Zip};
use rayon::iter::{ParallelIterator, ParallelBridge, IntoParallelIterator};

pub trait Formula1<O: ThermodynamicQuantity, I1: ThermodynamicQuantity> {
    #[allow(missing_docs)]
    fn compute_unchecked(i1: I1) -> O;

    #[allow(missing_docs)]
    #[allow(clippy::missing_errors_doc)]
    fn validate_inputs(i1: I1) -> Result<(), InputError>;

    #[allow(clippy::missing_errors_doc)]
    #[allow(missing_docs)]
    #[inline]
    fn compute(i1: I1) -> Result<O, InputError> {
        #[cfg(not(feature = "debug"))]
        Self::validate_inputs(i1)?;
        #[cfg(feature = "debug")]
        Self::validate_inputs_loggerr(i1)?;

        Ok(Self::compute_unchecked(i1))
    }

    #[cfg(feature = "debug")]
    #[inline(always)]
    #[allow(missing_docs)]
    #[allow(clippy::missing_errors_doc)]
    fn validate_inputs_loggerr(i1: I1) -> Result<(), InputError> {
        use std::any::type_name;

        Self::validate_inputs(i1).or_else(|err| {
            log::error!(
                "Formula {} calculating {} from inputs {:?} returned error: {}",
                type_name::<Self>(),
                type_name::<O>(),
                i1,
                err
            );
            Err(err)
        })
    }

    #[cfg(feature = "array")]
    #[allow(missing_docs)]
    #[allow(clippy::missing_errors_doc)]
    fn compute_vec(i1: &[I1]) -> Result<Vec<O>, InputError> {
        i1.iter().map(|&i1| Self::compute(i1)).collect()
    }

    #[cfg(feature = "array")]
    #[allow(missing_docs)]
    #[allow(clippy::missing_errors_doc)]
    fn compute_ndarray<D: Dimension>(i1: &Array<I1, D>) -> Result<Array<O, D>, InputError> {
        Zip::from(i1)
            .fold_while(Ok(()), |_, &a| match Self::validate_inputs(a) {
                Ok(_) => FoldWhile::Continue(Ok(())),
                Err(e) => FoldWhile::Done(Err(e)),
            })
            .into_inner()?;

        Ok(Zip::from(i1).map_collect(|&a| Self::compute_unchecked(a)))
    }

    #[cfg(feature = "array")]
    #[allow(missing_docs)]
    #[allow(clippy::missing_errors_doc)]
    fn compute_vec_parallel(i1: &[I1]) -> Result<Vec<O>, InputError> {

        i1.into_par_iter().map(|&i1| Self::compute(i1)).collect()
    }
}

pub trait Formula2<O: ThermodynamicQuantity, I1: ThermodynamicQuantity, I2: ThermodynamicQuantity> {
    #[allow(missing_docs)]
    fn compute_unchecked(i1: I1, i2: I2) -> O;

    #[allow(missing_docs)]
    #[allow(clippy::missing_errors_doc)]
    fn validate_inputs(i1: I1, i2: I2) -> Result<(), InputError>;

    #[allow(clippy::missing_errors_doc)]
    #[allow(missing_docs)]
    #[inline]
    fn compute(i1: I1, i2: I2) -> Result<O, InputError> {
        #[cfg(not(feature = "debug"))]
        Self::validate_inputs(i1, i2)?;
        #[cfg(feature = "debug")]
        Self::validate_inputs_loggerr(i1, i2)?;

        Ok(Self::compute_unchecked(i1, i2))
    }

    #[cfg(feature = "debug")]
    #[inline(always)]
    #[allow(missing_docs)]
    #[allow(clippy::missing_errors_doc)]
    fn validate_inputs_loggerr(i1: I1, i2: I2) -> Result<(), InputError> {
        use std::any::type_name;

        Self::validate_inputs(i1, i2).or_else(|err| {
            log::error!(
                "Formula {} calculating {} from inputs {:?} {:?} returned error: {}",
                type_name::<Self>(),
                type_name::<O>(),
                i1,
                i2,
                err
            );
            Err(err)
        })
    }
}

pub trait Formula3<
    O: ThermodynamicQuantity,
    I1: ThermodynamicQuantity,
    I2: ThermodynamicQuantity,
    I3: ThermodynamicQuantity,
>
{
    #[allow(missing_docs)]
    fn compute_unchecked(i1: I1, i2: I2, i3: I3) -> O;

    #[allow(missing_docs)]
    #[allow(clippy::missing_errors_doc)]
    fn validate_inputs(i1: I1, i2: I2, i3: I3) -> Result<(), InputError>;

    #[allow(clippy::missing_errors_doc)]
    #[allow(missing_docs)]
    #[inline]
    fn compute(i1: I1, i2: I2, i3: I3) -> Result<O, InputError> {
        #[cfg(not(feature = "debug"))]
        Self::validate_inputs(i1, i2, i3)?;
        #[cfg(feature = "debug")]
        Self::validate_inputs_loggerr(i1, i2, i3)?;

        Ok(Self::compute_unchecked(i1, i2, i3))
    }

    #[cfg(feature = "debug")]
    #[inline(always)]
    #[allow(missing_docs)]
    #[allow(clippy::missing_errors_doc)]
    fn validate_inputs_loggerr(i1: I1, i2: I2, i3: I3) -> Result<(), InputError> {
        use std::any::type_name;

        Self::validate_inputs(i1, i2, i3).or_else(|err| {
            log::error!(
                "Formula {} calculating {} from inputs {:?} {:?} {:?} returned error: {}",
                type_name::<Self>(),
                type_name::<O>(),
                i1,
                i2,
                i3,
                err
            );
            Err(err)
        })
    }
}
