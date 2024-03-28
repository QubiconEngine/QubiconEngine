/// Units of SI, but in a symbolic form.
pub mod symbolic {
    pub use super::{
        base_units::{
            Second as S,
            Metre as M,
            KiloGram as Kg,
            Kelvin as K,
            Candela as Cd
        },
        derived_units::{
            Hertz as Hz,
            Newton as N,
            Pascal as Pa,
            Joule as J,
            Watt as W,
            Volt as V,
            Ohm as Ohm, // :)
            Celsius as C,
            Lumen as Lm,
            Lux as Lx
        }
    };
}

/// Basic units of SI
pub mod base_units {
    use super::*;
    use num_traits::FromPrimitive;

    generate_types!{
        Second ("s"),
        Metre ("m"),
        KiloGram ("kg"),
        // Ampere ("A")
        Kelvin ("K"),
        // Mole ("mol")
        Candela ("cd")
    }

    impl<T: Num + FromPrimitive + Copy + 'static> From<derived_units::Celsius<T>> for Kelvin<T> {
        fn from(value: derived_units::Celsius<T>) -> Self {
            Self::from( value.as_() + FromPrimitive::from_f64(273.15).unwrap() )
        }
    }
}

/// Units derived from basic units of SI
pub mod derived_units {
    use super::*;
    use num_traits::FromPrimitive;

    generate_types!{
        // Radiant ("rad"),
        // Steradiant ("sr"),
        Hertz ("Hz"),
        Newton ("N"),
        Pascal ("Pa"),
        Joule ("J"),
        Watt ("W"),
        // Coulomb ("C"),
        Volt ("V"),
        // Farad ("F"),
        Ohm ("\u{03A9}"),
        // Siemens ("S"),
        // Weber ("Wb"),
        // Tesla ("T"),
        // Henry ("H"),
        Celsius ("\u{00B0}\u{0043}"),
        Lumen ("lm"),
        Lux ("lx")
        // Becquerel ("Bq"),
        // Gray ("Gy"),
        // Sievert ("Sv"),
        // Katal ("kat")
    }

    impl<T: Num + Copy + 'static> Hertz<T> {
        pub fn from_time_and_event_count(event_count: T, time: base_units::Second<T>) -> Self {
            Self::from( event_count / time.as_() )
        }
    }

    impl<T: Num + Copy + 'static> Pascal<T> {
        pub fn from_force_and_area(force: Newton<T>, area: base_units::Metre<T>) -> Self {
            Self::from( force.as_() / area.as_() )
        }
    }

    impl<T: Num + Copy + 'static> Joule<T> {
        pub fn from_force_and_distance(force: Newton<T>, dist: base_units::Metre<T>) -> Self {
            Self::from( force.as_() / dist.as_() )
        }
    }

    impl<T: Num + Copy + 'static> Watt<T> {
        pub fn from_work_and_time(work: Joule<T>, time: base_units::Second<T>) -> Self {
            Self::from( work.as_() / time.as_() )
        }
    }

    impl<T: Num + FromPrimitive + Copy + 'static> From<base_units::Kelvin<T>> for Celsius<T> {
        fn from(value: base_units::Kelvin<T>) -> Self {
            Self::from( value.as_() - FromPrimitive::from_f64(273.15).unwrap() )
        }
    }

    impl<T: Num + Copy + 'static> Lumen<T> {
        //pub fn from_intensity_and_angle() -> Self;
    }

    impl<T: Num + Copy + 'static> Lux<T> {
        pub fn from_flux_and_area(flux: Lumen<T>, area: base_units::Metre<T>) -> Self {
            Self::from( flux.as_() / area.as_() )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{base_units::*, derived_units::*};
    use num_traits::{AsPrimitive, Pow};

    // this test is not for types generated there, but for macro what generates them
    #[test]
    fn unit_types_compatibility_with_machine_types() {
        let mut unit = Metre::from(32u16);

        unit *= 2;

        assert_eq!(unit, Metre::from(64));

        #[allow(clippy::assign_op_pattern)]
        {unit = unit * 2}

        assert_eq!(unit, Metre::from(128));
    }

    #[test]
    fn seconds_and_event_count_2_hertz() {
        let event_count = 880.0;
        let time = Second::from(2.0);

        let freq = Hertz::from_time_and_event_count(event_count, time);

        assert_eq!(freq.as_(), event_count / time.as_());
    }

    #[test]
    fn newtons_and_area_2_pascals() {
        let force = Newton::from(2500.0); // average weight of discord mods
        let area = Metre::from(3.0).pow(2);

        let pressure = Pascal::from_force_and_area(force, area);

        assert_eq!(pressure.as_(), force.as_() / area.as_());
    }

    #[test]
    fn kelvin_2_celsius() {
        let t_c = Celsius::from(36.6);
        let t_k = Kelvin::<f32>::from(t_c);

        // of course comparing floats is UB in some way :)
        assert_eq!(t_k.as_(), 36.6 + 273.15);
    }

    #[test]
    fn newtons_and_meters_2_joules() {
        let dist = Metre::from(1.0);
        let force = Newton::from(10.0);

        let work = Joule::from_force_and_distance(force, dist);

        assert_eq!(work.as_(), 10.0 / 1.0);
    }

    #[test]
    fn joules_and_seconds_2_watts() {
        let work = Joule::from(10.0);
        let time = Second::from(5.0);

        let power = Watt::from_work_and_time(work, time);

        assert_eq!(power.as_(), 10.0 / 5.0);
    }

    #[test]
    fn lumens_and_area_to_lux() {
        let lumens = Lumen::from(1.0);
        let area = Metre::from(1.0).pow(2);

        let lux = Lux::from_flux_and_area(lumens, area);

        assert_eq!(lux.as_(), lumens.as_() / area.as_());
    }

    #[test]
    fn print() {
        println!("{}", Celsius::from(36.6));
        println!("{}", Metre::from(100.0));
        println!("{}", Hertz::from(44100.0));
        println!("{}", Ohm::from(2.5))
    }
}