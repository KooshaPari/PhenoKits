//! Service registry configuration validation

use phenotype_validation::{Result, ValidationResult, Validator};
use serde::{Deserialize, Serialize};

/// Service registration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    /// Service name
    pub name: String,
    /// Service version
    pub version: String,
    /// Host address
    pub host: String,
    /// Port number
    pub port: u16,
    /// Health check path
    pub health_check: Option<String>,
    /// Service metadata
    pub metadata: Option<serde_json::Value>,
}

/// Nexus registry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NexusConfig {
    /// Registry name
    pub name: String,
    /// Max services
    pub max_services: u32,
    /// TTL for service entries (seconds)
    pub ttl_seconds: u64,
    /// Heartbeat interval (seconds)
    pub heartbeat_interval: u64,
}

/// Validates service configuration
pub fn validate_service(config: &ServiceConfig) -> Result<ValidationResult> {
    let validator = Validator::new()
        .required("name")
        .string("name")
        .min_length("name", 1)
        .max_length("name", 64)
        .pattern("name", r"^[a-zA-Z0-9_-]+$")
        .required("version")
        .string("version")
        .pattern("version", r"^\d+\.\d+\.\d+(-.+)?$")
        .required("host")
        .string("host")
        .pattern("host", r"^[\w.-]+$")
        .required("port")
        .integer("port")
        .min(1.0)
        .max(65535.0);

    let json = serde_json::to_value(config).unwrap();
    validator.validate(&json)
}

/// Validates nexus registry configuration
pub fn validate_nexus_config(config: &NexusConfig) -> Result<ValidationResult> {
    let validator = Validator::new()
        .required("name")
        .string("name")
        .min_length("name", 1)
        .required("max_services")
        .integer("max_services")
        .min(1.0)
        .max(10000.0)
        .required("ttl_seconds")
        .integer("ttl_seconds")
        .min(1.0)
        .max(86400.0)
        .required("heartbeat_interval")
        .integer("heartbeat_interval")
        .min(1.0)
        .max(3600.0);

    let json = serde_json::to_value(config).unwrap();
    let mut result = validator.validate(&json)?;

    // Additional: heartbeat should be less than TTL
    if config.heartbeat_interval >= config.ttl_seconds {
        result.add_error(phenotype_validation::ValidationError::constraint(
            "heartbeat_interval",
            "heartbeat_interval must be less than ttl_seconds",
        ));
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_service() {
        let service = ServiceConfig {
            name: "user-service".to_string(),
            version: "1.0.0".to_string(),
            host: "localhost".to_string(),
            port: 8080,
            health_check: Some("/health".to_string()),
            metadata: None,
        };

        let result = validate_service(&service).unwrap();
        assert!(result.is_valid);
    }

    #[test]
    fn test_invalid_service_name() {
        let service = ServiceConfig {
            name: "".to_string(),
            version: "1.0.0".to_string(),
            host: "localhost".to_string(),
            port: 8080,
            health_check: None,
            metadata: None,
        };

        let result = validate_service(&service).unwrap();
        assert!(!result.is_valid);
    }

    #[test]
    fn test_invalid_service_port() {
        let service = ServiceConfig {
            name: "test".to_string(),
            version: "1.0.0".to_string(),
            host: "localhost".to_string(),
            port: 70000,
            health_check: None,
            metadata: None,
        };

        let result = validate_service(&service).unwrap();
        assert!(!result.is_valid);
    }

    #[test]
    fn test_valid_nexus_config() {
        let config = NexusConfig {
            name: "production-registry".to_string(),
            max_services: 1000,
            ttl_seconds: 300,
            heartbeat_interval: 30,
        };

        let result = validate_nexus_config(&config).unwrap();
        assert!(result.is_valid);
    }

    #[test]
    fn test_heartbeat_too_long() {
        let config = NexusConfig {
            name: "test".to_string(),
            max_services: 100,
            ttl_seconds: 30,
            heartbeat_interval: 60, // > TTL
        };

        let result = validate_nexus_config(&config).unwrap();
        assert!(!result.is_valid);
    }
}
