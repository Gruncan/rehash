use proc_macro::TokenStream;


#[proc_macro_attribute]
pub fn rehash_codec_ffi(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}