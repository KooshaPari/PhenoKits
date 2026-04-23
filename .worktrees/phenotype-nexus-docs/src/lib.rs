//! Phenotype library
#![doc = include_str!("../README.md")]

pub mod core {
    //! Core functionality
}

pub mod registry {
    //! Service registry and discovery.
    //! Traces to: E1.1, E1.2, E1.3, E2.1, E3.1, E4.1

    use dashmap::DashMap;
    use serde::{Deserialize, Serialize};
    use std::sync::Arc;
    use thiserror::Error;

    /// A registered service instance.
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub struct ServiceInstance {
        /// Service name (logical identifier)
        pub name: String,
        /// Network address (host:port)
        pub address: String,
        /// Key/value metadata tags
        pub tags: std::collections::HashMap<String, String>,
        /// Whether the instance is considered healthy
        pub healthy: bool,
    }

    impl ServiceInstance {
        /// Create a new healthy service instance.
        pub fn new(name: impl Into<String>, address: impl Into<String>) -> Self {
            Self {
                name: name.into(),
                address: address.into(),
                tags: Default::default(),
                healthy: true,
            }
        }

        /// Attach a key/value tag. Returns self for chaining.
        pub fn with_tag(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
            self.tags.insert(key.into(), value.into());
            self
        }

        /// Mark instance healthy or unhealthy.
        pub fn set_healthy(&mut self, healthy: bool) {
            self.healthy = healthy;
        }
    }

    /// Errors produced by the registry.
    #[derive(Debug, Error, PartialEq)]
    pub enum RegistryError {
        #[error("service not found: {0}")]
        NotFound(String),
    }

    /// In-memory service registry backed by DashMap for concurrent access.
    #[derive(Clone, Default)]
    pub struct ServiceRegistry {
        inner: Arc<DashMap<String, ServiceInstance>>,
        /// Round-robin counter per service name
        round_robin: Arc<DashMap<String, std::sync::atomic::AtomicUsize>>,
    }

    impl ServiceRegistry {
        /// Create an empty registry.
        pub fn new() -> Self {
            Self::default()
        }

        /// Register (or replace) a service instance.
        pub fn register(&self, svc: ServiceInstance) {
            self.inner.insert(svc.name.clone(), svc);
        }

        /// Register a service instance with multiple instances support.
        /// Each call adds a new instance to the service cluster.
        pub fn register_instance(&self, svc: ServiceInstance) {
            let key = svc.name.clone();
            if let Some(mut existing) = self.inner.get_mut(&key) {
                // Append to existing instances
                let instances = format!("{}:{}", existing.address, svc.address);
                existing.address = instances;
                // Merge tags
                for (k, v) in svc.tags {
                    existing.tags.insert(k, v);
                }
            } else {
                self.inner.insert(key, svc);
            }
        }

        /// Deregister a service by name.
        pub fn deregister(&self, name: &str) -> Result<(), RegistryError> {
            self.round_robin.remove(name);
            self.inner
                .remove(name)
                .map(|_| ())
                .ok_or_else(|| RegistryError::NotFound(name.to_owned()))
        }

        /// Discover all healthy instances of a named service.
        pub fn discover(&self, name: &str) -> Vec<ServiceInstance> {
            self.inner
                .iter()
                .filter(|e| e.key() == name && e.value().healthy)
                .map(|e| e.value().clone())
                .collect()
        }

        /// Discover the next healthy instance using round-robin.
        /// Returns None if no healthy instances exist.
        pub fn discover_next(&self, name: &str) -> Option<ServiceInstance> {
            let instances = self.discover(name);
            if instances.is_empty() {
                return None;
            }
            let count = instances.len();
            let counter = self.round_robin.entry(name.to_owned()).or_insert_with(|| {
                std::sync::atomic::AtomicUsize::new(0)
            });
            let idx = counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed) % count;
            instances.into_iter().nth(idx)
        }

        /// Discover healthy instances that match a tag key/value.
        pub fn discover_by_tag(&self, key: &str, value: &str) -> Vec<ServiceInstance> {
            self.inner
                .iter()
                .filter(|e| {
                    e.value().healthy
                        && e.value().tags.get(key).map(|v| v == value).unwrap_or(false)
                })
                .map(|e| e.value().clone())
                .collect()
        }

        /// Mark a registered service healthy or unhealthy.
        pub fn set_health(&self, name: &str, healthy: bool) -> Result<(), RegistryError> {
            match self.inner.get_mut(name) {
                Some(mut e) => {
                    e.value_mut().set_healthy(healthy);
                    Ok(())
                }
                None => Err(RegistryError::NotFound(name.to_owned())),
            }
        }

        /// Return total count of registered services (healthy and unhealthy).
        pub fn len(&self) -> usize {
            self.inner.len()
        }

        /// Return true if registry is empty.
        pub fn is_empty(&self) -> bool {
            self.inner.is_empty()
        }

        /// List all registered service names.
        pub fn service_names(&self) -> Vec<String> {
            self.inner.iter().map(|e| e.key().clone()).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
