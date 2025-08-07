use crate::errors::RehashError;

#[macro_export]
macro_rules! into_async {
    ($promise:expr) => {{
        use wasm_bindgen_futures::JsFuture;
        JsFuture::from($promise)
    }};
}

#[macro_export]
macro_rules! into_object {
    ($($key:expr => $value:expr),* $(,)?) => {{
        use js_sys::Object;
        use js_sys::Reflect;

        let obj = Object::new();
        let result: Result<_, JsValue> = (|| {
            $(
                Reflect::set(&obj, &JsValue::from($key), &JsValue::from($value))?;
            )*
            Ok(obj)
        })();

        result
    }};
}

enum InvokeCommands {
    GET_DESKTOP_BUILD,
    WASM_LOG,
    WASM_ERROR,
    CREATE_VIDEO_STREAM,
    GET_CHUNK,
}


impl TryFrom<&str> for InvokeCommands {
    type Error = RehashError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "get_desktop_build" => Ok(InvokeCommands::GET_DESKTOP_BUILD),
            "wasm_log" => Ok(InvokeCommands::WASM_LOG),
            "wasm_error" => Ok(InvokeCommands::WASM_ERROR),
            "create_video_stream" => Ok(InvokeCommands::CREATE_VIDEO_STREAM),
            "get_chunk" => Ok(InvokeCommands::GET_CHUNK),
            _ => Err("Unknown invoke command".into()),
        }
    }
}