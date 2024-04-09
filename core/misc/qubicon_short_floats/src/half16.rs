use super::{ ShortFloat, CastError };
// use num_traits::float::FloatCore;

#[derive(PartialEq, Clone, Copy)]
pub struct Half16 (u16);

//impl_math_consts!(Half16);

impl ShortFloat for Half16 {
    type Storage = u16;

    const SIGN_BITS: Self::Storage =     0b1000_0000_0000_0000;
    const EXPONENT_BITS: Self::Storage = 0b0111_1100_0000_0000;
    const MANTISSA_BITS: Self::Storage = 0b0000_0011_1111_1111;

    fn sign(&self) -> Self::Storage {
        self.sign()
    }

    fn exponent(&self) -> Self::Storage {
        self.exponent()
    }

    fn mantissa(&self) -> Self::Storage {
        self.mantissa()
    }
}

// Rust doesnt allow const in traits, so let it be there
impl Half16 {
    pub fn from_f32(value: f32) -> Self {
        #[allow(clippy::transmute_float_to_int)]
        let value: u32 = unsafe { core::mem::transmute(value) };

        let exponent = ((value >> 23) & 0xff) as i16 - 127;

        if !(-0x10..=0xf).contains(&exponent) { panic!("exponent out of range!") }

        let exponent = (exponent + 0xf) as u16;

        let mut out = 0u16;

        // sign
        out |= ((value >> 31) as u16) << 15;
        out |= exponent << 10;
        // and offcourse the mantissa
        out |= ((value >> 13) as u16) & Self::MANTISSA_BITS;

        //println!("{value:b}\n{out:b}\n{exponent}");

        Self ( out )
    }

    pub const fn into_f32(self) -> f32 {
        let exponent = self.exponent() as i16 - 0xf;
        let exponent = (exponent + 127) as u32;

        let mut out = 0u32;

        // sign
        out |= (self.sign() as u32) << 31;
        // exponent
        out |= exponent << 23;
        // mantissa
        out |= (self.mantissa() as u32) << 13; 

        #[allow(clippy::transmute_int_to_float)]
        unsafe { core::mem::transmute(out) }
    }

    pub const fn sign(&self) -> u16 {
        (self.0 & Self::SIGN_BITS) >> 15
    }

    pub const fn exponent(&self) -> u16 {
        (self.0 & Self::EXPONENT_BITS) >> 10
    }

    pub const fn mantissa(&self) -> u16 {
        self.0 & Self::MANTISSA_BITS
    }
}

impl TryFrom<f32> for Half16 {
    type Error = CastError;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Ok ( Self::from_f32(value) )
    }
}

impl From<Half16> for f32 {
    fn from(value: Half16) -> Self {
        value.into_f32()
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
    //use super::*;
    use qubicon_simd::F32x4;
    use core::arch::x86_64::*;

    #[repr(transparent)]
    #[derive(Clone, Copy)]
    pub struct Half16x4 ( u64 );

    #[cfg(target_feature = "f16c")]
    impl From<F32x4> for Half16x4 {
        fn from(value: F32x4) -> Self {
            unsafe {
                let m = _mm_cvtps_ph::<_MM_FROUND_TRUNC>( core::mem::transmute(value) );

                #[cfg(target_feature = "sse4.1")]
                let m = _mm_extract_epi64::<0>(m);
                #[cfg(not( target_feature = "sse4.1" ))]
                let m = core::mem::transmute::<_, [u64; 2]>(m)[0];
                
                Self ( m as u64 )
            }
        }
    }

    #[repr(transparent)]
    #[derive(Clone, Copy)]
    pub struct Half16x8 ( __m128i );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils;

    #[test]
    fn half16() {
        let t = Half16::from_f32(65535.0);
        let f: f32 = t.into_f32();

        test_utils::check_stability::<Half16>();
    }
}