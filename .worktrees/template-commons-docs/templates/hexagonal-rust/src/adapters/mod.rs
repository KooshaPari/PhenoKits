//! # Adapters Layer
//!
//! Infrastructure adapters implement outbound ports and provide inbound entry points.
//!
//! ## Adapter Types
//!
//! ### Inbound Adapters (Primary)
//! - `http/` - REST/gRPC controllers
//! - `cli/` - Command-line interfaces
//!
//! ### Outbound Adapters (Secondary)
//! - `persistence/` - Database implementations
//! - `cache/` - Cache implementations
//! - `external/` - External API clients
//!
//! ## Dependency Rule
//!
//! ```text
//! Inbound Adapter ──calls──► Application Use Cases
//! Outbound Adapter ──implements──► Domain Outbound Ports
//! ```

#[cfg(feature = "persistence-sqlx")]
pub mod persistence_sqlx;

#[cfg(feature = "cache-redis")]
pub mod cache_redis;

pub mod inbound;
pub mod outbound;
