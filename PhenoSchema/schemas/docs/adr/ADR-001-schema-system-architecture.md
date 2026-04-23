# ADR-001: Schema System Architecture — Polyglot Schema Management

**Date:** 2026-04-05

**Status:** ACCEPTED

**Author:** Phenotype Architecture Team

---

## Context

The Phenotype ecosystem comprises multiple services, libraries, and applications written in diverse languages (Rust, TypeScript, Python, Go). These components exchange data through various mechanisms:

- **HTTP APIs**: RESTful and GraphQL APIs between services
- **Event streaming**: Kafka, Pulsar for async communication
- **Configuration files**: YAML, JSON configs for deployment
- **Database schemas**: SQL schemas for persistence
- **Internal RPC**: Service-to-service communication

### Current Challenges

1. **Schema drift**: Services evolve independently, leading to incompatible data formats
2. **Duplication**: Type definitions duplicated in each language
3. **Validation inconsistency**: Same data validated differently across services
4. **Documentation gaps**: Schema definitions not synchronized with implementations
5. **No systematic versioning**: Schema changes happen ad-hoc

### Requirements

| Requirement | Priority | Description |
|-------------|----------|-------------|
| Cross-language support | P0 | Generate code for Rust, TypeScript, Python, Go |
| Validation capability | P0 | Validate data at runtime in all target languages |
| Schema evolution | P0 | Support backward and forward compatibility |
| Single source of truth | P0 | One schema definition, multiple outputs |
| Human readability | P1 | Source schemas must be human-readable |
| Binary serialization | P1 | Support compact binary formats |
| Schema registry | P1 | Centralized schema storage and versioning |
| Documentation generation | P2 | Auto-generate API documentation |

---

## Decision

Adopt a **polyglot schema architecture** with the following components:

### 1. Authoring Layer

**Primary format**: TypeSpec (`.tsp` files)
- Author all API schemas and service definitions in TypeSpec
- TypeSpec compiles to OpenAPI, JSON Schema, and Protobuf
- Familiar TypeScript-inspired syntax

**Secondary format**: CUE (`.cue` files)
- Author configuration schemas and policies in CUE
- CUE's validation-as-code model fits configuration use cases

### 2. Canonical Format

**JSON Schema** serves as the canonical interchange format:
- All schemas can be represented as JSON Schema
- Widest tooling support across languages
- Self-describing and human-readable
- Standardized specification (IETF)

### 3. Binary Serialization

**Protocol Buffers** for high-performance internal services:
- Compile TypeSpec to Protobuf for gRPC services
- Best-in-class binary serialization performance
- Mature ecosystem with excellent Rust/Go support

**Apache Avro** for event streaming:
- Schema registry integration (Confluent)
- Superior schema evolution story
- Kafka-native support

### 4. Schema Registry

Deploy **Confluent Schema Registry** (or Apicurio):
- Store canonical JSON Schema representations
- Version management with compatibility checking
- REST API for schema discovery

### Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────┐
│                      Schema Management System                        │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌─────────────┐      ┌─────────────┐      ┌─────────────────────┐   │
│  │  TypeSpec   │      │  CUE        │      │  JSON Schema        │   │
│  │  (.tsp)     │      │  (.cue)     │      │  (.json)            │   │
│  │             │      │             │      │                     │   │
│  │  APIs       │ ─┐   │  Config     │      │  Interchange        │   │
│  │  Services   │  │   │  Policies   │      │  Validation        │   │
│  └──────┬──────┘  │   └──────┬──────┘      └─────────┬─────────┘   │
│         │         │          │                        │             │
│         │         │          └────────────────────────┘             │
│         │         │                     │                          │
│         │         └─────────────────────┘                          │
│         │                              │                           │
│         ▼                              ▼                           │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                  Canonical Registry                           │   │
│  │  • JSON Schema as canonical format                            │   │
│  │  • Version management (semver)                                │   │
│  │  • Compatibility checking (BACKWARD/FORWARD/FULL)           │   │
│  │  • Schema evolution tracking                                  │   │
│  └────────────────────────┬────────────────────────────────────┘   │
│                           │                                         │
│  ┌────────────────────────▼─────────────────────────────────────┐   │
│  │                     Compilation Layer                           │   │
│  │                                                                │   │
│  │  TypeSpec ──┬──▶ OpenAPI ──▶ Documentation                     │   │
│  │             ├──▶ Protobuf ──▶ gRPC Services                     │   │
│  │             └──▶ JSON Schema ─▶ Validation                     │   │
│  │                                                                │   │
│  └────────────────────────┬───────────────────────────────────────┘   │
│                           │                                         │
│  ┌────────────────────────▼─────────────────────────────────────┐   │
│  │                     Consumption Layer                         │   │
│  │                                                                │   │
│  │  TypeScript:  Zod schemas, generated types                     │   │
│  │  Rust:        Serde derives, Prost (protobuf)                   │   │
│  │  Python:      Pydantic models, Avro schemas                      │   │
│  │  Go:          Generated structs, Protobuf                        │   │
│  │                                                                │   │
│  └────────────────────────────────────────────────────────────────┘   │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

---

## Rationale

### Why TypeSpec for Authoring?

1. **Modern syntax**: TypeScript-inspired, concise and expressive
2. **Multi-target**: Single source compiles to multiple formats
3. **IDE support**: Excellent autocomplete and error messages
4. **Microsoft backing**: Active development, enterprise-grade
5. **Validation decorators**: Rich constraint definitions

### Why JSON Schema as Canonical?

1. **Ubiquity**: Every language has validators
2. **Self-describing**: Schemas are valid JSON
3. **Standardized**: IETF-backed specification
4. **Tooling**: Extensive ecosystem
5. **Interchange**: Common format all tools understand

### Why Multiple Binary Formats?

| Format | Use Case | Justification |
|--------|----------|---------------|
| Protobuf | gRPC services | Best gRPC integration, mature Rust/Go support |
| Avro | Event streaming | Schema registry integration, Kafka-native |
| Parquet | Analytics | Columnar efficiency for data lakes |

### Why Not a Single Format?

No single format excels at all requirements:

| Format | Validation | Binary | Services | Registry | Readability |
|--------|------------|--------|----------|----------|-------------|
| JSON Schema | Excellent | No | No | Partial | Yes |
| Protobuf | Basic | Excellent | Excellent | Partial | No |
| Avro | Basic | Good | No | Excellent | No |
| TypeSpec | Via emitters | Via protobuf | Via emitters | No | Yes |

A polyglot approach lets each use case use the best tool.

---

## Consequences

### Positive

- **Single source of truth**: TypeSpec/CUE definitions generate all formats
- **Type safety**: Strong typing across all languages
- **Schema evolution**: Managed versioning with compatibility checks
- **Developer experience**: Modern IDE support, excellent error messages
- **Flexibility**: Each domain uses the optimal format

### Negative

- **Complexity**: Multiple tools in the build pipeline
- **Learning curve**: Team must learn TypeSpec syntax
- **Build overhead**: Compilation step adds build time
- **Debugging**: More layers to trace when issues occur

### Mitigations

| Risk | Mitigation |
|------|------------|
| Learning curve | Training sessions, good documentation |
| Build overhead | Caching, incremental builds |
| Debugging complexity | Clear error messages, good tooling |
| Tool fragility | Pin versions, test in CI |

---

## Alternatives Considered

### A1: Protocol Buffers for Everything

**Pros:** Single format, excellent binary serialization, mature tooling
**Cons:** Poor human readability, no rich validation, requires compilation
**Rejected:** Doesn't meet human readability and validation requirements

### A2: JSON Schema for Everything

**Pros:** Human-readable, excellent validation, wide support
**Cons:** Verbose, no binary serialization, poor code generation
**Rejected:** Doesn't meet binary serialization and code generation needs

### A3: GraphQL SDL for Everything

**Pros:** Excellent for APIs, introspection, type safety
**Cons:** Not general-purpose, no validation constraints, HTTP-centric
**Rejected:** Too API-specific, doesn't fit non-API use cases

### A4: Custom Schema Language

**Pros:** Perfect fit for our needs
**Cons:** Massive development effort, no ecosystem
**Rejected:** Opportunity cost too high

---

## Implementation

### Phase 1: Tooling Setup (Week 1)

1. Install TypeSpec CLI:
```bash
npm install -g @typespec/compiler
```

2. Initialize schema repository:
```bash
mkdir -p /Users/kooshapari/CodeProjects/Phenotype/repos/schemas
npm init
npm install @typespec/compiler @typespec/http @typespec/rest @typespec/openapi3 @typespec/json-schema @typespec/protobuf
```

3. Set up schema registry (Confluent or Apicurio)

### Phase 2: CI Integration (Week 2)

Add to CI pipeline:
```yaml
schema-check:
  steps:
    - checkout
    - run: npm install
    - run: tsp compile . --emit @typespec/json-schema
    - run: tsp compile . --emit @typespec/protobuf
    - run: schema-registry check-compatibility --schema ./schemas/
```

### Phase 3: Code Generation (Week 3-4)

1. TypeScript: `quicktype` or custom Zod generators
2. Rust: `prost` for protobuf, `schemars` for JSON Schema
3. Python: `datamodel-code-generator` for Pydantic
4. Go: `protoc-gen-go` for protobuf

---

## Validation

### Acceptance Criteria

- [ ] TypeSpec compiles to JSON Schema without errors
- [ ] TypeSpec compiles to Protobuf without errors
- [ ] Schemas validate in CI before merge
- [ ] Generated types compile in all target languages
- [ ] Schema registry tracks all schema versions
- [ ] Breaking changes are caught in CI
- [ ] Documentation generates from schemas

### Test Plan

1. **Compatibility tests**: Verify schema evolution rules
2. **Codegen tests**: Ensure generated types compile
3. **Validation tests**: Confirm runtime validation works
4. **Integration tests**: End-to-end data flow validation

---

## References

- [SCHEMA_SYSTEMS_SOTA.md](./SCHEMA_SYSTEMS_SOTA.md) - Research background
- [TypeSpec Documentation](https://typespec.io/docs)
- [JSON Schema Specification](https://json-schema.org/draft/2020-12/schema)
- [Confluent Schema Registry](https://docs.confluent.io/platform/current/schema-registry/)
- [CUE Language](https://cuelang.org)

---

**Decision Delta:**
- TypeSpec as primary authoring language
- JSON Schema as canonical interchange format
- Protobuf for gRPC services
- Avro for event streaming
- Confluent Schema Registry for schema management

**Review Date:** 2026-07-05
