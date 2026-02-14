use hmac::{Hmac, Mac};
use serde_json::{Map, Value};
use sha2::Sha256;
use std::collections::BTreeMap;

type HmacSha256 = Hmac<Sha256>;

/// Serializes a JSON value to a compact string.
pub fn stringify(value: &Value) -> Result<String, serde_json::Error> {
    serde_json::to_string(value)
}

/// Generates an HMAC-SHA256 signature for the given message.
pub fn sign(key: &str, message: &str) -> String {
    let mut mac =
        HmacSha256::new_from_slice(key.as_bytes()).expect("HMAC can take key of any size");
    mac.update(message.as_bytes());
    hex::encode(mac.finalize().into_bytes())
}

/// Flattens a JSON map and builds a URL-encoded query string.
pub fn http_build_query(dictionary: &Map<String, Value>) -> String {
    let mut flat: BTreeMap<String, Value> = BTreeMap::new();

    for (key, value) in dictionary {
        match value {
            Value::Array(items) => flatten_list(items, key, &mut flat),
            Value::Object(map) => flatten_dict(map, key, &mut flat),
            _ => {
                flat.insert(key.clone(), value.clone());
            }
        }
    }

    let mut strings: Vec<String> = Vec::new();
    for (key, value) in flat {
        let encoded = match value {
            Value::String(text) => urlencoding::encode(&text).into_owned(),
            Value::Array(items) => str_from_list(&items),
            Value::Object(map) => str_from_dict(&map),
            other => value_to_string(&other),
        };
        strings.push(format!("{key}={encoded}"));
    }
    strings.sort();
    strings.join("&")
}

fn flatten_list(list: &[Value], parent_key: &str, out: &mut BTreeMap<String, Value>) {
    for (index, value) in list.iter().enumerate() {
        let new_key = if parent_key.is_empty() {
            format!("[{index}]")
        } else {
            format!("{parent_key}[{index}]")
        };

        match value {
            Value::Array(items) => flatten_list(items, &new_key, out),
            Value::Object(map) => flatten_dict(map, &new_key, out),
            _ => {
                out.insert(new_key, value.clone());
            }
        }
    }
}

fn flatten_dict(map: &Map<String, Value>, parent_key: &str, out: &mut BTreeMap<String, Value>) {
    for (key, value) in map {
        let new_key = if parent_key.is_empty() {
            format!("[{key}]")
        } else {
            format!("{parent_key}[{key}]")
        };

        match value {
            Value::Array(items) => flatten_list(items, &new_key, out),
            Value::Object(map) => flatten_dict(map, &new_key, out),
            _ => {
                out.insert(new_key, value.clone());
            }
        }
    }
}

fn str_from_list(items: &[Value]) -> String {
    let mut strings: Vec<String> = Vec::new();

    for (index, value) in items.iter().enumerate() {
        let value = match value {
            Value::Object(map) => str_from_dict(map),
            Value::Array(items) => str_from_list(items),
            other => value_to_string(other),
        };
        strings.push(format!("{index}={value}"));
    }

    strings.join("&")
}

fn str_from_dict(map: &Map<String, Value>) -> String {
    let mut strings: Vec<String> = Vec::new();
    for (key, value) in map {
        let value = match value {
            Value::Object(map) => str_from_dict(map),
            Value::Array(items) => str_from_list(items),
            other => value_to_string(other),
        };
        strings.push(format!("{key}={value}"));
    }

    strings.sort();
    strings.join("&")
}

fn value_to_string(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        Value::Number(num) => num.to_string(),
        Value::Bool(flag) => flag.to_string(),
        Value::Null => "null".to_string(),
        Value::Array(items) => str_from_list(items),
        Value::Object(map) => str_from_dict(map),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sign_matches_known_hmac_sha256() {
        let signature = sign("secret", "payload123");
        assert_eq!(
            signature,
            "2d0b504df2ec038ce920731d9e4a3e0e9743cabbb6fc2c88ab923f1c52368b61"
        );
    }
}
