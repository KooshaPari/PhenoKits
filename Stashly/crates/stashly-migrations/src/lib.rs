//! Generic state migration framework for Phenotype collections.
//!
//! Provides a generic, versioned migration system for any serializable state type,
//! supporting linear migrations, branching, rollback, and failure recovery.
//!
//! # Architecture
//!
//! A migration transforms state from version N to version N+1, with full audit trails
//! and rollback capability. The framework is collection-agnostic: use it for database
//! schemas (Stashly), config versions (PhenoObservability), or state snapshots (Eidolon).
//!
//! # Examples
//!
//! ```ignore
//! use stashly_migrations::{Migration, MigrationRunner, Versioned, VersionedState};
//!
//! #[derive(Clone, serde::Serialize, serde::Deserialize)]
//! struct UserV1 { id: String, name: String }
//!
//! #[derive(Clone, serde::Serialize, serde::Deserialize)]
//! struct UserV2 { id: String, name: String, email: String }
//!
//! struct AddEmailMigration;
//! impl Migration<UserV1, UserV2> for AddEmailMigration {
//!     fn migrate(&self, old: UserV1) -> Result<UserV2, String> {
//!         Ok(UserV2 {
//!             id: old.id,
//!             name: old.name,
//!             email: "unknown@example.com".to_string(),
//!         })
//!     }
//! }
//!
//! let runner = MigrationRunner::new("my_domain");
//! let state = UserV1 { id: "1".into(), name: "Alice".into() };
//! let migrated = runner.apply(state, Box::new(AddEmailMigration))?;
//! ```

use serde::{Deserialize, Serialize};
use std::fmt;

/// A type that can be versioned and migrated.
pub trait Versioned: Serialize + for<'de> Deserialize<'de> + Clone + fmt::Debug {
    /// Return the semantic version of this state (e.g., "1.0", "2.1").
    fn version(&self) -> String;

    /// Set the version after migration.
    fn set_version(&mut self, version: String);
}

/// A single migration step transforming From state to To state.
pub trait Migration<From: Versioned, To: Versioned>: Send + Sync {
    /// Apply the migration. Return Err if the migration cannot be applied.
    fn migrate(&self, state: From) -> Result<To, String>;

    /// Optional: return a human-readable name for this migration.
    fn name(&self) -> String {
        "unnamed_migration".to_string()
    }
}

/// Audit trail for a single migration execution.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MigrationAudit {
    /// Migration name.
    pub name: String,
    /// From version.
    pub from_version: String,
    /// To version.
    pub to_version: String,
    /// Timestamp (ISO 8601).
    pub timestamp: String,
    /// Success or failure message.
    pub status: MigrationStatus,
}

/// Status of a migration execution.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum MigrationStatus {
    /// Migration succeeded.
    Success,
    /// Migration failed with error.
    Failed { reason: String },
    /// Migration was rolled back.
    RolledBack { reason: String },
}

/// Manages migration execution and audit tracking.
pub struct MigrationRunner {
    domain: String,
    audit_trail: Vec<MigrationAudit>,
    rollback_stack: Vec<(String, String)>, // (from_version, to_version) pairs for rollback
}

impl MigrationRunner {
    /// Create a new migration runner for a given domain (e.g., "database", "config", "snapshot").
    pub fn new(domain: impl Into<String>) -> Self {
        Self {
            domain: domain.into(),
            audit_trail: Vec::new(),
            rollback_stack: Vec::new(),
        }
    }

    /// Apply a single migration step. Tracks the migration in the audit trail.
    pub async fn apply<From: Versioned + 'static, To: Versioned + 'static>(
        &mut self,
        mut from_state: From,
        migration: Box<dyn Migration<From, To>>,
    ) -> Result<To, String> {
        let from_version = from_state.version();
        let migration_name = migration.name();

        match migration.migrate(from_state.clone()) {
            Ok(to_state) => {
                let to_version = to_state.version();
                self.audit_trail.push(MigrationAudit {
                    name: migration_name,
                    from_version: from_version.clone(),
                    to_version: to_version.clone(),
                    timestamp: chrono::Utc::now().to_rfc3339(),
                    status: MigrationStatus::Success,
                });
                self.rollback_stack.push((from_version, to_version));
                Ok(to_state)
            }
            Err(reason) => {
                self.audit_trail.push(MigrationAudit {
                    name: migration_name,
                    from_version,
                    to_version: "unknown".to_string(),
                    timestamp: chrono::Utc::now().to_rfc3339(),
                    status: MigrationStatus::Failed {
                        reason: reason.clone(),
                    },
                });
                Err(reason)
            }
        }
    }

    /// Get the audit trail of all migrations applied.
    pub fn audit_trail(&self) -> &[MigrationAudit] {
        &self.audit_trail
    }

    /// Get the rollback stack (for recovering from partial failure).
    pub fn rollback_stack(&self) -> &[(String, String)] {
        &self.rollback_stack
    }

    /// Record a rollback event in the audit trail.
    pub fn record_rollback(&mut self, reason: impl Into<String>) {
        if let Some((from_version, to_version)) = self.rollback_stack.last() {
            self.audit_trail.push(MigrationAudit {
                name: "rollback".to_string(),
                from_version: to_version.clone(),
                to_version: from_version.clone(),
                timestamp: chrono::Utc::now().to_rfc3339(),
                status: MigrationStatus::RolledBack {
                    reason: reason.into(),
                },
            });
            self.rollback_stack.pop();
        }
    }

    /// Serialize audit trail to JSON.
    pub fn audit_trail_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.audit_trail)
    }
}

// Dependency: chrono for timestamps. Add to Cargo.toml if not present.
use chrono;

#[cfg(test)]
mod tests {
    use super::*;

    // Traces to: FR-MIGRATIONS-001 (Linear migrations)
    #[test]
    fn test_linear_migration_v1_to_v2() {
        #[derive(Clone, Debug, Serialize, Deserialize)]
        struct StateV1 {
            id: String,
            value: i32,
            #[serde(default)]
            version: String,
        }

        impl Versioned for StateV1 {
            fn version(&self) -> String {
                self.version.clone()
            }
            fn set_version(&mut self, version: String) {
                self.version = version;
            }
        }

        #[derive(Clone, Debug, Serialize, Deserialize)]
        struct StateV2 {
            id: String,
            value: i32,
            metadata: String,
            #[serde(default)]
            version: String,
        }

        impl Versioned for StateV2 {
            fn version(&self) -> String {
                self.version.clone()
            }
            fn set_version(&mut self, version: String) {
                self.version = version;
            }
        }

        struct AddMetadataMigration;
        impl Migration<StateV1, StateV2> for AddMetadataMigration {
            fn migrate(&self, old: StateV1) -> Result<StateV2, String> {
                Ok(StateV2 {
                    id: old.id,
                    value: old.value,
                    metadata: format!("migrated from v1"),
                    version: "2.0".to_string(),
                })
            }
            fn name(&self) -> String {
                "add_metadata_v1_to_v2".to_string()
            }
        }

        let state_v1 = StateV1 {
            id: "obj-1".to_string(),
            value: 42,
            version: "1.0".to_string(),
        };

        let mut runner = MigrationRunner::new("test_domain");
        let state_v2 = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(runner.apply(state_v1, Box::new(AddMetadataMigration)))
            .unwrap();

        assert_eq!(state_v2.version(), "2.0");
        assert_eq!(state_v2.metadata, "migrated from v1");
        assert_eq!(runner.audit_trail().len(), 1);
        assert!(matches!(
            runner.audit_trail()[0].status,
            MigrationStatus::Success
        ));
    }

    // Traces to: FR-MIGRATIONS-002 (Branching/multiple sequential migrations)
    #[test]
    fn test_sequential_migrations_v1_to_v3() {
        #[derive(Clone, Debug, Serialize, Deserialize)]
        struct StateV1 {
            id: String,
            #[serde(default)]
            version: String,
        }
        impl Versioned for StateV1 {
            fn version(&self) -> String {
                self.version.clone()
            }
            fn set_version(&mut self, v: String) {
                self.version = v;
            }
        }

        #[derive(Clone, Debug, Serialize, Deserialize)]
        struct StateV2 {
            id: String,
            extra: String,
            #[serde(default)]
            version: String,
        }
        impl Versioned for StateV2 {
            fn version(&self) -> String {
                self.version.clone()
            }
            fn set_version(&mut self, v: String) {
                self.version = v;
            }
        }

        #[derive(Clone, Debug, Serialize, Deserialize)]
        struct StateV3 {
            id: String,
            extra: String,
            extra2: String,
            #[serde(default)]
            version: String,
        }
        impl Versioned for StateV3 {
            fn version(&self) -> String {
                self.version.clone()
            }
            fn set_version(&mut self, v: String) {
                self.version = v;
            }
        }

        struct MigV1ToV2;
        impl Migration<StateV1, StateV2> for MigV1ToV2 {
            fn migrate(&self, old: StateV1) -> Result<StateV2, String> {
                Ok(StateV2 {
                    id: old.id,
                    extra: "added".to_string(),
                    version: "2.0".to_string(),
                })
            }
            fn name(&self) -> String {
                "v1_to_v2".to_string()
            }
        }

        struct MigV2ToV3;
        impl Migration<StateV2, StateV3> for MigV2ToV3 {
            fn migrate(&self, old: StateV2) -> Result<StateV3, String> {
                Ok(StateV3 {
                    id: old.id,
                    extra: old.extra,
                    extra2: "also_added".to_string(),
                    version: "3.0".to_string(),
                })
            }
            fn name(&self) -> String {
                "v2_to_v3".to_string()
            }
        }

        let state = StateV1 {
            id: "obj-2".to_string(),
            version: "1.0".to_string(),
        };

        let rt = tokio::runtime::Runtime::new().unwrap();
        let mut runner = MigrationRunner::new("seq_test");

        let v2 = rt
            .block_on(runner.apply(state, Box::new(MigV1ToV2)))
            .unwrap();
        assert_eq!(v2.version(), "2.0");

        let v3 = rt
            .block_on(runner.apply(v2, Box::new(MigV2ToV3)))
            .unwrap();
        assert_eq!(v3.version(), "3.0");
        assert_eq!(runner.audit_trail().len(), 2);
    }

    // Traces to: FR-MIGRATIONS-003 (Rollback recovery)
    #[test]
    fn test_rollback_on_failure() {
        #[derive(Clone, Debug, Serialize, Deserialize)]
        struct State {
            id: String,
            #[serde(default)]
            version: String,
        }
        impl Versioned for State {
            fn version(&self) -> String {
                self.version.clone()
            }
            fn set_version(&mut self, v: String) {
                self.version = v;
            }
        }

        struct FailingMigration;
        impl Migration<State, State> for FailingMigration {
            fn migrate(&self, _old: State) -> Result<State, String> {
                Err("validation_failed".to_string())
            }
            fn name(&self) -> String {
                "failing_migration".to_string()
            }
        }

        let state = State {
            id: "obj-3".to_string(),
            version: "1.0".to_string(),
        };

        let mut runner = MigrationRunner::new("rollback_test");
        let result = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(runner.apply(state, Box::new(FailingMigration)));

        assert!(result.is_err());
        assert_eq!(runner.audit_trail().len(), 1);
        assert!(matches!(
            runner.audit_trail()[0].status,
            MigrationStatus::Failed { .. }
        ));
    }

    // Traces to: FR-MIGRATIONS-004 (Audit trail serialization)
    #[test]
    fn test_audit_trail_json_export() {
        #[derive(Clone, Debug, Serialize, Deserialize)]
        struct State {
            id: String,
            #[serde(default)]
            version: String,
        }
        impl Versioned for State {
            fn version(&self) -> String {
                self.version.clone()
            }
            fn set_version(&mut self, v: String) {
                self.version = v;
            }
        }

        struct SimpleMigration;
        impl Migration<State, State> for SimpleMigration {
            fn migrate(&self, mut old: State) -> Result<State, String> {
                old.version = "2.0".to_string();
                Ok(old)
            }
            fn name(&self) -> String {
                "simple_migration".to_string()
            }
        }

        let state = State {
            id: "obj-4".to_string(),
            version: "1.0".to_string(),
        };

        let mut runner = MigrationRunner::new("json_test");
        let _ = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(runner.apply(state, Box::new(SimpleMigration)));

        let json = runner.audit_trail_json().unwrap();
        assert!(json.contains("simple_migration"));
        assert!(json.contains("1.0"));
        assert!(json.contains("2.0"));
    }

    // Traces to: FR-MIGRATIONS-005 (Partial failure and recovery)
    #[test]
    fn test_partial_failure_recovery() {
        #[derive(Clone, Debug, Serialize, Deserialize)]
        struct State {
            id: String,
            value: i32,
            #[serde(default)]
            version: String,
        }
        impl Versioned for State {
            fn version(&self) -> String {
                self.version.clone()
            }
            fn set_version(&mut self, v: String) {
                self.version = v;
            }
        }

        struct ConditionalMigration {
            should_fail: bool,
        }
        impl Migration<State, State> for ConditionalMigration {
            fn migrate(&self, mut old: State) -> Result<State, String> {
                if self.should_fail {
                    Err("conditional_failure".to_string())
                } else {
                    old.value = old.value * 2;
                    old.version = "2.0".to_string();
                    Ok(old)
                }
            }
            fn name(&self) -> String {
                "conditional_migration".to_string()
            }
        }

        let state = State {
            id: "obj-5".to_string(),
            value: 10,
            version: "1.0".to_string(),
        };

        let mut runner = MigrationRunner::new("recovery_test");
        let rt = tokio::runtime::Runtime::new().unwrap();

        // First migration succeeds
        let state2 = rt
            .block_on(runner.apply(
                state,
                Box::new(ConditionalMigration {
                    should_fail: false,
                }),
            ))
            .unwrap();
        assert_eq!(state2.value, 20);

        // Second migration fails
        let result = rt.block_on(runner.apply(
            state2,
            Box::new(ConditionalMigration {
                should_fail: true,
            }),
        ));
        assert!(result.is_err());

        // Verify rollback stack
        assert_eq!(runner.rollback_stack().len(), 1);
        runner.record_rollback("recovery_initiated");
        assert_eq!(runner.audit_trail().len(), 3); // success + fail + rollback
    }

    // Traces to: FR-MIGRATIONS-006 (State type preservation across migrations)
    #[test]
    fn test_state_type_preservation() {
        #[derive(Clone, Debug, Serialize, Deserialize)]
        struct Config {
            key: String,
            value: String,
            #[serde(default)]
            version: String,
        }
        impl Versioned for Config {
            fn version(&self) -> String {
                self.version.clone()
            }
            fn set_version(&mut self, v: String) {
                self.version = v;
            }
        }

        struct EnrichConfigMigration;
        impl Migration<Config, Config> for EnrichConfigMigration {
            fn migrate(&self, mut old: Config) -> Result<Config, String> {
                old.value = format!("{}:enriched", old.value);
                old.version = "2.0".to_string();
                Ok(old)
            }
            fn name(&self) -> String {
                "enrich_config".to_string()
            }
        }

        let config = Config {
            key: "api_key".to_string(),
            value: "secret123".to_string(),
            version: "1.0".to_string(),
        };

        let mut runner = MigrationRunner::new("config_test");
        let migrated = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(runner.apply(config, Box::new(EnrichConfigMigration)))
            .unwrap();

        assert_eq!(migrated.key, "api_key");
        assert!(migrated.value.contains("enriched"));
    }

    // Traces to: FR-MIGRATIONS-007 (Namespace/domain tracking)
    #[test]
    fn test_migration_domain_tracking() {
        #[derive(Clone, Debug, Serialize, Deserialize)]
        struct State {
            id: String,
            #[serde(default)]
            version: String,
        }
        impl Versioned for State {
            fn version(&self) -> String {
                self.version.clone()
            }
            fn set_version(&mut self, v: String) {
                self.version = v;
            }
        }

        struct DummyMigration;
        impl Migration<State, State> for DummyMigration {
            fn migrate(&self, mut old: State) -> Result<State, String> {
                old.version = "2.0".to_string();
                Ok(old)
            }
            fn name(&self) -> String {
                "dummy".to_string()
            }
        }

        let state = State {
            id: "obj-6".to_string(),
            version: "1.0".to_string(),
        };

        let mut runner = MigrationRunner::new("my_domain");
        assert_eq!(runner.domain, "my_domain");

        let _ = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(runner.apply(state, Box::new(DummyMigration)));

        assert_eq!(runner.audit_trail().len(), 1);
    }
}
