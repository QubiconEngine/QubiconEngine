#[cfg(target_feature = "sse2")]
pub use i8x16::I8x16;
#[cfg(target_feature = "sse2")]
pub use i16x8::I16x8;
#[cfg(target_feature = "sse2")]
pub use i32x4::I32x4;
#[cfg(target_feature = "sse2")]
pub use i64x2::I64x2;


#[allow(unused_imports)]
use super::{ Vector, HorizontalAdd, HorizontalSub, Abs, MinMax, Extract };
use core::{
    arch::x86_64::*,
    ops::{ Add, Sub, BitAnd, BitOr, BitXor }
};


pub trait IntegerVector: Vector + BitAnd<Output = Self> + BitOr<Output = Self> + BitXor<Output = Self> {}


#[cfg(target_feature = "sse2")]
mod i8x16 {
    use super::*;

    #[repr(transparent)]
    #[derive(Clone, Copy)]
    pub struct I8x16 ( pub(crate) __m128i );

    impl I8x16 {
        // :(
        pub fn new(n: [i8; 16]) -> Self {
            Self ( unsafe { _mm_set_epi8(
                n[15], n[14], n[13], n[12], n[11], n[10], n[9], n[8],
                n[7], n[6], n[5], n[4], n[3], n[2], n[1], n[0]
            ) } )
        }

        pub fn new_fill(value: i8) -> Self {
            Self ( unsafe { _mm_set1_epi8(value) } )
        }
    }

    impl Add<Self> for I8x16 {
        type Output = Self;
        
        fn add(self, rhs: Self) -> Self::Output {
            unsafe {
                Self ( _mm_add_epi8(self.0, rhs.0) )
            }
        }
    }

    impl Sub<Self> for I8x16 {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            unsafe {
                Self ( _mm_sub_epi8(self.0, rhs.0) )
            }
        }
    }

    impl BitAnd for I8x16 {
        type Output = Self;
        
        fn bitand(self, rhs: Self) -> Self::Output {
            unsafe { Self ( _mm_and_si128(self.0, rhs.0) ) }
        }
    }

    impl BitOr for I8x16 {
        type Output = Self;

        fn bitor(self, rhs: Self) -> Self::Output {
            unsafe { Self ( _mm_or_si128(self.0, rhs.0) ) }
        }
    }

    impl BitXor for I8x16 {
        type Output = Self;

        fn bitxor(self, rhs: Self) -> Self::Output {
            unsafe { Self ( _mm_xor_si128(self.0, rhs.0) ) }
        }
    }

    // ———————————No mul and div?———————————
    // ⠀⣞⢽⢪⢣⢣⢣⢫⡺⡵⣝⡮⣗⢷⢽⢽⢽⣮⡷⡽⣜⣜⢮⢺⣜⢷⢽⢝⡽⣝
    // ⠸⡸⠜⠕⠕⠁⢁⢇⢏⢽⢺⣪⡳⡝⣎⣏⢯⢞⡿⣟⣷⣳⢯⡷⣽⢽⢯⣳⣫⠇
    // ⠀⠀⢀⢀⢄⢬⢪⡪⡎⣆⡈⠚⠜⠕⠇⠗⠝⢕⢯⢫⣞⣯⣿⣻⡽⣏⢗⣗⠏⠀
    // ⠀⠪⡪⡪⣪⢪⢺⢸⢢⢓⢆⢤⢀⠀⠀⠀⠀⠈⢊⢞⡾⣿⡯⣏⢮⠷⠁⠀⠀
    // ⠀⠀⠀⠈⠊⠆⡃⠕⢕⢇⢇⢇⢇⢇⢏⢎⢎⢆⢄⠀⢑⣽⣿⢝⠲⠉⠀⠀⠀⠀
    // ⠀⠀⠀⠀⠀⡿⠂⠠⠀⡇⢇⠕⢈⣀⠀⠁⠡⠣⡣⡫⣂⣿⠯⢪⠰⠂⠀⠀⠀⠀
    // ⠀⠀⠀⠀⡦⡙⡂⢀⢤⢣⠣⡈⣾⡃⠠⠄⠀⡄⢱⣌⣶⢏⢊⠂⠀⠀⠀⠀⠀⠀
    // ⠀⠀⠀⠀⢝⡲⣜⡮⡏⢎⢌⢂⠙⠢⠐⢀⢘⢵⣽⣿⡿⠁⠁⠀⠀⠀⠀⠀⠀⠀
    // ⠀⠀⠀⠀⠨⣺⡺⡕⡕⡱⡑⡆⡕⡅⡕⡜⡼⢽⡻⠏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    // ⠀⠀⠀⠀⣼⣳⣫⣾⣵⣗⡵⡱⡡⢣⢑⢕⢜⢕⡝⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    // ⠀⠀⠀⣴⣿⣾⣿⣿⣿⡿⡽⡑⢌⠪⡢⡣⣣⡟⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    // ⠀⠀⠀⡟⡾⣿⢿⢿⢵⣽⣾⣼⣘⢸⢸⣞⡟⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    // ⠀⠀⠀⠀⠁⠇⠡⠩⡫⢿⣝⡻⡮⣒⢽⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    // —————————————————————————————————————

    impl From<[i8; 16]> for I8x16 {
        fn from(value: [i8; 16]) -> Self {
            Self::new(value)
        }
    }

    impl From<i8> for I8x16 {
        fn from(value: i8) -> Self {
            Self::new_fill(value)
        }
    }

    
    #[cfg(target_feature = "ssse3")]
    impl Abs for I8x16 {
        fn abs(self) -> Self {
            unsafe { Self ( _mm_abs_epi8(self.0) ) }
        }
    }

    #[cfg(target_feature = "sse4.1")]
    impl MinMax for I8x16 {
        fn max(self, rhs: Self) -> Self {
            unsafe { Self ( _mm_min_epi8(self.0, rhs.0) ) }
        }

        fn min(self, rhs: Self) -> Self {
            unsafe { Self ( _mm_max_epi8(self.0, rhs.0) ) }
        }
    }

    #[cfg(target_feature = "sse4.1")]
    impl Extract for I8x16 {
        fn get<const IDX: i32>(&self) -> Self::ElementType {
            unsafe { _mm_extract_epi8::<IDX>(self.0) as i8 }
        }
    }



    impl Vector for I8x16 {
        type ElementType = i8;
        const ELEMENTS_COUNT: usize = 16;
    }

    impl IntegerVector for I8x16 {}
}

#[cfg(target_feature = "sse2")]
mod i16x8 {
    use super::*;

    #[repr(transparent)]
    #[derive(Clone, Copy)]
    pub struct I16x8 ( pub(crate) __m128i );

    impl I16x8 {
        pub fn new(n: [i16; 8]) -> Self {
            Self ( unsafe { _mm_set_epi16(n[7], n[6], n[5], n[4], n[3], n[2], n[1], n[0]) } )
        }

        pub fn new_fill(value: i16) -> Self {
            Self ( unsafe { _mm_set1_epi16(value) } )
        }
    }

    impl Add<Self> for I16x8 {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            unsafe {
                Self ( _mm_add_epi16(self.0, rhs.0) )
            }
        }
    }

    impl Sub<Self> for I16x8 {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            unsafe {
                Self ( _mm_sub_epi16(self.0, rhs.0) )
            }
        }
    }

    impl BitAnd for I16x8 {
        type Output = Self;

        fn bitand(self, rhs: Self) -> Self::Output {
            unsafe { Self ( _mm_and_si128(self.0, rhs.0) ) }
        }
    }

    impl BitOr for I16x8 {
        type Output = Self;

        fn bitor(self, rhs: Self) -> Self::Output {
            unsafe { Self ( _mm_or_si128(self.0, rhs.0) ) }
        }
    }

    impl BitXor for I16x8 {
        type Output = Self;

        fn bitxor(self, rhs: Self) -> Self::Output {
            unsafe { Self ( _mm_xor_si128(self.0, rhs.0) ) }
        }
    }

    // Multiple instructions for this. What to use ?
    // impl Mul<Self> for I16x8 {
    //     type Output = Self;
    //     fn mul(self, rhs: Self) -> Self::Output {
    //         unsafe {
    //             Self (  )
    //         }
    //     }
    // }

    impl From<[i16; 8]> for I16x8 {
        fn from(value: [i16; 8]) -> Self {
            Self::new(value)
        }
    }

    impl From<i16> for I16x8 {
        fn from(value: i16) -> Self {
            Self::new_fill(value)
        }
    }


    #[cfg(target_feature = "ssse3")]
    impl HorizontalAdd for I16x8 {
        fn hadd(self, rhs: Self) -> Self {
            unsafe { Self ( _mm_hadd_epi16(self.0, rhs.0) ) }
        }
    }

    #[cfg(target_feature = "ssse3")]
    impl HorizontalSub for I16x8 {
        fn hsub(self, rhs: Self) -> Self {
            unsafe { Self ( _mm_hsub_epi16(self.0, rhs.0) ) }
        }
    }
    
    #[cfg(target_feature = "ssse3")]
    impl Abs for I16x8 {
        fn abs(self) -> Self {
            unsafe { Self ( _mm_abs_epi16(self.0) ) }
        }
    }

    #[cfg(target_feature = "sse4.1")]
    impl MinMax for I16x8 {
        fn max(self, rhs: Self) -> Self {
            unsafe { Self ( _mm_max_epi16(self.0, rhs.0) ) }
        }

        fn min(self, rhs: Self) -> Self {
            unsafe { Self ( _mm_min_epi16(self.0, rhs.0) ) }
        }
    }

    #[cfg(target_feature = "sse4.1")]
    impl Extract for I16x8 {
        fn get<const IDX: i32>(&self) -> Self::ElementType {
            unsafe { _mm_extract_epi16::<IDX>(self.0) as i16 }
        }
    }



    impl Vector for I16x8 {
        type ElementType = i16;
        const ELEMENTS_COUNT: usize = 8;
    }

    impl IntegerVector for I16x8 {}
}

#[cfg(target_feature = "sse2")]
mod i32x4 {
    use super::*;

    #[repr(transparent)]
    #[derive(Clone, Copy)]
    pub struct I32x4 ( pub(crate) __m128i );

    impl I32x4 {
        pub fn new(n1: i32, n2: i32, n3: i32, n4: i32) -> Self {
            Self ( unsafe { _mm_set_epi32(n4, n3, n2, n1) } )
        }

        pub fn new_fill(value: i32) -> Self {
            Self ( unsafe { _mm_set1_epi32(value) } )
        }
    }

    impl Add<Self> for I32x4 {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            unsafe {
                Self ( _mm_add_epi32(self.0, rhs.0) )
            }
        }
    }

    impl Sub<Self> for I32x4 {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            unsafe {
                Self ( _mm_sub_epi32(self.0, rhs.0) )
            }
        }
    }

    impl BitAnd for I32x4 {
        type Output = Self;

        fn bitand(self, rhs: Self) -> Self::Output {
            unsafe { Self ( _mm_and_si128(self.0, rhs.0) ) }
        }
    }

    impl BitOr for I32x4 {
        type Output = Self;
        
        fn bitor(self, rhs: Self) -> Self::Output {
            unsafe { Self ( _mm_or_si128(self.0, rhs.0) ) }
        }
    }

    impl BitXor for I32x4 {
        type Output = Self;

        fn bitxor(self, rhs: Self) -> Self::Output {
            unsafe { Self ( _mm_xor_si128(self.0, rhs.0) ) }
        }
    }

    // Its actualy for SSE4.1
    // impl Mul<Self> for I32x4 {
    //     type Output = Self;
    //     fn mul(self, rhs: Self) -> Self::Output {
    //         unsafe {
    //             Self ( _mm_mul_epi32(self.0, rhs.0) )
    //         }
    //     }
    // }

    impl From<[i32; 4]> for I32x4 {
        fn from(value: [i32; 4]) -> Self {
            Self::new(value[0], value[1], value[2], value[3])
        }
    }

    impl From<(i32, i32, i32, i32)> for I32x4 {
        fn from((n1, n2, n3, n4): (i32, i32, i32, i32)) -> Self {
            Self::new(n1, n2, n3, n4)
        }
    }

    impl From<i32> for I32x4 {
        fn from(value: i32) -> Self {
            Self::new_fill(value)
        }
    }

    impl From<super::super::F32x4> for I32x4 {
        fn from(value: super::super::F32x4) -> Self {
            unsafe { Self ( _mm_cvtps_epi32(value.0) ) }
        }
    }


    #[cfg(target_feature = "ssse3")]
    impl HorizontalAdd for I32x4 {
        fn hadd(self, rhs: Self) -> Self {
            unsafe { Self ( _mm_hadd_epi32(self.0, rhs.0) ) }
        }
    }

    #[cfg(target_feature = "ssse3")]
    impl HorizontalSub for I32x4 {
        fn hsub(self, rhs: Self) -> Self {
            unsafe { Self ( _mm_hsub_epi32(self.0, rhs.0) ) }
        }
    }

    #[cfg(target_feature = "ssse3")]
    impl Abs for I32x4 {
        fn abs(self) -> Self {
            unsafe { Self ( _mm_abs_epi32(self.0) ) }
        }
    }

    #[cfg(target_feature = "sse4.1")]
    impl MinMax for I32x4 {
        fn max(self, rhs: Self) -> Self {
            unsafe { Self ( _mm_max_epi32(self.0, rhs.0) ) }
        }

        fn min(self, rhs: Self) -> Self {
            unsafe { Self ( _mm_min_epi32(self.0, rhs.0) ) }
        }
    }

    #[cfg(target_feature = "sse4.1")]
    impl Extract for I32x4 {
        fn get<const IDX: i32>(&self) -> Self::ElementType {
            unsafe { _mm_extract_epi32::<IDX>(self.0) }
        }
    }



    impl Vector for I32x4 {
        type ElementType = i32;
        const ELEMENTS_COUNT: usize = 4;
    }

    impl IntegerVector for I32x4 {}
}

#[cfg(target_feature = "sse2")]
mod i64x2 {
    use super::*;

    #[repr(transparent)]
    #[derive(Debug, Clone, Copy)]
    pub struct I64x2 ( pub(crate) __m128i );

    impl I64x2 {
        pub fn new(n1: i64, n2: i64) -> Self {
            // Why epi64x ? Why there are 'x' ?
            Self ( unsafe { _mm_set_epi64x(n2, n1) } )
        }

        pub fn new_fill(value: i64) -> Self {
            Self ( unsafe { _mm_set1_epi64x(value) } )
        }
    }

    impl Add<Self> for I64x2 {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            unsafe {
                Self ( _mm_add_epi64(self.0, rhs.0) )
            }
        }
    }

    impl Sub<Self> for I64x2 {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            unsafe {
                Self ( _mm_sub_epi64(self.0, rhs.0) )
            }
        }
    }

    impl BitAnd for I64x2 {
        type Output = Self;

        fn bitand(self, rhs: Self) -> Self::Output {
            unsafe { Self ( _mm_and_si128(self.0, rhs.0) ) }
        }
    }

    impl BitOr for I64x2 {
        type Output = Self;
        
        fn bitor(self, rhs: Self) -> Self::Output {
            unsafe { Self ( _mm_or_si128(self.0, rhs.0) ) }
        }
    }

    impl BitXor for I64x2 {
        type Output = Self;

        fn bitxor(self, rhs: Self) -> Self::Output {
            unsafe { Self ( _mm_xor_si128(self.0, rhs.0) ) }
        }
    }

    // fuck, no Mul and Div again ?

    impl From<[i64; 2]> for I64x2 {
        fn from(value: [i64; 2]) -> Self {
            Self::new(value[0], value[1])
        }
    }

    impl From<(i64, i64)> for I64x2 {
        fn from((n1, n2): (i64, i64)) -> Self {
            Self::new(n1, n2)
        }
    }

    impl From<i64> for I64x2 {
        fn from(value: i64) -> Self {
            Self::new_fill(value)
        }
    }

    #[cfg(target_feature = "sse4.1")]
    impl Extract for I64x2 {
        fn get<const IDX: i32>(&self) -> Self::ElementType {
            unsafe { _mm_extract_epi64::<IDX>(self.0) }
        }
    }



    impl Vector for I64x2 {
        type ElementType = i64;
        const ELEMENTS_COUNT: usize = 4;
    }

    impl IntegerVector for I64x2 {}
}