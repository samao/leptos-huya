pub mod app;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}

#[macro_export]
macro_rules! json {
    ($($key:expr => $value:expr),* $(,)?) => {{
        let obj = js_sys::Object::new();
        $(js_sys::Reflect::set(&obj, &$key.into(), &$value.into()).unwrap();)*
        obj
    }};
}
