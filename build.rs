use std::error::Error;
use std::process::Command;


fn main() -> Result<(), Box<dyn Error>> {
    let wasm_installed = Command::new("which").arg("wasm-pack").status()?;
    if !wasm_installed.success() {
        panic!("Cannot not find wasm-pack perhaps it is not installed!");
    }

    // Avoid infinite recursion
    if std::env::var("TARGET")? == "wasm32-unknown-unknown" || std::env::var("CARGO_WASM_BUILD").is_ok() {
        return Ok(());
    }

    let profile = std::env::var("PROFILE")?;
    let mut build = Command::new("wasm-pack");
    build.args(&["build", "--target", "web", "--out-dir", "pkg"]);
    build.env("CARGO_WASM_BUILD", "1");


    // TODO issue with release build getting stuck
    if profile == "release" {
        build.arg("--release");
        // panic!("{:?}", build.get_args())
    }

    let status = build.status()?;


    if !status.success() {
        panic!("wasm-pack build failed");
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/");

    Ok(())
}