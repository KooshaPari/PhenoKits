//! Integration tests for phenotype-nexus service registry.
//! Traces to: E1.1, E1.2, E1.3, E2.1, E2.2, E3.1

use nexus::registry::{RegistryError, ServiceInstance, ServiceRegistry};

// ---- E1.1: register a service ----

#[test]
fn test_register_service_success() {
    // Traces to: E1.1
    let registry = ServiceRegistry::new();
    let svc = ServiceInstance::new("user-svc", "localhost:8080");
    registry.register(svc);
    assert_eq!(registry.len(), 1);
}

#[test]
fn test_register_replaces_existing_entry() {
    // Traces to: E1.1 (registering same service twice replaces without error)
    let registry = ServiceRegistry::new();
    registry.register(ServiceInstance::new("auth-svc", "localhost:9000"));
    registry.register(ServiceInstance::new("auth-svc", "localhost:9001"));
    assert_eq!(registry.len(), 1, "duplicate registration should replace, not append");
    let results = registry.discover("auth-svc");
    assert_eq!(results[0].address, "localhost:9001");
}

// ---- E1.2: deregister a service ----

#[test]
fn test_deregister_removes_service() {
    // Traces to: E1.2
    let registry = ServiceRegistry::new();
    registry.register(ServiceInstance::new("user-svc", "localhost:8080"));
    registry.deregister("user-svc").expect("deregister should succeed");
    assert!(registry.discover("user-svc").is_empty());
}

#[test]
fn test_deregister_missing_service_returns_not_found() {
    // Traces to: E1.2
    let registry = ServiceRegistry::new();
    let err = registry.deregister("ghost-svc").unwrap_err();
    assert_eq!(err, RegistryError::NotFound("ghost-svc".to_owned()));
}

// ---- E1.3: tag-based filtering ----

#[test]
fn test_register_with_tags_and_discover_by_tag() {
    // Traces to: E1.3
    let registry = ServiceRegistry::new();
    registry
        .register(ServiceInstance::new("auth-svc", "us-west:9090").with_tag("region", "us-west"));
    registry
        .register(ServiceInstance::new("auth-svc-eu", "eu:9090").with_tag("region", "eu-central"));
    let results = registry.discover_by_tag("region", "us-west");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].address, "us-west:9090");
}

#[test]
fn test_discover_by_tag_no_match_returns_empty() {
    // Traces to: E1.3
    let registry = ServiceRegistry::new();
    registry.register(ServiceInstance::new("svc", "localhost:1234").with_tag("env", "prod"));
    let results = registry.discover_by_tag("env", "staging");
    assert!(results.is_empty());
}

// ---- E2.1: discover healthy instances ----

#[test]
fn test_discover_returns_healthy_instances_only() {
    // Traces to: E2.1, E3.1
    let registry = ServiceRegistry::new();
    registry.register(ServiceInstance::new("api-svc", "localhost:3000"));
    // Mark unhealthy
    registry.set_health("api-svc", false).unwrap();
    let results = registry.discover("api-svc");
    assert!(results.is_empty(), "unhealthy instance should not be returned by discover");
}

#[test]
fn test_discover_returns_empty_for_unknown_service() {
    // Traces to: E2.1
    let registry = ServiceRegistry::new();
    let results = registry.discover("nonexistent");
    assert!(results.is_empty());
}

// ---- E3.1: health state transitions ----

#[test]
fn test_set_health_marks_instance_unhealthy_then_healthy() {
    // Traces to: E3.1
    let registry = ServiceRegistry::new();
    registry.register(ServiceInstance::new("db-svc", "localhost:5432"));

    registry.set_health("db-svc", false).unwrap();
    assert!(registry.discover("db-svc").is_empty(), "should be unhealthy");

    registry.set_health("db-svc", true).unwrap();
    assert_eq!(registry.discover("db-svc").len(), 1, "should be re-included after recovery");
}

#[test]
fn test_set_health_on_missing_service_returns_error() {
    // Traces to: E3.1
    let registry = ServiceRegistry::new();
    let err = registry.set_health("phantom", true).unwrap_err();
    assert_eq!(err, RegistryError::NotFound("phantom".to_owned()));
}

// ---- Concurrent access ----

#[test]
fn test_registry_is_empty_initially() {
    let registry = ServiceRegistry::new();
    assert!(registry.is_empty());
}

#[test]
fn test_multiple_services_independent() {
    let registry = ServiceRegistry::new();
    registry.register(ServiceInstance::new("svc-a", "localhost:8001"));
    registry.register(ServiceInstance::new("svc-b", "localhost:8002"));
    registry.register(ServiceInstance::new("svc-c", "localhost:8003"));
    assert_eq!(registry.len(), 3);
    assert_eq!(registry.discover("svc-a").len(), 1);
    assert_eq!(registry.discover("svc-b").len(), 1);
}
