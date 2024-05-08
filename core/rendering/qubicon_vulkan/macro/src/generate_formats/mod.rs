use syn::Ident;
use quote::quote;
use core::fmt::Display;

use proc_macro2::{ TokenStream, Literal };


mod attributes;
mod type_resolver;

pub use attributes::*;


#[derive(Debug, Clone)]
pub struct Format {
    pub format_def_lit: Ident,
    pub channel_list: ChannelList,
    pub space: Space,
    pub pack: Option<Pack>
}

impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.channel_list, self.space)?;

        if let Some(pack) = self.pack {
            write!(f, "{}", pack)?;
        }

        Ok(())
    }
}

impl TryFrom<Ident> for Format {
    type Error = ();

    fn try_from(value: Ident) -> Result<Self, Self::Error> {
        let format_str = value.to_string();

        let mut format_str_split = format_str.split('_');

        let channel_list = format_str_split.next().ok_or( () )?.parse()?;
        let space = format_str_split.next().ok_or( () )?.parse()?;
        let pack = format_str_split.next()
            .and_then(| s | s.parse().ok());

        let result = Self {
            format_def_lit: value,
            channel_list,
            space,
            pack
        };

        Ok ( result )
    }
}

impl Format {
    pub fn generate_struct_decl(&self, enum_ident: &Ident) -> Option<TokenStream> {
        let align = match self.pack {
            // Return if we cant generate align attr
            Some( pack ) => Some( pack.generate_align_attr()? ),
            None => None
        };

        let enum_variant = &self.format_def_lit;

        let struct_name = Ident::new( &self.to_string(), self.format_def_lit.span() );
        let struct_fields = self.channel_list.generate_fields(self.space)?;
        let aspect_flags = self.aspect_flags();



        let bits_per_channel = | ty: ChannelType | self.bits_per_channel(ty).map(| b | b.get()).unwrap_or(0);

        let alpha_bits = bits_per_channel(ChannelType::Alpha);
        let red_bits = bits_per_channel(ChannelType::Red);
        let green_bits = bits_per_channel(ChannelType::Green);
        let blue_bits = bits_per_channel(ChannelType::Blue);
        let depth_bits = bits_per_channel(ChannelType::Depth);
        let stencil_bits = bits_per_channel(ChannelType::Stencil);
        let exponent_bits = bits_per_channel(ChannelType::Exponent);


        let result = quote! {
            #align
            #[derive(Clone, Copy, PartialEq)]
            pub struct #struct_name {
                #struct_fields
            }

            impl FormatRepr for #struct_name {}

            impl sealed::FormatRepr for #struct_name {
                const ASPECT_FLAGS: ImageAspectFlags = #aspect_flags;
                const ASSOCIATED_FORMAT: #enum_ident = #enum_ident::#enum_variant;

                const ALPHA_BITS: u8 = #alpha_bits;
                const RED_BITS: u8 = #red_bits;
                const GREEN_BITS: u8 = #green_bits;
                const BLUE_BITS: u8 = #blue_bits;
                const DEPTH_BITS: u8 = #depth_bits;
                const STENCIL_BITS: u8 = #stencil_bits;
                const EXPONENT_BITS: u8 = #exponent_bits;
            }

            unsafe impl BufferType for #struct_name {
                fn size() -> usize {
                    core::mem::size_of::<Self>()
                }
            }
        };

        Some ( result )
    }

    // calculates size for formats without representation too
    pub fn size(&self) -> usize {
        self.channel_list.iter().map(| c | c.bits as usize).sum::<usize>() / 8
    }

    pub fn aspect_flags(&self) -> TokenStream {
        enum AspectType {
            Color,
            Depth,
            Stencil
        }

        let mut aspect = AspectType::Color;

        for channel in self.channel_list.iter() {
            match channel.ty {
                ChannelType::Red | ChannelType::Green | ChannelType::Blue | ChannelType::Alpha => {
                    aspect = AspectType::Color
                },

                ChannelType::Depth => aspect = AspectType::Depth,
                ChannelType::Stencil => aspect = AspectType::Stencil,

                _ => {}
            }
        }

        match aspect {
            AspectType::Color => quote! { ImageAspectFlags::COLOR },
            AspectType::Depth => quote! { ImageAspectFlags::DEPTH },
            AspectType::Stencil => quote! { ImageAspectFlags::STENCIL }
        }
    }

    pub fn bits_per_channel(&self, ty: ChannelType) -> Option<core::num::NonZeroU8> {
        self.channel_list.iter()
            .find(| ch | ch.ty == ty)
            // there shouldn`t be any zeroes. unless some idiot didn`t 
            // write or edit any Format variants
            .and_then(| ch | core::num::NonZeroU8::new(ch.bits))
    }

    pub fn generate_size_match_arm(&self) -> Option<TokenStream> {
        let format_def_lit = &self.format_def_lit;
        let size = self.size();

        // now code in quote will be totaly safe
        if size == 0 {
            return None;
        }

        let size = Literal::usize_unsuffixed(size);
        
        let result = quote! {
            Self::#format_def_lit => unsafe { NonZeroDeviceSize::new_unchecked(#size) },
        };

        Some( result )
    }

    pub fn generate_align_match_arm(&self) -> Option<TokenStream> {
        let format_def_lit = &self.format_def_lit;
        let align = Literal::u8_unsuffixed(self.pack?.align()?.get());

        let result = quote! {
            Self::#format_def_lit => unsafe { NonZeroDeviceSize::new_unchecked(#align) },
        };

        Some( result )
    }

    pub fn generate_aspect_flags_match_arm(&self) -> TokenStream {
        let format_def_lit = &self.format_def_lit;
        let flags = self.aspect_flags();

        quote! {
            Self::#format_def_lit => #flags,
        }
    }

    pub fn generate_bits_per_channel_match_arm(&self, ty: ChannelType) -> Option<TokenStream> {
        let bits_per_channel = Literal::u8_unsuffixed(self.bits_per_channel(ty)?.get());
        let format_def_lit = &self.format_def_lit;

        let result = quote! {
            Self::#format_def_lit => unsafe { core::num::NonZeroU8::new_unchecked(#bits_per_channel) },
        };

        Some( result )
    }
}



pub fn generate_formats_from_def_list(format_def_list: impl Iterator<Item = Ident>) -> Vec<Format> {
    format_def_list
        .filter_map(| ident | ident.try_into().ok())
        .collect()
}