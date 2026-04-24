//! Configuration validation for Forge
//!
//! This module provides validation for Forge's TOML configuration files.

use serde::{Deserialize, Serialize};

/// Validation error messages
const ERR_NAME_REQUIRED: &str = "name is required";
const ERR_VERSION_REQUIRED: &str = "version is required";
const ERR_VERSION_FORMAT: &str = "version must follow semantic versioning (e.g., 1.0.0)";
const ERR_TASKS_MIN: &str = "at least one task is required";
const ERR_COMMAND_REQUIRED: &str = "command is required";

/// Validation result for config validation
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<String>,
}

impl ValidationResult {
    /// Create a passed validation result
    pub fn passed() -> Self {
        Self {
            valid: true,
            errors: Vec::new(),
        }
    }

    /// Create a failed validation result
    pub fn failed(errors: Vec<String>) -> Self {
        Self {
            valid: false,
            errors,
        }
    }
}

/// Validation error type
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub message: String,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ValidationError {}

/// Forge configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeConfig {
    /// Project name
    pub name: String,
    /// Project version
    pub version: String,
    /// Task definitions
    pub tasks: Vec<Task>,
    /// Global configuration
    pub globals: Option<GlobalConfig>,
}

/// Task definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// Task name
    pub name: String,
    /// Task command
    pub command: String,
    /// Task dependencies
    #[serde(default)]
    pub deps: Vec<String>,
    /// Task description
    pub description: Option<String>,
    /// Working directory
    pub working_dir: Option<String>,
    /// Environment variables
    pub env: Option<std::collections::HashMap<String, String>>,
}

/// Global configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalConfig {
    /// Default shell
    pub shell: Option<String>,
    /// Environment variables
    pub env: Option<std::collections::HashMap<String, String>>,
    /// Timeout for tasks
    pub timeout: Option<u64>,
}

/// Validates a Forge configuration
pub fn validate_config(config: &ForgeConfig) -> Result<ValidationResult, ValidationError> {
    let mut errors = Vec::new();

    // Validate name
    if config.name.is_empty() {
        errors.push(ERR_NAME_REQUIRED.to_string());
    }

    // Validate version (basic semver check)
    if config.version.is_empty() {
        errors.push(ERR_VERSION_REQUIRED.to_string());
    } else if !config
        .version
        .chars()
        .next()
        .map(|c| c.is_ascii_digit())
        .unwrap_or(false)
    {
        errors.push(ERR_VERSION_FORMAT.to_string());
    }

    // Validate tasks
    if config.tasks.is_empty() {
        errors.push(ERR_TASKS_MIN.to_string());
    }

    if errors.is_empty() {
        Ok(ValidationResult::passed())
    } else {
        Ok(ValidationResult::failed(errors))
    }
}

/// Validates a single task definition
pub fn validate_task(task: &Task) -> Result<ValidationResult, ValidationError> {
    let mut errors = Vec::new();

    // Validate name
    if task.name.is_empty() {
        errors.push(ERR_COMMAND_REQUIRED.to_string());
    }

    // Validate command
    if task.command.is_empty() {
        errors.push(ERR_COMMAND_REQUIRED.to_string());
    }

    if errors.is_empty() {
        Ok(ValidationResult::passed())
    } else {
        Ok(ValidationResult::failed(errors))
    }
}

/// Loads and validates a TOML configuration file
pub fn load_and_validate(
    path: &std::path::Path,
) -> Result<ForgeConfig, Box<dyn std::error::Error>> {
    // Load TOML
    let content = std::fs::read_to_string(path)?;
    let config: ForgeConfig = toml::from_str(&content)?;

    // Validate
    let result = validate_config(&config)?;

    if !result.valid {
        let errors = result.errors.to_vec();
        return Err(format!("Configuration validation failed:\n{}", errors.join("\n")).into());
    }

    // Validate each task
    for task in &config.tasks {
        let task_result = validate_task(task)?;
        if !task_result.valid {
            let errors: Vec<String> = task_result
                .errors
                .iter()
                .map(|e| format!("Task '{}': {}", task.name, e))
                .collect();
            return Err(format!("Task validation failed:\n{}", errors.join("\n")).into());
        }
    }

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_config() {
        let config = ForgeConfig {
            name: "my-project".to_string(),
            version: "1.0.0".to_string(),
            tasks: vec![Task {
                name: "build".to_string(),
                command: "cargo build".to_string(),
                deps: vec![],
                description: None,
                working_dir: None,
                env: None,
            }],
            globals: None,
        };

        let result = validate_config(&config).unwrap();
        assert!(
            result.valid,
            "Valid config should pass: {:?}",
            result.errors
        );
    }

    #[test]
    fn test_missing_name() {
        let config = ForgeConfig {
            name: "".to_string(),
            version: "1.0.0".to_string(),
            tasks: vec![],
            globals: None,
        };

        let result = validate_config(&config).unwrap();
        assert!(!result.valid, "Config with empty name should fail");
    }

    #[test]
    fn test_invalid_version() {
        let config = ForgeConfig {
            name: "test".to_string(),
            version: "not-a-version".to_string(),
            tasks: vec![Task {
                name: "build".to_string(),
                command: "cargo build".to_string(),
                deps: vec![],
                description: None,
                working_dir: None,
                env: None,
            }],
            globals: None,
        };

        let result = validate_config(&config).unwrap();
        assert!(!result.valid, "Config with invalid version should fail");
    }

    #[test]
    fn test_missing_tasks() {
        let config = ForgeConfig {
            name: "test".to_string(),
            version: "1.0.0".to_string(),
            tasks: vec![],
            globals: None,
        };

        let result = validate_config(&config).unwrap();
        assert!(!result.valid, "Config without tasks should fail");
    }

    #[test]
    fn test_invalid_task() {
        let task = Task {
            name: "".to_string(),
            command: "echo test".to_string(),
            deps: vec![],
            description: None,
            working_dir: None,
            env: None,
        };

        let result = validate_task(&task).unwrap();
        assert!(!result.valid, "Task with empty name should fail");
    }
}
