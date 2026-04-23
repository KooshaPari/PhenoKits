//! phenotype-patch - Unified diff parsing and application library
//!
//! Provides parsing, application, creation, and merging of unified diffs.

pub mod apply;
pub mod create;
pub mod merge;
pub mod parse;

use thiserror::Error;

pub use apply::{apply_patch, ApplyError};
pub use parse::{parse_diff, Hunk, ParseError};

/// Represents a parsed unified diff
#[derive(Debug, Clone)]
pub struct Diff {
    pub hunks: Vec<Hunk>,
}

/// Result type for diff creation
pub type CreateResult<T = Diff> = Result<T, CreateError>;

/// Error types for diff creation
#[derive(Debug, Error)]
pub enum CreateError {
    #[error("empty content provided")]
    EmptyContent,
    #[error("diff creation failed: {0}")]
    CreationFailed(String),
}
