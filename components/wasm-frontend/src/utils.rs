use crate::prelude::*;
use std::panic;
use wasm_bindgen::JsValue;


pub fn log_to_tauri(msg: &str) {
    let args = js_sys::Object::new();
    js_sys::Reflect::set(&args, &"message".into(), &msg.into()).unwrap();
    // TODO use the return value
    let _ = tauri_invoke("wasm_log", JsValue::from(args));
}


#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (log_to_tauri(&format_args!($($t)*).to_string()))
}

#[macro_export]
macro_rules! error_log {
    ($($t:tt)*) => (error(&format_args!($($t)*).to_string()))
}

#[macro_export]
macro_rules! debug_console_log {
    ($($t:tt)*) => {
        #[cfg(debug_assertions)]
        {
            log_to_tauri(&format_args!("[DEBUG] {}", format_args!($($t)*)).to_string())
        }
    };
}

pub fn set_panic_hook() {
    panic::set_hook(Box::new(move |info| {
        error_log!("Rust panic: {}", info);
    }))
}

