# Contributors Guide

Thank you for considering contributing to this crate. Here are some basic rules when writing a code for this library.

### How to contribute

The easiest way to contribute is by posting an Issue or Pull Request in the repository.

### Code standards

All functions in the crate should meet the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/checklist.html) to the gratest possible extent.

### Input checking

All functions should validate that their inputs are within a reasonable range. Exact limits have to be specified in the documentation of function. If values are out of range function should return `InputError::OutOfRange` along with the name of erronous input.

### Constants

Moreover, all functions should use pre-defined constants as much as possible. Constants should be hardcoded into function only if they are empirical or used to convert the order-of-magnitude of some value.
For example, if authors of the formula in the paper use latent heat of vaporization (Lv) constant with different value than used by `floccus` you should still use `floccus::constants::Lv` in your function. You can then open an issue to disscuss the value of `Lv` used in `floccus`.

### Naming conventions

All functions and modules should follow this naming convention:

```Rust
vapour_pressure::buck1(temperature, pressure);
```

Where the module name (`vapour_pressure`) indicates the computed quantity, function name (`buck1`) indicates the author of formula
and the function arguments (`temperature, pressure`) are variables used to compute the quantity.

### Tests

All functions should be tested with unit tests to check if they are working correctly and if they meet the promises of crate. `cargo check` will issue a dead code warning if any test is missing.

There is a [`float-cmp`](https://crates.io/crates/float-cmp) crate available as developmental dependency to compare floating-point numbers in tests.

A refrence test functions are provided in `tests_framework.rs`. Unit test for a function can simply call appropriate framework function with adequate parameters.

### Documentation

Documentation of all functions should contain following information:

- Recommended range of input variables (for which the formula is most accurate)
- Reference to the paper from which formula is taken
- In which conditions function returns an error and of what kind

### Benchmarks

To provide the user information about performance overhead of each function all functions must be benchmarked. This is done using [criterion.rs](https://github.com/bheisler/criterion.rs).

To add a new benchmark simply copy existing one and adapt it to benchmark the added function. Also, if neccessary, add appropriate entry in `Cargo.toml`. Github Actions will automatically run all benchmarks.

To learn more about criterion read its [guide](https://bheisler.github.io/criterion.rs/book/index.html).

### Other questions

If you are unsure how to write your own function take a look at the current code in repository. Also, do not hesitate to ask a question by posting an Issue. Finally, all pull requests are warmly welcome and we will help you if they need any improvements.
