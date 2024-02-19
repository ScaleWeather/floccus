#[macro_export]
macro_rules! compute_vec {
    ($fn_id:expr,$slice1:expr) => {
        $slice1
            .iter()
            .map(|a| $fn_id(*a))
            .collect::<Result<Vec<crate::Float>, crate::errors::InputError>>()
    };
    ($fn_id:expr,$slice1:expr,$slice2:expr) => {
        itertools::izip!($slice1, $slice2)
            .map(|(a, b)| $fn_id(a, b))
            .collect::<Result<Vec<crate::Float>, crate::errors::InputError>>()
    };
    ($fn_id:expr,$slice1:expr,$slice2:expr,$slice3:expr) => {
        itertools::izip!($slice1, $slice2, $slice3)
            .map(|(a, b, c)| $fn_id(a, b, c))
            .collect::<Result<Vec<crate::Float>, crate::errors::InputError>>()
    };
}

#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;

    use crate::{vapour_pressure, vapour_pressure_deficit, Float};

    #[test]
    fn vec_macro_1arg() {
        let temp = vec![300.0; 100];
        let result = compute_vec!(vapour_pressure::buck3_simplified, temp).unwrap();

        assert_approx_eq!(Float, result[50], 3533.6421536199978, epsilon = 0.01);
    }

    #[test]
    fn vec_macro_2arg() {
        let temp = vec![300.0; 100];
        let pressure = vec![101325.0; 100];
        let result = compute_vec!(vapour_pressure::buck3, temp, pressure).unwrap();

        assert_approx_eq!(Float, result[50], 3548.5041048035896, epsilon = 0.01);
    }

    #[test]
    fn vec_macro_3arg() {
        let temp = vec![300.0; 100];
        let pressure = vec![101325.0; 100];
        let relative_humidity = vec![0.5; 100];
        let result = compute_vec!(
            vapour_pressure_deficit::general3,
            temp,
            relative_humidity,
            pressure
        )
        .unwrap();

        assert_approx_eq!(Float, result[50], 1774.2520524017948, epsilon = 0.01);
    }
}
