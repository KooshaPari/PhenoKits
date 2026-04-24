//! Integration tests for phenotype-bdd
//!
//! Tests cover:
//! - Feature file parsing
//! - Scenario execution with mocked steps
//! - Step registry pattern matching
//! - Error handling
//! - Context sharing between steps

use phenotype_bdd::parser;
use phenotype_bdd::{
    BddError, BddRunner, Feature, Scenario, ScenarioResult, Step, StepContext, StepDefinition,
    StepResult, StepRegistry, StepType, Table,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// ============================================================================
// Feature Parsing Tests
// ============================================================================

#[test]
fn test_parse_simple_feature() {
    let content = r#"
Feature: User Management
  As an admin
  I want to manage users
  So that I can control access

  Scenario: Create a new user
    Given I am logged in as admin
    When I create a user with name "Alice"
    Then the user should exist in the system
"#;

    let feature = parser::parse_feature(content).unwrap();
    assert_eq!(feature.name, "User Management");
    assert!(feature.description.is_some());
    assert_eq!(feature.scenarios.len(), 1);
    assert_eq!(feature.scenarios[0].name, "Create a new user");
    assert_eq!(feature.scenarios[0].steps.len(), 3);
}

#[test]
fn test_parse_feature_with_background() {
    let content = r#"
Feature: Shopping Cart

  Background:
    Given the store is open
    And products are in stock

  Scenario: Add item to cart
    Given I am a customer
    When I add "Book" to cart
    Then cart should contain 1 item
"#;

    let feature = parser::parse_feature(content).unwrap();
    assert!(feature.background.is_some());
    let bg = feature.background.as_ref().unwrap();
    assert_eq!(bg.len(), 2);
    assert_eq!(bg[0].text, "the store is open");
}

#[test]
fn test_parse_feature_with_tags() {
    let content = r#"
@smoke @fast
Feature: Quick Tests

  @critical
  Scenario: Essential functionality
    Given the system is up
    Then everything works
"#;

    let feature = parser::parse_feature(content).unwrap();
    assert!(feature.tags.contains(&"smoke".to_string()));
    assert!(feature.tags.contains(&"fast".to_string()));
    assert!(feature.scenarios[0].tags.contains(&"critical".to_string()));
}

#[test]
fn test_parse_feature_with_data_table() {
    let content = r#"
Feature: Data Tables

  Scenario: Multiple users
    Given these users exist:
      | name  | age | role   |
      | Alice | 30  | admin  |
      | Bob   | 25  | user   |
    When I query all users
    Then I should see 2 users
"#;

    let feature = parser::parse_feature(content).unwrap();
    let step = &feature.scenarios[0].steps[0];
    assert!(step.table.is_some());
    let table = step.table.as_ref().unwrap();
    assert_eq!(table.header, vec!["name", "age", "role"]);
    assert_eq!(table.rows.len(), 2);
}

#[test]
fn test_parse_feature_with_doc_string() {
    let content = r#"
Feature: Doc Strings

  Scenario: Long description
    Given the following description:
      """
      This is a long description
      that spans multiple lines
      and should be preserved
      """
    When I process it
    Then it should be available
"#;

    let feature = parser::parse_feature(content).unwrap();
    let step = &feature.scenarios[0].steps[0];
    assert!(step.doc_string.is_some());
    let doc = step.doc_string.as_ref().unwrap();
    assert!(doc.contains("long description"));
    assert!(doc.contains("multiple lines"));
}

#[test]
fn test_parse_error_missing_feature() {
    let content = r#"
This is not a feature file
Just some random text
"#;

    let result = parser::parse_feature(content);
    assert!(matches!(result, Err(BddError::ParseError { .. })));
}

// ============================================================================
// Scenario Builder Tests
// ============================================================================

#[test]
fn test_scenario_builder_chain() {
    let scenario = Scenario::new("Login Flow")
        .given("user exists")
        .and("user is not logged in")
        .when("user enters credentials")
        .and("user clicks login")
        .then("user is authenticated")
        .but("session is not expired");

    assert_eq!(scenario.steps.len(), 6);
    assert_eq!(scenario.steps[0].step_type, StepType::Given);
    assert_eq!(scenario.steps[1].step_type, StepType::And);
    assert_eq!(scenario.steps[2].step_type, StepType::When);
}

#[test]
fn test_scenario_with_tags() {
    let scenario = Scenario::new("Critical Path")
        .with_tag("smoke")
        .with_tag("fast")
        .given("system is ready");

    assert!(scenario.tags.contains(&"smoke".to_string()));
    assert!(scenario.tags.contains(&"fast".to_string()));
}

#[test]
fn test_scenario_effective_steps_with_background() {
    let bg = vec![
        Step::new(StepType::Given, "background step 1"),
        Step::new(StepType::Given, "background step 2"),
    ];

    let scenario = Scenario::new("Test")
        .with_background(bg)
        .given("scenario step");

    let effective = scenario.effective_steps();
    assert_eq!(effective.len(), 3);
    assert_eq!(effective[0].text, "background step 1");
    assert_eq!(effective[2].text, "scenario step");
}

// ============================================================================
// Step Registry Tests
// ============================================================================

#[test]
fn test_step_registry_exact_match() {
    let mut registry = StepRegistry::new();

    let def = StepDefinition {
        pattern: "user logs in".to_string(),
        step_type: StepType::When,
        handler: Box::new(|ctx, _| {
            Box::pin(async move {
                Ok(ctx)
            })
        }),
    };
    registry.register(def);

    let step = Step::new(StepType::When, "user logs in");
    assert!(registry.find_match(&step).is_some());

    let wrong_step = Step::new(StepType::When, "admin logs in");
    assert!(registry.find_match(&wrong_step).is_none());
}

#[test]
fn test_step_registry_parameterized_pattern() {
    let registry = StepRegistry::new();

    let pattern = "user {name} logs in with password {password}";
    let text = "user alice logs in with password secret123";

    let params = registry.extract_params(pattern, text);
    assert_eq!(params.get("name"), Some(&"alice".to_string()));
    assert_eq!(params.get("password"), Some(&"secret123".to_string()));
}

#[test]
fn test_step_registry_canonical_type_matching() {
    let mut registry = StepRegistry::new();

    let def = StepDefinition {
        pattern: "system is ready".to_string(),
        step_type: StepType::Given,
        handler: Box::new(|ctx, _| Box::pin(async move { Ok(ctx) })),
    };
    registry.register(def);

    // And should match Given (canonical type)
    let and_step = Step::new(StepType::And, "system is ready");
    assert!(registry.find_match(&and_step).is_some());
}

// ============================================================================
// Async Runner Tests
// ============================================================================

#[tokio::test]
async fn test_runner_executes_simple_scenario() {
    let mut runner = BddRunner::new();
    let executed = Arc::new(Mutex::new(Vec::new()));

    let exec_clone = executed.clone();
    let def = StepDefinition {
        pattern: "step executes".to_string(),
        step_type: StepType::Given,
        handler: Box::new(move |ctx, step| {
            let exec = exec_clone.clone();
            Box::pin(async move {
                exec.lock().unwrap().push(step.text.clone());
                Ok(ctx)
            })
        }),
    };
    runner.register_step(def);

    let scenario = Scenario::new("Test").given("step executes");
    let result = runner.run_scenario(&scenario).await;

    assert!(result.passed);
    assert_eq!(result.step_results.len(), 1);
    assert!(matches!(result.step_results[0].1, StepResult::Passed));
}

#[tokio::test]
async fn test_runner_context_sharing() {
    let mut runner = BddRunner::new();

    // First step sets context
    let def1 = StepDefinition {
        pattern: "set user id".to_string(),
        step_type: StepType::Given,
        handler: Box::new(|mut ctx, _| {
            Box::pin(async move {
                ctx.set("user_id", "12345");
                Ok(ctx)
            })
        }),
    };
    runner.register_step(def1);

    // Second step reads context
    let verified = Arc::new(Mutex::new(false));
    let verified_clone = verified.clone();
    let def2 = StepDefinition {
        pattern: "verify user id".to_string(),
        step_type: StepType::Then,
        handler: Box::new(move |ctx, _| {
            let v = verified_clone.clone();
            Box::pin(async move {
                if let Some(user_id) = ctx.get("user_id") {
                    if user_id.as_str() == Some("12345") {
                        *v.lock().unwrap() = true;
                    }
                }
                Ok(ctx)
            })
        }),
    };
    runner.register_step(def2);

    let scenario = Scenario::new("Context Test")
        .given("set user id")
        .then("verify user id");

    let result = runner.run_scenario(&scenario).await;
    assert!(result.passed);
    assert!(*verified.lock().unwrap());
}

#[tokio::test]
async fn test_runner_missing_step_definition() {
    let runner = BddRunner::new(); // No steps registered

    let scenario = Scenario::new("Missing Step").given("undefined step");
    let result = runner.run_scenario(&scenario).await;

    assert!(!result.passed);
    assert!(result.error.is_some());
    assert!(result.error.unwrap().contains("No step definition found"));
}

#[tokio::test]
async fn test_runner_step_failure() {
    let mut runner = BddRunner::new();

    let def = StepDefinition {
        pattern: "failing step".to_string(),
        step_type: StepType::When,
        handler: Box::new(|_ctx, _| {
            Box::pin(async move {
                Err(BddError::StepFailed {
                    step: "failing step".to_string(),
                    reason: "Intentional failure".to_string(),
                })
            })
        }),
    };
    runner.register_step(def);

    let scenario = Scenario::new("Failing").when("failing step");
    let result = runner.run_scenario(&scenario).await;

    assert!(!result.passed);
    assert!(matches!(
        result.step_results[0].1,
        StepResult::Failed(ref msg) if msg.contains("Intentional failure")
    ));
}

#[tokio::test]
async fn test_runner_fail_fast_behavior() {
    let mut runner = BddRunner::new().with_fail_fast(true);

    let def = StepDefinition {
        pattern: "fail".to_string(),
        step_type: StepType::Given,
        handler: Box::new(|_ctx, _| {
            Box::pin(async move {
                Err(BddError::StepFailed {
                    step: "fail".to_string(),
                    reason: "stop here".to_string(),
                })
            })
        }),
    };
    runner.register_step(def);

    let scenario = Scenario::new("Fail Fast")
        .given("fail")
        .when("never reached")
        .then("also never reached");

    let result = runner.run_scenario(&scenario).await;

    // Only first step should be attempted
    assert_eq!(result.step_results.len(), 1);
}

#[tokio::test]
async fn test_runner_continue_on_failure() {
    let mut runner = BddRunner::new().with_fail_fast(false);

    let def = StepDefinition {
        pattern: "fail".to_string(),
        step_type: StepType::Given,
        handler: Box::new(|_ctx, _| {
            Box::pin(async move {
                Err(BddError::StepFailed {
                    step: "fail".to_string(),
                    reason: "error".to_string(),
                })
            })
        }),
    };
    runner.register_step(def);

    let scenario = Scenario::new("Continue")
        .given("fail")
        .when("fail") // Will be pending since not registered
        .then("fail"); // Will be pending

    let result = runner.run_scenario(&scenario).await;

    // All steps should be attempted
    assert_eq!(result.step_results.len(), 3);
}

#[tokio::test]
async fn test_runner_hooks() {
    let mut runner = BddRunner::new();

    let before_called = Arc::new(Mutex::new(false));
    let after_called = Arc::new(Mutex::new(false));

    let before_clone = before_called.clone();
    runner.before(move |_scenario| {
        *before_clone.lock().unwrap() = true;
        Ok(())
    });

    let after_clone = after_called.clone();
    runner.after(move |_result| {
        *after_clone.lock().unwrap() = true;
        Ok(())
    });

    let def = StepDefinition {
        pattern: "pass".to_string(),
        step_type: StepType::Given,
        handler: Box::new(|ctx, _| Box::pin(async move { Ok(ctx) })),
    };
    runner.register_step(def);

    let scenario = Scenario::new("Hooks Test").given("pass");
    runner.run_scenario(&scenario).await;

    assert!(*before_called.lock().unwrap());
    assert!(*after_called.lock().unwrap());
}

#[tokio::test]
async fn test_runner_before_hook_failure() {
    let mut runner = BddRunner::new();

    runner.before(|_scenario| {
        Err(BddError::HookFailed {
            hook: "before".to_string(),
            reason: "setup failed".to_string(),
        })
    });

    let def = StepDefinition {
        pattern: "never runs".to_string(),
        step_type: StepType::Given,
        handler: Box::new(|ctx, _| Box::pin(async move { Ok(ctx) })),
    };
    runner.register_step(def);

    let scenario = Scenario::new("Hook Failure").given("never runs");
    let result = runner.run_scenario(&scenario).await;

    assert!(!result.passed);
    assert!(result.error.unwrap().contains("setup failed"));
    // Steps should not run when before hook fails
    assert!(result.step_results.is_empty());
}

// ============================================================================
// Feature Runner Tests
// ============================================================================

#[tokio::test]
async fn test_run_feature_multiple_scenarios() {
    let mut runner = BddRunner::new();

    let counter = Arc::new(Mutex::new(0));
    let counter_clone = counter.clone();

    let def = StepDefinition {
        pattern: "count".to_string(),
        step_type: StepType::Given,
        handler: Box::new(move |ctx, _| {
            let c = counter_clone.clone();
            Box::pin(async move {
                *c.lock().unwrap() += 1;
                Ok(ctx)
            })
        }),
    };
    runner.register_step(def);

    let mut feature = Feature::new("Multi");
    feature.add_scenario(Scenario::new("One").given("count"));
    feature.add_scenario(Scenario::new("Two").given("count"));
    feature.add_scenario(Scenario::new("Three").given("count"));

    let results = runner.run_feature(&feature).await;

    assert_eq!(results.len(), 3);
    assert!(results.iter().all(|r| r.passed));
    assert_eq!(*counter.lock().unwrap(), 3);
}

// ============================================================================
// Table Tests
// ============================================================================

#[test]
fn test_table_row_as_map() {
    let table = Table::from_rows(
        vec!["name".to_string(), "age".to_string()],
        vec![
            vec!["Alice".to_string(), "30".to_string()],
            vec!["Bob".to_string(), "25".to_string()],
        ],
    );

    let row0 = table.row_as_map(0).unwrap();
    assert_eq!(row0.get("name"), Some(&String::from("Alice")));
    assert_eq!(row0.get("age"), Some(&String::from("30")));

    let row1 = table.row_as_map(1).unwrap();
    assert_eq!(row1.get("name"), Some(&String::from("Bob")));
}

#[test]
fn test_table_out_of_bounds() {
    let table = Table::from_rows(
        vec!["col".to_string()],
        vec![vec!["val".to_string()]],
    );

    assert!(table.row_as_map(0).is_some());
    assert!(table.row_as_map(1).is_none());
    assert!(table.row_as_map(100).is_none());
}

#[test]
fn test_table_to_maps() {
    let table = Table::from_rows(
        vec!["id".to_string(), "status".to_string()],
        vec![
            vec!["1".to_string(), "active".to_string()],
            vec!["2".to_string(), "inactive".to_string()],
            vec!["3".to_string(), "pending".to_string()],
        ],
    );

    let maps = table.to_maps();
    assert_eq!(maps.len(), 3);
    assert_eq!(maps[2].get("status"), Some(&String::from("pending")));
}

// ============================================================================
// Step Context Tests
// ============================================================================

#[test]
fn test_step_context_basic_operations() {
    let mut ctx = StepContext::new("Test Scenario");

    ctx.set("string_key", "string_value");
    ctx.set("int_key", 42i32);
    ctx.set("bool_key", true);
    ctx.set("json_key", serde_json::json!({"nested": "value"}));

    assert_eq!(ctx.get("string_key").unwrap().as_str(), Some("string_value"));
    assert_eq!(ctx.get("int_key").unwrap().as_i64(), Some(42));
    assert_eq!(ctx.get("bool_key").unwrap().as_bool(), Some(true));

    assert!(ctx.has("string_key"));
    assert!(!ctx.has("missing_key"));
}

#[test]
fn test_step_context_overwrite() {
    let mut ctx = StepContext::new("Test");

    ctx.set("key", "first");
    assert_eq!(ctx.get("key").unwrap().as_str(), Some("first"));

    ctx.set("key", "second");
    assert_eq!(ctx.get("key").unwrap().as_str(), Some("second"));
}

// ============================================================================
// Error Tests
// ============================================================================

#[test]
fn test_bdd_error_display_messages() {
    let err = BddError::StepNotFound("Given test".to_string());
    assert_eq!(err.to_string(), "Step definition not found: Given test");

    let err = BddError::ParseError {
        line: 42,
        message: "Unexpected token".to_string(),
    };
    assert!(err.to_string().contains("line 42"));
    assert!(err.to_string().contains("Unexpected token"));

    let err = BddError::Timeout("Slow Scenario".to_string());
    assert!(err.to_string().contains("timed out"));
}

#[test]
fn test_bdd_error_equality() {
    let err1 = BddError::StepNotFound("test".to_string());
    let err2 = BddError::StepNotFound("test".to_string());
    let err3 = BddError::StepNotFound("different".to_string());

    assert_eq!(err1, err2);
    assert_ne!(err1, err3);
}

// ============================================================================
// Scenario Result Tests
// ============================================================================

#[test]
fn test_scenario_result_properties() {
    let result = ScenarioResult {
        scenario_name: "Test".to_string(),
        step_results: vec![
            (Step::new(StepType::Given, "step 1"), StepResult::Passed),
            (Step::new(StepType::When, "step 2"), StepResult::Passed),
        ],
        duration_ms: 150,
        passed: true,
        error: None,
    };

    assert_eq!(result.scenario_name, "Test");
    assert!(result.passed);
    assert_eq!(result.duration_ms, 150);
    assert!(result.error.is_none());
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_empty_scenario() {
    let scenario = Scenario::new("Empty");
    assert!(scenario.steps.is_empty());
    assert!(scenario.effective_steps().is_empty());
}

#[test]
fn test_step_type_from_str() {
    assert_eq!(StepType::from_str("Given"), Some(StepType::Given));
    assert_eq!(StepType::from_str("When"), Some(StepType::When));
    assert_eq!(StepType::from_str("Then"), Some(StepType::Then));
    assert_eq!(StepType::from_str("And"), Some(StepType::And));
    assert_eq!(StepType::from_str("But"), Some(StepType::But));
    assert_eq!(StepType::from_str("Background"), Some(StepType::Background));
    assert_eq!(StepType::from_str("Unknown"), None);
    assert_eq!(StepType::from_str(""), None);
}

#[test]
fn test_step_canonical_type() {
    let given = Step::new(StepType::Given, "test");
    assert_eq!(given.canonical_type(), StepType::Given);

    let and = Step::new(StepType::And, "test");
    assert_eq!(and.canonical_type(), StepType::Given);

    let but = Step::new(StepType::But, "test");
    assert_eq!(but.canonical_type(), StepType::Given);

    let when = Step::new(StepType::When, "test");
    assert_eq!(when.canonical_type(), StepType::When);
}
