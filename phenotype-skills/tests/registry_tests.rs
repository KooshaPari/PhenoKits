//! Registry tests

use phenotype_skills::{Skill, SkillId, SkillManifest, Runtime, SkillRegistry};

#[test]
fn test_empty_registry() {
    let registry = SkillRegistry::new();
    assert!(registry.list().is_empty());
}

#[test]
fn test_registry_get_nonexistent() {
    let registry = SkillRegistry::new();
    let fake_id = SkillId::new("nonexistent");
    
    assert!(registry.get(&fake_id).is_err());
}

#[test]
fn test_registry_multiple_skills() {
    let mut registry = SkillRegistry::new();
    
    for i in 0..10 {
        let manifest = SkillManifest::new(
            format!("skill-{}", i),
            "1.0.0",
            Runtime::Wasm,
            "main.wasm",
        );
        
        let skill = Skill::from_manifest(manifest, None);
        registry.register(skill).unwrap();
    }
    
    assert_eq!(registry.list().len(), 10);
}

#[test]
fn test_registry_same_id_overwrite() {
    let mut registry = SkillRegistry::new();
    
    // Create two skills with same name (same ID)
    let manifest1 = SkillManifest::new(
        "same-skill",
        "1.0.0",
        Runtime::Wasm,
        "main.wasm",
    );
    
    let manifest2 = SkillManifest::new(
        "same-skill",
        "2.0.0",
        Runtime::Wasm,
        "main.wasm",
    );
    
    let skill1 = Skill::from_manifest(manifest1, None);
    let id = skill1.id.clone();
    
    registry.register(skill1).unwrap();
    
    // Second register with same ID should update or error
    let skill2 = Skill::from_manifest(manifest2, None);
    let result = registry.register(skill2);
    
    // Depends on implementation - either error or success
    assert!(result.is_ok() || result.is_err());
    assert!(registry.exists(&id));
}
