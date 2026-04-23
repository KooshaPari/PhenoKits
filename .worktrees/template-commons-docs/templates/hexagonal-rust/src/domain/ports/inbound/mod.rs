//! # Inbound Ports (Primary/Driving Ports)
//!
//! These ports define the **use cases** that the application exposes.
//! Inbound adapters (UI, API, CLI) call these ports to trigger business logic.
//!
//! ## Example
//!
//! ```rust
//! use crate::domain::ports::inbound::CreateOrderPort;
//!
//! // Application use case implements the port
//! impl CreateOrderPort for CreateOrderUseCase {
//!     fn create_order(&self, command: CreateOrderCommand) -> Result<Order, OrderError> {
//!         // Business logic here
//!     }
//! }
//! ```

// Add inbound port traits here
// Example: use_cases/mod.rs would define traits like CreateOrderPort
