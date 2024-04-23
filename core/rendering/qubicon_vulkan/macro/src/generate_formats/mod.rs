use std::{str::FromStr, fmt::Display};
use arrayvec::ArrayVec;

use quote::quote;
use syn::{ItemEnum, Expr, ExprLit, Ident};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pack {
    P8,
    P16,
    P32,
    Block
}

impl FromStr for Pack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PACK8" => Ok(Self::P8),
            "PACK16" => Ok(Self::P16),
            "PACK32" => Ok(Self::P32),
            "BLOCK" => Ok(Self::Block),

            _ => Err(())
        }
    }
}

impl Display for Pack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::P8 => "Pack8",
                Self::P16 => "Pack16",
                Self::P32 => "Pack32",
                Self::Block => "Block"
            }
        )
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Space {
    Unorm,
    Snorm,
    UScaled,
    SScaled,
    UInt,
    SInt,
    SRGB
}

impl FromStr for Space {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "UNORM" => Ok(Self::Unorm),
            "SNORM" => Ok(Self::Snorm),
            "USCALED" => Ok(Self::UScaled),
            "SSCALED" => Ok(Self::SScaled),
            "UINT" => Ok(Self::UInt),
            "SINT" => Ok(Self::SInt),
            "SRGB" => Ok(Self::SRGB),

            _ => Err(())
        }
    }
}

impl Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Unorm => "Unorm",
                Self::Snorm => "Snorm",
                Self::UScaled => "UScaled",
                Self::SScaled => "SScaled",
                Self::UInt => "UInt",
                Self::SInt => "SInt",
                Self::SRGB => "SRGB"
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Channel {
    Alpha,
    Red,
    Green,
    Blue,
    Depth,
    Stencil,
    Exponent
}

impl TryFrom<char> for Channel {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Self::Alpha),
            'R' => Ok(Self::Red),
            'G' => Ok(Self::Green),
            'B' => Ok(Self::Blue),
            'D' => Ok(Self::Depth),
            'S' => Ok(Self::Stencil),
            'E' => Ok(Self::Exponent),

            _ => Err(())
        }
    }
}

impl FromStr for Channel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s.chars().next().unwrap_or('\n'))
    }
}

impl Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Alpha => "A",
                Self::Red => "R",
                Self::Green => "G",
                Self::Blue => "B",
                Self::Depth => "D",
                Self::Stencil => "S",
                Self::Exponent => "E"
            }
        )
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct Channels {
    channels: ArrayVec<(Channel, u8), 5>
}

impl FromStr for Channels {
    type Err = ();

    // What the fuck is this ?
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut channels = ArrayVec::new();
        let mut channel = Channel::Alpha;
        let mut bits_count: u8 = 0;

        let mut first = true;
        let iter = s.chars().peekable();

        for c in iter {
            if c.is_ascii_alphabetic() {
                if !first {
                    channels.push((channel, bits_count))
                }

                channel = Channel::try_from(c)?;
                bits_count = 0;
                first = false;
                continue;
            } else if c.is_ascii_digit() {
                bits_count = bits_count * 10 + c.to_digit(10).unwrap() as u8
            } else {
                return Err(())
            }
        }

        channels.push((channel, bits_count));

        Ok(Self { channels })
    }
}

impl Display for Channels {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (channel, bits) in self.channels.iter() {
            write!(f, "{}{}", channel, bits)?
        }

        Ok(())
    }
}


// TODO: Add support for more formats(formats with compression for example)
#[derive(Clone)]
struct Format {
    format_id: ExprLit,
    channels: Channels,
    space: Space,
    pack: Option<Pack>
}

impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}_{}", self.channels, self.space)?;
        if let Some(pack) = self.pack {
            write!(f, "_{pack}")?;
        }

        Ok(())
    }
}

impl Format {
    fn new(s: &str, id: ExprLit) -> Result<Self, ()> {
        let mut iter = s.split('_');
        
        let channels = iter.next().ok_or(()).and_then(Channels::from_str)?;
        let space = iter.next().ok_or(()).and_then(Space::from_str)?;
        let pack = if let Some(pack) = iter.next() {
            Some(Pack::from_str(pack)?)
        } else {
            None
        };

        Ok(
            Self {
                format_id: id,
                channels,
                space,
                pack
            }
        )
    }

    fn generate_representation_structure_code(&self) -> TokenStream2 {
        let structure_name = TokenStream2::from_str(&self.to_string())
            .unwrap();

        let field_type = if let Some(pack) = self.pack {
            let ty = match pack {
                Pack::P8 => "u8",
                Pack::P16 => "u16",
                Pack::P32 => "u32",
                Pack::Block => ""
            };

            TokenStream2::from_str(ty)
                .unwrap()
        } else {
            let total_bits = self.channels.channels.iter()
                .fold(0u16, | a, &(_, c) | a + c as u16);

            let total_bytes = (total_bits / 8) as usize;
            
            quote! { [u8; #total_bytes] }
        };
        

        quote! {
            #[repr(C)]
            #[derive(Clone, Copy, PartialEq, Eq, Hash)]
            pub struct #structure_name(pub #field_type);
        }
    }

    fn generate_format_trait_implementation_code(&self, format_enum_name: &Ident) -> TokenStream2 {
        let structure_name = TokenStream2::from_str(&self.to_string())
            .unwrap();
        let id = &self.format_id;

        quote! {
            impl FormatSealed for #structure_name {
                const FORMAT_ENUM: super::super::#format_enum_name = unsafe { super::super::#format_enum_name::from_raw(#id) };
            }
        }
    }
}

pub fn generate_formats(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let format_enum: ItemEnum = syn::parse(input.clone()).unwrap();
    let formats: Vec<_> = format_enum.variants.iter()
        .filter_map(| v | {
            let id = if let Expr::Lit(l) = &v.discriminant.as_ref().unwrap().1 {
                l.clone()
            } else {
                panic!("Invalid format id");
            };

            Format::new(&v.ident.to_string(), id).ok()
        })
        .collect();

    let format_enum_name = format_enum.ident;
    let structure_defenitions = formats.iter()
        .map(| f | f.generate_representation_structure_code());
    let sealed_format_trait_implementations = formats.iter()
        .map(| f | f.generate_format_trait_implementation_code(&format_enum_name));

    let tokens = quote! {
        pub mod formats_representation {
            pub trait Format: sealed::FormatSealed {}

            impl<T: sealed::FormatSealed> Format for T {}

            mod sealed {
                use super::*;

                pub trait FormatSealed {
                    const FORMAT_ENUM: super::super::#format_enum_name;
                }

                #(#sealed_format_trait_implementations)*
            }

            #(#structure_defenitions)*
        }
    };

    TokenStream::from_iter(core::iter::once(input).chain(core::iter::once(TokenStream::from(tokens))))
}