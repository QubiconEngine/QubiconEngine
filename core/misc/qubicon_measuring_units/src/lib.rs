macro_rules! generate_types {
    { $( $unit_name:ident ),+ } => {
        use num_traits::{Num, NumAssign, Bounded, MulAdd, AsPrimitive, One, Zero, Signed, FromBytes, ToBytes};

        $(
            #[repr(transparent)]
            #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub struct $unit_name<T: Num> (T);

            impl<T: Num> From<T> for $unit_name<T> {
                fn from(value: T) -> Self {
                    Self ( value )
                }
            }

            impl<T: Num + Copy + 'static> AsPrimitive<T> for $unit_name<T> {
                fn as_(self) -> T {
                    self.0
                }
            }



            impl<T: Num> One for $unit_name<T> {
                fn one() -> Self {
                    Self ( T::one() )
                }
            }

            impl<T: Num> Zero for $unit_name<T> {
                fn zero() -> Self {
                    Self ( T::zero() )
                }

                fn is_zero(&self) -> bool {
                    self.0.is_zero()
                }
            }



            impl<T: Num> core::ops::Add for $unit_name<T> {
                type Output = Self;

                fn add(self, rhs: Self) -> Self::Output {
                    Self ( self.0 + rhs.0 )
                }
            }

            impl<T: NumAssign> core::ops::AddAssign for $unit_name<T> {
                fn add_assign(&mut self, rhs: Self) {
                    self.0 += rhs.0;
                }
            }



            impl<T: Num> core::ops::Sub for $unit_name<T> {
                type Output = Self;

                fn sub(self, rhs: Self) -> Self::Output {
                    Self ( self.0 - rhs.0 )
                }
            }

            impl<T: NumAssign> core::ops::SubAssign for $unit_name<T> {
                fn sub_assign(&mut self, rhs: Self) {
                    self.0 -= rhs.0;
                }
            }



            impl<T: Num> core::ops::Mul for $unit_name<T> {
                type Output = Self;

                fn mul(self, rhs: Self) -> Self::Output {
                    Self ( self.0 * rhs.0 )
                }
            }

            impl<T: Num> core::ops::Mul<T> for $unit_name<T> {
                type Output = Self;

                fn mul(self, rhs: T) -> Self::Output {
                    Self ( self.0 * rhs )
                }
            }

            impl<T: NumAssign> core::ops::MulAssign for $unit_name<T> {
                fn mul_assign(&mut self, rhs: Self) {
                    self.0 *= rhs.0;
                }
            }

            impl<T: NumAssign> core::ops::MulAssign<T> for $unit_name<T> {
                fn mul_assign(&mut self, rhs: T) {
                    self.0 *= rhs;
                }
            }



            impl<T: Num> core::ops::Div for $unit_name<T> {
                type Output = Self;

                fn div(self, rhs: Self) -> Self::Output {
                    Self ( self.0 / rhs.0 )
                }
            }

            impl<T: Num> core::ops::Div<T> for $unit_name<T> {
                type Output = Self;

                fn div(self, rhs: T) -> Self::Output {
                    Self ( self.0 / rhs )
                }
            }

            impl<T: NumAssign> core::ops::DivAssign for $unit_name<T> {
                fn div_assign(&mut self, rhs: Self) {
                    self.0 /= rhs.0;
                }
            }

            impl<T: NumAssign> core::ops::DivAssign<T> for $unit_name<T> {
                fn div_assign(&mut self, rhs: T) {
                    self.0 /= rhs;
                }
            }



            impl<T: Num> core::ops::Rem for $unit_name<T> {
                type Output = Self;

                fn rem(self, rhs: Self) -> Self::Output {
                    Self ( self.0 % rhs.0 )
                }
            }

            impl<T: NumAssign> core::ops::RemAssign for $unit_name<T> {
                fn rem_assign(&mut self, rhs: Self) {
                    self.0 %= rhs.0;
                }
            }



            impl<T: Num + core::ops::Neg<Output = T>> core::ops::Neg for $unit_name<T> {
                type Output = Self;

                fn neg(self) -> Self::Output {
                    Self ( -self.0 )
                }
            }




            // TODO: Maybe add little ending what will signal what measuring unit it is ?
            impl<T: Num> Num for $unit_name<T> {
                type FromStrRadixErr = T::FromStrRadixErr;

                fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
                    Ok ( Self ( T::from_str_radix(str, radix)? ) )
                }
            }

            impl<T: Num + Bounded> Bounded for $unit_name<T> {
                fn min_value() -> Self {
                    Self ( T::min_value() )
                }

                fn max_value() -> Self {
                    Self ( T::max_value() )
                }
            }

            impl<T: Num + MulAdd<Output = T>> MulAdd for $unit_name<T> {
                type Output = Self;

                fn mul_add(self, a: Self, b: Self) -> Self::Output {
                    Self ( self.0.mul_add(a.0, b.0) )
                }
            }

            impl<T: Num + FromBytes> FromBytes for $unit_name<T> {
                type Bytes = T::Bytes;

                fn from_be_bytes(bytes: &Self::Bytes) -> Self {
                    Self ( T::from_be_bytes(bytes) )
                }

                fn from_le_bytes(bytes: &Self::Bytes) -> Self {
                    Self ( T::from_le_bytes(bytes) )
                }

                fn from_ne_bytes(bytes: &Self::Bytes) -> Self {
                    Self ( T::from_ne_bytes(bytes) )
                }
            }

            impl<T: Num + ToBytes> ToBytes for $unit_name<T> {
                type Bytes = T::Bytes;

                fn to_be_bytes(&self) -> Self::Bytes {
                    self.0.to_be_bytes()
                }

                fn to_le_bytes(&self) -> Self::Bytes {
                    self.0.to_le_bytes()
                }

                fn to_ne_bytes(&self) -> Self::Bytes {
                    self.0.to_ne_bytes()
                }
            }

            impl<T: Signed> Signed for $unit_name<T> {
                fn abs(&self) -> Self {
                    Self ( self.0.abs() )
                }

                fn abs_sub(&self, other: &Self) -> Self {
                    Self ( self.0.abs_sub(&other.0) )
                }

                fn signum(&self) -> Self {
                    Self ( self.0.signum() )
                }

                fn is_positive(&self) -> bool {
                    self.0.is_positive()
                }

                fn is_negative(&self) -> bool {
                    self.0.is_negative()
                }
            }
        )+
    };
}



pub mod si;