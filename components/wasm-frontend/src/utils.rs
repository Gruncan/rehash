use js_sys::Promise;
use std::panic;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
extern "C" {
    pub(crate) fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub(crate) fn log(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub(crate) fn error(s: &str);

    #[wasm_bindgen(js_namespace=["__TAURI_INTERNALS__"], js_name = invoke)]
    pub(crate) fn tauri_invoke(cmd: &str, args: JsValue) -> Promise;

    #[wasm_bindgen(js_namespace=["__TAURI__", "event"], js_name=listen)]
    pub(crate) fn tauri_listen(event_name: &str, callback: &js_sys::Function);

    #[wasm_bindgen(js_namespace=["__TAURI__", "core"], js_name=convertFileSrc)]
    pub(crate) fn tauri_convert_file_src(src: &str, protocol: Option<&str>) -> JsValue;

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

