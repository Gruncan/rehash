use js_sys::{ArrayBuffer, Function, Promise, Reflect};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use wasm_bindings_lib::*;
use web_sys::Response;

fn base64_encode(input: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();

    for chunk in input.chunks(3) {
        let mut buf = [0u8; 3];
        for (i, &byte) in chunk.iter().enumerate() {
            buf[i] = byte;
        }

        let b = ((buf[0] as u32) << 16) | ((buf[1] as u32) << 8) | (buf[2] as u32);

        result.push(CHARS[((b >> 18) & 63) as usize] as char);
        result.push(CHARS[((b >> 12) & 63) as usize] as char);
        result.push(if chunk.len() > 1 { CHARS[((b >> 6) & 63) as usize] as char } else { '=' });
        result.push(if chunk.len() > 2 { CHARS[(b & 63) as usize] as char } else { '=' });
    }

    result
}

async fn load_wasm(name: &str) -> Result<JsValue, JsValue> {
    let js_url = format!("assets://pkg/{}.js", name);
    let wasm_url = format!("assets://pkg/{}_bg.wasm", name);

    let js_module = match JsFuture::from(dynamic_import(js_url.as_str())).await {
        Ok(module) => module,
        Err(e) => return Err(JsValue::from_str("Failed to dynamically import js file"))
    };


    let response = JsFuture::from(fetch(wasm_url.as_str())).await?;
    let response: Response = response.dyn_into()?;
    console_log!("Got WASM module");

    let array_buffer = JsFuture::from(response.array_buffer()?).await?;
    let array_buffer: ArrayBuffer = array_buffer.dyn_into()?;
    console_log!("Got WASM array buffer");

    let init_func = Reflect::get(&js_module, &"default".into())?;
    let init_func: Function = init_func.dyn_into()?;
    console_log!("Got WASM init function");

    let promise: Promise = init_func.call1(&JsValue::NULL, &array_buffer)?.dyn_into()?;
    console_log!("Called WASM init function");

    JsFuture::from(promise).await?;

    Ok(js_module)
}


#[wasm_bindgen(start)]
pub fn main() {
    set_panic_hook();

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