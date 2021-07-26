//!Module containing all error enums used by the crate

use thiserror::Error;

#[derive(Error, Debug)]
///Error enum returned when provided input is invalid
pub enum InputError {
    #[error("Value of {0} out of a reasonable range.")]
    OutOfRange(String),
}
