use super::{ ShortFloat, CastError };

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

impl TryFrom<f32> for BF16 {
    type Error = CastError;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Ok ( Self::from_f32(value) )
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
    use crate::test_utils;

    #[test]
    fn bfloat16() {
        test_utils::check_stability::<BF16>();
    }
}