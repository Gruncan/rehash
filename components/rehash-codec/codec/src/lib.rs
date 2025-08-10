use codec_proc_macro::rehash_codec_bind;

#[rehash_codec_bind]
fn add(left: u64, right: u64) -> u64 {
    left + right
}
