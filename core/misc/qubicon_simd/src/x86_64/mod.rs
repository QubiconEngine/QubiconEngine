// TODO: SSE3 ADDSUB

use core::ops::{ Add, Sub, Mul, Div };

pub use floats::*;
pub use integers::*;

mod floats;
mod integers;

pub trait HorizontalAdd {
    fn hadd(self, rhs: Self) -> Self;
}

pub trait HorizontalSub {
    fn hsub(self, rhs: Self) -> Self;
}

pub trait Abs {
    fn abs(self) -> Self;
}

// Trait names are cringe af
pub trait Vector: Sized + Add<Self, Output = Self> + Sub<Self, Output = Self> {
    type ElementType: Add<Output = Self::ElementType> + Sub<Output = Self::ElementType>;
    const ELEMENTS_COUNT: usize;
}

pub trait VectorExt: Vector + Mul<Self, Output = Self> + Div<Self, Output = Self>
    where Self::ElementType: Mul<Output = Self::ElementType> + Div<Output = Self::ElementType>
{}