# Product Requirements Document - Settly

**Version:** 1.0.0  
**Date:** 2026-04-05  
**Status:** Draft  
**Owner:** Phenotype Team  

---

## 1. Overview

### 1.1 Purpose

Settly is a universal configuration management framework for Rust applications, providing layered configs, validation, and environment support following hexagonal architecture principles.

### 1.2 Target Market

| Segment | Use Case | Primary Benefits |
|---------|----------|------------------|
| Rust Developers | Type-safe configuration | Compile-time checking, IDE support |
| Library Authors | Pluggable configuration | No coupling to specific config format |
| Platform Teams | Standardized config | Consistent patterns across services |
| DevOps | Environment management | Easy dev/staging/prod switching |

### 1.3 Success Metrics

| Metric | Current | Target | Measurement |
|--------|---------|--------|-------------|
| Configuration errors in production | 10/week | 1/week | Error tracking |
| Time to add new config key | 30 min | 5 min | Developer survey |
| Config-related bugs | 5/week | 1/week | Bug tracking |
| Library adoption | 0 | 10 crates | Crates.io downloads |

---

## 2. Problem Analysis

### 2.1 Pain Points

| Pain Point | Description | Frequency | Severity |
|------------|-------------|----------|----------|
| Scattered Config | Config from multiple sources hard to track | Daily | High |
| No Validation | Bad config causes runtime panics | Weekly | Critical |
| Type Errors | Wrong types cause crashes | Weekly | High |
| Environment Bugs | Dev settings leak to production | Monthly | Critical |
| Hot Reload | Need restart for config changes | Daily | Medium |

### 2.2 User Stories

| ID | Story | As A | I Want | So That |
|----|-------|------|--------|---------|
| US-001 | Layered Configs | Developer | Merge configs from files, env, CLI | Priority-based override works |
| US-002 | Type Safety | Developer | Get type-safe config values | Compiler catches type errors |
| US-003 | Validation | Developer | Validate config before use | Fail fast on bad config |
| US-004 | Environment Override | DevOps | Override settings per environment | prod/dev/staging work |
| US-005 | Hot Reload | Developer | Reload config without restart | Faster iteration |
| US-006 | Schema Validation | Platform Engineer | Define config schema | Catch errors early |

---

## 3. Product Goals

### 3.1 Primary Goals (2026)

| Goal | Key Result | Due Date |
|------|------------|----------|
| Core Framework | Layered config with priority merge | Q2 2026 |
| Type Safety | Macro-based config struct derivation | Q2 2026 |
| Validation | JSON Schema + custom validators | Q2 2026 |
| Format Support | TOML, YAML, JSON, ENV | Q2 2026 |
| Hot Reload | File watching + reload | Q3 2026 |

### 3.2 Secondary Goals (2027)

| Goal | Key Result | Due Date |
|------|------------|----------|
| Secret Integration | HashiCorp Vault, AWS SSM | Q1 2027 |
| Migration | Config versioning and migration | Q1 2027 |
| Framework Integration | Axum, Actix extractors | Q2 2027 |
| SDKs | Python, JS bindings | Q2 2027 |

---

## 4. Functional Requirements

### 4.1 Core Features

#### FR-001: Layered Configuration

**Description**: Support multiple configuration sources with priority-based merging.

**Acceptance Criteria**:
- Config can be loaded from files, environment variables, CLI arguments
- Priority order: CLI > ENV > File > Default
- Deep merge for nested objects
- Array replacement (not merge)
- Clear error messages for merge conflicts

**Priority**: P0

#### FR-002: Type-Safe Access

**Description**: Provide type-safe getters for configuration values.

**Acceptance Criteria**:
- `config.get<T>("path")` returns `Result<T, ConfigError>`
- Support types: String, u8-u64, i8-i64, f32, f64, bool
- Support nested access with dot notation: "database.host"
- Default values with `.unwrap_or()` or `.get_or_insert()`
- Path not found returns specific error

**Priority**: P0

#### FR-003: Format Support

**Description**: Parse configuration from multiple file formats.

**Acceptance Criteria**:
- TOML: Full support via `toml` crate
- YAML: Full support via `serde_yaml`
- JSON: Full support via `serde_json`
- ENV: Key=Value pairs with nested dots

**Priority**: P0

#### FR-004: Validation

**Description**: Validate configuration values against schemas.

**Acceptance Criteria**:
- JSON Schema validation
- Built-in validators: required, type, range, pattern, enum
- Custom validator support via trait
- Validation errors include path, message, value
- Optional validation (fail-fast vs collect all errors)

**Priority**: P0

#### FR-005: Environment Support

**Description**: Environment-specific configuration overrides.

**Acceptance Criteria**:
- Environment variable prefix filtering
- Double-underscore for nested keys: `APP__DB__HOST`
- APP_ENV for environment selection
- Built-in environments: development, staging, production

**Priority**: P0

#### FR-006: Hot Reload

**Description**: Watch configuration files and reload on change.

**Acceptance Criteria**:
- File system watcher using `notify`
- Debounce rapid changes (500ms default)
- Callback on config change
- Graceful handling of invalid intermediate states
- Memory-safe reload (no locks during reload)

**Priority**: P1

### 4.2 Feature Priority Matrix

| Feature | User Value | Effort | Priority |
|---------|-----------|--------|----------|
| Layered Configs | High | Medium | P0 |
| Type Safety | High | Medium | P0 |
| Format Support | High | Low | P0 |
| Validation | High | Medium | P0 |
| Environment Override | Medium | Low | P0 |
| Hot Reload | Medium | Medium | P1 |
| Secret Integration | Medium | High | P2 |
| Migration | Low | High | P2 |

---

## 5. Non-Functional Requirements

### 5.1 Performance

| Requirement | Target | Measurement |
|-------------|--------|-------------|
| Config build (3 sources) | < 10ms | Benchmark |
| Config get | < 1μs | Benchmark |
| Memory (100 keys) | < 100KB | Profiling |

### 5.2 Reliability

| Requirement | Target |
|-------------|--------|
| No panics on invalid config | 100% |
| Graceful degradation | Always |
| Error messages actionable | 100% |

### 5.3 Usability

| Requirement | Target |
|-------------|--------|
| Compile-time errors for typos | IDE support |
| Documentation examples | 10+ |
| Error message clarity | < 80 chars |

---

## 6. Milestones

| Milestone | Target | Deliverables |
|-----------|--------|--------------|
| M1: Core | 2026-05-01 | Layered config, type-safe access |
| M2: Validation | 2026-06-01 | JSON Schema, custom validators |
| M3: Ecosystem | 2026-07-01 | Hot reload, secret stores |
| M4: GA | 2026-09-01 | v1.0 release |

---

## 7. Success Criteria

### 7.1 Feature Complete

- [ ] All P0 features implemented
- [ ] All P1 features implemented or deferred

### 7.2 Quality Gates

- [ ] 80% code coverage
- [ ] No compiler warnings
- [ ] All tests pass
- [ ] Documentation complete

### 7.3 Adoption Criteria

- [ ] 5 internal crates using Settly
- [ ] crates.io publication
- [ ] Documentation site

---

**End of PRD**
