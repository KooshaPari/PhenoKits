//! Phenotype Skills - Modular skill system for agent orchestration
//!
//! Application: registry, dependency resolution
//! Adapters: storage, loaders, sandboxes
//! Runtime: WASM, Python, C# execution

pub mod adapters;
pub mod application;
pub mod domain;
pub mod ports;
pub mod runtime;

use std::sync::Arc;

pub use domain::{Skill, SkillId, SkillManifest, SkillDependency, Runtime, Version};
pub use application::registry::SkillRegistry;
pub use application::dependency_resolver::DependencyResolver;
pub use runtime::SkillRuntime;

/// Result type for skill operations
pub type Result<T> = std::result::Result<T, SkillsError>;

/// Errors that can occur in the skills system
#[derive(thiserror::Error, Debug)]
pub enum SkillsError {
    #[error("Skill not found: {0}")]
    NotFound(String),

    #[error("Skill already registered: {0}")]
    AlreadyRegistered(String),

    #[error("Invalid skill manifest: {0}")]
    InvalidManifest(String),

    #[error("Dependency error: {0}")]
    DependencyError(String),

    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("Execution error: {0}")]
    ExecutionError(String),

    #[error("Sandbox error: {0}")]
    SandboxError(String),

    #[error("Invalid version: {0}")]
    InvalidVersion(String),

    #[error("Loader error: {0}")]
    LoaderError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),
}

/// Builder for creating configured SkillRegistry instances
pub struct SkillRegistryBuilder;

impl SkillRegistryBuilder {
    pub fn new() -> Self {
        Self
    }

    pub fn build(self) -> SkillRegistry {
        SkillRegistry::new()
    }
}

impl Default for SkillRegistryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = SkillsError::NotFound("test-skill".to_string());
        assert!(format!("{}", error).contains("test-skill"));
    }

    #[test]
    fn test_registry_builder() {
        let registry = SkillRegistryBuilder::new()
            .build();

        assert!(registry.list().is_empty());
    }

    #[test]
    fn test_skills_error_from_impl() {
        // Test that the error types work correctly
        let err = SkillsError::NotFound("test".to_string());
        let formatted = format!("{}", err);
        assert!(formatted.contains("test"));
    }
}
