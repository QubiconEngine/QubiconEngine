Measuring unit for temperature.

Counting starts from an absolute zero - the lowest temperature in all universe.
[Celsius](crate::si::derived_units::Celsius) unit was derived from this system.
The only difference is - counting starts at ice melting point(approximately **273.15 K**).

# Examples
```
# use qubicon_measuring_units::si::{ base_units::Kelvin, derived_units::Celsius };

let t_k = Kelvin::from(273.15);
let t_c = Celsius::<f64>::from(t_k);

println!("Melting point of ice is {t_k} or {t_c}");
```