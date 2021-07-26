//!Functions to calculate partial vapour pressure and saturation vapour pressure in the unsaturated air

use crate::{constants::ZERO_CELSIUS, error_wrapper::InputError};

///Formula computing vapour pressure from air temperature and pressure.
///Most accurate in temperature range from 233K to 323K.
///
///Valid temperature range: 232K - 324K\
///Valid pressure range: 100Pa - 110000Pa\
///Derived by A. L. Buck (1981) [(doi: 10.1175/1520-0450(1981)020<1527:nefcvp>2.0.co;2)](https://doi.org/10.1175/1520-0450(1981)020%3C1527:NEFCVP%3E2.0.CO;2).
pub fn buck1(temperature: f64, pressure: f64) -> Result<f64, InputError> {
    //validate inputs
    if temperature < 232.0 || temperature > 324.0 {
        return Err(InputError::OutOfRange(String::from("temperature")));
    }

    if pressure < 100.0 || pressure > 110000.0 {
        return Err(InputError::OutOfRange(String::from("pressure")));
    }

    let temperature = temperature - ZERO_CELSIUS; //convert to C
    let pressure = pressure / 100.0; //convert to hPa

    let a = 6.1121;
    let b = 18.729;
    let c = 257.87;
    let d = 227.3;

    let upper_a = 0.00072;
    let upper_b = 0.0000032;
    let upper_c = 0.00000000059;

    let e = a * (((b - (temperature / d)) * temperature) / (temperature + c)).exp();
    let f = 1.0 + upper_a + (pressure * (upper_b + (upper_c * temperature * temperature)));

    Ok((e * f) * 100.0) //return in Pa
}

///Formula computing vapour pressure from air temperature over water. 
///Should be used for temperatures above 273K.
///
///Valid temperature range: 273K - 353K\
///Derived by O. Tetens (1930).
pub fn tetens1(temperature: f64) -> Result<f64, InputError> {
    //validate inputs
    if temperature < 273.0 || temperature > 354.0 {
        return Err(InputError::OutOfRange(String::from("temperature")));
    }

    let temperature = temperature - 273.15; //convert to C

    let a = 0.61078;
    let b = 17.27;
    let c = 237.3;

    let result = a * ((b * temperature) / (temperature + c)).exp();

    Ok(result * 1000.0) //return in Pa
}

#[cfg(test)]
mod tests {
    use crate::vapour_pressure;
    use float_cmp::assert_approx_eq;

    #[test]
    fn vapour_pressure_buck1() {
        let result = vapour_pressure::buck1(300.0, 101325.0).unwrap();
        let expected = 3550.6603579471303;
        assert_approx_eq!(f64, expected, result, ulps = 2);
        todo!("Add error checks");
    }

    #[test]
    fn vapour_pressure_tetens1() {
        let result = vapour_pressure::tetens1(300.0).unwrap();
        let expected = 3533.969137160892;
        assert_approx_eq!(f64, expected, result, ulps = 2);
    }
}