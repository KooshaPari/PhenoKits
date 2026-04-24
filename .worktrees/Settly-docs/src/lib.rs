//! Configuration management framework.
//!
//! # Architecture
//!
//! configkit follows hexagonal architecture:
//!
//! - **Domain**: Pure business logic (config entities, layers, validation)
//! - **Application**: Use cases and configuration builder
//! - **Adapters**: File parsers, env sources, validators
//! - **Infrastructure**: Cross-cutting concerns (error handling, logging)
//!
//! # Quick Start
//!
//! ```
//! use configkit::{Config, ConfigBuilder};
//!
//! let config = ConfigBuilder::new()
//!     .with_file("config.toml")?
//!     .with_env()
//!     .build()?;
//! ```

pub mod domain;
pub mod application;
pub mod adapters;
pub mod infrastructure;

// Re-exports
pub use domain::{Config, ConfigValue, Layer, LayerPriority};
pub use domain::errors::ConfigError;
pub use application::builder::ConfigBuilder;
pub use infrastructure::error::ConfigKitError;

/// Framework version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
