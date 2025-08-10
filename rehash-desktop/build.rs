use std::path::Path;
use std::{env, fs};

#[cfg(target_os = "linux")]
const CODEC_NAME: &str = "librehashcodec.so";

#[cfg(target_os = "windows")]
const CODEC_NAME: &str = "librehashcodec.dll";

fn main() {
    // println!("cargo:rerun-if-changed=.");
    let profile = env::var("PROFILE").unwrap();

    let s = format!("../target/{}/{}", profile, CODEC_NAME);
    let d = format!("codec/{}", CODEC_NAME);
    let dest = Path::new(d.as_str());
    let src = Path::new(s.as_str());
    if !dest.exists() {
        fs::copy(src, dest).or_else(|e| {
            eprintln!("Failed to copy {} to {}: {}", src.display(), dest.display(), e);
            Err(e)
        }).unwrap();
    }

    tauri_build::build()
}
