//! Driving ports - APIs exposed to clients

use crate::domain::{Skill, SkillId, SkillManifest};
use crate::Result;

/// Port for the skill registry API
pub trait RegistryPort: Send + Sync + std::fmt::Debug {
    /// Register a new skill
    fn register(&self, skill: Skill) -> Result<()>;

    /// Unregister a skill
    fn unregister(&self, skill_id: &SkillId) -> Result<Skill>;

    /// Get a skill by ID
    fn get(&self, skill_id: &SkillId) -> Result<Skill>;

    /// List all registered skills
    fn list(&self) -> Vec<Skill>;

    /// Update a skill's manifest
    fn update(&self, skill_id: &SkillId, manifest: SkillManifest) -> Result<Skill>;
}

/// Port for skill execution
pub trait ExecutorPort: Send + Sync + std::fmt::Debug {
    /// Execute a skill with given input
    fn execute(&self, skill_id: &SkillId, input: serde_json::Value) -> Result<serde_json::Value>;

    /// Check if a skill can be executed
    fn can_execute(&self, skill_id: &SkillId) -> bool;
}
