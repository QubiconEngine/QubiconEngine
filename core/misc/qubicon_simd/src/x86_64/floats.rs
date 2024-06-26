#[cfg(target_feature = "sse")]
pub use f32x4::F32x4;
#[cfg(target_feature = "sse2")]
pub use f64x2::F64x2;


#[allow(unused_imports)]
use super::{ Vector, VectorOps, VectorOpsExt, HorizontalAdd, HorizontalSub, MinMax, Extract };
use core::{
    arch::x86_64::*,
    ops::{ Add, Sub, Mul, Div }
};


// What the fuck ?
pub trait FloatVector: VectorOpsExt + MinMax
    where Self::ElementType: Mul<Output = Self::ElementType> + Div<Output = Self::ElementType>
{
    fn sqrt(self) -> Self;
}

pub trait FloatVectorExt: FloatVector
    where Self::ElementType: Mul<Output = Self::ElementType> + Div<Output =  Self::ElementType>
{
    fn rsqrt(self) -> Self;
    fn rcp(self) -> Self;
}




#[cfg(target_feature = "sse")]
mod f32x4 {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug, Clone, Copy)]
    pub struct F32x4 ( pub(crate) __m128 );

    impl F32x4 {
        pub fn new(n1: f32, n2: f32, n3: f32, n4: f32) -> Self {
            Self ( unsafe { _mm_set_ps(n4, n3, n2, n1) } )
        }

        pub fn new_fill(value: f32) -> Self {
            Self ( unsafe { _mm_set1_ps(value) } )
        }
    }

    impl Add<Self> for F32x4 {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            unsafe {
                Self ( _mm_add_ps(self.0, rhs.0) )
            }
        }
    }

    impl Sub<Self> for F32x4 {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            unsafe {
                Self ( _mm_sub_ps(self.0, rhs.0) )
            }
        }
    }

    impl Mul<Self> for F32x4 {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            unsafe {
                Self ( _mm_mul_ps(self.0, rhs.0) )
            }
        }
    }

    impl Div<Self> for F32x4 {
        type Output = Self;

        fn div(self, rhs: Self) -> Self::Output {
            unsafe {
                Self ( _mm_div_ps(self.0, rhs.0) )
            }
        }
    }

    impl From<[f32; 4]> for F32x4 {
        fn from(value: [f32; 4]) -> Self {
            Self::new(value[0], value[1], value[2], value[3])
        }
    }

    impl From<(f32, f32, f32, f32)> for F32x4 {
        fn from((n1, n2, n3, n4): (f32, f32, f32, f32)) -> Self {
            Self::new(n1, n2, n3, n4)
        }
    }

    impl From<f32> for F32x4 {
        fn from(value: f32) -> Self {
            Self::new_fill(value)
        }
    }

    #[cfg(target_feature = "sse2")]
    impl From<super::super::I32x4> for F32x4 {
        fn from(value: super::super::I32x4) -> Self {
            unsafe { Self ( _mm_cvtepi32_ps(value.0) ) }
        }
    }


    #[cfg(target_feature = "sse3")]
    impl HorizontalAdd for F32x4 {
        fn hadd(self, rhs: Self) -> Self {
            unsafe { Self ( _mm_hadd_ps(self.0, rhs.0) ) }
        }
    }

    #[cfg(target_feature = "sse3")]
    impl HorizontalSub for F32x4 {
        fn hsub(self, rhs: Self) -> Self {
            unsafe { Self ( _mm_hsub_ps(self.0, rhs.0) ) }
        }
    }

    impl MinMax for F32x4 {
        fn max(self, other: Self) -> Self {
            unsafe {
                Self ( _mm_max_ps(self.0, other.0) )
            }
        }

        fn min(self, other: Self) -> Self {
            unsafe {
                Self ( _mm_min_ps(self.0, other.0) )
            }
        }
    }



    impl Vector for F32x4 {
        type ElementType = f32;
        const ELEMENTS_COUNT: usize = 4;
    }
    impl VectorOps for F32x4 {}
    impl VectorOpsExt for F32x4 {}

    #[cfg(target_feature = "sse4.1")]
    impl Extract for F32x4 {
        // TODO: static assert
        fn get<const IDX: i32>(&self) -> Self::ElementType {
            f32::from_bits( unsafe { _mm_extract_ps::<IDX>(self.0) } as u32 )
        }
    }

    impl FloatVector for F32x4 {
        fn sqrt(self) -> Self {
            unsafe {
                Self ( _mm_sqrt_ps(self.0) )
            }
        }
    }
    impl FloatVectorExt for F32x4 {
        fn rsqrt(self) -> Self {
            unsafe {
                Self ( _mm_rsqrt_ps(self.0) )
            }
        }

        fn rcp(self) -> Self {
            unsafe {
                Self ( _mm_rcp_ps(self.0) )
            }
        }
    }



    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn f32x4() {
            let a = F32x4::new(10.0, 5.0, 3.15, -19.186);
            let b = F32x4::new_fill(core::f32::consts::PI);

            println!("{:?}\n{:?}\n{:?}\n{:?}\n{:?}", a + b, a - b, a * b, a / b, (b * b).sqrt());
        }
    }
}



#[cfg(target_feature = "sse2")]
mod f64x2 {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug, Clone, Copy)]
    pub struct F64x2 ( pub(crate) __m128d );

    impl F64x2 {
        pub fn new(n1: f64, n2: f64) -> Self {
            Self ( unsafe { _mm_set_pd(n2, n1) } )
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

    impl From<[f64; 2]> for F64x2 {
        fn from(value: [f64; 2]) -> Self {
            Self::new(value[0], value[1])
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


    #[cfg(target_feature = "sse3")]
    impl HorizontalAdd for F64x2 {
        fn hadd(self, rhs: Self) -> Self {
            unsafe { Self ( _mm_hadd_pd(self.0, rhs.0) ) }
        }
    }

    #[cfg(target_feature = "sse3")]
    impl HorizontalSub for F64x2 {
        fn hsub(self, rhs: Self) -> Self {
            unsafe { Self ( _mm_hsub_pd(self.0, rhs.0) ) }
        }
    }

    impl MinMax for F64x2 {
        fn min(self, other: Self) -> Self {
            unsafe { Self ( _mm_min_pd(self.0, other.0) ) }
        }

        fn max(self, other: Self) -> Self {
            unsafe { Self ( _mm_max_pd(self.0, other.0) ) }
        }
    }



    impl Vector for F64x2 {
        type ElementType = f64;
        const ELEMENTS_COUNT: usize = 2;
    }
    impl VectorOps for F64x2 {}
    impl VectorOpsExt for F64x2 {}


    impl FloatVector for F64x2 {
        fn sqrt(self) -> Self {
            unsafe { Self ( _mm_sqrt_pd(self.0) ) }
        }
    }



    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn f64x2() {
            let a = F64x2::new(5.0, 0.0);
            let b = F64x2::new_fill(core::f64::consts::PI);

            println!("{:?}\n{:?}\n{:?}\n{:?}\n", a + b, a - b, a * b, a / b);
        }
    }
}