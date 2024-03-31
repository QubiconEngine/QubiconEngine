Measuring unit for length.

One meter is a distance what light travels in vacuum during a 1/299 792 458 of a second.

# Examples
```
# use qubicon_measuring_units::si::{ base_units::Metre, derived_units::{ Newton, Joule } };

let distance = Metre::from(3.0);
let force = Newton::from(10.0);

let work = Joule::from_force_and_distance(force, distance);

println!("{work}");
```