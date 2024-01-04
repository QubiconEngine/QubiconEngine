use proc_macro::TokenStream;


#[proc_macro]
pub fn vk_format(token_stream: TokenStream) -> TokenStream {
    // let mut token_iter = token_stream.into_iter();

    println!("{}", token_stream);
    //let vk_format_name = token_iter.next()
    //    .expect("No vulkan format provided");
    
    TokenStream::new()
}