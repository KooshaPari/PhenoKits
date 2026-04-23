//! Main library entry point for apikit

pub mod domain;
pub mod application;
pub mod adapters;
pub mod infrastructure;

pub use domain::*;
pub use application::*;
pub use adapters::*;
pub use infrastructure::*;

/// Main library entry point (placeholder)
pub struct ApiKit;

impl ApiKit {
    pub fn new() -> Self { ApiKit }
}
