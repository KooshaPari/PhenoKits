//! # Outbound Ports (Secondary/Driven Ports)
//!
//! These ports define the **capabilities** that infrastructure must provide.
//! The domain defines these traits; adapters implement them.
//!
//! ## Common Outbound Ports
//!
//! | Port | Purpose |
//! |------|---------|
//! | `Repository` | Persist and retrieve entities |
//! | `EventPublisher` | Publish domain events |
//! | `ExternalService` | Call external APIs |
//! | `CacheProvider` | Cache data for performance |
//!
//! ## Example
//!
//! ```rust
//! use crate::domain::ports::outbound::OrderRepository;
//! use crate::domain::entities::Order;
//! use crate::domain::value_objects::OrderId;
//!
//! // Outbound port trait (defined by domain)
//! pub trait OrderRepository: Send + Sync {
//!     fn save(&self, order: &Order) -> Result<(), RepositoryError>;
//!     fn find_by_id(&self, id: &OrderId) -> Result<Option<Order>, RepositoryError>;
//! }
//!
//! // Adapter implements the port (in adapters crate)
//! pub struct SqlxOrderRepository { pool: PgPool }
//!
//! impl OrderRepository for SqlxOrderRepository {
//!     fn save(&self, order: &Order) -> Result<(), RepositoryError> { ... }
//!     fn find_by_id(&self, id: &OrderId) -> Result<Option<Order>, RepositoryError> { ... }
//! }
//! ```
//!
//! ## Dependency Rule
//!
//! ```text
//! Domain defines port ──implemented by──► Adapter
//! ```

// Add outbound port traits here
