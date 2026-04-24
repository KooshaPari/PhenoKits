//! Domain Errors

use thiserror::Error;

#[derive(Debug, Error)]
pub enum MetricError {
    #[error("Metric already registered: {0}")]
    AlreadyRegistered(String),

    #[error("Metric not found: {0}")]
    NotFound(String),

    #[error("Export error: {0}")]
    Export(String),
}

pub type MetricResult<T> = Result<T, MetricError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_already_registered_error() {
        let err = MetricError::AlreadyRegistered("test_metric".into());
        assert_eq!(err.to_string(), "Metric already registered: test_metric");
        assert!(matches!(err, MetricError::AlreadyRegistered(_)));
    }

    #[test]
    fn test_not_found_error() {
        let err = MetricError::NotFound("missing_metric".into());
        assert_eq!(err.to_string(), "Metric not found: missing_metric");
        assert!(matches!(err, MetricError::NotFound(_)));
    }

    #[test]
    fn test_export_error() {
        let err = MetricError::Export("connection failed".into());
        assert_eq!(err.to_string(), "Export error: connection failed");
        assert!(matches!(err, MetricError::Export(_)));
    }

    #[test]
    fn test_error_debug() {
        let err = MetricError::AlreadyRegistered("test".into());
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("AlreadyRegistered"));
    }

    #[test]
    fn test_error_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<MetricError>();
        assert_send_sync::<MetricResult<i32>>();
    }
}
