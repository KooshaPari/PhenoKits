//! Domain entities

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

use super::value_objects::SkillManifest;

/// Unique identifier for a skill
#[derive(Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct SkillId(String);

impl SkillId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl fmt::Display for SkillId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A registered skill with its metadata
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Skill {
    pub id: SkillId,
    pub name: String,
    pub version: super::Version,
    pub manifest: SkillManifest,
    pub path: Option<std::path::PathBuf>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Skill {
    pub fn from_manifest(manifest: SkillManifest, path: Option<std::path::PathBuf>) -> Self {
        let id = SkillId::new(&manifest.name);
        let name = manifest.name.clone();
        let version = manifest.version.clone();
        let now = Utc::now();

        Self {
            id,
            name,
            version,
            manifest,
            path,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn is_compatible_with(&self, other: &Skill) -> bool {
        self.manifest.is_compatible_with(&other.manifest)
    }

    pub fn requires_permission(&self, perm: &str) -> bool {
        self.manifest.permissions.iter().any(|p| p.as_str() == perm)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Runtime;

    #[test]
    fn test_skill_id() {
        let id = SkillId::new("test-skill");
        assert_eq!(id.as_str(), "test-skill");
        assert_eq!(id.to_string(), "test-skill");
    }

    #[test]
    fn test_skill_from_manifest() {
        let manifest = SkillManifest {
            name: "test".to_string(),
            version: super::super::Version::new(1, 0, 0),
            description: None,
            author: None,
            runtime: Runtime::Wasm,
            entry_point: "main.wasm".to_string(),
            permissions: vec![],
            dependencies: vec![],
            config: serde_json::json!({}),
        };

        let skill = Skill::from_manifest(manifest, None);
        assert_eq!(skill.id.as_str(), "test");
        assert_eq!(skill.name, "test");
    }
}
