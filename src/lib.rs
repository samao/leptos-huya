pub mod app;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}

#[cfg(feature = "hydrate")]
#[macro_export]
macro_rules! json {
    ($($key:expr => $value:expr),* $(,)?) => {{
        let obj = js_sys::Object::new();
        $(js_sys::Reflect::set(&obj, &$key.into(), &$value.into()).unwrap();)*
        obj
    }};
}

#[cfg(feature = "hydrate")]
pub fn has(lib_name: &str) -> bool {
    let window = js_sys::global();
    js_sys::Reflect::has(&window, &lib_name.into()).unwrap_or(false)
}

pub fn to_time_str(timestamp: i64) -> String {
    use chrono::{DateTime, Utc};
    let d = DateTime::from_timestamp(timestamp, 0).unwrap_or(Utc::now());
    format!("{}", d.format("%d日 %H:%M"))
}

pub fn to_time_str_format(timestamp: i64, format: &str) -> String {
    use chrono::{DateTime, Utc};
    let d = DateTime::from_timestamp(timestamp, 0).unwrap_or(Utc::now());
    format!("{}", d.format(format))
}
