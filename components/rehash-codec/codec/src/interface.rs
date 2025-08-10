use codec_proc_macro::rehash_codec_ffi;

#[rehash_codec_ffi]
fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[rehash_codec_ffi]
fn sub(left: u64, right: u64) -> u64 {
    left - right
}

fn test() {}

#[rehash_codec_ffi]
fn sub2(left: u64, right: u64) -> u64 {
    left - right
}