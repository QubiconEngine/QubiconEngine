Unit of energy

Equal to amount of work done when a force of 1 [newton](crate::si::derived_units::Newton)
displaces a mass through a distance of one [metre](crate::si::base_units::Metre).

Also equal for heat what dissipates when 1 ampere passes through a resistance
of 1 [ohm](crate::si::derived_units::Ohm) for 1 [second](crate::si::base_units::Second).

More information on [wikipedia](https://en.wikipedia.org/wiki/Joule)

# Examples
```
# use qubicon_measuring_units::si::{ base_units::Metre, derived_units::{ Newton, Joule } };
let distance = Metre::from(3.0);
let force = Newton::from(10.0);

let work = Joule::from_force_and_distance(force, distance);

println!("{work}");
```