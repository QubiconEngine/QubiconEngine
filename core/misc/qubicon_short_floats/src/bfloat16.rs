use super::{ ShortFloat, CompressionError };

#[derive(Default, PartialEq, Clone, Copy)]
pub struct BF16 (u16);

impl_display!(BF16);
impl_math_consts!(BF16);

impl ShortFloat for BF16 {
    type Storage = u16;

    const SIGN_BITS: Self::Storage =   0b1000_0000_0000_0000;
    const EXPONENT_BITS: Self::Storage = 0b0111_1111_1000_0000;
    const MANTISSA_BITS: Self::Storage = 0b0000_0000_0111_1111;

    fn sign(&self) -> i8 {
        if self.sign_bits() == 0 { 1 } else { -1 }
    }

    fn exponent(&self) -> i16 {
        self.exponent_bits() as i16 - 0x7f
    }

    fn mantissa(&self) -> u32 {
        self.mantissa_bits() as u32
    }
}

// Same as in HalfF32. Rust doesnt allow const in traits
impl BF16 {
    pub const fn from_f32_flawless_const(value: f32) -> Self {
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

    pub const fn into_f32_const(self) -> f32 {
        let mut out = 0u32;

        out |= (self.sign_bits() as u32) << 31;
        out |= (self.exponent_bits() as u32) << 23;
        out |= (self.mantissa_bits() as u32) << 16;

        #[allow(clippy::transmute_int_to_float)]
        unsafe { core::mem::transmute(out) }
    }

    pub const fn sign_bits(&self) -> u16 {
        (self.0 & Self::SIGN_BITS) >> 15
    }

    pub const fn exponent_bits(&self) -> u16 {
        (self.0 & Self::EXPONENT_BITS) >> 7
    }

    pub const fn mantissa_bits(&self) -> u16 {
        self.0 & Self::MANTISSA_BITS
    }
}

impl TryFrom<f32> for BF16 {
    type Error = CompressionError;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Ok ( Self::from_f32_flawless_const(value) )
    }
}

impl From<BF16> for f32 {
    fn from(value: BF16) -> Self {
        value.into_f32_const()
    }
}



impl PartialOrd for BF16 {
    // TODO: write a comparison instead of casting
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        let self_: f32 = (*self).into();
        let other: f32 = (*other).into();

        self_.partial_cmp(&other)
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