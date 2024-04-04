#![cfg_attr(not(any(test, feature = "std")), no_std)]

use core::ops::{ BitAnd, BitOr, BitXor };

pub trait ShortFloat: From<f32> + Into<f32> {
    type Storage: BitAnd + BitOr + BitXor;

    const SIGN_BITS: Self::Storage;
    const EXPONENT_BITS: Self::Storage;
    const MANTISSA_BITS: Self::Storage;

    fn sign(&self) -> Self::Storage;
    fn exponent(&self) -> Self::Storage;
    fn mantissa(&self) -> Self::Storage;
}

// impl<T: ShortFloat> From<T> for f64 {
//     fn from(value: T) -> Self {
//         value.into() as f64
//     }
// }

macro_rules! def_math_constants {
    ( $ty:ident, $( $name:ident ),+ ) => {
        impl $ty {
            $(
                pub const $name: Self = Self::from_f32(core::f32::consts::$name);
            )+
        }
    };
}

macro_rules! impl_math_consts {
    ($ty:ident) => {
        def_math_constants!{
            $ty,

            E,
            FRAC_1_PI,
            FRAC_1_SQRT_2,
            FRAC_2_PI,
            FRAC_2_SQRT_PI,
            FRAC_PI_2,
            FRAC_PI_3,
            FRAC_PI_4,
            FRAC_PI_6,
            FRAC_PI_8,
            LN_2,
            LN_10,
            LOG2_10,
            LOG2_E,
            LOG10_2,
            LOG10_E,
            PI,
            SQRT_2,
            TAU
        }

        impl num_traits::FloatConst for $ty {
            fn E() -> Self {
                Self::E
            }

            fn FRAC_1_PI() -> Self {
                Self::FRAC_1_PI
            }

            fn FRAC_1_SQRT_2() -> Self {
                Self::FRAC_1_SQRT_2
            }

            fn FRAC_2_PI() -> Self {
                Self::FRAC_2_PI
            }

            fn FRAC_2_SQRT_PI() -> Self {
                Self::FRAC_2_SQRT_PI
            }

            fn FRAC_PI_2() -> Self {
                Self::FRAC_PI_2
            }

            fn FRAC_PI_3() -> Self {
                Self::FRAC_PI_3
            }

            fn FRAC_PI_4() -> Self {
                Self::FRAC_PI_4
            }

            fn FRAC_PI_6() -> Self {
                Self::FRAC_PI_6
            }

            fn FRAC_PI_8() -> Self {
                Self::FRAC_PI_8
            }

            fn LN_10() -> Self {
                Self::LN_10
            }

            fn LN_2() -> Self {
                Self::LN_2
            }

            fn LOG10_E() -> Self {
                Self::LOG10_E
            }

            fn LOG2_E() -> Self {
                Self::LOG2_E
            }

            fn PI() -> Self {
                Self::PI
            }

            fn SQRT_2() -> Self {
                Self::SQRT_2
            }
        }
    };
}

pub mod fp16;