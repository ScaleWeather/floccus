use crate::{error_wrapper::InputError, mixing_ratio, vapour_pressure};

pub fn general1(mixing_ratio: f64, saturation_mixing_ratio: f64) -> Result<f64, InputError> {
    Ok(mixing_ratio / saturation_mixing_ratio)
}

pub fn general2(vapour_pressure: f64, saturation_vapour_pressure: f64) -> Result<f64, InputError> {
    Ok(vapour_pressure / saturation_vapour_pressure)
}

pub fn general3(temperature: f64, dewpoint: f64) -> Result<f64, InputError> {
     let vapour_pressure = vapour_pressure::tetens1(dewpoint)?;
     let saturation_vapour_pressure = vapour_pressure::tetens1(temperature)?;
     let result = general2(vapour_pressure, saturation_vapour_pressure)?;

     Ok(result)
}

pub fn general4(temperature: f64, dewpoint: f64, pressure: f64) -> Result<f64, InputError> {
    let vapour_pressure = vapour_pressure::buck1(dewpoint, pressure)?;
    let saturation_vapour_pressure = vapour_pressure::buck1(temperature, pressure)?;
    let result = general2(vapour_pressure, saturation_vapour_pressure)?;

    Ok(result)
}

pub fn general5(temperature: f64, dewpoint: f64, pressure: f64) -> Result<f64, InputError> {
    let mixing_ratio = mixing_ratio::accuracy1(dewpoint, pressure)?;
    let saturation_mixing_ratio = mixing_ratio::accuracy1(temperature, pressure)?;
    let result = general1(mixing_ratio, saturation_mixing_ratio)?;

    Ok(result)
}