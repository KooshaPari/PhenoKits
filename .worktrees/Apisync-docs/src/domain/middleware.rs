//! Middleware traits and implementations

use async_trait::async_trait;
use crate::domain::{Request, Response};

/// Middleware trait
#[async_trait]
pub trait Middleware: Send + Sync {
    async fn handle(&self, request: Request, next: Next) -> Response;
}

/// Next handler in middleware chain
pub struct Next<F>
where
    F: Fn(Request) -> std::pin::Pin<Box<dyn std::future::Future<Output = Response> + Send>> + Send + Sync,
{
    handler: F,
}

impl<F> Next<F>
where
    F: Fn(Request) -> std::pin::Pin<Box<dyn std::future::Future<Output = Response> + Send>> + Send + Sync,
{
    pub fn new(handler: F) -> Self {
        Self { handler }
    }
}

impl<F> Next<F>
where
    F: Fn(Request) -> std::pin::Pin<Box<dyn std::future::Future<Output = Response> + Send>> + Send + Sync,
{
    pub async fn run(&self, request: Request) -> Response {
        (self.handler)(request).await
    }
}
