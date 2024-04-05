use core::ops::{ Add, Sub, Mul, Div, Deref, DerefMut };

mod floats;

/// TODO: Comparison
pub unsafe trait SSE1: Sized +
    Add<Self, Output = Self> + Sub<Self, Output = Self> +
    Mul<Self, Output = Self> + Div<Self, Output = Self>
{
    fn rsqrt(self) -> Self;
    fn sqrt(self) -> Self;
    fn rcp(self) -> Self;

    fn max(self, other: Self) -> Self;
    fn min(self, other: Self) -> Self;
}

