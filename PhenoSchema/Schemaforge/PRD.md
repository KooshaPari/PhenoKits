# Product Requirements Document - Schemaforge

**Version:** 1.0.0  
**Date:** 2026-04-05  
**Status:** Draft  
**Owner:** Phenotype Team  

---

## 1. Overview

### 1.1 Purpose

Schemaforge is a schema management and validation framework designed to solve the problem of schema drift and compatibility in distributed systems. It provides a centralized system for defining, validating, and evolving data schemas across polyglot service environments.

### 1.2 Target Market

| Segment | Use Case | Primary Benefits |
|---------|----------|------------------|
| Platform Teams | Standardize organization-wide data contracts | Consistency, governance |
| API Providers | Define and document data formats | Developer experience, compatibility |
| Service Teams | Validate inter-service communication | Reliability, fail-fast |
| DevOps | Automate schema validation in CI/CD | Velocity, confidence |

### 1.3 Success Metrics

| Metric | Current | Target | Measurement |
|--------|---------|--------|-------------|
| Schema drift incidents | 5/week | 0/week | Incident tracking |
| Time to validate schema change | 2 hours | 5 minutes | CI pipeline timing |
| Breaking change detection rate | 60% | 99% | Automated testing |
| Schema registry adoption | 0% | 80% | Service coverage |

---

## 2. Problem Analysis

### 2.1 Pain Points

| Pain Point | Description | Frequency | Severity |
|------------|-------------|----------|----------|
| Schema Drift | Services evolve independently, causing incompatibility | Weekly | Critical |
| Manual Validation | Engineers manually check schema compatibility | Daily | High |
| No Central Registry | No single source of truth for organizational schemas | Daily | High |
| Migration Fear | Teams avoid necessary schema evolution due to break risk | Monthly | Medium |
| Format Silos | JSON Schema, Protobuf, GraphQL managed separately | Constant | Medium |
| Documentation Rot | Schema documentation falls out of sync with code | Weekly | Medium |

### 2.2 Root Causes

| Cause | Effect | Solution |
|-------|--------|----------|
| No schema ownership | Schemas evolve without coordination | Registry with ownership |
| Manual processes | Errors and delays | Automated validation |
| Format fragmentation | Inconsistent tooling | Unified abstraction |
| No compatibility tracking | Breaking changes discovered late | Version compatibility matrix |
| Poor diff visibility | Hard to understand schema evolution | Visual diff tools |

### 2.3 User Stories

| ID | Story | As A | I Want | So That |
|----|-------|------|--------|---------|
| US-001 | Schema Registration | Platform Engineer | Register schemas with version history | I can track schema evolution |
| US-002 | Automated Validation | Developer | Validate data against schemas automatically | My services stay compatible |
| US-003 | Compatibility Checking | API Producer | Check if my change is backward compatible | I don't break consumers |
| US-004 | Migration Planning | Developer | Generate migration paths automatically | Schema evolution is safe |
| US-005 | Multi-Format Support | Platform Engineer | Support JSON Schema, Protobuf, GraphQL | One tool for all formats |
| US-006 | Registry Search | API Consumer | Discover available schemas | I can find data contracts |
| US-007 | Change Notifications | Team Lead | Get notified of schema changes | My team stays updated |
| US-008 | CLI Integration | DevOps | Use schema validation in CI/CD | Automated quality gates |

---

## 3. Product Goals

### 3.1 Primary Goals (2026)

| Goal | Key Result | Due Date |
|------|------------|----------|
| Schema Registry | PostgreSQL-backed registry with 100K+ schema capacity | Q2 2026 |
| JSON Schema Validation | Full draft-07 and 2020-12 support | Q2 2026 |
| CI/CD Integration | GitHub Actions, GitLab CI plugins | Q3 2026 |
| Multi-Format | Protobuf and GraphQL support | Q3 2026 |
| Breaking Change Detection | 99% accuracy on common patterns | Q3 2026 |

### 3.2 Secondary Goals (2027)

| Goal | Key Result | Due Date |
|------|------------|----------|
| Migration Automation | Auto-generate migration code | Q1 2027 |
| Schema Evolution Policies | Organization-level governance | Q1 2027 |
| SDK Coverage | Rust, JS, Python, Go, Java SDKs | Q2 2027 |
| Service Mesh Integration | Istio, Linkerd policies | Q2 2027 |

### 3.3 Non-Goals (Out of Scope)

| Item | Reason |
|------|--------|
| Data transformation | Separate concern (use Dataform, dbt) |
| Data storage | Use existing databases |
| API gateway | Use existing solutions (Kong, Envoy) |
| Message queue schemas | Consider for v2 |

---

## 4. Functional Requirements

### 4.1 Core Features

#### FR-001: Schema Registration

**Description**: Store and version schemas in the registry

**Acceptance Criteria**:
- Schemas can be published with name, version, format, content
- Each version is immutable once published
- Duplicate versions (same name+version) are rejected
- Schema content is validated before storage
- Metadata (author, tags, description) is captured

**Priority**: P0

#### FR-002: JSON Schema Validation

**Description**: Validate JSON data against JSON Schema

**Acceptance Criteria**:
- Support JSON Schema draft-07
- Support JSON Schema 2019-09 and 2020-12
- All validation keywords implemented
- Custom format validators supported
- $ref resolution (local and remote)
- Detailed validation error messages

**Priority**: P0

#### FR-003: Schema Versioning

**Description**: Manage multiple versions of schemas

**Acceptance Criteria**:
- Semantic versioning (major.minor.patch)
- List all versions of a schema
- Get specific version by exact version number
- Get latest version
- Deprecate old versions
- Track superseded relationships

**Priority**: P0

#### FR-004: Compatibility Checking

**Description**: Detect breaking changes between versions

**Acceptance Criteria**:
- Backward compatibility check (producer perspective)
- Forward compatibility check (consumer perspective)
- Full compatibility check (bidirectional)
- Detailed change report with severity
- Configurable compatibility rules

**Priority**: P0

#### FR-005: Schema Search

**Description**: Discover schemas in the registry

**Acceptance Criteria**:
- Search by schema name (prefix, exact)
- Filter by format (JSON Schema, Protobuf, etc.)
- Filter by tag
- Filter by author
- Filter by deprecation status
- Sort by name, date, version

**Priority**: P1

#### FR-006: Protobuf Schema Support

**Description**: Parse and validate Protocol Buffer schemas

**Acceptance Criteria**:
- Parse .proto files
- Validate .proto syntax
- Extract message definitions
- Check compatibility between proto versions
- Generate JSON Schema from proto

**Priority**: P1

#### FR-007: GraphQL Schema Support

**Description**: Parse and validate GraphQL schemas

**Acceptance Criteria**:
- Parse GraphQL SDL
- Validate type definitions
- Extract object types and fields
- Check breaking changes (type removal, etc.)
- Generate JSON Schema from GraphQL

**Priority**: P1

#### FR-008: Migration Planning

**Description**: Generate migration paths between versions

**Acceptance Criteria**:
- Detect migration steps needed
- Generate migration code (where possible)
- Identify reversible migrations
- Estimate migration complexity
- Validate migration safety

**Priority**: P1

#### FR-009: CLI Interface

**Description**: Command-line interface for schema operations

**Acceptance Criteria**:
- `schemaforge validate` - Validate data against schema
- `schemaforge publish` - Publish schema to registry
- `schemaforge diff` - Show differences between versions
- `schemaforge check` - Check compatibility
- `schemaforge search` - Search registry
- `schemaforge migrate` - Generate migration

**Priority**: P0

#### FR-010: REST API

**Description**: HTTP API for programmatic access

**Acceptance Criteria**:
- CRUD operations for schemas
- Compatibility checking endpoint
- Validation endpoint
- Search endpoint
- Health check endpoint
- OpenAPI 3.0 specification

**Priority**: P1

### 4.2 Feature Priority Matrix

| Feature | User Value | Technical Effort | Priority |
|---------|-----------|------------------|----------|
| Schema Registration | High | Medium | P0 |
| JSON Schema Validation | High | Medium | P0 |
| Schema Versioning | High | Low | P0 |
| Compatibility Checking | High | Medium | P0 |
| CLI Interface | High | Low | P0 |
| Schema Search | Medium | Low | P1 |
| REST API | Medium | Medium | P1 |
| Protobuf Support | Medium | High | P1 |
| GraphQL Support | Medium | High | P1 |
| Migration Planning | Medium | High | P1 |

---

## 5. Non-Functional Requirements

### 5.1 Performance

| Requirement | Target | Measurement |
|-------------|--------|-------------|
| Schema publish latency | < 200ms p95 | APM tracing |
| Validation latency (1KB schema) | < 50ms p95 | Benchmark |
| Registry query latency | < 20ms p95 | APM tracing |
| Concurrent publishers | 100 | Load test |
| Registry capacity | 100,000 schemas | Capacity planning |

### 5.2 Reliability

| Requirement | Target | Measurement |
|-------------|--------|-------------|
| API availability | 99.9% | Uptime monitoring |
| Data durability | 99.999% | Storage replication |
| Consistency | Strong | Distributed systems testing |
| Recovery time | < 5 min | Disaster recovery test |

### 5.3 Security

| Requirement | Implementation |
|-------------|----------------|
| Authentication | API key, JWT |
| Authorization | RBAC (4 roles) |
| Encryption | TLS 1.3, AES-256 at rest |
| Audit logging | All write operations |
| Input validation | Sanitization, size limits |

### 5.4 Observability

| Requirement | Implementation |
|-------------|----------------|
| Metrics | Prometheus/OpenMetrics |
| Tracing | OpenTelemetry |
| Logging | Structured JSON |
| Health checks | /health, /ready endpoints |

---

## 6. User Flows

### 6.1 Schema Publication Flow

```
User                    CLI/API              Registry               Validator
  │                        │                     │                      │
  │  publish schema        │                     │                      │
  │───────────────────────>│                     │                      │
  │                        │  validate content   │                      │
  │                        │────────────────────────────────────────────>│
  │                        │                     │                      │
  │                        │  validation result  │                      │
  │                        │<────────────────────────────────────────────│
  │                        │                     │                      │
  │                        │  check compatibility│                      │
  │                        │────────────────────────────────────────────>│
  │                        │                     │                      │
  │                        │  store schema       │                      │
  │                        │────────────────────>│                      │
  │                        │                     │                      │
  │  success + version     │                     │                      │
  │<───────────────────────│                     │                      │
```

### 6.2 Compatibility Check Flow

```
User                    CLI/API              Registry
  │                        │                     │
  │  check compatibility  │                     │
  │  (current + proposed)   │                     │
  │───────────────────────>│                     │
  │                        │                     │
  │                        │  compare versions   │
  │                        │────────────────────>│
  │                        │                     │
  │                        │  generate report    │
  │                        │<────────────────────│
  │                        │                     │
  │  compatibility result  │                     │
  │<───────────────────────│                     │
  │                        │                     │
```

---

## 7. Milestones

| Milestone | Target | Deliverables |
|-----------|--------|--------------|
| M1: Core Engine | 2026-05-01 | Schema parsing, validation engine, CLI |
| M2: Registry | 2026-06-01 | PostgreSQL storage, versioning, search |
| M3: CI/CD | 2026-07-01 | GitHub Actions, GitLab CI plugins |
| M4: Multi-Format | 2026-08-01 | Protobuf, GraphQL support |
| M5: GA | 2026-09-01 | Performance optimization, documentation |

---

## 8. Success Criteria

### 8.1 Feature Complete

- [ ] All P0 features implemented
- [ ] All P1 features implemented or deferred with justification
- [ ] No known critical bugs

### 8.2 Quality Gates

- [ ] 80% code coverage on core modules
- [ ] All CLI commands have integration tests
- [ ] API has contract tests
- [ ] Performance targets met
- [ ] Security audit passed

### 8.3 Documentation Complete

- [ ] API documentation (OpenAPI)
- [ ] CLI documentation (--help, man pages)
- [ ] User guide with examples
- [ ] Architecture documentation
- [ ] ADR decisions recorded

### 8.4 Adoption Criteria

- [ ] 5 internal services using schema registry
- [ ] CI/CD integration tested in production pipeline
- [ ] No schema drift incidents in 30 days

---

## 9. Appendix

### 9.1 Terminology

| Term | Definition |
|------|------------|
| Schema | Structured definition of data format |
| Registry | Centralized storage for schemas |
| Validation | Checking data against schema rules |
| Compatibility | Ability to use new version without breaking existing consumers |
| Breaking Change | Change that breaks compatibility |
| Migration | Process of transitioning data between versions |

### 9.2 References

| Reference | URL |
|-----------|-----|
| JSON Schema | https://json-schema.org/ |
| Semantic Versioning | https://semver.org/ |
| Protobuf | https://developers.google.com/protocol-buffers |
| GraphQL | https://graphql.org/ |

---

**Document History**

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0.0 | 2026-04-05 | Phenotype Team | Initial draft |

---

**End of PRD**
