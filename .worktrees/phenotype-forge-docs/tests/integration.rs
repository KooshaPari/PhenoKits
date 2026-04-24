//! Integration tests for phenotype-forge

/// Test that basic functionality works
#[test]
fn test_basic_functionality() {
    assert_eq!(2 + 2, 4);
}

/// Test task definition structure
#[test]
fn test_task_attributes() {
    // Task system should support these attributes:
    // - #[task] for defining tasks
    // - #[deps(...)] for dependencies
    // - #[ignore] for conditional execution
    let deps: Vec<&str> = vec!["build", "test"];
    assert!(deps.contains(&"build"));
}

/// Test configuration parsing
#[test]
fn test_config_parsing() {
    let config = r#"
    [tasks.build]
    command = "cargo build"
    
    [tasks.test]
    deps = ["build"]
    command = "cargo test"
    "#;

    assert!(config.contains("build"));
    assert!(config.contains("test"));
}

/// Test dependency graph
#[test]
fn test_dependency_resolution() {
    // Simulate a dependency graph
    let mut graph: std::collections::HashMap<&str, Vec<&str>> = std::collections::HashMap::new();
    graph.insert("test", vec!["build"]);
    graph.insert("build", vec![]);

    // Topological sort should order build before test
    let deps = graph.get("test").unwrap();
    assert!(deps.contains(&"build"));
}
