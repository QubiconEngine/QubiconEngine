#[cfg(target_feature = "sse")]
pub use f32x4::F32x4;
#[cfg(target_feature = "sse2")]
pub use f64x2::F64x2;



#[cfg(target_feature = "sse")]
mod f32x4 {
    use super::super::SSE1;
    use core::{
        arch::x86_64::*,
        ops::{ Add, Sub, Mul, Div, Deref, DerefMut }
    };

    #[repr(align(16))]
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct F32x4 ( [f32; 4] );

    impl F32x4 {
        pub const fn new(elements: [f32; 4]) -> Self {
            Self ( elements )
        }

        pub const fn new_fill(value: f32) -> Self {
            Self::new([value, value, value, value])
        }
    }

    impl Add<Self> for F32x4 {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            unsafe {
                let value = _mm_add_ps(
                    core::mem::transmute(self.0),
                    core::mem::transmute(rhs.0)
                );

                Self ( core::mem::transmute(value) )
            }
        }
    }

    impl Sub<Self> for F32x4 {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            unsafe {
                let value = _mm_sub_ps(
                    core::mem::transmute(self.0),
                    core::mem::transmute(rhs.0)
                );

                Self ( core::mem::transmute(value) )
            }
        }
    }

    impl Mul<Self> for F32x4 {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            unsafe {
                let value = _mm_mul_ps(
                    core::mem::transmute(self.0),
                    core::mem::transmute(rhs.0)
                );

                Self ( core::mem::transmute(value) )
            }
        }
    }

    impl Div<Self> for F32x4 {
        type Output = Self;

        fn div(self, rhs: Self) -> Self::Output {
            unsafe {
                let value = _mm_div_ps(
                    core::mem::transmute(self.0),
                    core::mem::transmute(rhs.0)
                );

                Self ( core::mem::transmute(value) )
            }
        }
    }

    impl Deref for F32x4 {
        type Target = [f32; 4];

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl DerefMut for F32x4 {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    unsafe impl SSE1 for F32x4 {
        fn rsqrt(self) -> Self {
            unsafe {
                let value = _mm_rsqrt_ps(core::mem::transmute(self.0));

                Self ( core::mem::transmute(value) )
            }
        }

        fn sqrt(self) -> Self {
            unsafe {
                let value = _mm_sqrt_ps(core::mem::transmute(self.0));

                Self ( core::mem::transmute(value) )
            }
        }

        fn rcp(self) -> Self {
            unsafe {
                let value = _mm_rcp_ps(core::mem::transmute(self.0));

                Self ( core::mem::transmute(value) )
            }
        }


        fn max(self, other: Self) -> Self {
            unsafe {
                let value = _mm_max_ps(
                    core::mem::transmute(self.0),
                    core::mem::transmute(other.0)
                );

                Self ( core::mem::transmute(value) )
            }
        }

        fn min(self, other: Self) -> Self {
            unsafe {
                let value = _mm_min_ps(
                    core::mem::transmute(self.0),
                    core::mem::transmute(other.0)
                );

                Self ( core::mem::transmute(value) )
            }
        }
    }

    impl From<[f32; 4]> for F32x4 {
        fn from(value: [f32; 4]) -> Self {
            Self::new(value)
        }
    }

    impl From<f32> for F32x4 {
        fn from(value: f32) -> Self {
            Self::new_fill(value)
        }
    }

    impl From<F32x4> for [f32; 4] {
        fn from(value: F32x4) -> Self {
            value.0
        }
    }



    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn f32x4() {
            let a = F32x4::new([10.0, 5.0, 3.15, -19.186]);
            let b = F32x4::new_fill(core::f32::consts::PI);

            println!("{:?}\n{:?}\n{:?}\n{:?}\n{:?}", a + b, a - b, a * b, a / b, (b * b).sqrt());
        }
    }
}



#[cfg(target_feature = "sse2")]
mod f64x2 {
    use super::super::SSE1;
    use core::{
        arch::x86_64::*,
        ops::{ Add, Sub, Mul, Div }
    };

    #[repr(transparent)]
    #[derive(Debug, Clone, Copy)]
    pub struct F64x2 ( __m128d );

    impl F64x2 {
        pub fn new(elements: [f64; 2]) -> Self {
            // TODO: Better check. For my CPU there is reverse required. Maybe its endian ?
            Self ( unsafe { _mm_set_pd(elements[1], elements[0]) } )
        }

        pub fn new_fill(value: f64) -> Self {
            Self ( unsafe { _mm_set1_pd(value) } )
        }
    }

    impl Add<Self> for F64x2 {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            unsafe {
                Self ( _mm_add_pd(self.0, rhs.0) )
            }
        }
    }

    impl Sub<Self> for F64x2 {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            unsafe {
                Self ( _mm_sub_pd(self.0, rhs.0) )
            }
        }
    }

    impl Mul<Self> for F64x2 {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            unsafe {
                Self ( _mm_mul_pd(self.0, rhs.0) )
            }
        }
    }

    impl Div<Self> for F64x2 {
        type Output = Self;

        fn div(self, rhs: Self) -> Self::Output {
            unsafe {
                Self ( _mm_div_pd(self.0, rhs.0) )
            }
        }
    }

    unsafe impl SSE1 for F64x2 {
        /// Requires additional division, so not that effective as on f32
        fn rsqrt(self) -> Self {
            Self::new_fill(1.0) / self.sqrt()
        }

        fn sqrt(self) -> Self {
            unsafe { Self ( _mm_sqrt_pd(self.0) ) }
        }

        /// Requires additional division, so not that effective as on f32
        fn rcp(self) -> Self {
            Self::new_fill(1.0) / self
        }



        fn min(self, other: Self) -> Self {
            unsafe { Self ( _mm_min_pd(self.0, other.0) ) }
        }

        fn max(self, other: Self) -> Self {
            unsafe { Self ( _mm_max_pd(self.0, other.0) ) }
        }
    }

    impl From<[f64; 2]> for F64x2 {
        fn from(value: [f64; 2]) -> Self {
            Self::new(value)
        }
    }

    impl From<f64> for F64x2 {
        fn from(value: f64) -> Self {
            Self::new_fill(value)
        }
    }

    impl From<F64x2> for [f64; 2] {
        fn from(value: F64x2) -> Self {
            unsafe { core::mem::transmute(value) }
        }
    }



    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn f64x2() {
            let a = F64x2::new([5.0, 0.0]);
            let b = F64x2::new_fill(core::f64::consts::PI);

            println!("{:?}\n{:?}\n{:?}\n{:?}\n", a + b, a - b, a * b, a / b);
        }
    }
}