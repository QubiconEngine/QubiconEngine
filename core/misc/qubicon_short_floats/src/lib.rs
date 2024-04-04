#![cfg_attr(not(any(test, feature = "std")), no_std)]

use core::ops::{ BitAnd, BitOr, BitXor };

pub trait ShortFloat: From<f32> + Into<f32> {
    type Storage: BitAnd + BitOr + BitXor;

    const SIGN_BITS: Self::Storage;
    const EXP_BITS: Self::Storage;
    const MANTIS_BITS: Self::Storage;

    fn sign(&self) -> Self::Storage;
    fn exp(&self) -> Self::Storage;
    fn mantis(&self) -> Self::Storage;

    fn to_f64(self) -> f64 {
        self.into() as f64
    }
}

pub mod fp16;