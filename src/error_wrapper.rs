use thiserror::Error;

#[derive(Error, Debug)]
pub enum InputError {
    #[error("Value of {0} out of a reasonable range.")]
    OutOfRange(String),
}
