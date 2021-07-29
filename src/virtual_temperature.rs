use crate::{constants::EPSILON, error_wrapper::InputError, mixing_ratio};

pub fn general1(temperature: f64, mixing_ratio: f64) -> Result<f64, InputError> {
    let result = temperature * ((1.0 + (mixing_ratio / EPSILON)) / (1.0 + mixing_ratio));

    Ok(result)
}

pub fn performance1(temperature: f64, dewpoint: f64, pressure: f64) -> Result<f64, InputError> {
    let mixing_ratio = mixing_ratio::performance1(dewpoint, pressure)?;
    let result = general1(temperature, mixing_ratio)?;

    Ok(result)
}

pub fn accuracy1(temperature: f64, dewpoint: f64, pressure: f64) -> Result<f64, InputError> {
    let mixing_ratio = mixing_ratio::accuracy1(dewpoint, pressure)?;
    let result = general1(temperature, mixing_ratio)?;

    Ok(result)
}
