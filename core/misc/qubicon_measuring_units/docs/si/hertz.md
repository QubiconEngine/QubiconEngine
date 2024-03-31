Unit of frequency

Equivalent to one event (or cycle) per [second](crate::si::base_units::Second).

More information on [wikipedia](https://en.wikipedia.org/wiki/Hertz)

# Examples
```
# use qubicon_measuring_units::si::{ base_units::Second, derived_units::Hertz };
let event_count = 10.0;
let time = Second::from(1.0);

let frequency = Hertz::from_time_and_event_count(event_count, time);

println!("{frequency}");
```