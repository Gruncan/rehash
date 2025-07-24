use wasm_bindgen::prelude::wasm_bindgen;

pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}


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
            log(&format_args!( $ ( $t) * ).to_string())
        }
    };
}

