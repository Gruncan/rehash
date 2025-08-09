#[cfg(feature = "util")]
pub mod logging;
#[cfg(feature = "util")]
pub mod errors;
#[cfg(feature = "util")]
pub mod wrappers;


#[cfg(feature = "util")]
pub mod utils {
    use crate::error_log;
    use crate::logging::error_to_tauri;
    use js_sys::Promise;
    use js_sys::{Error, Reflect};
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


    pub fn set_panic_hook() {
        panic::set_hook(Box::new(move |info| {
            let msg = match info.payload().downcast_ref::<&str>() {
                Some(s) => *s,
                None => match info.payload().downcast_ref::<String>() {
                    Some(s) => &s[..],
                    None => "Unknown panic message",
                },
            };
            let location = if let Some(loc) = info.location() {
                format!("{}:{}", loc.file(), loc.line())
            } else {
                "unknown location".to_string()
            };

            let error = Error::new("panic");
            let stack = Reflect::get(&error, &JsValue::from_str("stack"))
                .ok()
                .and_then(|v| v.as_string())
                .unwrap_or_else(|| "No stack trace available".to_string());

            error_log!(
                "Rust panic!\n\tMessage: \t{}\nLocation: \t{}\nStack trace:\n\t{}",
                msg,
                location,
                stack
            );
        }))
    }

}


