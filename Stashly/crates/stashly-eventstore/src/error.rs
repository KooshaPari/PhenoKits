//! Re-export canonical error types from phenotype-errors.
//! Provides type aliases for event sourcing operations.

pub use phenotype_errors::{PhenoError, Result};

/// Event sourcing result type (alias for convenience).
pub type EventSourcingResult<T> = Result<T>;
