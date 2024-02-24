use crate::{
    quantities::{DryBulbTemperature, ThermodynamicQuantity},
    tests::testing_traits::TestingQuantity,
};

#[test]
fn quantity_name() {
    let quantity = DryBulbTemperature::default_si();
    assert_eq!(quantity.name(), "DryBulbTemperature");
}
