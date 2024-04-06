use core::ops::{ Add, Sub, Mul, Div };

pub use floats::*;
pub use integers::*;

mod floats;
mod integers;


// Trait names are cringe af
pub trait Vector: Sized + Add<Self, Output = Self> + Sub<Self, Output = Self> {
    type ElementType: Add<Output = Self::ElementType> + Sub<Output = Self::ElementType>;
    const ELEMENTS_COUNT: usize;
}

pub trait VectorExt: Vector + Mul<Self, Output = Self> + Div<Self, Output = Self>
    where Self::ElementType: Mul<Output = Self::ElementType> + Div<Output = Self::ElementType>
{}