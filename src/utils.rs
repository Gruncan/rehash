use std::panic;
use wasm_bindgen::prelude::wasm_bindgen;


#[wasm_bindgen]
extern "C" {
    pub(crate) fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub(crate) fn log(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub(crate) fn error(s: &str);
}


#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
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
            log(&format_args!("[DEBUG] {}", format_args!($($t)*)).to_string())
        }
    };
}

pub fn set_panic_hook() {
    panic::set_hook(Box::new(move |info| {
        error_log!("Rust panic: {}", info);
    }))
}

