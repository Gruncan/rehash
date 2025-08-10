use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::__private::quote::quote;
use syn::token::Pub;
use syn::{parse_macro_input, parse_quote, Abi, Attribute, ItemFn, LitStr, Visibility};

#[proc_macro_attribute]
pub fn rehash_codec_ffi(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input_fn = parse_macro_input!(item as ItemFn);


    let unsafe_attr: Attribute = parse_quote!(#[unsafe(no_mangle)]);
    input_fn.attrs.push(unsafe_attr);

    input_fn.vis = Visibility::Public(Pub {
        span: Span::call_site(),
    });

    input_fn.sig.abi = Some(Abi {
        extern_token: <syn::token::Extern>::default(),
        name: Some(LitStr::new("C", Span::call_site())),
    });

    TokenStream::from(quote!(#input_fn))
}