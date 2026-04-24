//! # Domain Layer
//!
//! Pure business logic with **ZERO external dependencies**.
//!
//! This layer contains:
//! - [`entities`] - Business objects with identity
//! - [`value_objects`] - Immutable types without identity
//! - [`aggregates`] - Aggregate roots managing consistency boundaries
//! - [`services`] - Domain services for cross-entity logic
//! - [`events`] - Domain events for event-driven patterns
//! - [`ports`] - Port interfaces (traits) for adapters
//!
//! ## Dependency Rule
//!
//! ```text
//! Domain ──NO DEPENDENCIES──► Application/Adapters
//! ```
//!
//! The domain layer is the innermost layer and must NOT depend on any
//! other layer in this crate. All dependencies point inward.

pub mod entities;
pub mod value_objects;
pub mod aggregates;
pub mod services;
pub mod events;
pub mod ports;

// Re-export commonly used types
pub use entities::*;
pub use value_objects::*;
pub use aggregates::*;
pub use services::*;
pub use events::*;
pub use ports::*;
