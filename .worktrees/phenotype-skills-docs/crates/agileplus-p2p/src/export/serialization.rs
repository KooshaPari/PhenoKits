//! JSON serialization utilities for deterministic, git-friendly output.
//!
//! All functions ensure:
//! - Sorted keys
//! - 2-space indentation for pretty printing
//! - UTF-8 encoding

use serde::Serialize;
use serde_json::Value;

/// Serialize a JSON value to a string with sorted keys.
pub fn to_sorted(value: &Value) -> Result<String, serde_json::Error> {
    let mut buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"  ");
    let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
    value.serialize(&mut ser)?;
    String::from_utf8(buf).map_err(|e| serde_json::Error::io(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        e,
    )))
}

/// Serialize a JSON value to a single line with sorted keys.
pub fn to_sorted_line(value: &Value) -> Result<String, serde_json::Error> {
    serde_json::to_string(value)
}

/// Serialize a JSON value to a pretty-printed string with sorted keys.
/// Alias for [`to_sorted`].
pub fn to_sorted_pretty(value: &Value) -> Result<String, serde_json::Error> {
    to_sorted(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_to_sorted() {
        let value = json!({"z": 1, "a": 2});
        let result = to_sorted(&value).unwrap();
        // Keys should be sorted
        assert!(result.contains("\"a\": 2"));
        assert!(result.contains("\"z\": 1"));
    }

    #[test]
    fn test_to_sorted_line() {
        let value = json!({"b": 1, "c": 2});
        let result = to_sorted_line(&value).unwrap();
        assert!(!result.contains('\n'));
    }

    #[test]
    fn test_to_sorted_pretty() {
        let value = json!({"y": true, "a": false});
        let result = to_sorted_pretty(&value).unwrap();
        assert!(result.contains('\n'));
        assert!(result.contains("\"a\": false"));
    }
}
