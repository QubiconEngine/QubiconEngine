Unit of preasure

Also used for internal preasure and stress.
There also an atmospheric presure unit(**atm** for short). The **atm**
unit is roughly equal to atmosperic presure on sea level(*101,325 Pa*).

Derived as one [newton](crate::si::derived_units::Newton) per [meter](crate::si::base_units::Metre)<sup>2</sup>.

More information on [wikipedia](https://en.wikipedia.org/wiki/Pascal_(unit))

# Examples
```
# use num_traits::Pow;
# use qubicon_measuring_units::si::{ base_units::Metre, derived_units::{ Newton, Pascal } };
let area = Metre::from(1.0).pow(2);
let force = Newton::from(1000.0); // Weight of 100 kg on Earth

let preasure = Pascal::from_force_and_area(force, area);

println!("{preasure}");
```