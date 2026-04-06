//! Integration tests for phenotype-gauge xDD testing utilities.
//! Covers: SpecParser, SpecValidator, ContractVerifier, property strategies, domain errors.

use gauge::contract::{Contract, ContractVerifier};
use gauge::domain::{ErrorCategory, XddError, XddResult};
use gauge::property::strategies;
use gauge::spec::{SpecParser, SpecValidator};

// ---- Domain error integration ----

#[test]
fn test_xdd_error_creation_and_serialization() {
    let err = XddError::new("bad input", ErrorCategory::Spec);
    assert_eq!(err.message, "bad input");
    let json = serde_json::to_string(&err).expect("should serialize");
    assert!(json.contains("bad input"));
}

#[test]
fn test_xdd_error_with_context() {
    let err = XddError::new("constraint violated", ErrorCategory::Spec)
        .with_context("field", "email")
        .with_context("value", "not-an-email");
    assert!(!err.context.is_null());
    assert_eq!(err.context["field"], "email");
}

// ---- SpecParser integration ----

#[test]
fn test_parse_minimal_valid_spec() {
    let yaml = r#"
spec:
  name: Minimal Spec
  version: 1.0.0
  features: []
"#;
    let spec = SpecParser::parse(yaml).expect("should parse minimal spec");
    assert_eq!(spec.spec.name, "Minimal Spec");
    assert_eq!(spec.spec.version, "1.0.0");
    assert!(spec.features.is_empty());
}

#[test]
fn test_parse_spec_with_feature_and_scenario() {
    let yaml = r#"
spec:
  name: Auth Spec
  version: 2.0.0
features:
  - id: AUTH-001
    name: User Login
    scenario:
      given: valid credentials
      when: user submits login form
      then: redirect to dashboard
"#;
    let spec = SpecParser::parse(yaml).expect("should parse spec with feature");
    assert_eq!(spec.features.len(), 1);
    let feature = &spec.features[0];
    assert_eq!(feature.id, "AUTH-001");
    assert_eq!(feature.name, "User Login");
    let scenario = feature.scenario.as_ref().expect("scenario should be present");
    assert_eq!(scenario.given, "valid credentials");
}

#[test]
fn test_parse_invalid_yaml_returns_error() {
    let bad_yaml = "{ unclosed: bracket";
    let result = SpecParser::parse(bad_yaml);
    assert!(result.is_err(), "invalid YAML should return an error");
}

#[test]
fn test_parse_spec_with_multiple_features() {
    let yaml = r#"
spec:
  name: Multi-Feature Spec
  version: 1.0.0
features:
  - id: FT-001
    name: Feature One
    scenario:
      given: initial state
      when: action occurs
      then: expected outcome
  - id: FT-002
    name: Feature Two
    scenario:
      given: second initial state
      when: second action occurs
      then: second expected outcome
  - id: FT-003
    name: Feature Three
    scenario:
      given: third initial state
      when: third action occurs
      then: third expected outcome
"#;
    let spec = SpecParser::parse(yaml).expect("should parse");
    assert_eq!(spec.features.len(), 3);
    assert_eq!(spec.features[2].id, "FT-003");
}

// ---- SpecValidator integration ----

#[test]
fn test_validator_accepts_valid_spec() {
    let yaml = r#"
spec:
  name: Valid Spec
  version: 1.0.0
features:
  - id: V-001
    name: Some Feature
    scenario:
      given: a precondition
      when: an action
      then: an outcome
"#;
    let spec = SpecParser::parse(yaml).unwrap();
    let mut validator = SpecValidator::new();
    let result = validator.validate(&spec);
    assert!(result.is_ok(), "valid spec should pass validation: {:?}", result.err());
}

#[test]
fn test_validator_errors_on_missing_name() {
    // A spec struct with empty name
    let yaml = r#"
spec:
  name: ""
  version: 1.0.0
features: []
"#;
    // SpecParser::parse also validates; empty name should fail
    let result = SpecParser::parse(yaml);
    assert!(result.is_err(), "empty spec name should fail validation");
}

// ---- ContractVerifier integration ----

struct AlwaysPassAdapter;

impl Contract for AlwaysPassAdapter {
    fn name() -> &'static str {
        "always_pass_adapter"
    }

    fn verify() -> XddResult<()> {
        Ok(())
    }
}

struct AlwaysFailAdapter;

impl Contract for AlwaysFailAdapter {
    fn name() -> &'static str {
        "always_fail_adapter"
    }

    fn verify() -> XddResult<()> {
        Err(XddError::contract("adapter does not implement required behavior"))
    }
}

#[test]
fn test_contract_verifier_passes_valid_adapter() {
    let mut verifier = ContractVerifier::new();
    let result = verifier.verify::<AlwaysPassAdapter>();
    assert!(result.is_ok(), "valid adapter should pass contract verification");
}

#[test]
fn test_contract_verifier_fails_bad_adapter() {
    let mut verifier = ContractVerifier::new();
    let result = verifier.verify::<AlwaysFailAdapter>();
    assert!(result.is_err(), "failing adapter should return error from verifier");
}

#[test]
fn test_contract_verifier_assert_true() {
    let mut verifier = ContractVerifier::new();
    verifier.assert(true, "x should equal 1", "x = 1");
    let report = verifier.result("test_contract");
    assert!(report.passed, "assert(true) should pass");
}

#[test]
fn test_contract_verifier_assert_false_records_failure() {
    let mut verifier = ContractVerifier::new();
    verifier.assert(false, "x should equal 1", "x = 2");
    let report = verifier.result("test_contract");
    assert!(!report.passed, "assert(false) should fail");
    assert!(!report.failures.is_empty(), "failure list should be non-empty");
}

// ---- Property strategy integration ----

#[test]
fn test_valid_uuid_accepts_correct_format() {
    let uuid = "550e8400-e29b-41d4-a716-446655440000";
    assert!(strategies::valid_uuid(uuid).is_ok(), "well-formed UUID should be accepted");
}

#[test]
fn test_valid_uuid_rejects_short_string() {
    let result = strategies::valid_uuid("not-a-uuid");
    assert!(result.is_err(), "short string should be rejected as UUID");
}

#[test]
fn test_valid_email_accepts_standard_address() {
    let result = strategies::valid_email("user@example.com");
    assert!(result.is_ok(), "standard email should be accepted");
}

#[test]
fn test_valid_email_rejects_missing_at() {
    let result = strategies::valid_email("notanemail");
    assert!(result.is_err(), "string without @ should be rejected as email");
}
