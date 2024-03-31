Unit of illuminance (or luminous flux per unit area)

Equal to 1 [lumen](crate::si::derived_units::Lumen)
per 1 [meter](crate::si::base_units::Metre)<sup>2</sup>.

More information on [wikipedia](https://en.wikipedia.org/wiki/Lux)

# Examples
```
# use num_traits::Pow;
# use qubicon_measuring_units::si::{ base_units::Metre, derived_units::{ Lumen, Lux } };
let area = Metre::from(2.0).pow(2);
let flux = Lumen::from(5.0);

let illuminance = Lux::from_flux_and_area(flux, area);

println!("{illuminance}");
```