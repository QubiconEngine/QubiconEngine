#![cfg_attr(not(test), no_std)]

pub trait ShortFloat {
    fn to_f32(&self) -> f32;
    fn to_f64(&self) -> f64 {
        self.to_f32() as f64
    }
}

