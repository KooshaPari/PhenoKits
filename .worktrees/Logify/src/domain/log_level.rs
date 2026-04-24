//! Log Level

use serde::{Deserialize, Serialize};

/// Log level
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
    Error,
    Fatal,
}

impl Level {
    pub fn as_str(&self) -> &'static str {
        match self {
            Level::Trace => "TRACE",
            Level::Debug => "DEBUG",
            Level::Info => "INFO",
            Level::Warn => "WARN",
            Level::Error => "ERROR",
            Level::Fatal => "FATAL",
        }
    }
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_as_str() {
        assert_eq!(Level::Trace.as_str(), "TRACE");
        assert_eq!(Level::Debug.as_str(), "DEBUG");
        assert_eq!(Level::Info.as_str(), "INFO");
        assert_eq!(Level::Warn.as_str(), "WARN");
        assert_eq!(Level::Error.as_str(), "ERROR");
        assert_eq!(Level::Fatal.as_str(), "FATAL");
    }

    #[test]
    fn test_level_display() {
        assert_eq!(format!("{}", Level::Info), "INFO");
        assert_eq!(format!("{}", Level::Error), "ERROR");
    }

    #[test]
    fn test_level_default() {
        assert_eq!(Level::default(), Level::Info);
    }

    #[test]
    fn test_level_ordering() {
        assert!(Level::Trace < Level::Debug);
        assert!(Level::Debug < Level::Info);
        assert!(Level::Info < Level::Warn);
        assert!(Level::Warn < Level::Error);
        assert!(Level::Error < Level::Fatal);
    }

    #[test]
    fn test_level_equality() {
        assert_eq!(Level::Info, Level::Info);
        assert_ne!(Level::Info, Level::Debug);
    }

    #[test]
    fn test_level_clone() {
        let level = Level::Error;
        let cloned = level;
        assert_eq!(level, cloned);
    }

    #[test]
    fn test_level_serialization() {
        let level = Level::Debug;
        let json = serde_json::to_string(&level).unwrap();
        assert_eq!(json, "\"debug\""); // lowercase due to serde rename
        let parsed: Level = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, level);
    }

    #[test]
    fn test_level_all_serialize_and_deserialize() {
        for level in
            [Level::Trace, Level::Debug, Level::Info, Level::Warn, Level::Error, Level::Fatal]
        {
            let json = serde_json::to_string(&level).unwrap();
            let parsed: Level = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, level);
        }
    }

    #[test]
    fn test_level_transitive_ordering() {
        assert!(Level::Trace < Level::Debug);
        assert!(Level::Debug < Level::Info);
        assert!(Level::Info < Level::Warn);
        assert!(Level::Warn < Level::Error);
        assert!(Level::Error < Level::Fatal);

        // Transitive: Trace < Info, Debug < Warn, etc.
        assert!(Level::Trace < Level::Info);
        assert!(Level::Debug < Level::Error);
        assert!(Level::Info < Level::Fatal);
        assert!(Level::Trace < Level::Fatal);
    }

    #[test]
    fn test_level_hash_consistency() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        for level in
            [Level::Trace, Level::Debug, Level::Info, Level::Warn, Level::Error, Level::Fatal]
        {
            set.insert(level);
        }
        assert_eq!(set.len(), 6); // All levels should be unique
    }

    #[test]
    fn test_level_from_str_valid() {
        for level in
            [Level::Trace, Level::Debug, Level::Info, Level::Warn, Level::Error, Level::Fatal]
        {
            let s = level.as_str();
            let display = format!("{}", level);
            assert_eq!(s, display);
        }
    }

    #[test]
    fn test_level_debug_format() {
        let level = Level::Error;
        let debug_str = format!("{:?}", level);
        assert!(debug_str.contains("Error"));
    }

    #[test]
    fn test_level_copy_semantics() {
        let level = Level::Warn;
        let copied = level;
        assert_eq!(level, copied);
        let _ = copied; // Explicitly use to avoid unused variable warning
        assert_eq!(level, Level::Warn); // Original still valid
    }
}

// Property-based tests
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_level_ordering_consistency(level1: u8, level2: u8) {
            let l1 = match level1 % 6 {
                0 => Level::Trace,
                1 => Level::Debug,
                2 => Level::Info,
                3 => Level::Warn,
                4 => Level::Error,
                _ => Level::Fatal,
            };
            let l2 = match level2 % 6 {
                0 => Level::Trace,
                1 => Level::Debug,
                2 => Level::Info,
                3 => Level::Warn,
                4 => Level::Error,
                _ => Level::Fatal,
            };

            // Reflexivity
            prop_assert!(l1 <= l1);

            // Anti-symmetry
            if l1 < l2 {
                prop_assert!(!(l2 < l1));
            } else if l2 < l1 {
                prop_assert!(!(l1 < l2));
            }
        }

        #[test]
        fn test_level_serialization_roundtrip(level: u8) {
            let level = match level % 6 {
                0 => Level::Trace,
                1 => Level::Debug,
                2 => Level::Info,
                3 => Level::Warn,
                4 => Level::Error,
                _ => Level::Fatal,
            };
            let json = serde_json::to_string(&level).unwrap();
            let parsed: Level = serde_json::from_str(&json).unwrap();
            prop_assert_eq!(parsed, level);
        }

        #[test]
        fn test_level_as_str_length(level: u8) {
            let level = match level % 6 {
                0 => Level::Trace,
                1 => Level::Debug,
                2 => Level::Info,
                3 => Level::Warn,
                4 => Level::Error,
                _ => Level::Fatal,
            };
            let s = level.as_str();
            prop_assert!(s.len() <= 5); // "FATAL" is max
        }
    }
}
