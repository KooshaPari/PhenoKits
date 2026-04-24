//! Integration plugin system for phenotype ecosystem.
//!
//! Provides WASM-based plugin architecture for:
//! - Ticketing integrations (Linear, GitHub Projects, Jira)
//! - CI/CD integrations (GitHub Actions, GitLab CI)
//! - Monitoring integrations (Prometheus, Datadog)
//! - Memory/Knowledge integrations (Cognee, Chunkhound)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Plugin errors
#[derive(Error, Debug)]
pub enum PluginError {
    #[error("Plugin not found: {0}")]
    NotFound(String),
    #[error("Plugin initialization failed: {0}")]
    InitFailed(String),
    #[error("Plugin execution failed: {0}")]
    ExecutionFailed(String),
    #[error("Plugin not available: {0}")]
    NotAvailable(String),
}

pub type PluginResult<T> = Result<T, PluginError>;

/// Plugin metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub category: PluginCategory,
    pub capabilities: Vec<String>,
}

/// Plugin categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PluginCategory {
    Ticketing,
    CICD,
    Monitoring,
    Memory,
    Governance,
    Custom,
}

/// Plugin trait for integration plugins
pub trait IntegrationPlugin: Send + Sync {
    /// Get plugin metadata
    fn metadata(&self) -> PluginMetadata;

    /// Check if plugin is available (credentials configured, etc.)
    fn check_available(&self) -> bool;

    /// Initialize the plugin
    fn init(&self) -> PluginResult<()>;

    /// Execute an operation
    fn execute(&self, operation: &str, params: serde_json::Value) -> PluginResult<serde_json::Value>;
}

/// Ticketing integration trait
pub trait TicketingIntegration: IntegrationPlugin {
    /// Sync items to ticketing system
    fn sync_to(&self, items: Vec<serde_json::Value>) -> PluginResult<SyncResult>;

    /// Fetch items from ticketing system
    fn sync_from(&self, query: Option<&str>) -> PluginResult<Vec<serde_json::Value>>;
}

/// CI/CD integration trait
pub trait CICDIntegration: IntegrationPlugin {
    /// Trigger a workflow
    fn trigger_workflow(&self, workflow: &str, params: serde_json::Value) -> PluginResult<String>;

    /// Get workflow status
    fn get_workflow_status(&self, run_id: &str) -> PluginResult<WorkflowStatus>;
}

/// Monitoring integration trait
pub trait MonitoringIntegration: IntegrationPlugin {
    /// Record a metric
    fn record_metric(&self, name: &str, value: f64, labels: HashMap<String, String>) -> PluginResult<()>;

    /// Query metrics
    fn query_metrics(&self, query: &str) -> PluginResult<Vec<serde_json::Value>>;
}

/// Sync result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResult {
    pub synced: usize,
    pub failed: usize,
    pub errors: Vec<String>,
}

/// Workflow status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStatus {
    pub run_id: String,
    pub status: String,  // pending, running, success, failure
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
}

/// Plugin registry
pub struct PluginRegistry {
    plugins: HashMap<String, Box<dyn IntegrationPlugin>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }

    pub fn register(&mut self, plugin: Box<dyn IntegrationPlugin>) -> PluginResult<()> {
        let metadata = plugin.metadata();
        self.plugins.insert(metadata.name.clone(), plugin);
        Ok(())
    }

    pub fn get(&self, name: &str) -> PluginResult<&dyn IntegrationPlugin> {
        self.plugins
            .get(name)
            .map(|p| p.as_ref())
            .ok_or_else(|| PluginError::NotFound(name.to_string()))
    }

    pub fn list_by_category(&self, category: PluginCategory) -> Vec<PluginMetadata> {
        self.plugins
            .values()
            .filter(|p| p.metadata().category == category)
            .map(|p| p.metadata())
            .collect()
    }

    pub fn list_all(&self) -> Vec<PluginMetadata> {
        self.plugins
            .values()
            .map(|p| p.metadata())
            .collect()
    }
}
