use std::collections::HashMap;
use urlencoding::FromUrlEncodingError;

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

pub fn all_raw(cookie_string: &str) -> HashMap<String, String> {
    cookie_string
        .split(';')
        .filter_map(|key_value| {
            let mut key_value_iter = key_value.split('=');

            match key_value_iter.next() {
                Some(key) => match key_value_iter.next() {
                    Some(value) => Some((key, value)),
                    None => None,
                },

                None => None,
            }
        })
        .map(|(key, value)| (key.trim().to_owned(), value.trim().to_owned()))
        .collect()
}

pub fn all(cookie_string: &str) -> Result<HashMap<String, String>, AllDecodeError> {
    all_raw(cookie_string)
        .into_iter()
        .map(|(key, value)| match urlencoding::decode(&key) {
            Ok(key) => match urlencoding::decode(&value) {
                Ok(value) => Ok((key.into(), value.into())),
                Err(error) => Err(AllDecodeError::Value(key.into(), error)),
            },

            Err(error) => Err(AllDecodeError::Key(key, error)),
        })
        .collect()
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
}
