// TODO: SSE3 ADDSUB, more conversions

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

pub trait MinMax {
    fn max(self, rhs: Self) -> Self;
    fn min(self, rhs: Self) -> Self;
}

// Trait names are cringe af
pub trait Vector: Sized {
    type ElementType: Sized;
    const ELEMENTS_COUNT: usize;
}

pub trait VectorOps: Vector + Add<Output = Self> + Sub<Output = Self>
    where Self::ElementType: Add<Output = Self::ElementType> + Sub<Output = Self::ElementType>
{}

pub trait VectorOpsExt: Vector + Mul<Output = Self> + Div<Output = Self>
    where Self::ElementType: Mul<Output = Self::ElementType> + Div<Output = Self::ElementType>
{}

pub trait Extract: Vector {
    // i32 is due to lack of static assert and const expressions.
    // For i32 there is a static assert inside _mm_extract_** functions.
    fn get<const IDX: i32>(&self) -> Self::ElementType;
}