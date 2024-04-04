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

    fn exp(&self) -> Self::Storage {
        (self.0 & Self::EXP_BITS) >> 10
    }

    fn mantis(&self) -> Self::Storage {
        self.0 & Self::MANTIS_BITS
    }
}

impl From<f32> for HalfF16 {
    fn from(value: f32) -> Self {
        #[allow(clippy::transmute_float_to_int)]
        let value: u32 = unsafe { core::mem::transmute(value) };

        let sign = value >> 31;
        let exp_sign = value << 1 >> 31;
        let exp = value << 1 >> 24;
        let mantis = value & 0b0000_0000_0111_1111_1111_1111_1111_1111;

        let mut out = 0u16;

        out |= (sign as u16) << 15;
        out |= (exp_sign as u16) << 14;
        out |= (exp as u16 & 0b1111) << 10;
        out |= (mantis >> 13) as u16;

        Self ( out )
    }
}

impl Into<f32> for HalfF16 {
    fn into(self) -> f32 {
        let sign = self.sign();
        let exp = self.exp();
        let mantis = self.mantis();

        let exp_sign = exp >> 4;
        let exp = exp & 0b1111;

        let mut out = 0u32;

        out |= (sign as u32) << 31;
        out |= (exp_sign as u32) << 30;
        out |= ((exp & 0b1111) as u32) << 23;
        out |= (mantis as u32) << 13;

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

    fn exp(&self) -> Self::Storage {
        (self.0 & Self::EXP_BITS) >> 7
    }

    fn mantis(&self) -> Self::Storage {
        self.0 & Self::MANTIS_BITS
    }
}

impl From<f32> for BF16 {
    fn from(value: f32) -> Self {
        #[allow(clippy::transmute_float_to_int)]
        let value: u32 = unsafe { core::mem::transmute(value) };

        let mut out = 0u16;

        let sign = (value >> 31) as u16;
        let exp = (value << 1 >> 24) as u16;
        let mantis = (value >> 22) as u16 & Self::MANTIS_BITS;

        out |= sign << 15;
        out |= exp << 7;
        out |= mantis;

        Self ( out )
    }
}

impl Into<f32> for BF16 {
    fn into(self) -> f32 {
        let mut out = 0u32;
        
        let sign = self.sign();
        let exp = self.exp();
        let mantis = self.mantis();

        out |= (sign as u32) << 31;
        out |= (exp as u32) << 23;
        out |= (mantis as u32) << 22;

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