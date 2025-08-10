mod video;

use crate::video::{VideoStreamChunk, VideoStreamHandler, VideoStreamMeta};
use rehash_codec_ffi::RehashCodecLibrary;
use std::fs;
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
const CODEC_NAME: &str = "librehashcodec.dll";

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
async fn create_video_stream(stream_handler: State<'_, VideoStreamHandler>, path: String) -> Result<VideoStreamMeta, String> {
    stream_handler.create_stream(path, 200).await
}

#[tauri::command]
async fn get_chunk(stream_handler: State<'_, VideoStreamHandler>) -> Result<Option<VideoStreamChunk>, String> {
    stream_handler.read_chunk().await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(target_os = "linux")]
    {
        std::env::set_var("LIBGL_ALWAYS_SOFTWARE", "1");
    }
    println!("Desktop version: {}", DESKTOP_VERSION);
    let video_stream_handler = VideoStreamHandler::new();

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
        .manage(video_stream_handler)
        .invoke_handler(tauri::generate_handler![wasm_log, get_desktop_build, wasm_error, create_video_stream, get_chunk])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");


    if let Ok(path) = app.path().resolve(format!("codec/{}", CODEC_NAME), BaseDirectory::Resource) {
        let rehash_codec = RehashCodecLibrary::new(&path.to_str().unwrap());
        rehash_codec.print_codec_version();
    }

    app.run(|_app_handle, _event| {});
}
