//! Sinks

use crate::domain::{Level, LogEntry, LogError};
use async_trait::async_trait;

#[async_trait]
pub trait Sink: Send + Sync {
    async fn write(&self, entry: &LogEntry) -> Result<(), LogError>;
    async fn flush(&self) -> Result<(), LogError>;
}

/// Console sink
#[derive(Clone, Default)]
pub struct ConsoleSink;

impl ConsoleSink {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Sink for ConsoleSink {
    async fn write(&self, entry: &LogEntry) -> Result<(), LogError> {
        if entry.level >= Level::Error {
            eprintln!("{} [{}]", entry.level, entry.message);
        } else {
            println!("{} [{}]", entry.level, entry.message);
        }
        Ok(())
    }

    async fn flush(&self) -> Result<(), LogError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockSink;

    #[async_trait]
    impl Sink for MockSink {
        async fn write(&self, _entry: &LogEntry) -> Result<(), LogError> {
            Ok(())
        }

        async fn flush(&self) -> Result<(), LogError> {
            Ok(())
        }
    }

    #[test]
    fn test_console_sink_new() {
        let _sink = ConsoleSink::new();
    }

    #[test]
    fn test_console_sink_default() {
        let _sink = ConsoleSink::default();
    }

    #[test]
    fn test_sink_trait_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<ConsoleSink>();
        // For trait objects, we verify at runtime
        fn _check_boxed_sink(sink: &dyn Sink) {
            //dyn Sink is Send + Sync by trait definition
        }
        _check_boxed_sink(&ConsoleSink::new());
    }

    #[tokio::test]
    async fn test_console_sink_write_error() {
        let sink = ConsoleSink::new();
        let entry = LogEntry::new(Level::Error, "test error");
        let result = sink.write(&entry).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_console_sink_write_info() {
        let sink = ConsoleSink::new();
        let entry = LogEntry::new(Level::Info, "test info");
        let result = sink.write(&entry).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_console_sink_flush() {
        let sink = ConsoleSink::new();
        let result = sink.flush().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_mock_sink_write() {
        let sink = MockSink;
        let entry = LogEntry::new(Level::Debug, "debug message");
        let result = sink.write(&entry).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_mock_sink_flush() {
        let sink = MockSink;
        let result = sink.flush().await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_console_sink_clone() {
        let sink = ConsoleSink::new();
        let _cloned = sink.clone();
    }
}
