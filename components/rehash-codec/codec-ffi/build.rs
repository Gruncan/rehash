use std::fs::{read_to_string, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use syn::__private::quote::quote;
use syn::__private::ToTokens;
use syn::{parse_file, Attribute, Item};

const FFI_OUT_PATH: &'static str = "src/";
const FFI_FILE_NAME: &'static str = "codec_ffi.rs";

const FFI_IN_PATH: &'static str = "../codec/src/interface.rs";

const FFI_STUB_NAME: &'static str = "rehash_codec_ffi";


fn has_ffi_stub_attribute(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|a| a.path().is_ident(FFI_STUB_NAME))
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed={}", FFI_IN_PATH);
    println!("cargo:rerun-if-changed=.");
    let src_path = Path::new(FFI_IN_PATH);
    let mut fn_to_gen = Vec::new();

    let content = read_to_string(&src_path).expect("Failed to read string from FFI_IN_PATH");
    let syntax = parse_file(&content).expect("Failed to parse interface file");

    for item in syntax.items {
        if let Item::Fn(func) = item {
            if has_ffi_stub_attribute(&func.attrs) {
                fn_to_gen.push(func);
            }
        }
    }

    let generated_code = fn_to_gen.iter().map(|func| {
        let fn_name = &func.sig.ident;
        let inputs = &func.sig.inputs;
        let outputs = &func.sig.output;
        // let generics = &input_fn.sig.generics;
        let block = &func.block;
        let fn_ffi_gen = quote! {
            #[unsafe(no_mangle)]
            pub unsafe extern "C" fn #fn_name(#inputs) #outputs
                    #block
        };
        fn_ffi_gen
    }).collect::<Vec<_>>();


    let out_dir = PathBuf::from(FFI_OUT_PATH);
    let out_path = Path::new(&out_dir).join(FFI_FILE_NAME);

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(out_path).expect("Failed to open output file");

    for func in generated_code {
        writeln!(&mut file, "{}\n", func)?;
    }

    Ok(())
}