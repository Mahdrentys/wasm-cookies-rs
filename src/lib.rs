mod cookies;
pub use cookies::{AllDecodeError, CookieOptions, SameSite};
pub use urlencoding::FromUrlEncodingError;

use std::collections::HashMap;
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

/// If it exists, returns URI decoded cookie
/// (with the [urlencoding crate](https://crates.io/crates/urlencoding))
/// or an error if the value's URI decoding fails.
pub fn get(name: &str) -> Option<Result<String, FromUrlEncodingError>> {
    cookies::get(&cookie_string(), name)
}

/// Sets a cookie, with non encoded name and value.
pub fn set_raw(name: &str, value: &str, options: &CookieOptions) {
    set_cookie_string(&cookies::set_raw(name, value, options));
}

/// Sets a cookie, with URI encoded name and value
/// (with the [urlencoding crate](https://crates.io/crates/urlencoding)).
pub fn set(name: &str, value: &str, options: &CookieOptions) {
    set_cookie_string(&cookies::set(name, value, options));
}

/// Delete a cookie without encoding its name.
pub fn delete_raw(name: &str) {
    set_cookie_string(&cookies::delete_raw(name));
}

/// Delete a cookie, URI encoding its name.
pub fn delete(name: &str) {
    set_cookie_string(&cookies::delete(name));
}
