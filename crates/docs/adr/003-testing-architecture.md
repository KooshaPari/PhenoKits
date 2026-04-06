# ADR 003: Testing and Quality Assurance Architecture

## Status
========================================

**Accepted**

## Context
========================================

The Phenotype crates ecosystem requires comprehensive testing and quality assurance to maintain reliability across 40+ interdependent crates. The testing strategy must address:

1. **Unit testing**: Individual crate correctness
2. **Integration testing**: Cross-crate interaction validation
3. **Contract testing**: API stability and backward compatibility
4. **Performance testing**: Compile-time and runtime benchmarks
5. **Security testing**: Vulnerability scanning and audit compliance

### Current Gaps

| Gap | Impact | Urgency |
|-----|--------|---------|
| No unified test harness | Inconsistent testing across crates | High |
| Limited integration coverage | Breaking changes discovered late | High |
| No performance baselines | Performance regressions undetected | Medium |
| Manual SemVer checking | Accidental breaking changes released | High |
| No mutation testing | Test quality unknown | Low |
| Limited BDD coverage | Business requirements untraced | Medium |

### Objectives

1. **Fast feedback**: Sub-minute test execution for local development
2. **Complete coverage**: All crates tested in CI on every PR
3. **Breaking change detection**: Automated API compatibility checking
4. **Performance tracking**: Benchmark regression detection
5. **Security assurance**: Continuous vulnerability monitoring

## Decision
========================================

We will implement a **layered testing architecture** with the following structure:

### Layer 1: Fast Unit Tests (cargo-nextest)

Standard unit tests with parallel execution:

```rust
// phenotype-core/src/entity_id.rs
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn entity_id_generates_unique_ids() {
        let id1 = EntityId::<User>::new();
        let id2 = EntityId::<User>::new();
        assert_ne!(id1, id2);
    }
    
    #[test]
    fn entity_id_parses_valid_uuid() {
        let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
        let id = EntityId::<User>::parse(uuid_str);
        assert!(id.is_some());
    }
}
```

Configuration:

```toml
# .config/nextest.toml
[profile.default]
retries = 1                          # Retry flaky tests
fail-fast = false                    # Run all tests even if some fail
status-level = "skip"                # Show skip reasons

[profile.ci]
fail-fast = true
retries = 0
```

### Layer 2: Integration Tests (phenotype-test-infra)

Centralized integration testing infrastructure:

```rust
// tests/integration/state_machine_flow.rs
use phenotype_test_infra::{TestContext, IntegrationTest};
use phenotype_state_machine::{StateMachine, State, Event};

#[tokio::test]
async fn order_state_machine_lifecycle() {
    let ctx = TestContext::new().await;
    
    let machine = OrderStateMachine::new();
    let order = ctx.create_test_order().await;
    
    // Pending -> Confirmed
    machine.transition(order.id, OrderEvent::Confirm).await?;
    assert_eq!(machine.state(order.id).await?, OrderState::Confirmed);
    
    // Confirmed -> Shipped
    machine.transition(order.id, OrderEvent::Ship).await?;
    assert_eq!(machine.state(order.id).await?, OrderState::Shipped);
}
```

Features:

- TestContainers for database/service dependencies
- Async test runtime (tokio)
- Shared test fixtures (phenotype-test-fixtures)
- Parallel test isolation

### Layer 3: Contract Tests (phenotype-contract + BDD)

API contract validation using cucumber:

```gherkin
# tests/bdd/features/health_checks.feature
Feature: Health Check System
  As an operator
  I want to verify system health
  So that I can detect and respond to failures

  Scenario: All services healthy
    Given the system is running
    And all external dependencies are available
    When I request the health status
    Then the response status should be 200
    And the overall status should be "healthy"
    And all component checks should pass

  Scenario: Database unavailable
    Given the system is running
    And the database is unavailable
    When I request the health status
    Then the response status should be 503
    And the database check should fail
```

```rust
// tests/bdd/steps/health_steps.rs
use cucumber::{given, when, then};
use phenotype_bdd::{World, Result};

#[given("the system is running")]
async fn system_running(world: &mut World) -> Result {
    world.system = TestSystem::start().await?;
    Ok(())
}

#[when("I request the health status")]
async fn request_health(world: &mut World) -> Result {
    world.response = world.system.health_check().await?;
    Ok(())
}

#[then("the response status should be {int}")]
async fn check_status(world: &mut World, expected: u16) -> Result {
    assert_eq!(world.response.status, expected);
    Ok(())
}
```

### Layer 4: API Compatibility (cargo-semver-checks)

Automated SemVer violation detection:

```bash
# Check for breaking changes
cargo semver-checks

# Output:
# ---
# Crate: phenotype-core
# Version: 0.2.0 -> 0.2.1
# 
# ---
# Trait method added to public trait (minor change)
# ---
# Enum variant removed: PhenotypeError::LegacyVariant (MAJOR breaking change!)
```

CI integration:

```yaml
# .github/workflows/pr.yml
jobs:
  semver:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: obi1kenobi/cargo-semver-checks-action@v2
        with:
          feature-group: default-features
```

### Layer 5: Performance Benchmarks (criterion.rs)

```rust
// benches/entity_id.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use phenotype_core::EntityId;

fn bench_entity_id_creation(c: &mut Criterion) {
    c.bench_function("entity_id_new", |b| {
        b.iter(|| EntityId::<String>::new())
    });
}

fn bench_entity_id_parsing(c: &mut Criterion) {
    let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
    c.bench_function("entity_id_parse", |b| {
        b.iter(|| EntityId::<String>::parse(black_box(uuid_str)))
    });
}

criterion_group!(benches, bench_entity_id_creation, bench_entity_id_parsing);
criterion_main!(benches);
```

CI tracking:

```yaml
# .github/workflows/bench.yml
- uses: benchmark-action/github-action-benchmark@v1
  with:
    tool: 'cargo'
    output-file-path: target/criterion/report.json
    github-token: ${{ secrets.GITHUB_TOKEN }}
    auto-push: true
```

### Layer 6: Security Audit (cargo-audit + cargo-deny)

```toml
# deny.toml
[advisories]
ignore = [
    # Dev-only dependency, not in production
    "RUSTSEC-2021-0145", # atty potential misalignment
]

[licenses]
allow = ["MIT", "Apache-2.0", "BSD-3-Clause"]
deny = ["GPL-2.0", "GPL-3.0"]

[bans]
multiple-versions = "warn"
skip = [
    # Acceptable: different major versions
    { name = "syn", version = "1.0" },
]
```

```yaml
# .github/workflows/audit.yml
- uses: rustsec/audit-check@v1
  with:
    token: ${{ secrets.GITHUB_TOKEN }}
- uses: EmbarkStudios/cargo-deny-action@v1
```

## Consequences
========================================

### Positive

1. **Fast iteration**: cargo-nextest provides 10x faster test execution
2. **Confidence**: Multiple layers catch different defect classes
3. **Automation**: CI enforces quality gates without manual intervention
4. **Documentation**: BDD tests serve as living documentation
5. **Prevention**: SemVer checking prevents accidental breaking changes

### Negative

1. **Maintenance burden**: Multiple test frameworks to learn and maintain
2. **CI time**: Comprehensive testing increases build time
3. **Flakiness**: Integration tests may be non-deterministic
4. **False positives**: Security scanning may flag acceptable risks

### Mitigations

1. **Test categorization**: Use test tags to run subsets locally
2. **Parallel CI**: Sharded test execution across multiple runners
3. **Retry logic**: Automatic retry for known-flaky tests
4. **Exception process**: Documented workflow for security scan exceptions

## Implementation
========================================

### Phase 1: Infrastructure Crates

Create testing infrastructure crates:

```bash
cargo new --lib crates/phenotype-testing
cargo new --lib crates/phenotype-test-infra
cargo new --lib crates/phenotype-test-fixtures
cargo new --lib crates/phenotype-bdd
cargo new --lib crates/phenotype-contract
```

### Phase 2: CI/CD Integration

```yaml
# .github/workflows/quality.yml
name: Quality Gates

on: [push, pull_request]

jobs:
  test:
    strategy:
      matrix:
        crate: [core, errors, logging, telemetry]
    steps:
      - uses: taiki-e/install-action@cargo-nextest
      - run: cargo nextest run -p phenotype-${{ matrix.crate }}

  integration:
    needs: test
    steps:
      - run: cargo test --test '*' -- --test-threads=1

  semver:
    needs: test
    steps:
      - uses: obi1kenobi/cargo-semver-checks-action@v2

  audit:
    needs: test
    steps:
      - uses: rustsec/audit-check@v1
```

### Phase 3: Local Development

```bash
# Makefile for local quality checks
quality:
	cargo fmt --check
	cargo clippy --workspace --all-targets -- -D warnings
	cargo nextest run --workspace
	cargo semver-checks
	cargo audit
	cargo deny check
```

### Phase 4: Documentation

Document testing approach in each crate:

```markdown
# phenotype-core Testing

## Unit Tests
Run with: `cargo nextest run -p phenotype-core`

## Integration Tests
Run with: `cargo test --test integration -p phenotype-core`

## Benchmarks
Run with: `cargo bench -p phenotype-core`
```

## Testing Pyramid
========================================

```
                    /\
                   /  \
                  / E2E \      phenotype-bdd (cucumber)
                 /--------\
                /          \
               / Integration \  phenotype-test-infra
              /----------------\
             /                  \
            /     Unit Tests      \  cargo-nextest
           /------------------------\
          /                            \
         /   Static Analysis (clippy, fmt) \
        /------------------------------------\
```

Target distribution:

| Layer | Target % | Tool |
|-------|----------|------|
| Static | 100% code | clippy, rustfmt |
| Unit | 80% coverage | cargo-nextest |
| Integration | Critical paths | phenotype-test-infra |
| E2E | User journeys | phenotype-bdd |

## Success Metrics
========================================

| Metric | Target | Measurement |
|--------|--------|-------------|
| Unit test execution | < 60s | `cargo nextest run --workspace` |
| Integration test execution | < 5min | CI pipeline |
| Code coverage | > 80% | tarpaulin |
| SemVer violations in releases | 0 | cargo-semver-checks |
| Security advisories | < 7 days to resolution | rustsec tracking |
| Flaky tests | < 1% | CI analytics |

## References
========================================

- cargo-nextest: https://nexte.st/
- cargo-semver-checks: https://github.com/obi1kenobi/cargo-semver-checks
- cucumber-rs: https://cucumber-rs.github.io/
- criterion.rs: https://bheisler.github.io/criterion.rs/book/
- TestContainers: https://testcontainers.com/

## Decision Log
========================================

| Date | Event |
|------|-------|
| 2024-01-10 | Initial testing strategy proposed |
| 2024-01-17 | Evaluated cargo-test vs cargo-nextest; selected nextest |
| 2024-01-24 | Added BDD layer after contract testing discussion |
| 2024-02-01 | Integrated cargo-semver-checks into quality gates |
| 2024-02-10 | ADR accepted; implementation in progress |

**Traceability:** `/// @trace CRATES-ADR-003`
