use js_sys::Promise;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;


#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn error(s: &str);

    #[cfg(feature = "tauri")]
    #[wasm_bindgen(js_namespace=["__TAURI_INTERNALS__"], js_name = invoke)]
    pub fn tauri_invoke(cmd: &str, args: JsValue) -> Promise;

    #[cfg(feature = "tauri")]
    #[wasm_bindgen(js_namespace=["__TAURI__", "event"], js_name=listen)]
    pub fn tauri_listen(event_name: &str, callback: &js_sys::Function);

    #[cfg(feature = "tauri")]
    #[wasm_bindgen(js_namespace=["__TAURI__", "core"], js_name=convertFileSrc)]
    pub fn tauri_convert_file_src(src: &str, protocol: Option<&str>) -> JsValue;
}


