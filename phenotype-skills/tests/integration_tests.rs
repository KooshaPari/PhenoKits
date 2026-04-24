//! Integration tests for phenotype-skills

use phenotype_skills::{
    Skill, SkillId, SkillManifest, Runtime,
    SkillRegistry, DependencyResolver,
};

#[test]
fn test_skill_creation() {
    let manifest = SkillManifest::new(
        "test-skill",
        "1.0.0",
        Runtime::Wasm,
        "main.wasm",
    );
    
    let skill = Skill::from_manifest(manifest.clone(), None);
    assert_eq!(skill.name, "test-skill");
    assert_eq!(skill.manifest.name, "test-skill");
}

#[test]
fn test_skill_registry_registration() {
    let mut registry = SkillRegistry::new();
    
    let manifest = SkillManifest::new(
        "test-skill",
        "1.0.0",
        Runtime::Wasm,
        "main.wasm",
    );
    
    let skill = Skill::from_manifest(manifest, None);
    let skill_id = skill.id.clone();
    
    registry.register(skill).unwrap();
    
    assert!(registry.exists(&skill_id));
    assert_eq!(registry.list().len(), 1);
}

#[test]
fn test_skill_registry_multiple_skills() {
    let mut registry = SkillRegistry::new();
    
    let manifest1 = SkillManifest::new(
        "skill-1",
        "1.0.0",
        Runtime::Wasm,
        "main.wasm",
    );
    
    let manifest2 = SkillManifest::new(
        "skill-2",
        "1.0.0",
        Runtime::Python,
        "main.py",
    );
    
    let skill1 = Skill::from_manifest(manifest1, None);
    let skill2 = Skill::from_manifest(manifest2, None);
    
    registry.register(skill1).unwrap();
    registry.register(skill2).unwrap();
    
    assert_eq!(registry.list().len(), 2);
}

#[test]
fn test_registry_unregister() {
    let mut registry = SkillRegistry::new();
    
    let manifest = SkillManifest::new(
        "test-skill",
        "1.0.0",
        Runtime::Wasm,
        "main.wasm",
    );
    
    let skill = Skill::from_manifest(manifest, None);
    let skill_id = skill.id.clone();
    
    registry.register(skill).unwrap();
    assert!(registry.exists(&skill_id));
    
    registry.unregister(&skill_id).unwrap();
    assert!(!registry.exists(&skill_id));
}

#[test]
fn test_dependency_resolver_empty() {
    let resolver = DependencyResolver::new();
    
    let result = resolver.resolve(&[]);
    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}

#[test]
fn test_dependency_resolver_single_skill() {
    let resolver = DependencyResolver::new();
    
    let manifest = SkillManifest::new(
        "test-skill",
        "1.0.0",
        Runtime::Wasm,
        "main.wasm",
    );
    
    let skill = Skill::from_manifest(manifest, None);
    let result = resolver.resolve(&[skill]);
    assert!(result.is_ok());
}

#[test]
fn test_skill_id_unique() {
    let manifest = SkillManifest::new(
        "test-skill",
        "1.0.0",
        Runtime::Wasm,
        "main.wasm",
    );
    
    let skill1 = Skill::from_manifest(manifest.clone(), None);
    let skill2 = Skill::from_manifest(manifest, None);
    
    // Skills created from same manifest should have same ID (based on name)
    assert_eq!(skill1.id.to_string(), skill2.id.to_string());
}

#[test]
fn test_version_parsing() {
    use phenotype_skills::Version;
    
    let version = Version::parse("1.2.3").unwrap();
    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 2);
    assert_eq!(version.patch, 3);
}

#[test]
fn test_runtime_display() {
    assert_eq!(format!("{}", Runtime::Wasm), "wasm");
    assert_eq!(format!("{}", Runtime::Python), "python");
    assert_eq!(format!("{}", Runtime::CSharp), "csharp");
}
