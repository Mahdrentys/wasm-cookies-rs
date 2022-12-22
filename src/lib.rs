pub mod cookies;
pub use cookies::{AllDecodeError, CookieOptions, SameSite};
pub use urlencoding::FromUrlEncodingError;

#[cfg(target_arch = "wasm32")]
use std::collections::HashMap;
#[cfg(target_arch = "wasm32")]
use web_sys::HtmlDocument;

#[cfg(target_arch = "wasm32")]
fn document() -> HtmlDocument {
    use wasm_bindgen::JsCast;

    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .dyn_into::<HtmlDocument>()
        .unwrap()
}

#[cfg(target_arch = "wasm32")]
fn cookie_string() -> String {
    document().cookie().unwrap()
}

#[cfg(target_arch = "wasm32")]
fn set_cookie_string(value: &str) {
    document().set_cookie(value).unwrap();
}

/// Returns all cookies, with undecoded keys and values.
///
/// Available only on `wasm32-unknown-unknown` target.
#[cfg(target_arch = "wasm32")]
pub fn all_raw() -> HashMap<String, String> {
    cookies::all_raw(&cookie_string())
}

/// Returns all cookies, with URI decoded keys and values
/// (with the [urlencoding crate](https://crates.io/crates/urlencoding)),
/// or an error if URI decoding fails on a key or a value.
///
/// Available only on `wasm32-unknown-unknown` target.
#[cfg(target_arch = "wasm32")]
pub fn all() -> Result<HashMap<String, String>, AllDecodeError> {
    cookies::all(&cookie_string())
}

/// Returns undecoded cookie if it exists.
///
/// Available only on `wasm32-unknown-unknown` target.
#[cfg(target_arch = "wasm32")]
pub fn get_raw(name: &str) -> Option<String> {
    cookies::get_raw(&cookie_string(), name)
}

/// If it exists, returns URI decoded cookie
/// (with the [urlencoding crate](https://crates.io/crates/urlencoding))
/// or an error if the value's URI decoding fails.
///
/// Available only on `wasm32-unknown-unknown` target.
#[cfg(target_arch = "wasm32")]
pub fn get(name: &str) -> Option<Result<String, FromUrlEncodingError>> {
    cookies::get(&cookie_string(), name)
}

/// Sets a cookie, with non encoded name and value.
///
/// Available only on `wasm32-unknown-unknown` target.
#[cfg(target_arch = "wasm32")]
pub fn set_raw(name: &str, value: &str, options: &CookieOptions) {
    set_cookie_string(&cookies::set_raw(name, value, options));
}

/// Sets a cookie, with URI encoded name and value
/// (with the [urlencoding crate](https://crates.io/crates/urlencoding)).
///
/// Available only on `wasm32-unknown-unknown` target.
#[cfg(target_arch = "wasm32")]
pub fn set(name: &str, value: &str, options: &CookieOptions) {
    set_cookie_string(&cookies::set(name, value, options));
}

/// Deletes a cookie without encoding its name.
///
/// Available only on `wasm32-unknown-unknown` target.
#[cfg(target_arch = "wasm32")]
pub fn delete_raw(name: &str) {
    set_cookie_string(&cookies::delete_raw(name));
}

/// Deletes a cookie, URI encoding its name.
///
/// Available only on `wasm32-unknown-unknown` target.
#[cfg(target_arch = "wasm32")]
pub fn delete(name: &str) {
    set_cookie_string(&cookies::delete(name));
}
