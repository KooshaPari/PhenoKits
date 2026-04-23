//! Integration tests for Logify
//!
//! These tests verify end-to-end functionality across modules.

use logkit::{Level, LogEntry};
use serde_json::json;
use std::thread;

// ============================================================================
// Integration Tests - LogEntry with Various Levels
// ============================================================================

/// Tests LogEntry creation with Trace level
#[test]
fn test_log_entry_trace_level() {
    let entry = LogEntry::new(Level::Trace, "Trace message");
    assert_eq!(entry.level, Level::Trace);
}

/// Tests LogEntry creation with Debug level
#[test]
fn test_log_entry_debug_level() {
    let entry = LogEntry::new(Level::Debug, "Debug message");
    assert_eq!(entry.level, Level::Debug);
}

/// Tests LogEntry creation with Info level
#[test]
fn test_log_entry_info_level() {
    let entry = LogEntry::new(Level::Info, "Info message");
    assert_eq!(entry.level, Level::Info);
}

/// Tests LogEntry creation with Warn level
#[test]
fn test_log_entry_warn_level() {
    let entry = LogEntry::new(Level::Warn, "Warn message");
    assert_eq!(entry.level, Level::Warn);
}

/// Tests LogEntry creation with Error level
#[test]
fn test_log_entry_error_level() {
    let entry = LogEntry::new(Level::Error, "Error message");
    assert_eq!(entry.level, Level::Error);
}

/// Tests LogEntry creation with Fatal level
#[test]
fn test_log_entry_fatal_level() {
    let entry = LogEntry::new(Level::Fatal, "Fatal message");
    assert_eq!(entry.level, Level::Fatal);
}

/// Tests LogEntry creation with empty message
#[test]
fn test_log_entry_empty_message() {
    let entry = LogEntry::new(Level::Info, "");
    assert_eq!(entry.message, "");
}

// ============================================================================
// Concurrency Tests
// ============================================================================

/// Tests concurrent entry creation
#[test]
fn test_concurrent_entry_creation() {
    let handles: Vec<_> = (0..10)
        .map(|i| {
            thread::spawn(move || {
                let entry = LogEntry::new(Level::Info, format!("Thread {}", i));
                entry
            })
        })
        .collect();

    let entries: Vec<LogEntry> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    assert_eq!(entries.len(), 10);
    // All IDs should be unique
    let mut ids: Vec<_> = entries.iter().map(|e| e.id).collect();
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), 10);
}

/// Tests concurrent level access
#[test]
fn test_concurrent_level_access() {
    let levels =
        vec![Level::Trace, Level::Debug, Level::Info, Level::Warn, Level::Error, Level::Fatal];

    let handles: Vec<_> = (0..6)
        .map(|i| {
            let lvls = levels.clone();
            thread::spawn(move || {
                let level = lvls[i];
                let entry = LogEntry::new(level, "Test");
                (level, entry.level)
            })
        })
        .collect();

    for handle in handles {
        let (orig, parsed) = handle.join().unwrap();
        assert_eq!(orig, parsed);
    }
}

// ============================================================================
// Edge Cases
// ============================================================================

/// Tests LogEntry with very long message
#[test]
fn test_log_entry_long_message() {
    let long_msg = "x".repeat(1_000_000);
    let entry = LogEntry::new(Level::Info, long_msg.clone());
    assert_eq!(entry.message, long_msg);
}

/// Tests LogEntry with unicode message
#[test]
fn test_log_entry_unicode_message() {
    let entry = LogEntry::new(Level::Info, "Hello 世界 🌍");
    assert_eq!(entry.message, "Hello 世界 🌍");
}

/// Tests LogEntry with special characters
#[test]
fn test_log_entry_special_characters() {
    let messages =
        vec!["Normal text", "With \"quotes\"", "With \\ backslash", "With\nnewline", "With\ttab"];

    for msg in messages {
        let entry = LogEntry::new(Level::Info, msg);
        assert_eq!(entry.message, msg);
    }
}

// ============================================================================
// Serialization Integration Tests
// ============================================================================

/// Test suite for serialization integration
mod serialization_integration {
    use super::*;

    #[test]
    fn test_log_entry_json_structure() {
        let entry = LogEntry::new(Level::Info, "Test message")
            .with_field("user_id", json!(123))
            .with_field("action", json!("login"));

        let json_str = serde_json::to_string(&entry).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

        // Verify structure
        assert!(parsed.get("id").is_some());
        assert_eq!(parsed["level"], "info");
        assert_eq!(parsed["message"], "Test message");
        assert!(parsed.get("timestamp").is_some());
        assert!(parsed.get("fields").is_some());

        // Verify fields
        let fields = parsed["fields"].as_array().unwrap();
        assert_eq!(fields.len(), 2);
    }

    #[test]
    fn test_log_entry_batch_serialization() {
        let entries: Vec<LogEntry> = vec![
            LogEntry::new(Level::Trace, "Trace entry"),
            LogEntry::new(Level::Debug, "Debug entry"),
            LogEntry::new(Level::Info, "Info entry"),
            LogEntry::new(Level::Warn, "Warn entry"),
            LogEntry::new(Level::Error, "Error entry"),
            LogEntry::new(Level::Fatal, "Fatal entry"),
        ];

        let json_str = serde_json::to_string(&entries).unwrap();
        let parsed: Vec<serde_json::Value> = serde_json::from_str(&json_str).unwrap();

        assert_eq!(parsed.len(), 6);
        assert_eq!(parsed[0]["level"], "trace");
        assert_eq!(parsed[5]["level"], "fatal");
    }

    #[test]
    fn test_nested_json_fields() {
        let entry = LogEntry::new(Level::Info, "Complex data")
            .with_field(
                "user",
                json!({
                    "name": "Alice",
                    "roles": ["admin", "user"],
                    "metadata": {"active": true}
                }),
            )
            .with_field(
                "request",
                json!({
                    "headers": {"content-type": "application/json"},
                    "body": [1, 2, 3]
                }),
            );

        let json_str = serde_json::to_string(&entry).unwrap();
        let parsed: LogEntry = serde_json::from_str(&json_str).unwrap();

        assert_eq!(parsed.fields.len(), 2);
    }
}

// ============================================================================
// Level Integration Tests
// ============================================================================

/// Test suite for Level integration
mod level_integration {
    use super::*;

    #[test]
    fn test_level_severity_order() {
        let levels =
            vec![Level::Trace, Level::Debug, Level::Info, Level::Warn, Level::Error, Level::Fatal];

        for i in 0..levels.len() {
            for j in (i + 1)..levels.len() {
                assert!(levels[i] < levels[j], "{:?} should be < {:?}", levels[i], levels[j]);
            }
        }
    }

    #[test]
    fn test_level_to_from_json() {
        for level in
            [Level::Trace, Level::Debug, Level::Info, Level::Warn, Level::Error, Level::Fatal]
        {
            let json_val = serde_json::to_value(level).unwrap();
            let parsed: Level = serde_json::from_value(json_val).unwrap();
            assert_eq!(level, parsed);
        }
    }
}

// ============================================================================
// Stress Tests
// ============================================================================

/// Test suite for stress and edge cases
mod stress_tests {
    use super::*;

    #[test]
    fn test_many_fields() {
        let mut entry = LogEntry::new(Level::Info, "Many fields");
        for i in 0..1000 {
            entry = entry.with_field(format!("field_{}", i), json!(i));
        }
        assert_eq!(entry.fields.len(), 1000);
    }

    #[test]
    fn test_large_log_entry() {
        let large_msg = "x".repeat(1_000_000);
        let entry = LogEntry::new(Level::Info, large_msg);
        let json = serde_json::to_string(&entry).unwrap();
        assert!(json.len() > 1_000_000);
    }

    #[test]
    fn test_many_entries() {
        let entries: Vec<LogEntry> =
            (0..10_000).map(|i| LogEntry::new(Level::Info, format!("Entry {}", i))).collect();

        let json = serde_json::to_string(&entries).unwrap();
        let parsed: Vec<LogEntry> = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.len(), 10_000);
    }

    #[test]
    fn test_special_characters_in_message() {
        let messages = vec![
            "Hello\tWorld",         // Tab
            "Line1\nLine2",         // Newline
            "Quotes \"here\"",      // Quotes
            "Backslash \\",         // Backslash
            "Unicode: \u{1F600}",   // Emoji
            "Mixed: Hello 世界 🌍", // Mixed
        ];

        for msg in messages {
            let entry = LogEntry::new(Level::Info, msg);
            let json = serde_json::to_string(&entry).unwrap();
            let parsed: LogEntry = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed.message, msg);
        }
    }
}

// ============================================================================
// Schema Tests
// ============================================================================

/// Test suite for JSON Schema compliance
mod schema_tests {
    use super::*;

    #[test]
    fn test_required_fields_present() {
        let entry = LogEntry::new(Level::Info, "Test");
        let json_str = serde_json::to_string(&entry).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

        // All required fields must be present
        assert!(parsed.get("id").is_some(), "id required");
        assert!(parsed.get("level").is_some(), "level required");
        assert!(parsed.get("message").is_some(), "message required");
        assert!(parsed.get("timestamp").is_some(), "timestamp required");
        assert!(parsed.get("fields").is_some(), "fields required");
    }

    #[test]
    fn test_level_is_lowercase_string() {
        let entry = LogEntry::new(Level::Error, "Test");
        let json_str = serde_json::to_string(&entry).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

        let level = &parsed["level"];
        assert!(level.is_string(), "level must be string");
        assert_eq!(level.as_str().unwrap(), "error");
    }

    #[test]
    fn test_fields_is_array() {
        let entry = LogEntry::new(Level::Info, "Test");
        let json_str = serde_json::to_string(&entry).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

        assert!(parsed["fields"].is_array(), "fields must be array");
    }

    #[test]
    fn test_field_structure() {
        let entry = LogEntry::new(Level::Info, "Test").with_field("key", json!("value"));
        let json_str = serde_json::to_string(&entry).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

        let fields = parsed["fields"].as_array().unwrap();
        assert_eq!(fields.len(), 1);

        let field = &fields[0];
        assert!(field.is_array(), "field must be [key, value] tuple");
        let arr = field.as_array().unwrap();
        assert_eq!(arr.len(), 2);
        assert_eq!(arr[0], "key");
    }
}
