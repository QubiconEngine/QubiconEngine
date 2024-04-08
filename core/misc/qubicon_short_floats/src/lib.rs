#![cfg_attr(not(any(test, feature = "std")), no_std)]

use core::ops::{ BitAnd, BitOr, BitXor, Shl, Shr };

pub trait ShortFloat: From<f32> + Into<f32> {
    type Storage: BitAnd + BitOr + BitXor + Shl + Shr;

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

macro_rules! def_float_const_fns {
    ( $( $const_name:ident ),+ ) => {
        $(
            fn $const_name() -> Self {
                Self::$const_name
            }
        )+
    };
}

macro_rules! impl_float_const {
    ($ty:ident) => {
        impl num_traits::float::FloatConst for $ty {
            def_float_const_fns!{
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

macro_rules! impl_math_consts {
    ($ty:ident) => {
        impl_float_const!($ty);
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
    };
}

pub mod half16;
pub mod bfloat16;

#[cfg(test)]
mod test_utils {
    use super::ShortFloat;

    pub fn check_stability<T: ShortFloat + PartialEq + Copy>() {
        let num = T::from(-19.0);
        let num_f32: f32 = num.into();
        let num_r = T::from(num_f32);

        assert!(num == num_r);
    }
}