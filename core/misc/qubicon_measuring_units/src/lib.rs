macro_rules! generate_types {
    { $( $unit_name:ident ),+ } => {
        use num_traits::{Num, NumAssign, Bounded, MulAdd};

        $(
            #[repr(transparent)]
            #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct $unit_name<T: Num> (T);

            impl<T: Num> From<T> for $unit_name<T> {
                fn from(value: T) -> Self {
                    Self ( value )
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

            impl<T: NumAssign> core::ops::MulAssign for $unit_name<T> {
                fn mul_assign(&mut self, rhs: Self) {
                    self.0 *= rhs.0;
                }
            }



            impl<T: Num> core::ops::Div for $unit_name<T> {
                type Output = Self;

                fn div(self, rhs: Self) -> Self::Output {
                    Self ( self.0 / rhs.0 )
                }
            }

            impl<T: NumAssign> core::ops::DivAssign for $unit_name<T> {
                fn div_assign(&mut self, rhs: Self) {
                    self.0 /= rhs.0;
                }
            }



            impl<T: Num + core::ops::Rem> core::ops::Rem for $unit_name<T> {
                type Output = Self;

                fn rem(self, rhs: Self) -> Self::Output {
                    Self ( self.0 % rhs.0 )
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
        )+
    };
}



pub mod si;