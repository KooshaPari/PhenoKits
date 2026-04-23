//! Storage adapters

use std::path::PathBuf;
use crate::domain::Skill;
use crate::{SkillsError, Result};
use tracing::{debug, info};

/// File system storage for skills
#[derive(Debug, Clone)]
pub struct FileSystemStorage {
    base_path: PathBuf,
}

impl FileSystemStorage {
    pub fn new(base_path: impl Into<PathBuf>) -> Self {
        Self {
            base_path: base_path.into(),
        }
    }

    pub fn save(&self, skill: &Skill) -> Result<()> {
        debug!("Saving skill {} to {:?}", skill.id, self.base_path);
        // Implementation would serialize and save to disk
        Ok(())
    }

    pub fn load(&self, skill_id: &str) -> Result<Skill> {
        info!("Loading skill {} from {:?}", skill_id, self.base_path);
        // Implementation would load from disk
        Err(SkillsError::NotFound(skill_id.to_string()))
    }

    pub fn delete(&self, skill_id: &str) -> Result<()> {
        debug!("Deleting skill {} from {:?}", skill_id, self.base_path);
        Ok(())
    }
}

/// In-memory storage for testing
#[derive(Debug, Clone, Default)]
pub struct InMemoryStorage;

impl InMemoryStorage {
    pub fn new() -> Self {
        Self
    }
}
