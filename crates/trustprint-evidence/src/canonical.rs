//! RFC 8785 JSON Canonicalization Scheme (JCS) Minimal Implementation
//!
//! Ensures deterministic byte-for-byte serialization of JSON objects
//! by sorting keys lexicographically and eliminating unnecessary whitespace.

use serde_json::Value;

pub struct CanonicalJson;

impl CanonicalJson {
    /// Converts any serde_json::Value into its canonical JSON string representation.
    pub fn canonicalize(value: &Value) -> String {
        match value {
            Value::Null => "null".to_string(),
            Value::Bool(b) => if *b { "true".to_string() } else { "false".to_string() },
            Value::Number(n) => n.to_string(),
            Value::String(s) => serde_json::to_string(s).unwrap_or_else(|_| format!("\"{}\"", s)),
            Value::Array(arr) => {
                let items: Vec<String> = arr.iter().map(Self::canonicalize).collect();
                format!("[{}]", items.join(","))
            }
            Value::Object(map) => {
                // Lexicographical sort on object keys
                let mut keys: Vec<&String> = map.keys().collect();
                keys.sort();

                let pairs: Vec<String> = keys
                    .into_iter()
                    .map(|k| {
                        let val_str = Self::canonicalize(&map[k]);
                        format!("{}:{}", serde_json::to_string(k).unwrap(), val_str)
                    })
                    .collect();

                format!("{{{}}}", pairs.join(","))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_canonical_json_sorting() {
        let val1 = json!({
            "zebra": 1,
            "apple": 2,
            "banana": {
                "delta": true,
                "charlie": [3, 2, 1]
            }
        });

        let val2 = json!({
            "apple": 2,
            "banana": {
                "charlie": [3, 2, 1],
                "delta": true
            },
            "zebra": 1
        });

        let canon1 = CanonicalJson::canonicalize(&val1);
        let canon2 = CanonicalJson::canonicalize(&val2);

        assert_eq!(canon1, canon2);
        assert_eq!(
            canon1,
            r#"{"apple":2,"banana":{"charlie":[3,2,1],"delta":true},"zebra":1}"#
        );
    }
}

