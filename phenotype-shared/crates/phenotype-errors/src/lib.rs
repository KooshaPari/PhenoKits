//! Canonical error types for Phenotype collections.
//!
//! Consolidates common error families across Sidekick, Eidolon, Stashly, PhenoObservability, and Paginary.
//! Reduces boilerplate and unifies error contracts across collections.

use thiserror::Error;

/// Canonical result type for Phenotype operations.
pub type Result<T> = std::result::Result<T, PhenoError>;

/// Canonical error type for Phenotype collections.
#[derive(Error, Debug, Clone)]
pub enum PhenoError {
    /// Resource not found.
    #[error("Not found: {0}")]
    NotFound(String),

    /// Resource already exists.
    #[error("Already exists: {0}")]
    AlreadyExists(String),

    /// Conflict (e.g., concurrent modification, duplicate key).
    #[error("Conflict: {0}")]
    Conflict(String),

    /// Validation error (invalid input, constraint violation).
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// Backend unavailable (network, database, service down).
    #[error("Backend unavailable: {0}")]
    BackendUnavailable(String),

    /// Operation timed out.
    #[error("Operation timeout: {0}")]
    Timeout(String),

    /// Unauthorized or permission denied.
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    /// Rate limited.
    #[error("Rate limited: {0}")]
    RateLimited(String),

    /// Internal server error (unexpected state, invariant violation).
    #[error("Internal error: {0}")]
    Internal(String),

    /// Unsupported operation.
    #[error("Unsupported: {0}")]
    Unsupported(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = PhenoError::NotFound("user:123".to_string());
        assert_eq!(err.to_string(), "Not found: user:123");
    }

    #[test]
    fn test_result_type() {
        fn example() -> Result<i32> {
            Err(PhenoError::Unauthorized("invalid token".to_string()))
        }
        assert!(example().is_err());
    }

    #[test]
    fn test_clone_and_debug() {
        let err = PhenoError::Conflict("concurrent write".to_string());
        let cloned = err.clone();
        assert_eq!(cloned.to_string(), err.to_string());
    }
}
