#![cfg_attr(not(any(test, feature = "std")), no_std)]

use core::ops::{ BitAnd, BitOr, BitXor };

pub trait ShortFloat: From<f32> + Into<f32> {
    type Storage: BitAnd + BitOr + BitXor;

    const SIGN_BITS: Self::Storage;
    const EXP_BITS: Self::Storage;
    const MANTIS_BITS: Self::Storage;

    fn sign(&self) -> Self::Storage;
    fn exponent(&self) -> Self::Storage;
    fn mantissa(&self) -> Self::Storage;
}

// impl<T: ShortFloat> From<T> for f64 {
//     fn from(value: T) -> Self {
//         value.into() as f64
//     }
// }

pub mod fp16;