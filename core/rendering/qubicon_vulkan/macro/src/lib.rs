mod generate_formats;


use quote::quote;
use proc_macro::TokenStream;

use generate_formats::ChannelType;


#[proc_macro_derive(GenerateFormats)]
pub fn generate_formats(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    
    let en = match input.data {
        syn::Data::Enum(en) => en,

        _ => panic!("GenerateFormats can be applied only to enums")
    };

    let enum_ident = input.ident;
    let formats = generate_formats::generate_formats_from_def_list(
        en.variants.into_iter().map(| v | v.ident)
    );

    let structs_iter = formats.iter().filter_map(| f | f.generate_struct_decl(&enum_ident));
    let size_match_iter = formats.iter().map(| f | f.generate_size_match_arm());
    let align_match_iter = formats.iter().map(| f | f.generate_align_match_arm());
    let aspect_flags_match_iter = formats.iter().map(| f | f.generate_aspect_flags_match_arm());

    let alpha_bits_iter = formats.iter().filter_map(| f | f.generate_bits_per_channel_match_arm(ChannelType::Alpha));
    let red_bits_iter = formats.iter().filter_map(| f | f.generate_bits_per_channel_match_arm(ChannelType::Red));
    let green_bits_iter = formats.iter().filter_map(| f | f.generate_bits_per_channel_match_arm(ChannelType::Green));
    let blue_bits_iter = formats.iter().filter_map(| f | f.generate_bits_per_channel_match_arm(ChannelType::Blue));
    let depth_bits_iter = formats.iter().filter_map(| f | f.generate_bits_per_channel_match_arm(ChannelType::Depth));
    let stencil_bits_iter = formats.iter().filter_map(| f | f.generate_bits_per_channel_match_arm(ChannelType::Stencil));
    let exponent_bits_iter = formats.iter().filter_map(| f | f.generate_bits_per_channel_match_arm(ChannelType::Exponent));

    quote! {
        pub mod formats_repr {
            use super::*;

            pub trait FormatRepr: sealed::FormatRepr + BufferType + Sized {}

            mod sealed {
                use super::{ #enum_ident, ImageAspectFlags };

                pub trait FormatRepr {
                    const ASPECT_FLAGS: ImageAspectFlags;
                    const ASSOCIATED_FORMAT: #enum_ident;

                    const ALPHA_BITS: u8;
                    const RED_BITS: u8;
                    const GREEN_BITS: u8;
                    const BLUE_BITS: u8;
                    const DEPTH_BITS: u8;
                    const STENCIL_BITS: u8;
                    const EXPONENT_BITS: u8;

                    fn aspect_flags() -> ImageAspectFlags {
                        Self::ASPECT_FLAGS
                    }
                    fn associated_format() -> #enum_ident {
                        Self::ASSOCIATED_FORMAT
                    }
                }
            }

            #(#structs_iter)*
        }

        // this is a macro, so :]
        #[allow(unreachable_code)]
        impl #enum_ident {
            pub const fn size(&self) -> Option<NonZeroDeviceSize> {
                let result = match self {
                    #(#size_match_iter)*

                    _ => return None
                };

                Some( result )
            }

            pub const fn align(&self) -> Option<NonZeroDeviceSize> {
                let result = match self {
                    #(#align_match_iter)*
                    
                    _ => return None
                };

                Some( result )
            }

            // isn`t const because BitOr isn`t const
            pub fn aspect_flags(&self) -> ImageAspectFlags {
                match self {
                    #(#aspect_flags_match_iter)*

                    // TODO: Rework. It isn`t true for all cases
                    _ => ImageAspectFlags::COLOR
                }
            }



            pub const fn alpha_bits(&self) -> Option<core::num::NonZeroU8> {
                let result = match self {
                    #(#alpha_bits_iter)*

                    _ => return None
                };

                Some( result )
            }

            pub const fn red_bits(&self) -> Option<core::num::NonZeroU8> {
                let result = match self {
                    #(#red_bits_iter)*

                    _ => return None
                };

                Some( result )
            }

            pub const fn green_bits(&self) -> Option<core::num::NonZeroU8> {
                let result = match self {
                    #(#green_bits_iter)*

                    _ => return None
                };

                Some( result )
            }

            pub const fn blue_bits(&self) -> Option<core::num::NonZeroU8> {
                let result = match self {
                    #(#blue_bits_iter)*

                    _ => return None
                };

                Some( result )
            }

            pub const fn depth_bits(&self) -> Option<core::num::NonZeroU8> {
                let result = match self {
                    #(#depth_bits_iter)*

                    _ => return None
                };

                Some( result )
            }

            pub const fn stencil_bits(&self) -> Option<core::num::NonZeroU8> {
                let result = match self {
                    #(#stencil_bits_iter)*

                    _ => return None
                };

                Some( result )
            }

            pub const fn exponent_bits(&self) -> Option<core::num::NonZeroU8> {
                let result = match self {
                    #(#exponent_bits_iter)*

                    _ => return None
                };

                Some( result )
            }
        }
    }.into()
}