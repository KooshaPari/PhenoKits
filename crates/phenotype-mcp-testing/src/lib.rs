//! Phenotype MCP Testing - Testing framework for MCP servers
//!
//! Provides:
//! - Game process management (mock process spawning)
//! - Test execution and result tracking
//! - Test suite management
//! - Report generation

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod game_manager;
pub mod handler;
pub mod runner;
pub mod report;
pub mod types;

pub use game_manager::*;
pub use handler::*;
pub use runner::*;
pub use report::*;
pub use types::*;

/// Version of this crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
