use crate::utils::tauri_invoke;
use crate::into_object;
use wasm_bindgen::JsValue;

#[cfg(feature = "tauri")]
pub fn log_to_tauri(msg: &str) {
    let args = into_object!("message" => msg)
        .expect("Failed to create message object");

    // TODO use the return value
    let _ = tauri_invoke("wasm_log", JsValue::from(args));
}

#[cfg(feature = "tauri")]
pub fn error_to_tauri(msg: &str) {
    let args = into_object!("message" => msg)
        .expect("Failed to create message object");
    // TODO use the return value
    let _ = tauri_invoke("wasm_error", JsValue::from(args));
}


#[cfg(feature = "tauri")]
#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => {
        log_to_tauri(&format_args!($($t)*).to_string())
    }
}

#[cfg(not(feature = "tauri"))]
#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => {
        log(&format_args!($($t)*).to_string())
    }
}

#[cfg(feature = "tauri")]
#[macro_export]
macro_rules! error_log {

    ($($t:tt)*) => {
        error_to_tauri(&format_args!($($t)*).to_string())
    }
}

#[cfg(not(feature = "tauri"))]
#[macro_export]
macro_rules! error_log {

    ($($t:tt)*) => {
        error(&format_args!($($t)*).to_string())
    }
}

#[cfg(feature = "tauri")]
#[macro_export]
macro_rules! debug_console_log {
    ($($t:tt)*) => {
        #[cfg(debug_assertions)]
        {
            log_to_tauri(&format_args!("[DEBUG] {}", format_args!($($t)*)).to_string())
        }
    };
}

#[cfg(not(feature = "tauri"))]
#[macro_export]
macro_rules! debug_console_log {
    ($($t:tt)*) => {
        #[cfg(debug_assertions)]
        {
            log(&format_args!("[DEBUG] {}", format_args!($($t)*)).to_string())
        }
    };
}