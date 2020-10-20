mod cookies;

pub use cookies::AllDecodeError;
use std::collections::HashMap;
pub use urlencoding::FromUrlEncodingError;
use wasm_bindgen::JsCast;
use web_sys::HtmlDocument;

fn document() -> HtmlDocument {
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .dyn_into::<HtmlDocument>()
        .unwrap()
}

fn cookie_string() -> String {
    document().cookie().unwrap()
}

fn set_cookie_string(value: &str) {
    document().set_cookie(value).unwrap();
}

/// Returns all cookies, with undecoded keys and values.
pub fn all_raw() -> HashMap<String, String> {
    cookies::all_raw(&cookie_string())
}

/// Returns all cookies, with URI decoded keys and values
/// (with the [urlencoding crate](https://crates.io/crates/urlencoding)),
/// or an error if URI decoding fails on a key or a value.
pub fn all() -> Result<HashMap<String, String>, AllDecodeError> {
    cookies::all(&cookie_string())
}

/// Returns undecoded cookie if it exists.
pub fn get_raw(name: &str) -> Option<String> {
    cookies::get_raw(&cookie_string(), name)
}

/// If it exists, returns cookie or an error if value URI decoding fails.
pub fn get(name: &str) -> Option<Result<String, FromUrlEncodingError>> {
    cookies::get(&cookie_string(), name)
}
