use tauri::menu::{AboutMetadata, Menu, MenuBuilder, MenuItem, MenuItemBuilder, Submenu, SubmenuBuilder};
use tauri::Window;
use tauri::{Emitter, Manager};
use tauri_plugin_dialog::{DialogExt, FileDialogBuilder, FilePath};

#[tauri::command]
fn wasm_log(message: String) {
    println!("[WASM] {}", message);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            let open = MenuItemBuilder::new("Open")
                .id("open")
                .build(app)?;

            let app_submenu = SubmenuBuilder::new(app, "File")
                .item(&open)
                .separator()
                .services()
                .separator()
                .hide()
                .hide_others()
                .quit()
                .build()?;

            let menu = MenuBuilder::new(app)
                .items(&[
                    &app_submenu,
                ])
                .build()?;
            app.set_menu(menu)?;

            app.on_menu_event(move |app, event| {
                if event.id() == open.id() {
                    let cloned = app.clone();
                    app.dialog().file()
                        .add_filter("Video", &["mp4"])
                        .pick_file(move |path_buf| match path_buf {
                            Some(p) => {
                                cloned.emit("select-video-event", p.to_string()).expect("Failed to send file path to front end");
                            }
                            None => {}
                        });
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![wasm_log])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
