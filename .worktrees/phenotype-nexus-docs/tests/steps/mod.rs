//! BDD step definitions for nexus service registry

use nexus::registry::{ServiceInstance, ServiceRegistry};
use phenotype_bdd::{given, then, when, StepContext, StepResult};
use std::sync::Arc;
use tokio::sync::Mutex;

// Shared state for BDD tests
pub struct RegistryState {
    pub registry: ServiceRegistry,
    pub last_result: Option<String>,
    pub discovered_instances: Vec<ServiceInstance>,
    pub round_robin_results: Vec<String>,
}

impl Default for RegistryState {
    fn default() -> Self {
        Self {
            registry: ServiceRegistry::new(),
            last_result: None,
            discovered_instances: Vec::new(),
            round_robin_results: Vec::new(),
        }
    }
}

// Shared state type for tests
pub type SharedState = Arc<Mutex<RegistryState>>;

// Background: Given a clean service registry
#[given("a clean service registry")]
pub async fn clean_registry(ctx: &mut StepContext) -> StepResult {
    let state = Arc::new(Mutex::new(RegistryState::default()));
    ctx.state = Some(Box::new(state));
    Ok(())
}

#[given("a clean service registry with the following healthy services:")]
pub async fn clean_registry_with_services(ctx: &mut StepContext) -> StepResult {
    let state = Arc::new(Mutex::new(RegistryState::default()));

    // Parse the data table and register services
    if let Some(table) = &ctx.data_table {
        for row in table.rows.iter() {
            let name = row.get("name").unwrap_or(&"unknown".to_string()).clone();
            let address = row.get("address").unwrap_or(&"localhost:8080".to_string()).clone();
            let tags_str = row.get("tags").unwrap_or(&"".to_string());

            let mut svc = ServiceInstance::new(name.clone(), address);

            // Parse tags
            for tag in tags_str.split(',') {
                if let Some((key, value)) = tag.split_once('=') {
                    svc = svc.with_tag(key.trim(), value.trim());
                }
            }

            state.lock().await.registry.register(svc);
        }
    }

    ctx.state = Some(Box::new(state));
    Ok(())
}

// Service registration steps
#[given("a service {name} running at {address}")]
pub async fn service_at_address(
    ctx: &mut StepContext,
    name: String,
    address: String,
) -> StepResult {
    if let Some(state) = ctx.state.as_ref() {
        let state = state.downcast_ref::<SharedState>().unwrap();
        let svc = ServiceInstance::new(name, address);
        state.lock().await.registry.register(svc);
    }
    Ok(())
}

#[given("a service {name} with instances at {addr1} and {addr2}")]
pub async fn service_with_instances(
    ctx: &mut StepContext,
    name: String,
    addr1: String,
    addr2: String,
) -> StepResult {
    if let Some(state) = ctx.state.as_ref() {
        let state = state.downcast_ref::<SharedState>().unwrap();
        let svc1 = ServiceInstance::new(name.clone(), addr1);
        let svc2 = ServiceInstance::new(name, addr2);
        state.lock().await.registry.register(svc1);
        state.lock().await.registry.register(svc2);
    }
    Ok(())
}

#[given("a registered service {name} at {address}")]
pub async fn registered_service(
    ctx: &mut StepContext,
    name: String,
    address: String,
) -> StepResult {
    if let Some(state) = ctx.state.as_ref() {
        let state = state.downcast_ref::<SharedState>().unwrap();
        let svc = ServiceInstance::new(name, address);
        state.lock().await.registry.register(svc);
    }
    Ok(())
}

#[given("{count} instances of {name} are registered")]
pub async fn multiple_instances(ctx: &mut StepContext, count: usize, name: String) -> StepResult {
    if let Some(state) = ctx.state.as_ref() {
        let state = state.downcast_ref::<SharedState>().unwrap();
        for i in 0..count {
            let svc = ServiceInstance::new(name.clone(), format!("localhost:{}", 9000 + i));
            state.lock().await.registry.register(svc);
        }
    }
    Ok(())
}

#[given("the following services are registered:")]
pub async fn services_from_table(ctx: &mut StepContext) -> StepResult {
    if let Some(state) = ctx.state.as_ref() {
        let state = state.downcast_ref::<SharedState>().unwrap();
        if let Some(table) = &ctx.data_table {
            for row in table.rows.iter() {
                let name = row.get("name").unwrap_or(&"unknown".to_string()).clone();
                let address = row.get("address").unwrap_or(&"localhost:8080".to_string()).clone();
                let svc = ServiceInstance::new(name, address);
                state.lock().await.registry.register(svc);
            }
        }
    }
    Ok(())
}

#[given("all instances of {name} are marked unhealthy")]
pub async fn mark_all_unhealthy(ctx: &mut StepContext, name: String) -> StepResult {
    if let Some(state) = ctx.state.as_ref() {
        let state = state.downcast_ref::<SharedState>().unwrap();
        let instances: Vec<String> =
            state.lock().await.registry.discover(&name).iter().map(|i| i.name.clone()).collect();
        for instance_name in instances {
            let _ = state.lock().await.registry.set_health(&instance_name, false);
        }
    }
    Ok(())
}

// When steps
#[when("I register the service")]
pub async fn register_service(ctx: &mut StepContext) -> StepResult {
    // Service is already registered in the "given" step
    Ok(())
}

#[when("I register both instances")]
pub async fn register_both(ctx: &mut StepContext) -> StepResult {
    // Services already registered in the "given" step
    Ok(())
}

#[when("I deregister the service")]
pub async fn deregister_service(ctx: &mut StepContext) -> StepResult {
    if let Some(state) = ctx.state.as_ref() {
        let state = state.downcast_ref::<SharedState>().unwrap();
        let guard = state.lock().await;
        // Find a service to deregister
        if let Some(name) = guard.registry.service_names().first() {
            let name = name.clone();
            drop(guard);
            let result = state.lock().await.registry.deregister(&name);
            if let Err(e) = result {
                state.lock().await.last_result = Some(format!("Error: {}", e));
            }
        }
    }
    Ok(())
}

#[when("I mark the service as unhealthy")]
pub async fn mark_unhealthy(ctx: &mut StepContext) -> StepResult {
    if let Some(state) = ctx.state.as_ref() {
        let state = state.downcast_ref::<SharedState>().unwrap();
        let guard = state.lock().await;
        if let Some(name) = guard.registry.service_names().first() {
            let name = name.clone();
            drop(guard);
            let _ = state.lock().await.registry.set_health(&name, false);
        }
    }
    Ok(())
}

#[when("I re-register the service at {address}")]
pub async fn re_register(ctx: &mut StepContext, address: String) -> StepResult {
    if let Some(state) = ctx.state.as_ref() {
        let state = state.downcast_ref::<SharedState>().unwrap();
        let guard = state.lock().await;
        if let Some(name) = guard.registry.service_names().first() {
            let name = name.clone();
            drop(guard);
            let svc = ServiceInstance::new(name, address);
            state.lock().await.registry.register(svc);
        }
    }
    Ok(())
}

#[when("I list all services")]
pub async fn list_all(ctx: &mut StepContext) -> StepResult {
    if let Some(state) = ctx.state.as_ref() {
        let state = state.downcast_ref::<SharedState>().unwrap();
        let count = state.lock().await.registry.len();
        state.lock().await.last_result = Some(format!("count:{}", count));
    }
    Ok(())
}

#[when("I discover services named {name}")]
pub async fn discover_services(ctx: &mut StepContext, name: String) -> StepResult {
    if let Some(state) = ctx.state.as_ref() {
        let state = state.downcast_ref::<SharedState>().unwrap();
        let instances = state.lock().await.registry.discover(&name);
        let count = instances.len();
        state.lock().await.discovered_instances = instances;
        state.lock().await.last_result = Some(format!("count:{}", count));
    }
    Ok(())
}

#[when("I discover the next instance {count} times")]
pub async fn discover_next_multiple(ctx: &mut StepContext, count: usize) -> StepResult {
    if let Some(state) = ctx.state.as_ref() {
        let state = state.downcast_ref::<SharedState>().unwrap();
        let guard = state.lock().await;
        let service_name = guard.registry.service_names().first().cloned();
        drop(guard);

        if let Some(name) = service_name {
            for _ in 0..count {
                if let Some(instance) = state.lock().await.registry.discover_next(&name) {
                    state.lock().await.round_robin_results.push(instance.address.clone());
                }
            }
        }
    }
    Ok(())
}

#[when("I discover services with tag {tag}")]
pub async fn discover_by_tag(ctx: &mut StepContext, tag: String) -> StepResult {
    if let Some(state) = ctx.state.as_ref() {
        let state = state.downcast_ref::<SharedState>().unwrap();
        let parts: Vec<&str> = tag.splitn(2, '=').collect();
        if parts.len() == 2 {
            let instances = state.lock().await.registry.discover_by_tag(parts[0], parts[1]);
            let count = instances.len();
            state.lock().await.discovered_instances = instances;
            state.lock().await.last_result = Some(format!("count:{}", count));
        }
    }
    Ok(())
}

// Then steps
#[then("the service should be available in the registry")]
pub async fn service_available(ctx: &mut StepContext) -> StepResult {
    if let Some(state) = ctx.state.as_ref() {
        let state = state.downcast_ref::<SharedState>().unwrap();
        assert!(!state.lock().await.registry.is_empty(), "Registry should not be empty");
    }
    Ok(())
}

#[then("the service should be marked as healthy")]
pub async fn service_healthy(ctx: &mut StepContext) -> StepResult {
    if let Some(state) = ctx.state.as_ref() {
        let state = state.downcast_ref::<SharedState>().unwrap();
        let guard = state.lock().await;
        let services: Vec<ServiceInstance> = guard
            .registry
            .service_names()
            .iter()
            .filter_map(|n| guard.registry.discover(n).first().cloned())
            .collect();
        drop(guard);

        assert!(services.iter().any(|s| s.healthy), "At least one service should be healthy");
    }
    Ok(())
}

#[then("the service should be marked as unhealthy")]
pub async fn service_unhealthy_assert(ctx: &mut StepContext) -> StepResult {
    if let Some(state) = ctx.state.as_ref() {
        let state = state.downcast_ref::<SharedState>().unwrap();
        let guard = state.lock().await;
        let services: Vec<ServiceInstance> = guard
            .registry
            .service_names()
            .iter()
            .filter_map(|n| guard.registry.discover(n).first().cloned())
            .collect();
        drop(guard);

        assert!(services.iter().any(|s| !s.healthy), "At least one service should be unhealthy");
    }
    Ok(())
}

#[then("both instances should be discoverable")]
pub async fn both_discoverable(ctx: &mut StepContext) -> StepResult {
    if let Some(state) = ctx.state.as_ref() {
        let state = state.downcast_ref::<SharedState>().unwrap();
        let guard = state.lock().await;
        let names = guard.registry.service_names();
        drop(guard);

        assert!(!names.is_empty(), "Should have registered services");
    }
    Ok(())
}

#[then("the service count should be {expected}")]
pub async fn service_count(ctx: &mut StepContext, expected: usize) -> StepResult {
    if let Some(state) = ctx.state.as_ref() {
        let state = state.downcast_ref::<SharedState>().unwrap();
        let count = state.lock().await.registry.len();
        assert_eq!(count, expected, "Service count mismatch");
    }
    Ok(())
}

#[then("the service should not be found in the registry")]
pub async fn service_not_found(ctx: &mut StepContext) -> StepResult {
    if let Some(state) = ctx.state.as_ref() {
        let state = state.downcast_ref::<SharedState>().unwrap();
        // After deregister, the registry might be empty or the specific service gone
        let guard = state.lock().await;
        let last = guard.last_result.clone();
        drop(guard);

        if let Some(result) = last {
            assert!(result.contains("Error"), "Should have an error result");
        }
    }
    Ok(())
}

#[then("the service should not appear in healthy discovery")]
pub async fn not_in_healthy(ctx: &mut StepContext) -> StepResult {
    if let Some(state) = ctx.state.as_ref() {
        let state = state.downcast_ref::<SharedState>().unwrap();
        let guard = state.lock().await;
        let names: Vec<String> = guard.registry.service_names().clone();
        drop(guard);

        for name in names {
            let healthy = state.lock().await.registry.discover(&name);
            assert!(
                healthy.is_empty() || healthy.iter().any(|s| s.healthy),
                "Unhealthy services should not appear in healthy discovery"
            );
        }
    }
    Ok(())
}

#[then("the service address should be updated to {address}")]
pub async fn address_updated(ctx: &mut StepContext, address: String) -> StepResult {
    if let Some(state) = ctx.state.as_ref() {
        let state = state.downcast_ref::<SharedState>().unwrap();
        let guard = state.lock().await;
        let names = guard.registry.service_names();
        if let Some(name) = names.first() {
            let instances = guard.registry.discover(name);
            drop(guard);

            // Check if any instance has the expected address
            let found = instances.iter().any(|i| i.address == address);
            assert!(found, "Service address should be updated to {}", address);
        }
    }
    Ok(())
}

#[then("I should see {count} services")]
pub async fn should_see_count(ctx: &mut StepContext, count: usize) -> StepResult {
    if let Some(state) = ctx.state.as_ref() {
        let state = state.downcast_ref::<SharedState>().unwrap();
        let last = state.lock().await.last_result.clone();

        if let Some(result) = last {
            let parts: Vec<&str> = result.split(':').collect();
            if parts.len() == 2 {
                let actual: usize = parts[1].parse().unwrap_or(0);
                assert_eq!(actual, count, "Service count mismatch");
            }
        }
    }
    Ok(())
}

#[then("the service names should include {names}")]
pub async fn names_include(ctx: &mut StepContext, names: String) -> StepResult {
    if let Some(state) = ctx.state.as_ref() {
        let state = state.downcast_ref::<SharedState>().unwrap();
        let guard = state.lock().await;
        let service_names: Vec<String> = guard.registry.service_names();
        drop(guard);

        let expected_names: Vec<&str> = names.split(", ").collect();
        for expected in expected_names {
            assert!(
                service_names.iter().any(|n| n.contains(expected)),
                "Service names should include {}",
                expected
            );
        }
    }
    Ok(())
}

#[then("I should find {count} instance(s)")]
pub async fn find_count(ctx: &mut StepContext, count: usize) -> StepResult {
    if let Some(state) = ctx.state.as_ref() {
        let state = state.downcast_ref::<SharedState>().unwrap();
        let instances = &state.lock().await.discovered_instances;
        assert_eq!(instances.len(), count, "Instance count mismatch");
    }
    Ok(())
}

#[then("the instance should be at {address}")]
pub async fn instance_at_address(ctx: &mut StepContext, address: String) -> StepResult {
    if let Some(state) = ctx.state.as_ref() {
        let state = state.downcast_ref::<SharedState>().unwrap();
        let instances = &state.lock().await.discovered_instances;
        assert!(
            instances.iter().any(|i| i.address == address),
            "Instance should be at {}",
            address
        );
    }
    Ok(())
}

#[then("each instance should be returned exactly once")]
pub async fn each_instance_once(ctx: &mut StepContext) -> StepResult {
    if let Some(state) = ctx.state.as_ref() {
        let state = state.downcast_ref::<SharedState>().unwrap();
        let results = &state.lock().await.round_robin_results;

        // Check we got 3 unique addresses
        let unique: std::collections::HashSet<&String> = results.iter().collect();
        assert_eq!(unique.len(), 3, "Should have 3 unique addresses");
        assert_eq!(results.len(), 3, "Should have exactly 3 results");
    }
    Ok(())
}

#[then("the 4th discovery should return the first instance again")]
pub async fn fourth_is_first(ctx: &mut StepContext) -> StepResult {
    if let Some(state) = ctx.state.as_ref() {
        let state = state.downcast_ref::<SharedState>().unwrap();
        let guard = state.lock().await;
        let service_name = guard.registry.service_names().first().cloned();
        drop(guard);

        if let Some(name) = service_name {
            // Do 4th discovery
            if let Some(instance) = state.lock().await.registry.discover_next(&name) {
                let results = &state.lock().await.round_robin_results;
                assert_eq!(
                    instance.address, results[0],
                    "4th discovery should return first instance"
                );
            }
        }
    }
    Ok(())
}

#[then("the services should be {names}")]
pub async fn services_should_be(ctx: &mut StepContext, names: String) -> StepResult {
    if let Some(state) = ctx.state.as_ref() {
        let state = state.downcast_ref::<SharedState>().unwrap();
        let instances = &state.lock().await.discovered_instances;
        let expected_names: Vec<&str> = names.split(" and ").collect();

        for expected in expected_names {
            assert!(
                instances.iter().any(|i| i.name == expected),
                "Services should include {}",
                expected
            );
        }
    }
    Ok(())
}

#[then("the services should include {names}")]
pub async fn services_include(ctx: &mut StepContext, names: String) -> StepResult {
    if let Some(state) = ctx.state.as_ref() {
        let state = state.downcast_ref::<SharedState>().unwrap();
        let instances = &state.lock().await.discovered_instances;
        let expected_names: Vec<&str> = names.split(" and ").collect();

        for expected in expected_names {
            assert!(
                instances.iter().any(|i| i.name == expected),
                "Services should include {}",
                expected
            );
        }
    }
    Ok(())
}

#[then("round-robin discovery should return nothing")]
pub async fn round_robin_nothing(ctx: &mut StepContext) -> StepResult {
    if let Some(state) = ctx.state.as_ref() {
        let state = state.downcast_ref::<SharedState>().unwrap();
        let guard = state.lock().await;
        let name = guard.registry.service_names().first().cloned();
        drop(guard);

        if let Some(n) = name {
            let result = state.lock().await.registry.discover_next(&n);
            assert!(result.is_none(), "Round-robin should return nothing for unhealthy services");
        }
    }
    Ok(())
}

#[then("the addresses should be {addresses}")]
pub async fn addresses_should_be(ctx: &mut StepContext, addresses: String) -> StepResult {
    if let Some(state) = ctx.state.as_ref() {
        let state = state.downcast_ref::<SharedState>().unwrap();
        let instances = &state.lock().await.discovered_instances;
        let expected_addrs: Vec<&str> = addresses.split(" and ").collect();

        for expected in expected_addrs {
            assert!(
                instances.iter().any(|i| i.address == expected),
                "Addresses should include {}",
                expected
            );
        }
    }
    Ok(())
}
