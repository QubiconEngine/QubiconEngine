mod generate_formats;


use quote::quote;
use proc_macro::TokenStream;


#[proc_macro_derive(GenerateFormats)]
pub fn generate_formats(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    
    let en = match input.data {
        syn::Data::Enum(en) => en,

        _ => panic!("GenerateFormats can be applied only to enums")
    };

    let formats = generate_formats::generate_formats_from_def_list(
        en.variants.into_iter().map(| v | v.ident)
    );

    let structs_iter = formats.iter().filter_map(| f | f.generate_struct_decl());

    quote! {
        pub mod formats_repr {
            #(#structs_iter)*
        }
    }.into()
}