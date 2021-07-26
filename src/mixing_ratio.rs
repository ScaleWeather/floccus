//!Functions to calculate mixing ratio of fluids

use crate::error_wrapper::InputError;

fn general1<T>(temperature: f64, pressure: f64, function: T) -> Result<f64, InputError>
where
    T: Fn() -> Result<f64, InputError>,
{
    Ok(0.0)
}

//double calcMixingRatio(double temperature, double pressure)
//{
//    //input in K & Pa; output in ratio of kg/kg
//    //function for caluclating both mixing ratio and saturation mixing ratio
//
//    //first calculate (saturation) vapour pressure
//    double wvpres = calcVapourPressure(temperature, pressure);
//
//    //second return calculated mixing ratio
//    return EPSILON * (wvpres / (pressure - wvpres));
//}

#[cfg(test)]
mod tests {
    #[test]
    fn mixing_ratio_general1() {}
}
