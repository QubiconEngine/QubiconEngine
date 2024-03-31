Unit of time

Historical defenition is 1/86400 of a day (24 * 60 * 60).
But Earth rotation speed changes with time, so this value is inaccurate.

Nowadays second is defined in a complex way(I cant understand, so just copy from [wiki](https://en.wikipedia.org/wiki/Second)):
> Second is defined by taking the fixed numerical value of the caesium frequency,
> the unperturbed ground-state hyperfine transition frequency of the caesium 133 atom,
> to be 9 192 631 770 Hz, which is equal to s<sup>-1</sup>

# Examples
```
# use qubicon_measuring_units::si::{ base_units::Second, derived_units::{ Joule, Watt } };

let time = Second::from(5.0);
let work = Joule::from(20.0);

let power = Watt::from_work_and_time(work, time);

println!("{power}");
```