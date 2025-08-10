// mod codec;
//
// use libloading::{Library, Symbol};
//
// struct RehashCodecLibrary<'a>(&'a str);
//
//
// impl<'a> RehashCodecLibrary<'a> {
//     pub fn new<T: AsRef<str>>(path: &'a T) -> Self {
//         println!("Loaded rehashcodec!");
//         unsafe {
//             let lib = Library::new(path.as_ref()).expect("Failed to loaded library");
//             let add: Symbol<unsafe extern "C" fn(i32, i32) -> i32> = lib.get(b"add").expect("Failed to load symbol");
//             let result = add(10, 20);
//             println!("Result {}", result);
//         }
//         Self {
//             0: path.as_ref()
//         }
//     }
// }

use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::__private::quote::quote;
use syn::token::Pub;
use syn::{parse_macro_input, parse_quote, Abi, Attribute, ItemFn, LitStr, Visibility};

#[proc_macro_attribute]
pub fn rehash_codec_bind(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input_fn = parse_macro_input!(item as ItemFn);


    let unsafe_attr: Attribute = parse_quote!(#[unsafe(no_mangle)]);
    input_fn.attrs.push(unsafe_attr);

    input_fn.vis = Visibility::Public(Pub {
        span: Span::call_site(),
    });

    let fn_name = &input_fn.sig.ident;

    input_fn.sig.abi = Some(Abi {
        extern_token: <syn::token::Extern>::default(),
        name: Some(LitStr::new("C", Span::call_site())),
    });

    TokenStream::from(quote!(#input_fn))
}