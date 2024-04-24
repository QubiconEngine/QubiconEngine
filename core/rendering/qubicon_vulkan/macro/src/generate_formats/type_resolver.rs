use super::*;

use quote::quote;
use proc_macro2::TokenStream;

pub fn resolve(space: Space, bits: u8) -> Option<TokenStream> {
    let result = match (space, bits) {
        (Space::Unorm, 32) => quote! { u32 },
        (Space::Unorm, 16) => quote! { u16 },
        (Space::Unorm, 8) => quote! { u8 },

        (Space::Snorm, 32) => quote! { i32 },
        (Space::Snorm, 16) => quote! { i16 },
        (Space::Snorm, 8) => quote! { i8 },

        (Space::UScaled, 32) => quote! { u32 },
        (Space::UScaled, 16) => quote! { u16 },
        (Space::UScaled, 8) => quote! { u8 },

        (Space::SScaled, 32) => quote! { i32 },
        (Space::SScaled, 16) => quote! { i16 },
        (Space::SScaled, 8) => quote! { i8 },

        (Space::UInt, 32) => quote! { u32 },
        (Space::UInt, 16) => quote! { u16 },
        (Space::UInt, 8) => quote! { u8 },

        (Space::SInt, 32) => quote! { i32 },
        (Space::SInt, 16) => quote! { i16 },
        (Space::SInt, 8) => quote! { i8 },


        (Space::SFloat, 32) => quote! { f32 },
        (Space::SFloat, 16) => quote! { qubicon_short_floats::half16::Half16 },

        (Space::SRGB, 32) => quote! { f32 },
        (Space::SRGB, 16) => quote! { qubicon_short_floats::half16::Half16 },


        _ => return None
    };

    Some( result )
}