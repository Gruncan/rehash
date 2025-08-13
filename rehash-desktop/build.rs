use std::path::Path;
use std::{env, fs};

#[cfg(target_os = "linux")]
const CODEC_NAME: &str = "librehashcodec.so";

#[cfg(target_os = "windows")]
const CODEC_NAME: &str = "rehashcodec.dll";

fn main() {
    let profile = env::var("PROFILE").unwrap();
    let shared_object_target = format!("../target/{}/{}", profile, CODEC_NAME);
    let shared_object_abs = fs::canonicalize(Path::new(&shared_object_target)).expect("Failed to canonicalize shared object target");

    println!("cargo:rerun-if-changed={}", shared_object_abs.display());

    let d = format!("codec/{}", CODEC_NAME);
    let dest = Path::new(d.as_str());
    if !shared_object_abs.exists() {
        panic!("{} does not exist, ensure codec ({}) library is build", shared_object_abs.display(), CODEC_NAME);
    }

    eprintln!("{}", shared_object_abs.display());

    fs::copy(&shared_object_abs, dest).or_else(|e| {
        eprintln!("Failed to copy {} to {}: {}", shared_object_abs.display(), dest.display(), e);
        Err(e)
    }).unwrap();


    tauri_build::build()
}
