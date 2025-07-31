#[macro_export]
macro_rules! into_async {
    ($promise:expr) => {
        JsFuture::from($promise)
    };
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