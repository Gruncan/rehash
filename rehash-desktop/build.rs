use std::path::Path;
use std::{env, fs};

#[cfg(target_os = "linux")]
const CODEC_NAME: &str = "librehashcodec.so";

#[cfg(target_os = "windows")]
const CODEC_NAME: &str = "rehashcodec.dll";

fn main() {
    let profile = env::var("PROFILE").unwrap();
    let shared_object_target = format!("../target/{}/{}", profile, CODEC_NAME);
    let shared_object_abs_res = fs::canonicalize(Path::new(&shared_object_target));


    if let Ok(shared_object_abs) = shared_object_abs_res {
        println!("cargo:rerun-if-changed={}", shared_object_abs.display());
        let directory = Path::new("codec/").canonicalize().unwrap();
        if !directory.exists() {
            fs::create_dir_all(&directory).expect("Failed to create directory.");
        }
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
    }
    tauri_build::build()
}
