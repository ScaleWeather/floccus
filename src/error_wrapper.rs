//!Module containing all error enums used by the crate

use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
///Error enum returned when provided input is out of reasonable range.
///Contains the name of variable that is out of range.
///
///In theory, all formulae should provide a result, although in some cases physically unreasonable, for any inputs.
///However, due to the nature of floating-point arithmetic ([read more about it here](https://docs.oracle.com/cd/E19957-01/806-3568/ncg_goldberg.html))
///for very small and very large inputs functions, especially those using `exp()`, will return `NaN` or `Inf`.
///
///When using thermodynamic formulae in loops (for example in numerical models), due to a bug one of the quantities can become physically unreasonable.
///That can result in a program returning `NaN`s and `Inf`s all over the place and even a program crash.
///Bugs like that can be really difficult to track down and truly annoying.
///An issue with a thickness of model bottom layer in WRF ([discussed here](https://forum.mmm.ucar.edu/phpBB3/viewtopic.php?t=8325#p14866)) is a good example of such bug.
///
///The [`InputError::OutOfRange`] error has been introduced to simplify debugging process and comply with [`C-VALIDATE`](https://rust-lang.github.io/api-guidelines/dependability.html#c-validate) rule.
///Although restrictive the advantages of having [`InputError::OutOfRange`] in crate outweigh the inconveniences of its presence, and most users should not realise it is present.
///Furthermore, it increases users' awarness of practicality of different formulae.
///
///If you find that in your use case input ranges are too narrow you should first look for a more relevant formula.
///If such formula does not exist do not hesitate to create an issue in Github repository.
pub enum InputError {
    #[error("Value of {0} out of a reasonable range.")]
    OutOfRange(String),
}
