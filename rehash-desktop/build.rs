use std::path::Path;
use std::{env, fs};

#[cfg(target_os = "linux")]
const CODEC_NAME: &str = "librehashcodec.so";

#[cfg(target_os = "windows")]
const CODEC_NAME: &str = "librehashcodec.dll";

fn main() {
    let profile = env::var("PROFILE").unwrap();

    let s = format!("../target/{}/{}", profile, CODEC_NAME);
    let d = format!("codec/{}", CODEC_NAME);
    let src = Path::new(s.as_str());
    if !src.exists() {
        let dest = Path::new(d.as_str());
        fs::copy(src, dest).unwrap();
    }

    tauri_build::build()
}
