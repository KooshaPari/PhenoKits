//! # Application Layer
//!
//! Orchestrates domain logic and coordinates between inbound and outbound ports.
//!
//! This layer contains:
//! - [`use_cases`] - Application use cases
//! - [`dto`] - Data Transfer Objects
//! - [`commands`] - Command handlers (write operations)
//! - [`queries`] - Query handlers (read operations)
//!
//! ## Dependency Rule
//!
//! ```text
//! Application ──depends on──► Domain
//! Application ──uses ports──► Outbound Ports
//! ```

pub mod use_cases;
pub mod dto;
pub mod commands;
pub mod queries;

// Re-exports
pub use use_cases::*;
pub use dto::*;
pub use commands::*;
pub use queries::*;
