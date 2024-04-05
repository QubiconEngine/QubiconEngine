use super::ShortFloat;
// use num_traits::float::FloatCore;

#[derive(PartialEq, Clone, Copy)]
pub struct HalfF16 (u16);

pub mod half16v;

impl_math_consts!(HalfF16);

impl ShortFloat for HalfF16 {
    type Storage = u16;

    const SIGN_BITS: Self::Storage =   0b1000_0000_0000_0000;
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
impl HalfF16 {
    pub const fn from_f32(value: f32) -> Self {
        #[allow(clippy::transmute_float_to_int)]
        let value: u32 = unsafe { core::mem::transmute(value) };

        let mut out = 0u16;

        // sign
        out |= ((value >> 31) as u16) << 15;
        // exponent sign
        out |= (((value >> 30) as u16) & 0b01) << 14;
        // exponent itself
        out |= (((value >> 23) as u16) & 0b1111) << 10;
        // and offcourse the mantissa
        out |= ((value >> 13) as u16) & Self::MANTISSA_BITS;

        Self ( out )
    }

    pub const fn into_f32(self) -> f32 {
        let exponent = self.exponent();

        let mut out = 0u32;

        // sign
        out |= (self.sign() as u32) << 31;
        // exponent sign
        out |= ((exponent & 0b1_0000) as u32) << 26;
        // exponent
        out |= ((exponent & 0b1111) as u32) << 23;
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

impl From<f32> for HalfF16 {
    fn from(value: f32) -> Self {
        Self::from_f32(value)
    }
}

impl From<HalfF16> for f32 {
    fn from(value: HalfF16) -> Self {
        value.into_f32()
    }
}

// impl FloatCore for HalfF16 {
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



#[derive(PartialEq, Clone, Copy)]
pub struct BF16 (u16);

impl_math_consts!(BF16);

impl ShortFloat for BF16 {
    type Storage = u16;

    const SIGN_BITS: Self::Storage =   0b1000_0000_0000_0000;
    const EXPONENT_BITS: Self::Storage = 0b0111_1111_1000_0000;
    const MANTISSA_BITS: Self::Storage = 0b0000_0000_0111_1111;

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

// Same as in HalfF32. Rust doesnt allow const in traits
impl BF16 {
    pub const fn from_f32(value: f32) -> Self {
        #[allow(clippy::transmute_float_to_int)]
        let value: u32 = unsafe { core::mem::transmute(value) };

        let mut out = 0u16;

        // sign
        out |= ((value >> 31) as u16) << 15;
        // exponent(with sign)
        out |= (((value >> 23) as u16) & 0b1111_1111) << 7;
        // mantissa
        out |= ((value >> 16) as u16) & Self::MANTISSA_BITS;

        Self ( out )
    }

    pub const fn into_f32(self) -> f32 {
        let mut out = 0u32;

        out |= (self.sign() as u32) << 31;
        out |= (self.exponent() as u32) << 23;
        out |= (self.mantissa() as u32) << 16;

        #[allow(clippy::transmute_int_to_float)]
        unsafe { core::mem::transmute(out) }
    }

    pub const fn sign(&self) -> u16 {
        (self.0 & Self::SIGN_BITS) >> 15
    }

    pub const fn exponent(&self) -> u16 {
        (self.0 & Self::EXPONENT_BITS) >> 7
    }

    pub const fn mantissa(&self) -> u16 {
        self.0 & Self::MANTISSA_BITS
    }
}

impl From<f32> for BF16 {
    fn from(value: f32) -> Self {
        Self::from_f32(value)
    }
}

impl From<BF16> for f32 {
    fn from(value: BF16) -> Self {
        value.into_f32()
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    fn check_stability<T: ShortFloat + PartialEq + Copy>() {
        let num = T::from(-19.0);
        let num_f32: f32 = num.into();
        let num_r = T::from(num_f32);

        assert!(num == num_r);
    }

    #[test]
    fn bf16() {
        check_stability::<BF16>();
    }

    #[test]
    fn half_f16() {
        check_stability::<HalfF16>();
    }
}