//! Log Entry

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Level;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub id: Uuid,
    pub level: Level,
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub fields: Vec<(String, serde_json::Value)>,
}

impl LogEntry {
    pub fn new(level: Level, message: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            level,
            message: message.into(),
            timestamp: Utc::now(),
            fields: Vec::new(),
        }
    }

    pub fn with_field(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.fields.push((key.into(), value));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_entry_creation() {
        let entry = LogEntry::new(Level::Info, "Test message");
        assert_eq!(entry.message, "Test message");
        assert_eq!(entry.level, Level::Info);
        assert!(!entry.fields.is_empty() == false); // Initially empty
    }

    #[test]
    fn test_log_entry_with_field() {
        let entry = LogEntry::new(Level::Debug, "Test")
            .with_field("key1", serde_json::json!("value1"))
            .with_field("key2", serde_json::json!(42));

        assert_eq!(entry.fields.len(), 2);
        assert_eq!(entry.fields[0].0, "key1");
        assert_eq!(entry.fields[1].0, "key2");
    }

    #[test]
    fn test_log_entry_serialization() {
        let entry = LogEntry::new(Level::Warn, "Warning message");
        let json = serde_json::to_string(&entry).unwrap();
        assert!(json.contains("Warning message"));
        assert!(json.contains("Warn"));
    }

    #[test]
    fn test_log_entry_clone() {
        let entry = LogEntry::new(Level::Error, "Error");
        let cloned = entry.clone();
        assert_eq!(entry.message, cloned.message);
        assert_eq!(entry.level, cloned.level);
    }

    #[test]
    fn test_log_entry_id_uniqueness() {
        let entry1 = LogEntry::new(Level::Info, "First");
        let entry2 = LogEntry::new(Level::Info, "Second");
        assert_ne!(entry1.id, entry2.id);
    }

    #[test]
    fn test_log_entry_all_levels() {
        for level in
            [Level::Trace, Level::Debug, Level::Info, Level::Warn, Level::Error, Level::Fatal]
        {
            let entry = LogEntry::new(level, "Test");
            assert_eq!(entry.level, level);
        }
    }

    #[test]
    fn test_log_entry_empty_message() {
        let entry = LogEntry::new(Level::Info, "");
        assert_eq!(entry.message, "");
    }

    #[test]
    fn test_log_entry_unicode_message() {
        let entry = LogEntry::new(Level::Info, "Hello 世界 🌍");
        assert_eq!(entry.message, "Hello 世界 🌍");
    }

    #[test]
    fn test_log_entry_timestamp_is_recent() {
        let entry = LogEntry::new(Level::Info, "Test");
        let now = Utc::now();
        let diff = now.signed_duration_since(entry.timestamp);
        assert!(diff.num_seconds() < 1);
    }

    #[test]
    fn test_log_entry_with_multiple_fields() {
        let entry = LogEntry::new(Level::Info, "Test")
            .with_field("string", serde_json::json!("value"))
            .with_field("number", serde_json::json!(42))
            .with_field("bool", serde_json::json!(true))
            .with_field("null", serde_json::json!(null))
            .with_field("array", serde_json::json!([1, 2, 3]))
            .with_field("object", serde_json::json!({"key": "value"}));

        assert_eq!(entry.fields.len(), 6);
    }

    #[test]
    fn test_log_entry_with_special_characters_in_keys() {
        let entry = LogEntry::new(Level::Info, "Test")
            .with_field("key.with.dots", serde_json::json!(1))
            .with_field("key-with-dashes", serde_json::json!(2))
            .with_field("key_with_underscores", serde_json::json!(3))
            .with_field("key with spaces", serde_json::json!(4));

        assert_eq!(entry.fields.len(), 4);
    }

    #[test]
    fn test_log_entry_deserialization_roundtrip() {
        let entry = LogEntry::new(Level::Error, "Error occurred")
            .with_field("code", serde_json::json!(500))
            .with_field("error", serde_json::json!("Not found"));

        let json = serde_json::to_string(&entry).unwrap();
        let parsed: LogEntry = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.message, entry.message);
        assert_eq!(parsed.level, entry.level);
        assert_eq!(parsed.fields.len(), entry.fields.len());
    }

    #[test]
    fn test_log_entry_large_message() {
        let large_message = "x".repeat(1_000_000);
        let entry = LogEntry::new(Level::Info, large_message.clone());
        assert_eq!(entry.message, large_message);
    }

    #[test]
    fn test_log_entry_field_override() {
        // Adding field with same key appends (doesn't override)
        let entry = LogEntry::new(Level::Info, "Test")
            .with_field("key", serde_json::json!(1))
            .with_field("key", serde_json::json!(2));

        assert_eq!(entry.fields.len(), 2);
    }
}

// Property-based tests
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_log_entry_level_serialization(level: u8) {
            // Only test valid level values (0-5)
            if level <= 5 {
                let level = match level {
                    0 => Level::Trace,
                    1 => Level::Debug,
                    2 => Level::Info,
                    3 => Level::Warn,
                    4 => Level::Error,
                    5 => Level::Fatal,
                    _ => Level::Info,
                };
                let entry = LogEntry::new(level, "Test");
                let json = serde_json::to_string(&entry).unwrap();
                let parsed: LogEntry = serde_json::from_str(&json).unwrap();
                prop_assert_eq!(parsed.level, level);
            }
        }

        #[test]
        fn test_log_entry_message_roundtrip(message: String) {
            let entry = LogEntry::new(Level::Info, message.clone());
            prop_assert_eq!(entry.message, message);
        }

        #[test]
        fn test_log_entry_with_field_value_roundtrip(value: String) {
            let entry = LogEntry::new(Level::Info, "test")
                .with_field("field", serde_json::json!(value));
            prop_assert_eq!(entry.fields.len(), 1);
        }
    }
}
