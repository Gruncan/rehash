use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use wasm_bindgen::JsValue;
use web_sys::Element;

pub type RehashResult<T> = Result<T, RehashError>;
pub type RehashResultUnit = Result<(), RehashError>;

pub struct RehashError(pub String);

impl<'a> From<&'a str> for RehashError {
    fn from(s: &'a str) -> Self {
        RehashError(s.to_string())
    }
}

impl<'a> From<String> for RehashError {
    fn from(s: String) -> Self {
        RehashError(s)
    }
}

impl<'a> From<JsValue> for RehashError {
    fn from(value: JsValue) -> Self {
        RehashError(format!("{:?}", value))
    }
}

impl<'a> From<Element> for RehashError {
    fn from(value: Element) -> Self {
        RehashError(format!("{:?}", value))
    }
}


impl Debug for RehashError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for RehashError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for RehashError {}