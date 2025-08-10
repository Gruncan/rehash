const FFI_OUT_ENV: &'static str = "FFI_OUT_DIR";
const FFI_OUT_DIR: &'static str = "../codec-ffi/src/";

fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=force_build");
    println!("cargo:rustc-env={}={}", FFI_OUT_ENV, FFI_OUT_DIR);
}