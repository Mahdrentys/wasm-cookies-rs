//! This module provides the same functions as the root module, but which don't operate
//! directly on the browser's cookie string, so it can be used outside a browser.
//!
//! Instead of reading the browser's cookie string, functions in this module take it as an
//! argument. Instead of writing to the browser's cookie string, they return it.

use js_sys::Date;
use std::collections::HashMap;
use std::time::Duration;
use urlencoding::FromUrlEncodingError;
use wasm_bindgen::JsValue;

/// URI decoding error on a key or a value, when calling `wasm_cookie::all`.
#[derive(Debug)]
pub enum AllDecodeError {
    /// URI decoding error on a key.
    ///
    /// - The first field is the raw key.
    /// - The second field is the URI decoding error.
    Key(String, FromUrlEncodingError),

    /// URI decoding error on a value.
    ///
    /// - The first field is the URI decoded key corresponding to the value.
    /// - The second field is the URI decoding error.
    Value(String, FromUrlEncodingError),
}

fn process_key_value_str(key_value_str: &str) -> Result<(&str, &str), ()> {
    let mut key_value_iter = key_value_str.split('=');

    match key_value_iter.next() {
        Some(key) => match key_value_iter.next() {
            Some(value) => Ok((key.trim(), value.trim())),
            None => Err(()),
        },

        None => Err(()),
    }
}

/// Returns all cookies as key-value pairs, with undecoded keys and values.
pub fn all_iter_raw(cookie_string: &str) -> impl Iterator<Item = (&str, &str)> {
    cookie_string.split(';').filter_map(|key_value_str| {
        match process_key_value_str(key_value_str) {
            Ok((key, value)) => Some((key, value)),
            Err(_) => None,
        }
    })
}

/// Returns all cookies as key-value pairs, with URI decoded keys and values
/// (with the [urlencoding crate](https://crates.io/crates/urlencoding)),
/// or an error if URI decoding fails on a key or a value.
pub fn all_iter(
    cookie_string: &str,
) -> impl Iterator<Item = Result<(String, String), AllDecodeError>> + '_ {
    all_iter_raw(cookie_string).map(|(key, value)| match urlencoding::decode(key) {
        Ok(key) => match urlencoding::decode(value) {
            Ok(value) => Ok((key, value)),
            Err(error) => Err(AllDecodeError::Value(key, error)),
        },

        Err(error) => Err(AllDecodeError::Key(key.to_owned(), error)),
    })
}

/// Returns all cookies, with undecoded keys and values.
pub fn all_raw(cookie_string: &str) -> HashMap<String, String> {
    all_iter_raw(cookie_string)
        .map(|(key, value)| (key.to_owned(), value.to_owned()))
        .collect()
}

/// Returns all cookies, with URI decoded keys and values
/// (with the [urlencoding crate](https://crates.io/crates/urlencoding)),
/// or an error if URI decoding fails on a key or a value.
pub fn all(cookie_string: &str) -> Result<HashMap<String, String>, AllDecodeError> {
    all_iter(cookie_string).collect()
}

/// Returns undecoded cookie if it exists.
pub fn get_raw(cookie_string: &str, name: &str) -> Option<String> {
    cookie_string
        .split(';')
        .find_map(|key_value_str| match process_key_value_str(key_value_str) {
            Ok((key, value)) => {
                if key == name {
                    Some(value.to_owned())
                } else {
                    None
                }
            }

            Err(_) => None,
        })
}

/// If it exists, returns URI decoded cookie
/// (with the [urlencoding crate](https://crates.io/crates/urlencoding))
/// or an error if the value's URI decoding fails.
pub fn get(cookie_string: &str, name: &str) -> Option<Result<String, FromUrlEncodingError>> {
    let name = urlencoding::encode(name);

    cookie_string
        .split(';')
        .find_map(|key_value_str| match process_key_value_str(key_value_str) {
            Ok((key, value)) => {
                if key == name {
                    Some(urlencoding::decode(value))
                } else {
                    None
                }
            }

            Err(_) => None,
        })
}

/// Cookies options (see [https://developer.mozilla.org/en-US/docs/Web/API/Document/cookie](https://developer.mozilla.org/en-US/docs/Web/API/Document/cookie)).
///
/// You can create it by calling `CookieOptions::default()`.
#[derive(Default, Clone, Debug)]
pub struct CookieOptions<'a> {
    /// If `None`, defaults to the current path of the current document location.
    pub path: Option<&'a str>,

    /// If `None`, defaults to the host portion of the current document location.
    pub domain: Option<&'a str>,

    /// Expiration date in GMT string format.
    /// If `None`, the cookie will expire at the end of session.
    pub expires: Option<String>,

    /// If true, the cookie will only be transmitted over secure protocol as HTTPS.
    /// The default value is false.
    pub secure: bool,

    /// SameSite prevents the browser from sending the cookie along with cross-site requests
    /// (see [https://developer.mozilla.org/en-US/docs/Web/HTTP/Cookies#SameSite_attribute](https://developer.mozilla.org/en-US/docs/Web/HTTP/Cookies#SameSite_attribute)).
    pub same_site: SameSite,
}

impl<'a> CookieOptions<'a> {
    /// Set the path.
    /// The default value is the current path of the current document location.
    pub fn with_path(mut self, path: &'a str) -> Self {
        self.path = Some(path);
        self
    }

    /// Set the domain.
    /// The default value is the host portion of the current document location.
    pub fn with_domain(mut self, domain: &'a str) -> Self {
        self.domain = Some(domain);
        self
    }

    /// Expires the cookie at a specific date.
    /// The default behavior of the cookie is to expire at the end of session.
    pub fn expires_at_date(mut self, date: &Date) -> Self {
        self.expires = Some(date.to_utc_string().into());
        self
    }

    /// Expires the cookie at a specific timestamp (in seconds).
    /// The default behavior of the cookie is to expire at the end of session.
    pub fn expires_at_timestamp(self, timestamp: u64) -> Self {
        self.expires_at_date(&Date::new(&JsValue::from_f64(timestamp as f64 * 1000.0)))
    }

    /// Expires the cookie after a certain duration.
    /// The default behavior of the cookie is to expire at the end of session.
    pub fn expires_after(self, duration: Duration) -> Self {
        self.expires_at_timestamp((Date::now() / 1000.0 + duration.as_secs_f64()) as u64)
    }

    /// Set the cookie to be only transmitted over secure protocol as HTTPS.
    pub fn secure(mut self) -> Self {
        self.secure = true;
        self
    }

    /// Set the SameSite value.
    /// SameSite prevents the browser from sending the cookie along with cross-site requests
    /// (see [https://developer.mozilla.org/en-US/docs/Web/HTTP/Cookies#SameSite_attribute](https://developer.mozilla.org/en-US/docs/Web/HTTP/Cookies#SameSite_attribute)).
    pub fn with_same_site(mut self, same_site: SameSite) -> Self {
        self.same_site = same_site;
        self
    }
}

/// SameSite value for [CookieOptions](struct.CookieOptions.html).
///
/// SameSite prevents the browser from sending the cookie along with cross-site requests
/// (see [https://developer.mozilla.org/en-US/docs/Web/HTTP/Cookies#SameSite_attribute](https://developer.mozilla.org/en-US/docs/Web/HTTP/Cookies#SameSite_attribute)).
#[derive(Clone, Debug)]
pub enum SameSite {
    /// The `Lax` value value will send the cookie for all same-site requests and top-level navigation GET requests.
    /// This is sufficient for user tracking, but it will prevent many CSRF attacks.
    /// This is the default value when calling `SameSite::default()`.
    Lax,

    /// The `Strict` value will prevent the cookie from being sent by the browser to the
    /// target site in all cross-site browsing contexts, even when following a regular link.
    Strict,

    /// The `None` value explicitly states no restrictions will be applied.
    /// The cookie will be sent in all requests - both cross-site and same-site.
    None,
}

impl Default for SameSite {
    fn default() -> Self {
        Self::Lax
    }
}

impl SameSite {
    fn cookie_string_value(&self) -> &'static str {
        match self {
            SameSite::Lax => "lax",
            SameSite::Strict => "strict",
            SameSite::None => "none",
        }
    }
}

/// Return the cookie string that sets a cookie, with non encoded name and value.
pub fn set_raw(name: &str, value: &str, options: &CookieOptions) -> String {
    let mut cookie_string = name.to_owned();
    cookie_string.push('=');
    cookie_string.push_str(value);

    if let Some(path) = options.path {
        cookie_string.push_str(";path=");
        cookie_string.push_str(path);
    }

    if let Some(domain) = options.domain {
        cookie_string.push_str(";domain=");
        cookie_string.push_str(domain);
    }

    if let Some(expires_str) = &options.expires {
        cookie_string.push_str(";expires=");
        cookie_string.push_str(expires_str);
    }

    if options.secure {
        cookie_string.push_str(";secure");
    }

    cookie_string.push_str(";samesite=");
    cookie_string.push_str(options.same_site.cookie_string_value());

    cookie_string
}

/// Return the cookie string that sets a cookie, with URI encoded name and value
/// (with the [urlencoding crate](https://crates.io/crates/urlencoding)).
pub fn set(name: &str, value: &str, options: &CookieOptions) -> String {
    set_raw(
        &urlencoding::encode(name),
        &urlencoding::encode(value),
        options,
    )
}

/// Return the cookie string that deletes a cookie without encoding its name.
pub fn delete_raw(name: &str) -> String {
    format!("{}=;expires=Thu, 01 Jan 1970 00:00:00 GMT", name)
}

/// Return the cookie string that deletes a cookie, URI encoding its name.
pub fn delete(name: &str) -> String {
    delete_raw(&urlencoding::encode(name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_raw() {
        let cookies = all_raw(" key1=value1;key2=value2 ; key3  = value3");
        assert_eq!(cookies.len(), 3);
        assert_eq!(cookies["key1"], "value1");
        assert_eq!(cookies["key2"], "value2");
        assert_eq!(cookies["key3"], "value3");

        let cookies = all_raw("");
        assert!(cookies.is_empty());
    }

    #[test]
    fn test_all() {
        let cookies =
            all("key%20%251=value%20%251  ;key%20%252=value%20%252;key%20%253  = value%20%253")
                .unwrap();
        assert_eq!(cookies.len(), 3);
        assert_eq!(cookies["key %1"], "value %1");
        assert_eq!(cookies["key %2"], "value %2");
        assert_eq!(cookies["key %3"], "value %3");

        let cookies = all("").unwrap();
        assert!(cookies.is_empty());

        let error = all("key1=value1;key2%AA=value2").unwrap_err();

        match error {
            AllDecodeError::Key(raw_key, _) => assert_eq!(raw_key, "key2%AA"),
            _ => panic!(),
        }

        let error = all("key1=value1;key%202=value2%AA").unwrap_err();

        match error {
            AllDecodeError::Value(key, _) => assert_eq!(key, "key 2"),
            _ => panic!(),
        }
    }

    #[test]
    fn test_get_raw() {
        assert_eq!(
            get_raw("key1=value1 ; key2= value2;key3=value3", "key2"),
            Some("value2".to_owned())
        );

        assert_eq!(
            get_raw("key1=value1 ; key2= value2;key3=value3", "key4"),
            None
        );
    }

    #[test]
    fn test_get() {
        assert_eq!(
            get("key1=value1 ; key%202= value%202;key3=value3", "key 2")
                .map(|result| result.unwrap()),
            Some("value 2".to_owned())
        );

        assert!(get("key1=value1 ; key2= value2;key3=value3", "key4").is_none());
        assert!(get("key1=value1 ; key2= value2%AA;key3=value3", "key2")
            .unwrap()
            .is_err());
    }

    #[test]
    fn test_set_raw() {
        assert_eq!(
            set_raw("key", "value", &CookieOptions::default()),
            "key=value;samesite=lax"
        );

        assert_eq!(
            set_raw("key", "value", &CookieOptions::default().with_path("/path")),
            "key=value;path=/path;samesite=lax"
        );

        assert_eq!(
            set_raw(
                "key",
                "value",
                &CookieOptions::default()
                    .with_path("/path")
                    .with_domain("example.com")
            ),
            "key=value;path=/path;domain=example.com;samesite=lax"
        );

        assert_eq!(
            set_raw(
                "key",
                "value",
                &CookieOptions::default()
                    .with_path("/path")
                    .with_domain("example.com")
                    .secure()
            ),
            "key=value;path=/path;domain=example.com;secure;samesite=lax"
        );
    }
}
