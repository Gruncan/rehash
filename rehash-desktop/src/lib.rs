mod video;

use crate::video::VideoState;
use rehash_codec_ffi::RehashCodecLibrary;
use std::ffi::CString;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tauri::menu::{
    AboutMetadata, Menu, MenuBuilder, MenuItem, MenuItemBuilder, Submenu, SubmenuBuilder,
};
use tauri::path::BaseDirectory;
use tauri::{Emitter, Manager};
use tauri::{State, Window};
use tauri_plugin_dialog::{DialogExt, FileDialogBuilder, FilePath};

pub const DESKTOP_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(target_os = "linux")]
const CODEC_NAME: &str = "librehashcodec.so";

#[cfg(target_os = "windows")]
const CODEC_NAME: &str = "rehashcodec.dll";

#[tauri::command]
fn get_desktop_build() -> &'static str {
    DESKTOP_VERSION
}

#[tauri::command]
fn wasm_log(message: String) {
    println!("[WASM] {}", message);
}

#[tauri::command]
fn wasm_error(message: String) {
    eprintln!("[WASM] {}", message);
}

#[tauri::command]
fn get_video(state: State<VideoState>, path: String) -> Result<usize, String> {
    let codec = state.codec.lock().unwrap();
    let c_path = CString::new(path).expect("CString::new failed");
    let mut len: usize = 0;
    let data_ptr = codec.get_bytes_from_video(c_path.as_ptr(), &mut len as *mut usize);

    if data_ptr.is_null() {
        return Err(String::from("Failed to get video data"));
    }

    let bytes: Vec<u8> = unsafe {
        Vec::from_raw_parts(data_ptr, len, len)
    };
    state.set_bytes(bytes);

    Ok(len)
}

#[tauri::command]
fn get_video_chunk(state: State<VideoState>) -> Result<Vec<u8>, String> {
    let bytes = state.get_bytes().ok_or("Failed to get video data".into());
    bytes
}



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(target_os = "linux")]
    {
        std::env::set_var("LIBGL_ALWAYS_SOFTWARE", "1");
    }
    println!("Desktop version: {}", DESKTOP_VERSION);

    let app = tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            let open = MenuItemBuilder::new("Open").id("open").build(app)?;

            let app_submenu = SubmenuBuilder::new(app, "File").item(&open).build()?;

            let menu = MenuBuilder::new(app).items(&[&app_submenu]).build()?;
            app.set_menu(menu)?;

            app.on_menu_event(move |app, event| {
                if event.id() == open.id() {
                    let cloned = app.clone();
                    app.dialog()
                        .file()
                        .add_filter("Video", &["mp4"])
                        .pick_file(move |path_buf| match path_buf {
                            Some(p) => {
                                cloned
                                    .emit("select-video-event", p.to_string())
                                    .expect("Failed to send file path to front end");
                            }
                            None => {}
                        });
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![wasm_log, get_desktop_build, wasm_error, get_video, get_video_chunk])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");


    if let Ok(path) = app.path().resolve(format!("codec/{}", CODEC_NAME), BaseDirectory::Resource) {
        let rehash_codec = RehashCodecLibrary::new(&path.to_str().unwrap());
        rehash_codec.print_codec_version();
        app.manage(VideoState::new(rehash_codec));
    }

    app.run(|_app_handle, _event| {});
}
