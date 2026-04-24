//! REST adapter skeleton

use crate::domain::{Request, Response, Endpoint};
use async_trait::async_trait;

pub mod hyper_server;

pub use hyper_server::HyperServer;

