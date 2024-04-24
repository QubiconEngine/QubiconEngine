use syn::Ident;
use quote::quote;
use core::fmt::Display;

use proc_macro2::{ Span, TokenStream };


mod attributes;
mod type_resolver;

use attributes::*;


#[derive(Debug, Clone)]
pub struct Format {
    pub format_def_lit: Ident,
    pub channel_list: ChannelList,
    pub space: Space,
    pub pack: Option<Pack>
}

impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}_{}", self.channel_list, self.space)?;

        if let Some(pack) = self.pack {
            write!(f, "_{}", pack)?;
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
    pub fn generate_struct_decl(&self) -> Option<TokenStream> {
        // temporary
        if self.pack.is_some() {
            return None;
        }

        let struct_name = Ident::new( &self.to_string(), self.format_def_lit.span() );
        let struct_fields = self.channel_list.generate_fields(self.space);

        let result = quote! {
            #[derive(Clone, Copy, PartialEq)]
            pub struct #struct_name {
                #struct_fields
            }
        };

        Some ( result )
    }
}



pub fn generate_formats_from_def_list(format_def_list: impl Iterator<Item = Ident>) -> Vec<Format> {
    format_def_list
        .filter_map(| ident | ident.try_into().ok())
        .collect()
}