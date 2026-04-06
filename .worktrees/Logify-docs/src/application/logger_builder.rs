//! Logger Builder

use crate::domain::{Level, LogEntry, LogError, Logger};
use async_trait::async_trait;

pub struct LoggerBuilder {
    name: String,
    level: Level,
}

impl LoggerBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), level: Level::Info }
    }

    pub fn level(mut self, level: Level) -> Self {
        self.level = level;
        self
    }

    pub fn build(self) -> impl Logger {
        ConsoleLogger { name: self.name, level: self.level }
    }
}

pub struct ConsoleLogger {
    name: String,
    level: Level,
}

#[async_trait]
impl Logger for ConsoleLogger {
    async fn log(&self, entry: LogEntry) -> Result<(), LogError> {
        if entry.level >= self.level {
            println!("[{}] {}: {}", entry.level, self.name, entry.message);
        }
        Ok(())
    }

    fn level(&self) -> Level {
        self.level
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger_builder_new() {
        let builder = LoggerBuilder::new("test-logger");
        assert_eq!(builder.name, "test-logger");
        assert_eq!(builder.level, Level::Info);
    }

    #[test]
    fn test_logger_builder_with_level() {
        let builder = LoggerBuilder::new("test").level(Level::Debug);
        assert_eq!(builder.level, Level::Debug);
    }

    #[test]
    fn test_logger_builder_level_chaining() {
        let logger = LoggerBuilder::new("test").level(Level::Warn).build();
        assert_eq!(logger.level(), Level::Warn);
    }

    #[test]
    fn test_console_logger_name() {
        let logger = ConsoleLogger { name: "test".to_string(), level: Level::Debug };
        assert_eq!(logger.level(), Level::Debug);
    }
}
