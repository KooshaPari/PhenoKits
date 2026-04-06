//! Caching framework.
//!
//! # Architecture
//!
//! cachekit follows hexagonal architecture:
//!
//! - **Domain**: Pure business logic (cache entities, policies)
//! - **Application**: Use cases and cache service
//! - **Adapters**: Backend implementations
//! - **Infrastructure**: Cross-cutting concerns

pub mod domain;
pub mod application;
pub mod adapters;
pub mod infrastructure;

// Re-exports
pub use domain::{Cache, CacheKey, CacheValue, Entry};
pub use domain::errors::CacheError;
pub use adapters::memory::InMemoryCache;
pub use infrastructure::error::CacheKitError;

/// Framework version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
