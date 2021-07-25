# floccus

[![License](https://img.shields.io/github/license/ScaleWeather/floccus)](https://choosealicense.com/licenses/apache-2.0/)
[![Crates.io](https://img.shields.io/crates/v/floccus)](https://crates.io/crates/floccus)
[![dependency status](https://deps.rs/crate/floccus/0.1.0/status.svg)](https://deps.rs/crate/floccus)

Rust crate providing formulae for thermodynamic calculations.

This crate contains functions for computing thermodynamic quantities commonly used in meteorology, oceangraphy and other fields. It is mainly developed by a meteorologist so **if there is a function you would like to be added do not hesitate to post an issue or pull request in the [Github repository](https://github.com/ScaleWeather/floccus)**. 

The purpose of this crate is to be an academic reference of thermodynamic formulae, 
so that researchers looking for a particular formula do not need to search it in the literature. 
Therefore all functions documentation provides a reference to the paper from which formula is taken.

Also, check the [contributors guide](https://github.com/ScaleWeather/floccus/blob/main/CONTRIBUTE.md) of this crate.

## How to use

To use this crate simply import it with `use` statement and then use desired function from chosen module.

```Rust
use floccus::vapour_pressure;

//Set temperature and pressure in SI units
let temperature = 300.0; //in K
let pressure = 101325.0; //in Pa

//Compute vapour pressure using Buck (1981) formula
let vapour_pressure = vapour_pressure::buck1(temperature, pressure);

//The result is 3550.6603579471303
```

## Naming of modules and functions

Because some thermodynamic formulae are empirical there are several ways to compute a value of given quantity.
Therefore there are multiple functions available to compute the same parameter, which are grouped into modules.

The naming of modules and functions follows this convention:

```Rust
vapour_pressure::buck1(temperature, pressure);
```

Where the module name (`vapour_pressure`) indicates the computed quantity, function name (`buck1`) indicates the author of formula
and the function arguments (`temperature, pressure`) are variables used to compute the quantity.
