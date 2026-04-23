//! # Data Transfer Objects
//!
//! DTOs are plain data structures used for transferring data between layers.
//!
//! ## Rules
//!
//! - DTOs should be serializable (for API endpoints)
//! - DTOs should not contain business logic
//! - DTOs may differ from domain entities (different granularity, naming)
//!
//! ## Example
//!
//! ```rust
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Debug, Clone, Serialize, Deserialize)]
//! pub struct CreateOrderDto {
//!     pub customer_id: String,
//!     pub items: Vec<OrderItemDto>,
//! }
//!
//! #[derive(Debug, Clone, Serialize, Deserialize)]
//! pub struct OrderDto {
//!     pub id: String,
//!     pub customer_id: String,
//!     pub items: Vec<OrderItemDto>,
//!     pub status: String,
//!     pub total: f64,
//! }
//! ```

pub mod order_dto;
