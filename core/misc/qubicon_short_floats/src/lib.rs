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
    ( $( $name:ident ),+ ) => {
        $(
            pub const $name: Self = Self::from_f32(core::f32::consts::$name);
        )+
    };
}

macro_rules! impl_math_consts {
    ($ty:ident) => {
        impl $ty {
            def_math_constants!{
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
        }
    };
}

pub mod fp16;