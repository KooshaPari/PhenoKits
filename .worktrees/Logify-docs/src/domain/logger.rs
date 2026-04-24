//! Logger Trait

use super::{Level, LogEntry};
use async_trait::async_trait;

#[async_trait]
pub trait Logger: Send + Sync {
    async fn log(&self, entry: LogEntry) -> Result<(), LogError>;
    fn level(&self) -> Level;
}

#[derive(Debug, Clone)]
pub enum LogError {
    Io(String),
    Serialization(String),
}

impl std::fmt::Display for LogError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogError::Io(s) => write!(f, "IO error: {}", s),
            LogError::Serialization(s) => write!(f, "Serialization error: {}", s),
        }
    }
}

impl std::error::Error for LogError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_error_io_display() {
        let err = LogError::Io("file not found".to_string());
        assert_eq!(err.to_string(), "IO error: file not found");
    }

    #[test]
    fn test_log_error_serialization_display() {
        let err = LogError::Serialization("invalid json".to_string());
        assert_eq!(err.to_string(), "Serialization error: invalid json");
    }

    #[test]
    fn test_log_error_debug() {
        let err = LogError::Io("test".to_string());
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("Io"));
    }

    #[test]
    fn test_log_error_clone() {
        let err = LogError::Io("test".to_string());
        let cloned = err.clone();
        assert_eq!(cloned.to_string(), err.to_string());
    }

    #[test]
    fn test_log_error_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<LogError>();
    }
}
