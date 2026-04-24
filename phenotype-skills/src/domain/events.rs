//! Domain events

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::value_objects::SkillManifest;
use super::entities::SkillId;

/// Events that occur in the skills domain
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum SkillEvent {
    SkillRegistered {
        skill_id: SkillId,
        name: String,
        version: String,
        timestamp: DateTime<Utc>,
    },
    SkillUnregistered {
        skill_id: SkillId,
        timestamp: DateTime<Utc>,
    },
    SkillUpdated {
        skill_id: SkillId,
        old_version: String,
        new_version: String,
        timestamp: DateTime<Utc>,
    },
    SkillExecuted {
        skill_id: SkillId,
        success: bool,
        duration_ms: u64,
        timestamp: DateTime<Utc>,
    },
    DependencyResolved {
        skill_id: SkillId,
        dependency_name: String,
        resolved_version: String,
        timestamp: DateTime<Utc>,
    },
}

impl SkillEvent {
    pub fn registered(skill_id: SkillId, name: String, version: String) -> Self {
        Self::SkillRegistered {
            skill_id,
            name,
            version,
            timestamp: Utc::now(),
        }
    }

    pub fn unregistered(skill_id: SkillId) -> Self {
        Self::SkillUnregistered {
            skill_id,
            timestamp: Utc::now(),
        }
    }

    pub fn updated(skill_id: SkillId, old_manifest: SkillManifest, new_manifest: SkillManifest) -> Self {
        Self::SkillUpdated {
            skill_id,
            old_version: old_manifest.version.to_string(),
            new_version: new_manifest.version.to_string(),
            timestamp: Utc::now(),
        }
    }

    pub fn executed(skill_id: SkillId, success: bool, duration_ms: u64) -> Self {
        Self::SkillExecuted {
            skill_id,
            success,
            duration_ms,
            timestamp: Utc::now(),
        }
    }

    pub fn dependency_resolved(skill_id: SkillId, dependency_name: String, resolved_version: String) -> Self {
        Self::DependencyResolved {
            skill_id,
            dependency_name,
            resolved_version,
            timestamp: Utc::now(),
        }
    }
}
