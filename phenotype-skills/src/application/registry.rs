//! Skill Registry - Central management for skills

use std::collections::HashMap;
use std::sync::RwLock;

use crate::domain::{Skill, SkillId, SkillManifest};

/// Main registry for managing skills
pub struct SkillRegistry {
    /// Internal skill storage
    skills: RwLock<HashMap<SkillId, Skill>>,
}

impl SkillRegistry {
    /// Create a new skill registry
    pub fn new() -> Self {
        Self {
            skills: RwLock::new(HashMap::new()),
        }
    }

    /// Register a new skill
    pub fn register(&self, skill: Skill) -> crate::Result<()> {
        let skill_id = skill.id.clone();

        // Check if already registered
        {
            let skills = self.skills.read().map_err(|_| crate::SkillsError::StorageError("Lock poisoned".to_string()))?;
            if skills.contains_key(&skill_id) {
                return Err(crate::SkillsError::AlreadyRegistered(skill_id.to_string()));
            }
        }

        // Store the skill
        {
            let mut skills = self.skills.write().map_err(|_| crate::SkillsError::StorageError("Lock poisoned".to_string()))?;
            skills.insert(skill_id.clone(), skill);
        }

        Ok(())
    }

    /// Unregister a skill
    pub fn unregister(&self, skill_id: &SkillId) -> crate::Result<Skill> {
        let mut skills = self.skills.write().map_err(|_| crate::SkillsError::StorageError("Lock poisoned".to_string()))?;
        skills.remove(skill_id).ok_or_else(|| crate::SkillsError::NotFound(skill_id.to_string()))
    }

    /// Get a skill by ID
    pub fn get(&self, skill_id: &SkillId) -> crate::Result<Skill> {
        let skills = self.skills.read().map_err(|_| crate::SkillsError::StorageError("Lock poisoned".to_string()))?;
        skills.get(skill_id)
            .cloned()
            .ok_or_else(|| crate::SkillsError::NotFound(skill_id.to_string()))
    }

    /// List all registered skills
    pub fn list(&self) -> Vec<Skill> {
        let skills = match self.skills.read() {
            Ok(s) => s,
            Err(_) => return vec![],
        };
        skills.values().cloned().collect()
    }

    /// List skills by tag
    pub fn list_by_tag(&self, _tag: &str) -> Vec<Skill> {
        // Tags not yet implemented in this simplified version
        self.list()
    }

    /// Update a skill's manifest
    pub fn update(&self, skill_id: &SkillId, manifest: SkillManifest) -> crate::Result<Skill> {
        let mut skills = self.skills.write().map_err(|_| crate::SkillsError::StorageError("Lock poisoned".to_string()))?;
        let skill = skills.get_mut(skill_id).ok_or_else(|| crate::SkillsError::NotFound(skill_id.to_string()))?;

        skill.manifest = manifest;
        skill.updated_at = chrono::Utc::now();

        Ok(skill.clone())
    }

    /// Check if a skill exists
    pub fn exists(&self, skill_id: &SkillId) -> bool {
        let skills = match self.skills.read() {
            Ok(s) => s,
            Err(_) => return false,
        };
        skills.contains_key(skill_id)
    }

    /// Get the count of registered skills
    pub fn count(&self) -> usize {
        let skills = match self.skills.read() {
            Ok(s) => s,
            Err(_) => return 0,
        };
        skills.len()
    }
}

impl Default for SkillRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Runtime;

    fn make_manifest(name: &str) -> SkillManifest {
        SkillManifest::new(
            name,
            "1.0.0",
            Runtime::Wasm,
            "main.wasm",
        )
    }

    #[test]
    fn test_register_and_get() {
        let registry = SkillRegistry::new();

        let skill = Skill::from_manifest(make_manifest("test-skill"), None);

        registry.register(skill.clone()).unwrap();

        let retrieved = registry.get(&skill.id).unwrap();
        assert_eq!(retrieved.name, "test-skill");
    }

    #[test]
    fn test_unregister() {
        let registry = SkillRegistry::new();

        let skill = Skill::from_manifest(make_manifest("test-skill"), None);

        registry.register(skill.clone()).unwrap();
        registry.unregister(&skill.id).unwrap();

        assert!(!registry.exists(&skill.id));
    }

    #[test]
    fn test_list() {
        let registry = SkillRegistry::new();

        for i in 0..3 {
            let skill = Skill::from_manifest(make_manifest(&format!("skill-{}", i)), None);
            registry.register(skill).unwrap();
        }

        assert_eq!(registry.count(), 3);
    }

    #[test]
    fn test_already_registered() {
        let registry = SkillRegistry::new();

        let skill = Skill::from_manifest(make_manifest("test-skill"), None);

        registry.register(skill.clone()).unwrap();
        let result = registry.register(skill);

        assert!(result.is_err());
    }

    #[test]
    fn test_not_found() {
        let registry = SkillRegistry::new();
        let skill_id = SkillId::new("nonexistent");

        let result = registry.get(&skill_id);
        assert!(result.is_err());
    }
}
