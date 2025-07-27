use js_sys::{Array, Function, Object, Reflect};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use wasm_bindings_lib::*;
use web_sys::{Blob, BlobPropertyBag, HtmlElement, Url};


pub const WASM_VERSION: &str = env!("CARGO_PKG_VERSION");


async fn load_wasm(name: &str) -> Result<(), JsValue> {
    let js_path = format!("pkg/{}.js", name);
    let wasm_path = format!("pkg/{}_bg.wasm", name);

    let js_path = JsFuture::from(tauri_resolve_resource(js_path.as_str())).await?;
    debug_console_log!("Loading frontend JS from: {}", js_path.as_string().unwrap_or("NULL".to_string()));

    let content = JsFuture::from(tauri_read_text_file(js_path.as_string().unwrap().as_str())).await?;
    let blob_options = BlobPropertyBag::new();
    blob_options.set_type("application/javascript");

    let array = Array::new();
    array.push(&content);

    let blob = Blob::new_with_u8_array_sequence_and_options(&array, &blob_options)?;

    let blob_url = Url::create_object_url_with_blob(&blob)?;

    debug_console_log!("Blob url {}", blob_url);
    let options = Object::new();
    let base_dir = JsValue::from(11u16);
    Reflect::set(&options, &JsValue::from("baseDir"), &base_dir)?;

    let wasm_blob = JsFuture::from(tauri_read_binary_file(wasm_path.as_str(), &options)).await?;
    debug_console_log!("Wasm blob: {:?}", wasm_blob);

    let module = JsFuture::from(dynamic_import(blob_url.as_str())).await?;

    let init_fn = Reflect::get(&module, &"default".into())?;
    let init_fn = init_fn.dyn_into::<Function>()?;

    // TODO fix this to that is does not warn about deprecated
    // using deprecated parameters for the initialization function; pass a single object instead
    // let obj = Object::new();
    // Reflect::set(&obj, &JsValue::from_str("bytes"), &wasm_blob)?;

    let _ = init_fn.call1(&JsValue::NULL, &wasm_blob)?;

    Ok(())
}


#[wasm_bindgen(start)]
pub fn main() {
    console_log!("Loader version: {}", WASM_VERSION);
    set_panic_hook();

    #[cfg(debug_assertions)]
    {
        let window = web_sys::window().ok_or("Failed to get window").unwrap();
        let document = window.document().ok_or("Failed to get document").unwrap();

        let version_header = document.get_element_by_id("build-loader")
            .unwrap()
            .dyn_into::<HtmlElement>().unwrap();

        version_header.set_text_content(Some(&format!("Build Loader: {}", WASM_VERSION)));
    }

    wasm_bindgen_futures::spawn_local(async move {
        match load_wasm("rehash_wasm_frontend").await {
            Ok(_) => {
                console_log!("Loaded Rehash frontend");
            },
            Err(e) => {
                error_log!("Error loading rehash frontend: {:?}", e);
            }
        }
    })
}