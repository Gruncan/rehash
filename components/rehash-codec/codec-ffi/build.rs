use proc_macro2::Literal;
use std::fs::{read_to_string, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use syn::__private::quote::quote;
use syn::__private::ToTokens;
use syn::{parse_file, Attribute, FnArg, Item, Pat, PatIdent};

const FFI_OUT_PATH: &'static str = "src/";
const FFI_FILE_NAME: &'static str = "codec_ffi.rs";

const FFI_IN_PATH: &'static str = "../codec/src/interface.rs";

const FFI_STUB_NAME: &'static str = "rehash_codec_ffi";

fn has_ffi_stub_attribute(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|a| a.path().is_ident(FFI_STUB_NAME))
}

fn run_rust_fmt(path: &Path) -> std::io::Result<()> {
    eprintln!("Running rustfmt at {}", path.display());
    let status = Command::new("rustfmt").arg(path).status()?;
    if !status.success() {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to run rustfmt, status: {}", status),
        ))
    } else {
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed={}", FFI_IN_PATH);
    // println!("cargo:rerun-if-changed=.");
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
        // let generics = &func.sig.generics;

        let func_input_types = inputs.iter().filter_map(|fn_arg| {
            if let FnArg::Typed(pat_type) = fn_arg {
                Some(pat_type.ty.clone())
            } else {
                None
            }
        });

        let func_input_parameters = inputs.iter().filter_map(|fn_arg| {
            if let FnArg::Typed(pat_type) = fn_arg {
                match *pat_type.pat {
                    Pat::Ident(PatIdent { ref ident, .. }) => Some(ident.clone()),
                    _ => None,
                }
            } else {
                None
            }
        });

        let fn_name_str = Literal::byte_string(fn_name.to_string().as_bytes());

        let fn_ffi_gen = quote! {
            pub fn #fn_name(&self, #inputs) #outputs {
                unsafe {
                    let func: Symbol<unsafe extern "C" fn(#(#func_input_types),*) #outputs> = self.lib.get(#fn_name_str).expect("Failed to load symbol");
                    func(#(#func_input_parameters),*)
                }
            }
        };
        fn_ffi_gen
    }).collect::<Vec<_>>();

    let out_dir = PathBuf::from(FFI_OUT_PATH);
    let out_path = Path::new(&out_dir).join(FFI_FILE_NAME);

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(&out_path)
        .expect("Failed to open output file");

    writeln!(&mut file, "{}\n", quote! {
        // GENERATED
        use libloading::Symbol;
        use crate::RehashCodecLibrary;
        use std::ffi::{c_char};
    })?;

    let generated_code = quote! {
        impl RehashCodecLibrary {
            #(#generated_code)*
        }
    };

    writeln!(&mut file, "{}", generated_code)?;
    file.flush()?;
    file.sync_all()?;
    drop(file);

    if let Some(dir) = out_path.parent() {
        let dir = OpenOptions::new().read(true).open(dir)?;
        dir.sync_all()?;
    }

    // run_rust_fmt(&out_path.to_path_buf())?;

    Ok(())
}
