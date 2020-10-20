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

pub fn all_raw(cookie_string: &str) -> HashMap<String, String> {
    cookie_string
        .split(';')
        .filter_map(|key_value_str| match process_key_value_str(key_value_str) {
            Ok((key, value)) => Some((key.to_owned(), value.to_owned())),
            Err(_) => None,
        })
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
}
