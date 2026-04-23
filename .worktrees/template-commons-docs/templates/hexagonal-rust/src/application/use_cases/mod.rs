//! # Application Use Cases
//!
//! Use cases orchestrate the domain logic and coordinate between ports.
//!
//! ## Structure
//!
//! Each use case should:
//! 1. Receive input (DTO or Command)
//! 2. Interact with domain services/entities
//! 3. Use outbound ports for persistence/external calls
//! 4. Return result (DTO or Entity)
//!
//! ## Example
//!
//! ```rust
//! use crate::domain::entities::Order;
//! use crate::domain::ports::outbound::OrderRepository;
//! use crate::application::dto::CreateOrderDto;
//! use thiserror::Error;
//!
//! #[derive(Debug, Error)]
//! pub enum UseCaseError {
//!     #[error("Repository error: {0}")]
//!     Repository(#[from] RepositoryError),
//!     #[error("Validation error: {0}")]
//!     Validation(String),
//! }
//!
//! pub struct CreateOrderUseCase<R: OrderRepository> {
//!     repository: R,
//! }
//!
//! impl<R: OrderRepository> CreateOrderUseCase<R> {
//!     pub fn new(repository: R) -> Self {
//!         Self { repository }
//!     }
//!
//!     pub fn execute(&self, dto: CreateOrderDto) -> Result<Order, UseCaseError> {
//!         // 1. Validate input
//!         if dto.items.is_empty() {
//!             return Err(UseCaseError::Validation("Order must have items".into()));
//!         }
//!
//!         // 2. Create domain entity
//!         let mut order = Order::new(dto.customer_id);
//!         for item in dto.items {
//!             order.add_item(item)?;
//!         }
//!
//!         // 3. Persist via outbound port
//!         self.repository.save(&order)?;
//!
//!         // 4. Return result
//!         Ok(order)
//!     }
//! }
//! ```

pub mod create_order;
