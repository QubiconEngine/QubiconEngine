Unit of power

One watt is equal to one [joule](crate::si::derived_units::Joule) per [second](crate::si::base_units::Second).

More information on [wikipedia](https://en.wikipedia.org/wiki/Watt)

# Examples
```
# use qubicon_measuring_units::si::{ base_units::Second, derived_units::{ Joule, Watt } };
let work = Joule::from(15.0);
let time = Second::from(3.0);

let power = work / time; // same as Watt::from_work_and_time

println!("{power}");
```