use js_sys::Promise;
use std::panic;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;


#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn error(s: &str);

    #[wasm_bindgen(js_name = import)]
    pub fn dynamic_import(module: &str) -> Promise;

    #[wasm_bindgen(js_name = import)]
    pub fn fetch(url: &str) -> Promise;

    #[cfg(feature = "tauri")]
    #[wasm_bindgen(js_namespace=["__TAURI_INTERNALS__"], js_name = invoke)]
    pub fn tauri_invoke(cmd: &str, args: JsValue) -> Promise;

    #[cfg(feature = "tauri")]
    #[wasm_bindgen(js_namespace=["__TAURI__", "event"], js_name=listen)]
    pub fn tauri_listen(event_name: &str, callback: &js_sys::Function);

    #[cfg(feature = "tauri")]
    #[wasm_bindgen(js_namespace=["__TAURI__", "core"], js_name=convertFileSrc)]
    pub fn tauri_convert_file_src(src: &str, protocol: Option<&str>) -> JsValue;

    #[cfg(feature = "tauri")]
    #[wasm_bindgen(js_namespace=["__TAURI__", "path"], js_name=resolveResource)]
    pub fn tauri_resolve_resource(src: &str) -> Promise;

    #[cfg(feature = "tauri")]
    #[wasm_bindgen(js_namespace=["__TAURI__", "fs"], js_name=readTextFile)]
    pub fn tauri_read_text_file(src: &str) -> Promise;

    #[cfg(feature = "tauri")]
    #[wasm_bindgen(js_namespace=["__TAURI__", "fs"], js_name=readFile)]
    pub fn tauri_read_binary_file(src: &str, options: &JsValue) -> Promise;
    
}


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


