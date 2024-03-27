pub mod base_units {
    use super::*;
    use num_traits::FromPrimitive;

    generate_types!{
        Second,
        Metre,
        KiloGram,
        // Ampere
        Kelvin,
        // Mole
        Candela
    }

    impl<T: Num + FromPrimitive + Copy + 'static> From<derived_units::Celsius<T>> for Kelvin<T> {
        fn from(value: derived_units::Celsius<T>) -> Self {
            Self::from( value.as_() + FromPrimitive::from_f64(273.15).unwrap() )
        }
    }
}

pub mod derived_units {
    use super::*;
    use num_traits::FromPrimitive;

    generate_types!{
        // Radiant,
        // Steradiant,
        Hertz,
        Newton,
        Pascal,
        Joule,
        Watt,
        // Coulomb,
        Volt,
        // Farad,
        Ohm,
        // Siemens,
        // Weber,
        // Tesla,
        // Henry,
        Celsius,
        Lumen,
        Lux
        // Becquerel,
        // Gray,
        // Sievert,
        // Katal
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_traits::AsPrimitive;

    #[test]
    fn kelvin_2_celsius() {
        let t_c = derived_units::Celsius::from(36.6);
        let t_k = base_units::Kelvin::<f32>::from(t_c);

        // of course comparing floats is UB in some way :)
        assert_eq!(t_k.as_(), 36.6 + 273.15);
    }
}