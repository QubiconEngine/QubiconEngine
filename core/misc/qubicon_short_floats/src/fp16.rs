use super::ShortFloat;

#[derive(PartialEq, Clone, Copy)]
pub struct HalfF16 (u16);

impl ShortFloat for HalfF16 {
    type Storage = u16;

    const SIGN_BITS: Self::Storage =   0b1000_0000_0000_0000;
    const EXP_BITS: Self::Storage =    0b0111_1100_0000_0000;
    const MANTIS_BITS: Self::Storage = 0b0000_0011_1111_1111;

    fn sign(&self) -> Self::Storage {
        (self.0 & Self::SIGN_BITS) >> 15
    }

    fn exponent(&self) -> Self::Storage {
        (self.0 & Self::EXP_BITS) >> 10
    }

    fn mantissa(&self) -> Self::Storage {
        self.0 & Self::MANTIS_BITS
    }
}

impl From<f32> for HalfF16 {
    fn from(value: f32) -> Self {
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
        out |= ((value >> 13) as u16) & Self::MANTIS_BITS;

        Self ( out )
    }
}

impl From<HalfF16> for f32 {
    fn from(value: HalfF16) -> Self {
        let exponent = value.exponent();

        let mut out = 0u32;

        // sign
        out |= (value.sign() as u32) << 31;
        // exponent sign
        out |= ((exponent & 0b1_0000) as u32) << 26;
        // exponent
        out |= ((exponent & 0b1111) as u32) << 23;
        // mantissa
        out |= (value.mantissa() as u32) << 13; 

        #[allow(clippy::transmute_int_to_float)]
        unsafe { core::mem::transmute(out) }
    }
}




#[derive(PartialEq, Clone, Copy)]
pub struct BF16 (u16);

impl ShortFloat for BF16 {
    type Storage = u16;

    const SIGN_BITS: Self::Storage =   0b1000_0000_0000_0000;
    const EXP_BITS: Self::Storage =    0b0111_1111_1000_0000;
    const MANTIS_BITS: Self::Storage = 0b0000_0000_0111_1111;

    fn sign(&self) -> Self::Storage {
        (self.0 & Self::SIGN_BITS) >> 15
    }

    fn exponent(&self) -> Self::Storage {
        (self.0 & Self::EXP_BITS) >> 7
    }

    fn mantissa(&self) -> Self::Storage {
        self.0 & Self::MANTIS_BITS
    }
}

impl From<f32> for BF16 {
    fn from(value: f32) -> Self {
        #[allow(clippy::transmute_float_to_int)]
        let value: u32 = unsafe { core::mem::transmute(value) };

        let mut out = 0u16;

        // sign
        out |= ((value >> 31) as u16) << 15;
        // exponent(with sign)
        out |= (((value >> 23) as u16) & 0b1111_1111) << 7;
        // mantissa
        out |= ((value >> 16) as u16) & Self::MANTIS_BITS;

        Self ( out )
    }
}

impl From<BF16> for f32 {
    fn from(value: BF16) -> Self {
        let mut out = 0u32;

        out |= (value.sign() as u32) << 31;
        out |= (value.exponent() as u32) << 23;
        out |= (value.mantissa() as u32) << 16;

        #[allow(clippy::transmute_int_to_float)]
        unsafe { core::mem::transmute(out) }
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