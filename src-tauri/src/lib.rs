#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[tauri::command]
fn wasm_log(message: String) {
    println!("[WASM] {}", message);
}


pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![wasm_log])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
