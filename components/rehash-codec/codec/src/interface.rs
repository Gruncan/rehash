use rehash_codec_proc_macro::rehash_codec_ffi;

#[rehash_codec_ffi]
fn print_codec_version() {
    println!("Codec version: {}", env!("CARGO_PKG_VERSION"));
}