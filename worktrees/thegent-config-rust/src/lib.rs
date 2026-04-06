//! Configuration validation for thegent in Rust
//!
//! Provides fast configuration parsing and validation with PyO3 bindings.

use pyo3::prelude::*;
use pyo3::types::PyDict;
use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Invalid path: {0}")]
    InvalidPath(String),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, ConfigError>;

/// Configuration for thegent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GentConfig {
    pub workspace_path: String,
    pub session_dir: String,
    pub retention_days: u32,
    pub max_concurrent: u32,
}

impl Default for GentConfig {
    fn default() -> Self {
        Self {
            workspace_path: ".".to_string(),
            session_dir: ".thegent/sessions".to_string(),
            retention_days: 30,
            max_concurrent: 10,
        }
    }
}

impl GentConfig {
    /// Validate the configuration
    pub fn validate(&self) -> Result<()> {
        // Validate workspace path
        let path = Path::new(&self.workspace_path);
        if !path.exists() {
            return Err(ConfigError::InvalidPath(format!(
                "Workspace path does not exist: {}",
                self.workspace_path
            )));
        }

        // Validate retention days
        if self.retention_days == 0 {
            return Err(ConfigError::Validation(
                "Retention days must be greater than 0".to_string(),
            ));
        }

        // Validate max concurrent
        if self.max_concurrent == 0 {
            return Err(ConfigError::Validation(
                "Max concurrent must be greater than 0".to_string(),
            ));
        }

        Ok(())
    }

    /// Parse from JSON string
    pub fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str(json)
            .map_err(|e| ConfigError::Parse(e.to_string()))
    }

    /// Convert to JSON string
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| ConfigError::Serialize(e.to_string()))
    }
}

/// Validation helpers
pub mod validation {
    use super::*;

    /// Validate a file path
    pub fn validate_path(path: &str) -> Result<PathBuf> {
        let path = Path::new(path);
        if !path.exists() {
            return Err(ConfigError::InvalidPath(format!(
                "Path does not exist: {:?}",
                path
            )));
        }
        Ok(path.to_path_buf())
    }

    /// Validate a port number
    pub fn validate_port(port: u16) -> Result<u16> {
        if port == 0 {
            return Err(ConfigError::Validation(
                "Port cannot be 0".to_string(),
            ));
        }
        Ok(port)
    }

    /// Validate a URL
    pub fn validate_url(url: &str) -> Result<String> {
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(ConfigError::Validation(
                "URL must start with http:// or https://".to_string(),
            ));
        }
        Ok(url.to_string())
    }
}

#[cfg(feature = "python")]
mod python {
    use super::*;
    use pyo3::wrap_pyfunction;

    /// Python bindings
    #[pymodule]
    pub fn gent_config(_py: Python, m: &PyModule) -> PyResult<()> {
        m.add_class::<GentConfig>()?;
        m.add_function(wrap_pyfunction!(validate_config, m)?)?;
        Ok(())
    }

    #[pyfunction]
    fn validate_config(config_json: &str) -> PyResult<String> {
        let config = GentConfig::from_json(config_json)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        config.validate()
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        config.to_json()
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }
}
