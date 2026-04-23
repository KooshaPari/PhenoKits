//! Dependency resolver tests

use phenotype_skills::{Skill, SkillManifest, Runtime, DependencyResolver, SkillDependency};

fn create_skill(name: &str, version: &str) -> Skill {
    let manifest = SkillManifest::new(name, version, Runtime::Wasm, "main.wasm");
    Skill::from_manifest(manifest, None)
}

fn create_skill_with_dep(name: &str, version: &str, dep_name: &str, dep_constraint: &str) -> Skill {
    let mut manifest = SkillManifest::new(name, version, Runtime::Wasm, "main.wasm");
    manifest.dependencies.push(SkillDependency {
        name: dep_name.to_string(),
        version_constraint: dep_constraint.to_string(),
        optional: false,
    });
    Skill::from_manifest(manifest, None)
}

#[test]
fn test_resolver_empty() {
    let resolver = DependencyResolver::new();
    let result = resolver.resolve(&[]).unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_resolver_single_skill_no_deps() {
    let resolver = DependencyResolver::new();
    let skill = create_skill("test", "1.0.0");
    
    let result = resolver.resolve(&[skill.clone()]).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, skill.id);
}

#[test]
fn test_resolver_with_dependency() {
    let resolver = DependencyResolver::new();
    
    // Create dependency first
    let dep = create_skill("dep", "1.0.0");
    let skill = create_skill_with_dep("test", "1.0.0", "dep", ">=1.0.0");
    
    // Resolve both together
    let result = resolver.resolve(&[skill.clone(), dep.clone()]).unwrap();
    assert_eq!(result.len(), 2);
    // Dependencies should come first
    assert_eq!(result[0].id, dep.id);
    assert_eq!(result[1].id, skill.id);
}

#[test]
fn test_resolver_circular_dependency_error() {
    let resolver = DependencyResolver::new();
    
    // Create circular dependency: A -> B -> A
    let skill_a = create_skill_with_dep("skill-a", "1.0.0", "skill-b", ">=1.0.0");
    let skill_b = create_skill_with_dep("skill-b", "1.0.0", "skill-a", ">=1.0.0");
    
    let result = resolver.resolve(&[skill_a, skill_b]);
    assert!(result.is_err());
}

#[test]
fn test_check_circular_no_cycle() {
    let resolver = DependencyResolver::new();
    
    let skill1 = create_skill("skill1", "1.0.0");
    let skill2 = create_skill("skill2", "1.0.0");
    
    let result = resolver.check_circular(&[skill1, skill2]);
    assert!(result.is_ok());
}

#[test]
fn test_check_circular_with_cycle() {
    let resolver = DependencyResolver::new();
    
    let skill_a = create_skill_with_dep("skill-a", "1.0.0", "skill-b", ">=1.0.0");
    let skill_b = create_skill_with_dep("skill-b", "1.0.0", "skill-a", ">=1.0.0");
    
    let result = resolver.check_circular(&[skill_a, skill_b]);
    assert!(result.is_err());
}

#[test]
fn test_resolver_multiple_skills() {
    let resolver = DependencyResolver::new();
    
    let skill1 = create_skill("skill1", "1.0.0");
    let skill2 = create_skill("skill2", "1.0.0");
    let skill3 = create_skill("skill3", "1.0.0");
    
    let result = resolver.resolve(&[skill1.clone(), skill2.clone(), skill3.clone()]).unwrap();
    assert_eq!(result.len(), 3);
}

#[test]
fn test_to_dot_graph() {
    let resolver = DependencyResolver::new();
    
    let dep = create_skill("dep", "1.0.0");
    let skill = create_skill_with_dep("test", "1.0.0", "dep", ">=1.0.0");
    
    let dot = resolver.to_dot(&[skill, dep]);
    assert!(dot.contains("digraph dependencies"));
    assert!(dot.contains("rankdir"));
}

#[test]
fn test_get_all_dependencies() {
    let resolver = DependencyResolver::new();
    
    let dep1 = create_skill("dep1", "1.0.0");
    let dep2 = create_skill("dep2", "1.0.0");
    let skill = create_skill_with_dep("test", "1.0.0", "dep1", ">=1.0.0");
    let all_skills = vec![skill.clone(), dep1.clone(), dep2.clone()];
    
    let deps = resolver.get_all_dependencies(&skill, &all_skills);
    assert!(!deps.is_empty());
}
