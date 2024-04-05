use super::HalfF16;
use core::arch::x86_64::{
    __m128 as m128,
    _mm_cvtph_ps,
    _mm_cvtps_ph,

    _mm_extract_epi64,

    _MM_FROUND_TO_ZERO
};

#[repr(align(16))]
struct F32x4 ( [f32; 4] );

#[repr(align(8))]
pub struct Half16x4 ( [HalfF16; 4] );

impl Half16x4 {
    pub const fn new(nums: [HalfF16; 4]) -> Self {
        Self ( nums )
    }
}

impl From<F32x4> for Half16x4 {
    fn from(value: F32x4) -> Self {
        unsafe {
            let value = _mm_cvtps_ph::<_MM_FROUND_TO_ZERO>(
                core::mem::transmute(value)
            );

            let value = _mm_extract_epi64::<0>(value);

            Self ( core::mem::transmute(value) )
        }
    }
}

#[test]
fn test() {
    let vec = Half16x4::from(F32x4([3.4, core::f32::consts::PI, 89.0, 35.0]));
    
    let n1 = vec.0[0].into_f32();
    let n2 = vec.0[1].into_f32();
    let n3 = vec.0[2].into_f32();
    let n4 = vec.0[3].into_f32();

    println!("{n1} {n2} {n3} {n4}");
}