//! Domain layer - pure configuration logic.

pub mod config;
pub mod layers;
pub mod sources;
pub mod validation;
pub mod ports;
pub mod errors;

// Re-exports
pub use config::{Config, ConfigValue, ConfigPath};
pub use layers::{Layer, LayerPriority, MergeStrategy};
pub use sources::Source;
pub use validation::Validator;
pub use errors::ConfigError;
