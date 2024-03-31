Measuring unit for temperature

Celsius scale is derived from [Kelvins](crate::si::base_units::Kelvin) scale.
In difference to [Kelvins](crate::si::base_units::Kelvin), counting in Celsius starts from melting point of ice.

More information on [wikipedia](https://en.wikipedia.org/wiki/Celsius)

# Example
```
# use qubicon_measuring_units::si::{ base_units::Kelvin, derived_units::Celsius };
let t_c = Celsius::from(100.0f64);
let t_k = Kelvin::<f64>::from(t_c);

println!("Water under normal atmospheric pressure boils at {t_c} or {t_k}");
```