use crate::{constants::ZERO_CELSIUS, error_wrapper::InputError};

///Derived by R. Stull (2011) [(doi:10.1175/JAMC-D-11-0143.1)](https://doi.org/10.1175/JAMC-D-11-0143.1)
pub fn stull1(temperature: f64, relative_humidity: f64) -> Result<f64, InputError> {
    //convert units
    let temperature = temperature - ZERO_CELSIUS;
    let relative_humidity = relative_humidity * 100.0;

    let result = (temperature * (0.151_977 * (relative_humidity + 8.313_659).sqrt()).atan())
        + (temperature + relative_humidity).atan()
        - (relative_humidity - 1.676_331).atan()
        + (0.003_918_38 * relative_humidity.powf(1.5) * (0.023_101 * relative_humidity).atan())
        - 4.686_035;

    Ok(result + ZERO_CELSIUS)
}
