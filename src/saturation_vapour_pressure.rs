///Formula for computing **ONLY** saturation vapour pressure from vapour pressure and relative humidity.
///For vapour pressure use [`saturation_specific1`]
///
///# Errors
///
///Returns [`InputError::OutOfRange`] when input is out of range.\
///Valid `vapour_pressure` range: 0Pa - 10000Pa\
///Valid `relative_humidity` range: 0.00001 - 1.0
pub struct SaturationSpecific2;

impl SaturationSpecific2 {
    #[allow(missing_docs)]
    #[inline(always)]
    #[allow(clippy::missing_errors_doc)]

    pub fn validate_inputs(
        vapour_pressure: Float,
        relative_humidity: Float,
    ) -> Result<(), InputError> {
        if !(0.00001..=2.0).contains(&relative_humidity) {
            return Err(InputError::OutOfRange(String::from("relative_humidity")));
        }

        if !(0.0..=10_000.0).contains(&vapour_pressure) {
            return Err(InputError::OutOfRange(String::from("vapour_pressure")));
        }

        Ok(())
    }

    #[inline(always)]
    #[allow(missing_docs)]
    pub fn compute_unchecked(vapour_pressure: Float, relative_humidity: Float) -> Float {
        vapour_pressure / relative_humidity
    }
}
