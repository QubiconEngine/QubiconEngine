use super::{ ShortFloat, CompressionError };
// use num_traits::float::FloatCore;

#[derive(Default, PartialEq, Clone, Copy)]
pub struct Half16 (u16);

impl_math_consts!(Half16);

impl ShortFloat for Half16 {
    type Storage = u16;

    const SIGN_BITS: Self::Storage =     0b1000_0000_0000_0000;
    const EXPONENT_BITS: Self::Storage = 0b0111_1100_0000_0000;
    const MANTISSA_BITS: Self::Storage = 0b0000_0011_1111_1111;

    fn sign(&self) -> i8 {
        if self.sign_bits() == 0 { 1 } else { -1 }
    }

    fn exponent(&self) -> i16 {
        self.exponent_bits() as i16 - 0xf
    }

    fn mantissa(&self) -> u32 {
        self.mantissa_bits() as u32
    }
}

impl Half16 {
    pub const fn from_f32_const(value: f32) -> Result<Self, CompressionError> {
        #[allow(clippy::transmute_float_to_int)]
        let value: u32 = unsafe { core::mem::transmute(value) };


        // unbiased exponent
        let exponent = ((value >> 23) & 0xff) as i16 - 0x7f;

        // check if it fits in range, what can be represented by 5 bits
        if -0x10 > exponent || exponent > 0xf {
            return Err( CompressionError )
        }



        // rebias exponent
        let exponent = (exponent + 0xf) as u16;


        let mut out = 0u16;

        // sign
        out |= ((value >> 31) as u16) << 15;
        // exponent
        out |= (exponent as u16) << 10;
        // and offcourse the mantissa
        out |= ((value >> 13) as u16) & Self::MANTISSA_BITS;



        Ok ( Self ( out ) )
    }

    /// Returns NaN instead of error
    pub const fn from_f32_flawless_const(value: f32) -> Self {
        // unwrap_or is not available in const fn's :(
        match Self::from_f32_const(value) {
            Ok ( v ) => v,
            Err ( _ ) => Self ( 0b0111_1100_1000_0000 )
        }
    } 

    pub const fn into_f32_const(self) -> f32 {
        // rebias exponent
        let exponent = (self.exponent_bits() as i16 - 0xf + 0x7f) as u32;

        let mut out = 0u32;

        // sign
        out |= (self.sign_bits() as u32) << 31;
        // exponent
        out |= exponent << 23;
        // mantissa
        out |= (self.mantissa_bits() as u32) << 13; 

        #[allow(clippy::transmute_int_to_float)]
        unsafe { core::mem::transmute(out) }
    }

    pub const fn sign_bits(&self) -> u16 {
        (self.0 & Self::SIGN_BITS) >> 15
    }

    pub const fn exponent_bits(&self) -> u16 {
        (self.0 & Self::EXPONENT_BITS) >> 10
    }

    pub const fn mantissa_bits(&self) -> u16 {
        self.0 & Self::MANTISSA_BITS
    }
}

impl TryFrom<f32> for Half16 {
    type Error = CompressionError;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        #[cfg(target_arch = "x86_64")] {
            #[cfg(target_feature = "f16c")] unsafe {
                use core::arch::x86_64::*;

                let m = _mm_set1_ps(value);
                let m = _mm_cvtps_ph::<_MM_FROUND_TRUNC>(m);

                let m = _mm_extract_epi16::<0>(m);

                return Ok ( Self ( m as u16 ) );
            }
        }


        #[allow(unreachable_code)]
        Self::from_f32_const(value)
    }
}

impl From<Half16> for f32 {
    fn from(value: Half16) -> Self {
        #[cfg(target_arch = "x86_64")] {
            #[cfg(target_feature = "f16c")] unsafe {
                use core::arch::x86_64::*;

                let m = _mm_set1_epi16(value.0 as i16);
                let m = _mm_cvtph_ps(m);

                // sse4.1 is implicitly enabled by f16c
                return f32::from_bits(_mm_extract_ps::<0>(m) as u32);
            }
        }

        #[allow(unreachable_code)]
        value.into_f32_const()
    }
}

// impl FloatCore for Half16 {
//     fn infinity() -> Self {
//         Self ( 0b0111_1100_0000_0000 )
//     }

//     fn neg_infinity() -> Self {
//         Self ( 0b1111_1100_0000_0000 )
//     }

//     fn nan() -> Self {
//         Self ( 0b0111_1100_1000_0000 )
//     }

//     fn neg_zero() -> Self {
//         Self::from_f32(-0.0)
//     }

//     fn min_value() -> Self {
//         Self ( 0b1011_1100_0000_0001 )
//     }

//     fn min_positive_value() -> Self {
//         Self ( 0b0111_1000_0000_0001 )
//     }

//     fn epsilon() -> Self {
//         Self::min_positive_value()
//     }

//     fn max_value() -> Self {
//         Self ( 0b0011_1011_1111_1111 )
//     }

//     fn classify(self) -> core::num::FpCategory {
//         todo!()
//     }

//     fn to_degrees(self) -> Self {
//         todo!()
//     }

//     fn to_radians(self) -> Self {
//         todo!()
//     }

//     fn integer_decode(self) -> (u64, i16, i8) {
//         let sign = self.sign();
//         let exponent = self.exponent();
//         let mantissa = self.mantissa();

//         let exponent_sign = (exponent & 0b1_0000) >> 4;
//         let exponent = (exponent & 0b0_1111) as i16;

//         let sign = if sign == 0 { 1i8 } else { -1i8 };
//         let exponent = if exponent_sign == 0 { exponent } else { -exponent };

//         (mantissa as u64, exponent, sign)
//     }
// }

#[cfg(feature = "vectors")]
mod vec {
    #[cfg(target_arch = "x86_64")]
    pub use x86_64::*;


    use super::*;
    
    #[cfg(target_arch = "x86_64")]
    mod x86_64 {
        use super::*;

        #[allow(unused_imports)]
        #[cfg(target_feature = "sse")]
        use qubicon_simd::F32x4;
        use qubicon_simd::Vector;
        use core::arch::x86_64::*;

        #[repr(transparent)]
        #[derive(Clone, Copy)]
        pub struct Half16x4 ( u64 );

        impl Vector for Half16x4 {
            type ElementType = Half16;
            const ELEMENTS_COUNT: usize = 4;
        }

        #[cfg(target_feature = "f16c")]
        impl From<F32x4> for Half16x4 {
            fn from(value: F32x4) -> Self {
                unsafe {
                    let m = _mm_cvtps_ph::<_MM_FROUND_TRUNC>( core::mem::transmute(value) );

                    // f16c enables avx, avx - sse4.2 and sse4.2 - sse4.1
                    let m = _mm_extract_epi64::<0>(m);
                
                    Self ( m as u64 )
                }
            }
        }

        #[cfg(target_feature = "f16c")]
        impl From<Half16x4> for F32x4 {
            fn from(value: Half16x4) -> Self {
                unsafe {
                    let m = _mm_set1_epi64x(value.0 as i64);
                    let m = _mm_cvtph_ps(m);

                    core::mem::transmute(m)
                }
            }
        }

        // compile error because I dont want panic in runtime
        #[cfg(all( not(target_feature = "f16c"), target_feature = "sse" ))]
        impl From<F32x4> for Half16x4 {
            fn from(value: F32x4) -> Self {
                compile_error!("TODO: conversion from f32x4 to half16x4 without f16c")
            }
        }

        #[cfg(all( not(target_feature = "f16c"), target_feature = "sse" ))]
        impl From<Half16x4> for F32x4 {
            fn from(value: F32x4) -> Self {
                compile_error!("TODO: conversion from half16x4 to f32x4 without f16c")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils;

    #[test]
    fn half16() {
        let t = Half16::from_f32_const(56.1267).unwrap();

        println!("{}", t.into_f32_const());

        test_utils::check_stability::<Half16>();
    }
}