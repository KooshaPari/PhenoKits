//! Domain tests

use phenotype_skills::{SkillId, Version, Runtime};

#[test]
fn test_skill_id_from_string() {
    let id = SkillId::new("test-id");
    assert_eq!(id.to_string(), "test-id");
}

#[test]
fn test_skill_id_clone() {
    let id1 = SkillId::new("test-id");
    let id2 = id1.clone();
    assert_eq!(id1.to_string(), id2.to_string());
}

#[test]
fn test_version_comparison() {
    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(1, 1, 0);
    let v3 = Version::new(2, 0, 0);
    
    assert!(v1 < v2);
    assert!(v2 < v3);
    assert!(v1 < v3);
}

#[test]
fn test_version_equality() {
    let v1 = Version::new(1, 2, 3);
    let v2 = Version::new(1, 2, 3);
    assert_eq!(v1.major, v2.major);
    assert_eq!(v1.minor, v2.minor);
    assert_eq!(v1.patch, v2.patch);
}

#[test]
fn test_version_parse_valid() {
    let version = Version::parse("2.5.1").unwrap();
    assert_eq!(version.major, 2);
    assert_eq!(version.minor, 5);
    assert_eq!(version.patch, 1);
}

#[test]
fn test_version_parse_invalid() {
    assert!(Version::parse("invalid").is_err());
    assert!(Version::parse("1.2").is_err());
    assert!(Version::parse("1.2.3.4").is_err());
}

#[test]
fn test_version_satisfies_caret() {
    let v = Version::new(1, 5, 0);
    assert!(v.satisfies("^1.0.0"));
    assert!(v.satisfies("^1.5.0"));
    assert!(!v.satisfies("^2.0.0"));
}

#[test]
fn test_version_satisfies_gte() {
    let v = Version::new(2, 0, 0);
    assert!(v.satisfies(">=1.0.0"));
    assert!(v.satisfies(">=2.0.0"));
    assert!(!v.satisfies(">=3.0.0"));
}

#[test]
fn test_runtime_variants() {
    let runtimes = vec![
        Runtime::Wasm,
        Runtime::Python,
        Runtime::JavaScript,
        Runtime::Rust,
        Runtime::CSharp,
        Runtime::Go,
        Runtime::Shell,
        Runtime::Binary,
        Runtime::Custom,
    ];
    
    for runtime in runtimes {
        let s = format!("{}", runtime);
        assert!(!s.is_empty());
    }
}

#[test]
fn test_runtime_default() {
    let default: Runtime = Default::default();
    assert_eq!(format!("{}", default), "wasm");
}
