# Schemaforge Technical Specification

> Universal Schema Management, Validation, and Code Generation Platform for the Phenotype Ecosystem

**Version:** 2.0.0  
**Status:** Draft  
**Date:** 2026-04-05  
**Owner:** Phenotype Team  

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [SOTA Schema & Codegen Landscape](#2-sota-schema--codegen-landscape)
3. [Problem Statement](#3-problem-statement)
4. [Architecture Overview](#4-architecture-overview)
5. [Core Components](#5-core-components)
6. [Data Models](#6-data-models)
7. [API Specifications](#7-api-specifications)
8. [Configuration](#8-configuration)
9. [Validation System](#9-validation-system)
10. [Migration Framework](#10-migration-framework)
11. [Integration Points](#11-integration-points)
12. [Security Considerations](#12-security-considerations)
13. [Performance Requirements](#13-performance-requirements)
14. [Testing Strategy](#14-testing-strategy)
15. [Deployment & Operations](#15-deployment--operations)
16. [Technical Deep Dive](#16-technical-deep-dive)
17. [Glossary](#17-glossary)
18. [Appendix A: ADRs](#appendix-a-architecture-decision-records)
19. [Appendix B: Reference URLs (200+)](#appendix-b-reference-urls-200)

---

## 1. Executive Summary

### 1.1 Purpose

Schemaforge is a comprehensive schema management and validation framework for the Phenotype ecosystem. It provides a unified system for defining, validating, and evolving data schemas across distributed services and polyglot environments. Schemaforge bridges the gap between schema-first and code-first approaches, enabling teams to maintain data consistency while supporting multiple serialization formats and programming languages.

### 1.2 Scope

| Component | Description | Priority | Status |
|-----------|-------------|----------|--------|
| Schema Definition | JSON Schema, Protobuf, GraphQL SDL support | P0 | ✅ Implemented |
| Schema Validation | Multi-format validation engine | P0 | ✅ Implemented |
| Schema Registry | Centralized schema storage and versioning | P1 | ✅ Implemented |
| Migration System | Schema evolution with backward compatibility | P1 | 🚧 In Progress |
| Code Generation | Type definitions from schemas | P2 | 📋 Planned |
| OpenAPI Support | REST API schema management | P1 | ✅ Implemented |
| Avro Support | Binary serialization schemas | P2 | 📋 Planned |
| AsyncAPI Support | Event-driven API schemas | P3 | 🔬 Research |

### 1.3 Key Differentiators

| Feature | Schemaforge | Confluent SR | Buf | Apollo | AWS Glue |
|---------|-------------|--------------|-----|--------|----------|
| Multi-format Support | 8+ formats | 3 formats | Protobuf only | GraphQL only | 3 formats |
| Schema Registry | ✅ Deep | ✅ Mature | ✅ Modern | ✅ Cloud | ✅ AWS-only |
| Migration Validation | ✅ Full | ⚠️ Basic | ⚠️ Breaking | ❌ No | ⚠️ Basic |
| Code Generation | ✅ Multi-lang | ❌ No | ✅ Protobuf | ✅ Client only | ❌ No |
| Ecosystem Integration | ✅ Deep | ⚠️ Kafka-only | ⚠️ Protobuf | ⚠️ GraphQL | ⚠️ AWS-only |
| Self-Hosted | ✅ Full | ✅ Yes | ✅ Yes | ❌ Cloud | ❌ Managed |
| CI/CD Integration | ✅ Deep | ⚠️ Manual | ✅ Good | ⚠️ Limited | ❌ No |

### 1.4 Target Users

1. **Platform Engineers**: Define organizational schema standards, enforce compliance
2. **Service Developers**: Validate data contracts between services, generate client code
3. **API Consumers**: Understand available data structures, generate type-safe clients
4. **DevOps**: Automated schema validation in CI/CD pipelines, drift detection
5. **Data Engineers**: Schema evolution for data lakes, ETL pipeline validation

---

## 2. SOTA Schema & Codegen Landscape

### 2.1 Schema Definition Formats (2024-2026)

| Format | Type System | Binary | Human-Readable | Version | Primary Use Case | Enterprise Adoption |
|--------|-------------|--------|----------------|---------|------------------|---------------------|
| **JSON Schema** | Structural | No | Yes | 2020-12 | API validation | ⭐⭐⭐⭐⭐ |
| **Protocol Buffers** | Strong | Yes | No | Editions 2023 | Microservices | ⭐⭐⭐⭐⭐ |
| **GraphQL SDL** | Strong | No | Yes | Oct 2021 | APIs, clients | ⭐⭐⭐⭐ |
| **OpenAPI 3.1** | Structural | No | Yes | 3.1.0 | REST documentation | ⭐⭐⭐⭐⭐ |
| **Avro** | Strong | Yes | JSON | 1.11 | Data lakes | ⭐⭐⭐⭐ |
| **Thrift** | Strong | Yes | IDL | 0.20 | Cross-language RPC | ⭐⭐⭐ |
| **Cap'n Proto** | Strong | Yes | No | 1.0 | Zero-copy IPC | ⭐⭐ |
| **FlatBuffers** | Strong | Yes | No | 24.3 | Games, mobile | ⭐⭐⭐ |
| **TypeSpec** | Strong | No | Yes | 0.52 | API definition | ⭐⭐ |
| **Smithy** | Strong | No | Yes | 1.0 | AWS services | ⭐⭐⭐ |
| **CUE** | Configuration | No | Yes | 0.7 | Config validation | ⭐⭐ |
| **Zod Schema** | Strong | No | Yes | 3.x | TypeScript first | ⭐⭐⭐⭐ |
| **Pydantic v2** | Strong | No | Yes | 2.x | Python validation | ⭐⭐⭐⭐⭐ |

### 2.2 JSON Schema Ecosystem

#### Draft Version Evolution

| Draft | Year | Status | Key Features | Schemaforge Priority |
|-------|------|--------|--------------|---------------------|
| Draft-04 | 2013 | Deprecated | Basic validation | ❌ Unsupported |
| Draft-06 | 2017 | Deprecated | `propertyNames` | ❌ Unsupported |
| Draft-07 | 2019 | Mature | `if/then/else` | ✅ Supported |
| 2019-09 | 2019 | Stable | `$recursiveAnchor` | ✅ Supported |
| **2020-12** | 2020 | **Current** | `$dynamicAnchor`, `prefixItems` | ✅ **Primary** |

#### JSON Schema Implementations

| Implementation | Language | Draft | Speed | Features | Schemaforge Status |
|----------------|----------|-------|-------|----------|-------------------|
| **jsonschema (Rust)** | Rust | 2020-12 | ~1M ops/s | Async, WASM | ✅ Core Engine |
| **AJV** | JavaScript | 2020-12 | ~500K ops/s | Standalone | ✅ Node SDK |
| **jsonschema-rs (Python)** | Python/Rust | 2020-12 | ~800K ops/s | PyO3 | ✅ Python SDK |
| **hyperjump** | JavaScript | 2020-12 | ~600K ops/s | Draft 2019-09 | 📋 Planned |
| **rapidjson** | C++ | Draft-07 | ~2M ops/s | SIMD | 🔬 Research |

### 2.3 Protocol Buffers Landscape

#### Version Evolution

| Version | Year | Key Features | Breaking Changes | Status |
|---------|------|--------------|-------------------|--------|
| Proto2 | 2008 | Required/optional, groups | - | Legacy |
| Proto3 | 2016 | Simplified, JSON mapping | No required fields | Mature |
| **Editions 2023** | 2023 | **Flexible defaults** | New syntax | **Current** |

#### Code Generation Tools

| Tool | Input | Output Languages | Features | Schemaforge Priority |
|------|-------|------------------|----------|---------------------|
| **protoc** | Protobuf | All | Official | ✅ Core |
| **buf** | Protobuf | All | Linting, breaking | ✅ Core |
| **ts-proto** | Protobuf | TypeScript | Idiomatic TS | ✅ Core |
| **prost** | Protobuf | Rust | No proto runtime | ✅ Core |
| **protoc-gen-validate (PGV)** | Protobuf | Multi | Validation rules | ✅ Core |
| **grpc-gateway** | Protobuf | Go | REST gateway | 📋 Planned |

### 2.4 GraphQL Ecosystem

| Tool | Type | Language | Features | Schemaforge Priority |
|------|------|----------|----------|---------------------|
| **graphql-js** | Reference | JavaScript | Reference impl | ✅ Core |
| **gqlgen** | Server | Go | Type-safe | ✅ Core |
| **async-graphql** | Server | Rust | Actix, warp | ✅ Core |
| **apollo-server** | Server | Node.js | Federation | 🔬 Research |
| **graphql-codegen** | Codegen | Multi | 15+ plugins | ✅ Core |

### 2.5 Schema Registry Solutions Comparison

| Solution | Formats | Registry | Validation | Code Gen | Deployment | Schemaforge Target |
|----------|---------|----------|------------|----------|------------|-------------------|
| **Confluent SR** | Avro/Proto/JSON | ✅ | ✅ | ❌ | Self-hosted | Match + exceed |
| **Buf** | Protobuf | ✅ | ✅ | ✅ | SaaS/Self | API design |
| **Apollo Studio** | GraphQL | ✅ | ✅ | ✅ | SaaS | Federation |
| **AWS Glue** | Avro/Proto/JSON | ✅ | ⚠️ | ❌ | Managed | Cloud-native |
| **Schemaforge** | **All major** | ✅ | ✅ | ✅ | **Self-hosted** | **Universal** |

### 2.6 Code Generation Landscape

| Generator | Input | Languages | Special Features | Status |
|-----------|-------|-----------|------------------|--------|
| **quicktype** | JSON/Schema | 20+ | IDE plugins | ✅ Supported |
| **openapi-generator** | OpenAPI | 50+ | Large ecosystem | ✅ Supported |
| **prisma** | Schema DSL | TS/Go/Python/Rust | DB integration | 🔬 Research |
| **buf.generate** | Protobuf | Configurable | BSR integration | ✅ Supported |
| **typespec** | TypeSpec | OpenAPI/JSON Schema | Microsoft | 🔬 Research |

### 2.7 Validation Libraries by Language

| Language | Library | Size | Speed | Features | SDK Status |
|----------|---------|------|-------|----------|------------|
| **TypeScript** | Zod | 12KB | Fast | Type inference | ✅ Core |
| **TypeScript** | Valibot | <1KB | Fast | Tree-shakable | 📋 Planned |
| **TypeScript** | AJV | 25KB | Very Fast | JSON Schema | ✅ Core |
| **Python** | Pydantic v2 | ~50KB | Very Fast | Rust core | ✅ Core |
| **Go** | go-playground/validator | ~30KB | Fast | Struct tags | ✅ Core |
| **Rust** | jsonschema | ~100KB | Very Fast | 2020-12 | ✅ Core |
| **Rust** | validator | ~20KB | Fast | Derive macros | ✅ Core |
| **Java** | Hibernate Validator | ~200KB | Medium | Bean Validation | 📋 Planned |
| **C#** | FluentValidation | ~100KB | Fast | Fluent API | 📋 Planned |

---

## 3. Problem Statement

### 3.1 Current Challenges

| Challenge | Impact | Frequency | Root Cause |
|-----------|--------|-----------|------------|
| **Schema Drift** | Services break due to incompatible changes | Weekly | No centralized governance |
| **Lack of Registry** | No single source of truth for schemas | Daily | Fragmented tooling |
| **Manual Validation** | Error-prone human processes | Daily | No automation |
| **Migration Complexity** | Fear of breaking changes | Monthly | No compatibility tooling |
| **Format Fragmentation** | Inconsistent tooling per format | Always | Siloed solutions |
| **Code Generation Gap** | Hand-written clients drift | Weekly | No automated sync |

### 3.2 Business Requirements

| ID | Requirement | Priority | Acceptance Criteria |
|----|-------------|----------|---------------------|
| BR-001 | Enable schema evolution without service disruption | P0 | 99.9% backward compatible migrations |
| BR-002 | Provide visibility into schema changes across organization | P0 | Real-time notifications, audit logs |
| BR-003 | Automate validation to catch breaking changes early | P0 | CI integration <30s validation |
| BR-004 | Support multiple schema formats in unified system | P0 | JSON Schema, Protobuf, GraphQL, OpenAPI |
| BR-005 | Generate type-safe client code from schemas | P1 | 8+ languages supported |
| BR-006 | Self-hosted deployment option | P1 | Docker, K8s, bare metal |

### 3.3 Functional Requirements

| ID | Requirement | Description | Priority |
|----|-------------|-------------|----------|
| FR-001 | Schema storage with version history | Immutable schema versions | P0 |
| FR-002 | JSON Schema validation | Draft 2020-12 full support | P0 |
| FR-003 | Protobuf schema validation | Editions 2023 support | P1 |
| FR-004 | GraphQL SDL validation | Spec-compliant parsing | P1 |
| FR-005 | OpenAPI validation | 3.1.0 specification | P1 |
| FR-006 | Schema compatibility checking | Backward/forward/full | P0 |
| FR-007 | Migration path generation | Auto-generated migration scripts | P1 |
| FR-008 | CLI for schema operations | Full CRUD operations | P0 |
| FR-009 | REST API for schema management | HTTP API with auth | P1 |
| FR-010 | Webhook notifications | Change events | P2 |
| FR-011 | Schema diff visualization | Human-readable diffs | P2 |
| FR-012 | Code generation | 8+ languages | P1 |
| FR-013 | CI/CD integration | GitHub Actions, etc. | P1 |
| FR-014 | Search and discovery | Full-text search | P1 |
| FR-015 | Access control | RBAC for schemas | P1 |

---

## 4. Architecture Overview

### 4.1 High-Level Architecture

Schemaforge uses a layered, modular architecture designed for extensibility:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         User Interface Layer                                 │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │    CLI      │  │   REST API  │  │   SDKs      │  │   Web UI    │        │
│  │   (Rust)    │  │  (Axum)     │  │  (Multi)    │  │  (React)    │        │
│  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘        │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                          Core Engine Layer                                   │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                     Schema Processing Pipeline                         │   │
│  │                                                                       │   │
│  │   Input → Parse → Normalize → Validate → Register → Store           │   │
│  │      ↓       ↓         ↓          ↓         ↓        ↓               │   │
│  │   Format  Format-  Canonical   Schema   Compat   Persistent        │   │
│  │   Detect  Specific   Form       Rules    Check    Storage          │   │
│  │                                                                       │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                     Plugin Architecture                              │   │
│  │                                                                       │   │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐ │   │
│  │  │  JSON    │  │  Proto-  │  │  GraphQL │  │  OpenAPI │  │   Avro   │ │   │
│  │  │ Schema   │  │   buf    │  │   SDL    │  │    3.1   │  │  (plan)  │ │   │
│  │  └──────────┘  └──────────┘  └──────────┘  └──────────┘  └──────────┘ │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                        Storage Abstraction Layer                              │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │  PostgreSQL │  │    Redis    │  │   S3/GCS    │  │   Filesystem│        │
│  │  (Primary)  │  │   (Cache)   │  │  (Large)    │  │   (Dev)     │        │
│  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘        │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 4.2 Design Principles

| Principle | Description | Application |
|-----------|-------------|-------------|
| **Schema-First** | Schemas are the source of truth | All validation derives from schema |
| **Immutable Versions** | Once published, versions cannot be modified | Enables reproducibility |
| **Compatibility Over Restriction** | Prefer additive changes | Enables evolution |
| **Fail Fast** | Validate early in the pipeline | CI/CD integration |
| **Observable** | All operations produce metrics/traces | Debugging and monitoring |
| **Pluggable** | Format-specific implementations via traits | Extensibility |
| **Resource-Bounded** | Validation has limits (depth, time, memory) | Prevents DoS |

### 4.3 Data Flow

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                          Schema Processing Flow                              │
│                                                                              │
│  1. Input                                                                   │
│     └── Raw schema content (JSON, .proto, .graphql, etc.)                  │
│                                    │                                        │
│  2. Format Detection ───────────────────────────────────────────────►      │
│     ├── Content sniffing (magic bytes, patterns)                            │
│     ├── Filename extension analysis                                        │
│     └── Explicit format hint                                                │
│                                    │                                        │
│  3. Parsing ─────────────────────────────────────────────────────────►     │
│     ├── Format-specific parser (syn, tree-sitter, etc.)                    │
│     ├── AST generation                                                     │
│     └── Syntax error reporting (streaming)                                  │
│                                    │                                        │
│  4. Normalization ────────────────────────────────────────────────────►    │
│     ├── Convert to canonical internal representation                       │
│     ├── Resolve references ($ref, imports)                                 │
│     └── Build dependency graph                                              │
│                                    │                                        │
│  5. Validation ───────────────────────────────────────────────────────►    │
│     ├── Structural validation (well-formedness)                             │
│     ├── Semantic validation (logical consistency)                           │
│     └── Reference validation (resolve all $refs)                            │
│                                    │                                        │
│  6. Compatibility Check ──────────────────────────────────────────────►    │
│     ├── Compare with previous versions                                     │
│     ├── Determine compatibility level (none/backward/forward/full)         │
│     └── Generate migration plan                                             │
│                                    │                                        │
│  7. Registration ────────────────────────────────────────────────────►     │
│     ├── Generate content hash (SHA-256)                                    │
│     ├── Check for duplicates                                                │
│     └── Assign version if not specified                                    │
│                                    │                                        │
│  8. Storage ───────────────────────────────────────────────────────────►   │
│     ├── Write to PostgreSQL (metadata + content)                            │
│     ├── Update Redis cache                                                  │
│     └── Emit webhook notifications                                          │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 4.4 Component Interaction Diagram

```
                    ┌─────────────────┐
                    │   CLI Client    │
                    └────────┬────────┘
                             │ gRPC/HTTP
                             ▼
                    ┌─────────────────┐
                    │   API Gateway   │
                    │   (Rate Limit)  │
                    └────────┬────────┘
                             │
            ┌────────────────┼────────────────┐
            │                │                │
            ▼                ▼                ▼
   ┌────────────────┐ ┌────────────────┐ ┌────────────────┐
   │ Schema Service │ │ Validate Service│ │  Registry Svc  │
   └────────┬───────┘ └────────┬───────┘ └────────┬───────┘
            │                  │                  │
            └──────────────────┼──────────────────┘
                               │
                               ▼
                    ┌─────────────────┐
                    │   Event Bus     │
                    │  (Redis PubSub) │
                    └────────┬────────┘
                             │
            ┌────────────────┼────────────────┐
            │                │                │
            ▼                ▼                ▼
   ┌────────────────┐ ┌────────────────┐ ┌────────────────┐
   │   PostgreSQL   │ │     Redis      │ │  Webhook Svc   │
   └────────────────┘ └────────────────┘ └────────────────┘
```

---

## 5. Core Components

### 5.1 Schema Type System

```rust
/// Unique identifier for a schema
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SchemaId {
    pub namespace: String,
    pub name: String,
    pub version: Version,
    pub format: SchemaFormat,
}

/// Semantic version following SemVer 2.0
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub prerelease: Option<String>,
    pub build: Option<String>,
}

impl Version {
    /// Parse a SemVer string
    pub fn parse(version: &str) -> Result<Self, VersionError> {
        // SemVer 2.0 parsing
    }

    /// Check if this version is compatible with another
    pub fn is_compatible_with(&self, other: &Version, level: CompatibilityLevel) -> bool {
        match level {
            CompatibilityLevel::None => false,
            CompatibilityLevel::Backward => self.major == other.major,
            CompatibilityLevel::Forward => self.major == other.major && self.minor >= other.minor,
            CompatibilityLevel::Full => self == other,
        }
    }
}

/// Supported schema formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString)]
#[strum(serialize_all = "kebab-case")]
pub enum SchemaFormat {
    #[strum(serialize = "json-schema")]
    JsonSchema,
    #[strum(serialize = "protobuf")]
    Protobuf,
    #[strum(serialize = "graphql")]
    GraphQL,
    #[strum(serialize = "openapi")]
    OpenAPI,
    #[strum(serialize = "avro")]
    Avro,
    #[strum(serialize = "typescript")]
    TypeScript, // Type-first schemas via Zod/TypeBox
    #[strum(serialize = "python")]
    Python,     // Pydantic models
    #[strum(serialize = "custom")]
    Custom(&'static str),
}

/// Core schema trait - implemented by all format-specific schemas
#[async_trait]
pub trait Schema: Send + Sync + std::any::Any {
    /// Get the schema identifier
    fn id(&self) -> &SchemaId;

    /// Get the format
    fn format(&self) -> SchemaFormat;

    /// Get the canonical representation
    fn canonical(&self) -> &CanonicalSchema;

    /// Get the original source content
    fn source(&self) -> &str;

    /// Get metadata
    fn metadata(&self) -> &SchemaMetadata;

    /// Convert to JSON representation
    fn to_json(&self) -> Result<Value, SchemaError>;

    /// Get references to other schemas
    fn references(&self) -> Vec<SchemaReference>;
}

/// Metadata associated with a schema
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SchemaMetadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub author: String,
    pub created_at: DateTime<Utc>,
    pub tags: Vec<String>,
    pub deprecated: bool,
    pub deprecation_reason: Option<String>,
    pub custom: HashMap<String, Value>,
}
```

### 5.2 Parser Architecture

```rust
/// Parser plugin interface
#[async_trait]
pub trait SchemaParser: Send + Sync {
    /// The format this parser handles
    fn format(&self) -> SchemaFormat;

    /// Parse raw content into a schema
    async fn parse(&self, input: &str) -> Result<Box<dyn Schema>, ParseError>;

    /// Convert to canonical internal representation
    async fn canonicalize(&self, schema: &dyn Schema) -> Result<CanonicalSchema, ParseError>;

    /// Detect if content is this format
    fn can_parse(&self, content: &str) -> f32; // 0.0 - 1.0 confidence
}

/// Parser registry
pub struct ParserRegistry {
    parsers: HashMap<SchemaFormat, Box<dyn SchemaParser>>,
}

impl ParserRegistry {
    pub fn new() -> Self {
        let mut registry = Self { parsers: HashMap::new() };
        
        // Register built-in parsers
        registry.register(Box::new(JsonSchemaParser::new()));
        registry.register(Box::new(ProtobufParser::new()));
        registry.register(Box::new(GraphQLParser::new()));
        registry.register(Box::new(OpenAPIParser::new()));
        
        registry
    }

    pub fn register(&mut self, parser: Box<dyn SchemaParser>) {
        self.parsers.insert(parser.format(), parser);
    }

    pub fn detect_format(&self, content: &str) -> Option<SchemaFormat> {
        let mut best_match = None;
        let mut best_confidence = 0.0f32;

        for (format, parser) in &self.parsers {
            let confidence = parser.can_parse(content);
            if confidence > best_confidence && confidence > 0.7 {
                best_confidence = confidence;
                best_match = Some(*format);
            }
        }

        best_match
    }
}

/// JSON Schema parser implementation
pub struct JsonSchemaParser {
    compiler: Arc<JSONSchema<'static>>,
}

#[async_trait]
impl SchemaParser for JsonSchemaParser {
    fn format(&self) -> SchemaFormat { SchemaFormat::JsonSchema }

    async fn parse(&self, input: &str) -> Result<Box<dyn Schema>, ParseError> {
        let value: Value = serde_json::from_str(input)
            .map_err(|e| ParseError::InvalidJson(e))?;

        // Validate it's a valid JSON Schema
        self.compiler.compile(&value)
            .map_err(|e| ParseError::InvalidSchema(e.to_string()))?;

        let id = extract_schema_id(&value)?;
        let metadata = extract_metadata(&value)?;

        Ok(Box::new(JsonSchemaImpl {
            id,
            source: input.to_string(),
            value,
            metadata,
        }))
    }

    fn can_parse(&self, content: &str) -> f32 {
        // Check for JSON Schema indicators
        if content.contains("$schema") && content.contains("json-schema.org") {
            return 1.0;
        }
        if content.contains("\"type\"") && content.contains("\"properties\"") {
            return 0.8;
        }
        if serde_json::from_str::<Value>(content).is_ok() {
            return 0.3;
        }
        0.0
    }
}
```

### 5.3 Validation Engine

```rust
/// Validation severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Severity {
    Error,   // Must fix
    Warning, // Should fix
    Info,    // FYI
}

/// Validation issue
#[derive(Debug, Clone, Serialize)]
pub struct ValidationIssue {
    pub severity: Severity,
    pub code: String,
    pub message: String,
    pub path: String,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub suggestion: Option<String>,
}

/// Validation report
#[derive(Debug, Clone, Serialize)]
pub struct ValidationReport {
    pub valid: bool,
    pub issues: Vec<ValidationIssue>,
    pub stats: ValidationStats,
}

/// Validation statistics
#[derive(Debug, Clone, Default, Serialize)]
pub struct ValidationStats {
    pub duration_ms: u64,
    pub schemas_validated: usize,
    pub total_checks: usize,
    pub cached_results: usize,
}

/// Validator plugin interface
#[async_trait]
pub trait SchemaValidator: Send + Sync {
    /// Formats supported by this validator
    fn supported_formats(&self) -> Vec<SchemaFormat>;

    /// Validate schema well-formedness
    async fn validate_schema(
        &self,
        schema: &dyn Schema,
    ) -> Result<ValidationReport, ValidationError>;

    /// Validate data against a schema
    async fn validate_data(
        &self,
        schema: &dyn Schema,
        data: &Value,
    ) -> Result<ValidationReport, ValidationError>;

    /// Check compatibility between two schema versions
    async fn check_compatibility(
        &self,
        old: &dyn Schema,
        new: &dyn Schema,
    ) -> Result<CompatibilityReport, ValidationError>;
}

/// Compatibility levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum CompatibilityLevel {
    None,      // Breaking changes
    Backward,  // Old consumers can read new data
    Forward,   // New consumers can read old data
    Full,      // Both directions
}

/// Compatibility report
#[derive(Debug, Clone, Serialize)]
pub struct CompatibilityReport {
    pub level: CompatibilityLevel,
    pub breaking_changes: Vec<BreakingChange>,
    pub warnings: Vec<String>,
}

/// Breaking change description
#[derive(Debug, Clone, Serialize)]
pub struct BreakingChange {
    pub category: String,
    pub description: String,
    pub path: String,
    pub migration_hint: String,
}
```

### 5.4 Registry Service

```rust
/// Schema registry interface
#[async_trait]
pub trait SchemaRegistry: Send + Sync {
    /// Publish a new schema version
    async fn publish(&self, schema: Box<dyn Schema>) -> Result<SchemaId, RegistryError>;

    /// Get a schema by ID
    async fn get(&self, id: &SchemaId) -> Result<Box<dyn Schema>, RegistryError>;

    /// Get a specific version by name
    async fn get_version(
        &self,
        namespace: &str,
        name: &str,
        version: &Version,
    ) -> Result<Box<dyn Schema>, RegistryError>;

    /// List all versions of a schema
    async fn list_versions(
        &self,
        namespace: &str,
        name: &str,
    ) -> Result<Vec<Version>, RegistryError>;

    /// Search for schemas
    async fn search(&self, criteria: &SearchCriteria) -> Result<Vec<SchemaSummary>, RegistryError>;

    /// Check compatibility before publishing
    async fn check_compatibility(
        &self,
        namespace: &str,
        name: &str,
        proposed: &dyn Schema,
    ) -> Result<CompatibilityReport, RegistryError>;

    /// Deprecate a schema version
    async fn deprecate(
        &self,
        id: &SchemaId,
        reason: &str,
    ) -> Result<(), RegistryError>;
}

/// Schema summary for listing
#[derive(Debug, Clone, Serialize)]
pub struct SchemaSummary {
    pub id: SchemaId,
    pub title: Option<String>,
    pub description: Option<String>,
    pub author: String,
    pub created_at: DateTime<Utc>,
    pub deprecated: bool,
}

/// Search criteria
#[derive(Debug, Clone, Default)]
pub struct SearchCriteria {
    pub query: Option<String>,
    pub namespace: Option<String>,
    pub format: Option<SchemaFormat>,
    pub author: Option<String>,
    pub tags: Vec<String>,
    pub created_after: Option<DateTime<Utc>>,
    pub created_before: Option<DateTime<Utc>>,
    pub deprecated: Option<bool>,
    pub limit: usize,
    pub offset: usize,
}
```

---

## 6. Data Models

### 6.1 Schema Storage Model

| Field | Type | Description | Constraints | Index |
|-------|------|-------------|-------------|-------|
| id | UUID | Primary key | Not null, unique | Primary |
| namespace | VARCHAR(255) | Schema namespace | Not null, default 'default' | ✅ |
| name | VARCHAR(255) | Schema name | Not null | ✅ |
| version_major | INTEGER | Major version | Not null, >= 0 | ✅ |
| version_minor | INTEGER | Minor version | Not null, >= 0 | ✅ |
| version_patch | INTEGER | Patch version | Not null, >= 0 | ✅ |
| format | VARCHAR(50) | Schema format | Not null | ✅ |
| content | TEXT | Schema content | Not null | ❌ |
| content_hash | VARCHAR(64) | SHA-256 of content | Not null, unique | ✅ |
| content_compressed | BYTEA | Gzipped content | Nullable | ❌ |
| title | VARCHAR(500) | Human-readable title | Nullable | ✅ (partial) |
| description | TEXT | Schema description | Nullable | ✅ (full-text) |
| author | VARCHAR(255) | Creator | Not null | ✅ |
| created_at | TIMESTAMPTZ | Creation time | Not null | ✅ |
| updated_at | TIMESTAMPTZ | Last update | Not null | ✅ |
| deprecated | BOOLEAN | Deprecation flag | Default false | ✅ |
| deprecation_reason | TEXT | Why deprecated | Nullable | ❌ |
| superseded_by | UUID | Newer version | Nullable, FK to schemas | ✅ |
| tags | TEXT[] | Array of tags | Default '{}' | ✅ (GIN) |
| metadata | JSONB | Additional metadata | Nullable | ✅ (GIN) |
| search_vector | TSVECTOR | Full-text index | Generated | ✅ (GIN) |

### 6.2 PostgreSQL Schema Definition

```sql
-- Core schemas table
CREATE TABLE schemas (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    namespace VARCHAR(255) NOT NULL DEFAULT 'default',
    name VARCHAR(255) NOT NULL,
    version_major INT NOT NULL,
    version_minor INT NOT NULL,
    version_patch INT NOT NULL,
    format VARCHAR(50) NOT NULL,
    content TEXT NOT NULL,
    content_hash VARCHAR(64) NOT NULL,
    content_compressed BYTEA,
    title VARCHAR(500),
    description TEXT,
    author VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deprecated BOOLEAN NOT NULL DEFAULT FALSE,
    deprecation_reason TEXT,
    superseded_by UUID REFERENCES schemas(id),
    tags TEXT[] DEFAULT '{}',
    metadata JSONB DEFAULT '{}',
    search_vector TSVECTOR,

    CONSTRAINT unique_version UNIQUE (namespace, name, version_major, version_minor, version_patch),
    CONSTRAINT unique_content UNIQUE (content_hash),
    CONSTRAINT check_version CHECK (version_major >= 0 AND version_minor >= 0 AND version_patch >= 0)
);

-- Indexes for common queries
CREATE INDEX idx_schemas_namespace_name ON schemas(namespace, name);
CREATE INDEX idx_schemas_format ON schemas(format);
CREATE INDEX idx_schemas_author ON schemas(author);
CREATE INDEX idx_schemas_created ON schemas(created_at DESC);
CREATE INDEX idx_schemas_deprecated ON schemas(deprecated) WHERE deprecated = TRUE;
CREATE INDEX idx_schemas_tags ON schemas USING GIN(tags);
CREATE INDEX idx_schemas_metadata ON schemas USING GIN(metadata jsonb_path_ops);
CREATE INDEX idx_schemas_search ON schemas USING GIN(search_vector);

-- Trigger for full-text search
CREATE OR REPLACE FUNCTION schemas_search_update()
RETURNS TRIGGER AS $$
BEGIN
    NEW.search_vector := 
        setweight(to_tsvector('english', COALESCE(NEW.name, '')), 'A') ||
        setweight(to_tsvector('english', COALESCE(NEW.title, '')), 'A') ||
        setweight(to_tsvector('english', COALESCE(NEW.description, '')), 'B') ||
        setweight(to_tsvector('english', array_to_string(NEW.tags, ' ')), 'C') ||
        setweight(to_tsvector('english', COALESCE(NEW.content, '')), 'D');
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER schemas_search_trigger
    BEFORE INSERT OR UPDATE ON schemas
    FOR EACH ROW
    EXECUTE FUNCTION schemas_search_update();
```

### 6.3 Compatibility Matrix

| Change Type | Backward Compatible | Forward Compatible | Notes |
|-------------|---------------------|--------------------|-------|
| Add optional field | ✅ Yes | ✅ Yes | Additive change |
| Add required field | ❌ No | ✅ Yes | Breaking for consumers |
| Remove optional field | ✅ Yes | ❌ No | Breaking for producers |
| Remove required field | ❌ No | ❌ No | Major breaking |
| Rename field | ❌ No | ❌ No | Requires alias mapping |
| Change field type | ❌ No | ❌ No | Potentially breaking |
| Widen type (int→float) | ⚠️ Maybe | ✅ Yes | Depends on context |
| Narrow type (float→int) | ❌ No | ⚠️ Maybe | Data loss possible |
| Add enum value | ✅ Yes | ❌ No | Consumers may break |
| Remove enum value | ❌ No | ✅ Yes | Producers may break |
| Relax constraints | ✅ Yes | ❌ No | Producers may break |
| Tighten constraints | ❌ No | ✅ Yes | Consumers may break |
| Add pattern constraint | ❌ No | ✅ Yes | New validation |
| Change default value | ⚠️ Warning | ⚠️ Warning | Behavioral change |
| Change documentation | ✅ Yes | ✅ Yes | Non-breaking |
| Deprecate field | ⚠️ Warning | ✅ Yes | Soft breaking |

### 6.4 Breaking Change Classification

| Severity | Category | Detection | Resolution | Example |
|----------|----------|-----------|------------|---------|
| Critical | Remove required field | Automated | Migration required | `user.id` removed |
| Critical | Change type incompatibly | Automated | Migration required | `string` → `number` |
| Critical | Remove enum value in use | Semi-auto | Deprecation period | Remove `STATUS_DRAFT` |
| High | Add required field | Automated | Schema evolution | Add `user.email` required |
| High | Remove optional field in use | Semi-auto | Deprecation period | Remove `user.bio` |
| High | Tighten constraints | Automated | Gradual rollout | Add `maxLength: 100` |
| Medium | Rename field | Pattern match | Alias mapping | `firstName` → `first_name` |
| Medium | Change default | Automated | Version bump | Default `timeout: 30` → `60` |
| Medium | Move field | Semi-auto | Migration script | `address.street` → `street` |
| Low | Documentation changes | Manual review | None needed | Fix typo in description |
| Low | Add examples | Automated | None needed | Add `examples: ["foo"]` |
| Low | Format metadata | Automated | None needed | Change `title` |

---

## 7. API Specifications

### 7.1 REST API Endpoints

| Method | Endpoint | Description | Auth Required | Rate Limit |
|--------|----------|-------------|---------------|------------|
| GET | /api/v1/health | Health check | No | N/A |
| GET | /api/v1/schemas | List all schemas | Yes | 100/min |
| POST | /api/v1/schemas | Publish new schema | Yes | 60/min |
| GET | /api/v1/schemas/{namespace}/{name} | Get schema details | Yes | 300/min |
| GET | /api/v1/schemas/{namespace}/{name}/versions | List versions | Yes | 300/min |
| GET | /api/v1/schemas/{namespace}/{name}/versions/{version} | Get specific version | Yes | 300/min |
| GET | /api/v1/schemas/{id} | Get by UUID | Yes | 300/min |
| POST | /api/v1/schemas/validate | Validate schema | Yes | 120/min |
| POST | /api/v1/schemas/{id}/validate-data | Validate data | Yes | 600/min |
| GET | /api/v1/schemas/{namespace}/{name}/compatibility | Check compatibility | Yes | 60/min |
| POST | /api/v1/schemas/{namespace}/{name}/migrate | Generate migration | Yes | 30/min |
| GET | /api/v1/schemas/{namespace}/{name}/diff/{version} | Diff versions | Yes | 120/min |
| POST | /api/v1/schemas/{id}/deprecate | Deprecate version | Yes (Admin) | 30/min |
| DELETE | /api/v1/schemas/{id} | Delete (soft) | Yes (Admin) | 10/min |
| GET | /api/v1/search | Search schemas | Yes | 60/min |
| GET | /api/v1/formats | List supported formats | No | N/A |
| GET | /api/v1/stats | Registry statistics | Yes (Admin) | 10/min |

### 7.2 Request/Response Examples

#### Publish Schema Request

```http
POST /api/v1/schemas HTTP/1.1
Content-Type: application/json
Authorization: Bearer {token}

{
  "namespace": "phenotype",
  "name": "user-profile",
  "version": "1.2.0",
  "format": "json-schema",
  "content": {
    "$schema": "https://json-schema.org/draft/2020-12/schema",
    "$id": "https://phenotype.dev/schemas/user-profile",
    "title": "User Profile",
    "type": "object",
    "required": ["id", "email"],
    "properties": {
      "id": { "type": "string", "format": "uuid" },
      "email": { "type": "string", "format": "email" },
      "name": {
        "type": "object",
        "properties": {
          "first": { "type": "string" },
          "last": { "type": "string" }
        }
      },
      "createdAt": { "type": "string", "format": "date-time" }
    }
  },
  "author": "platform-team",
  "tags": ["user", "profile", "v2"],
  "metadata": {
    "team": "platform",
    "domain": "identity",
    "slack-channel": "#platform-alerts"
  }
}
```

#### Publish Schema Response

```http
HTTP/1.1 201 Created
Content-Type: application/json
Location: /api/v1/schemas/phenotype/user-profile/versions/1.2.0

{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "namespace": "phenotype",
  "name": "user-profile",
  "version": "1.2.0",
  "format": "json-schema",
  "content_hash": "sha256:a1b2c3d4e5f6...",
  "created_at": "2026-04-05T12:00:00Z",
  "compatibility": {
    "level": "backward",
    "previous_version": "1.1.0",
    "breaking_changes": [],
    "warnings": [
      {
        "code": "NEW_OPTIONAL_FIELD",
        "message": "Added optional field 'name.middle'",
        "path": "#/properties/name/properties/middle"
      }
    ]
  },
  "links": {
    "self": "/api/v1/schemas/phenotype/user-profile/versions/1.2.0",
    "versions": "/api/v1/schemas/phenotype/user-profile/versions",
    "validate": "/api/v1/schemas/550e8400-e29b-41d4-a716-446655440000/validate-data"
  }
}
```

#### Validate Data Request

```http
POST /api/v1/schemas/550e8400-e29b-41d4-a716-446655440000/validate-data HTTP/1.1
Content-Type: application/json

{
  "id": "123e4567-e89b-12d3-a456-426614174000",
  "email": "user@example.com",
  "name": {
    "first": "John",
    "last": "Doe"
  },
  "createdAt": "2026-04-05T10:30:00Z"
}
```

#### Validation Error Response

```http
HTTP/1.1 422 Unprocessable Entity
Content-Type: application/json

{
  "valid": false,
  "errors": [
    {
      "instancePath": "/email",
      "schemaPath": "#/properties/email/format",
      "keyword": "format",
      "params": { "format": "email" },
      "message": "must match format 'email'",
      "data": "invalid-email"
    }
  ],
  "stats": {
    "duration_ms": 2,
    "checks_performed": 12
  }
}
```

### 7.3 Error Response Format

| Code | HTTP Status | Error | Description | Retryable |
|------|-------------|-------|-------------|-----------|
| 400 | Bad Request | INVALID_SCHEMA | Schema content is malformed | No |
| 400 | Bad Request | INCOMPATIBLE_SCHEMA | Breaking change detected | No |
| 400 | Bad Request | VALIDATION_FAILED | Request body validation failed | No |
| 401 | Unauthorized | UNAUTHENTICATED | Missing or invalid credentials | Yes |
| 403 | Forbidden | PERMISSION_DENIED | Insufficient permissions | No |
| 404 | Not Found | SCHEMA_NOT_FOUND | Requested schema doesn't exist | No |
| 409 | Conflict | SCHEMA_EXISTS | Version already published | No |
| 409 | Conflict | HASH_COLLISION | Content hash conflict | No |
| 422 | Unprocessable | DATA_VALIDATION_FAILED | Data doesn't match schema | No |
| 429 | Too Many Requests | RATE_LIMITED | Request quota exceeded | Yes |
| 500 | Internal Error | INTERNAL_ERROR | Unexpected server error | Yes |
| 503 | Service Unavailable | SERVICE_UNAVAILABLE | Temporary outage | Yes |

---

## 8. Configuration

### 8.1 Configuration Schema (TOML)

```toml
[server]
host = "0.0.0.0"
port = 8080
workers = 4
max_request_size_mb = 10
timeout_seconds = 30
enable_compression = true
enable_cors = true

[storage]
type = "postgres"
connection_string = "postgresql://user:pass@localhost/schemaforge"
pool_size = 10
max_connections = 100
connection_timeout_seconds = 5

[storage.cache]
enabled = true
type = "redis"
url = "redis://localhost:6379"
ttl_seconds = 3600
max_memory_mb = 256

[storage.large_schema]
enabled = true
backend = "s3"
bucket = "schemaforge-schemas"
max_size_kb = 100

[validation]
strict_mode = true
validate_on_publish = true
allow_deprecated = false
max_validation_depth = 50
max_validation_time_seconds = 30
max_schema_size_kb = 5120
cache_results = true
cache_ttl_seconds = 300

[validation.formats]
json_schema = { enabled = true, default_draft = "2020-12" }
protobuf = { enabled = true, editions = ["2023"] }
graphql = { enabled = true, federation = false }
openapi = { enabled = true, versions = ["3.0", "3.1"] }
avro = { enabled = false }

[registry]
require_description = true
require_author = true
max_schema_size_kb = 5120
allowed_formats = ["json-schema", "protobuf", "graphql", "openapi"]
default_namespace = "default"
enable_search = true
search_language = "english"

[compatibility]
default_policy = "backward"
auto_deprecate_old = true
deprecation_grace_period_days = 30
require_compatibility_check = true
allow_breaking_with_override = true

[security]
auth_enabled = true
auth_type = "jwt"  # jwt, api_key, oauth2
jwt_secret = "${JWT_SECRET}"
jwt_issuer = "schemaforge"
jwt_audience = "schemaforge-api"
token_expiry_hours = 24

[security.rate_limit]
enabled = true
requests_per_minute = 100
burst_size = 20

[logging]
level = "info"  # trace, debug, info, warn, error
format = "json"  # json, pretty
output = "stdout"  # stdout, stderr, file
file_path = "/var/log/schemaforge.log"
max_file_size_mb = 100
max_files = 5

[logging.fields]
include_timestamp = true
include_request_id = true
include_user_id = true
include_ip = false  # Privacy consideration

[metrics]
enabled = true
port = 9090
path = "/metrics"
format = "prometheus"
include_latency_histograms = true

[codegen]
enabled = true
default_language = "typescript"
target_languages = ["typescript", "python", "go", "rust", "java"]
output_directory = "./generated"
templates_directory = "./templates"

[notifications]
webhook_enabled = true
webhook_url = "https://hooks.example.com/schemaforge"
webhook_secret = "${WEBHOOK_SECRET}"
events = ["schema.published", "schema.deprecated", "compatibility.broken"]

[notifications.slack]
enabled = false
webhook_url = "${SLACK_WEBHOOK_URL}"
channel = "#schema-changes"
```

### 8.2 Environment Variables

| Variable | Type | Description | Default | Required |
|----------|------|-------------|---------|----------|
| SCHEMAFORGE_CONFIG | Path | Config file path | ./config.toml | No |
| SCHEMAFORGE_HOST | String | Server host | 0.0.0.0 | No |
| SCHEMAFORGE_PORT | Integer | Server port | 8080 | No |
| SCHEMAFORGE_LOG_LEVEL | String | Log level | info | No |
| SCHEMAFORGE_DB_URL | String | Database URL | - | Yes |
| SCHEMAFORGE_DB_PASSWORD | String | Database password | - | Yes |
| SCHEMAFORGE_REDIS_URL | String | Redis URL | redis://localhost:6379 | No |
| SCHEMAFORGE_JWT_SECRET | String | JWT signing secret | - | Yes (if auth enabled) |
| SCHEMAFORGE_WEBHOOK_SECRET | String | Webhook HMAC secret | - | No |
| SCHEMAFORGE_S3_BUCKET | String | S3 bucket for large schemas | - | No |
| SCHEMAFORGE_S3_ACCESS_KEY | String | S3 access key | - | If S3 enabled |
| SCHEMAFORGE_S3_SECRET_KEY | String | S3 secret key | - | If S3 enabled |

---

## 9. Validation System

### 9.1 JSON Schema Validation

Supported JSON Schema 2020-12 keywords:

| Keyword | Description | Example | Complexity |
|---------|-------------|---------|------------|
| `type` | Data type validation | `{ "type": "string" }` | Low |
| `properties` | Object properties | `{ "properties": { "name": {} } }` | Low |
| `required` | Required fields | `{ "required": ["id", "name"] }` | Low |
| `additionalProperties` | Extra properties | `{ "additionalProperties": false }` | Low |
| `unevaluatedProperties` | Tracking | `{ "unevaluatedProperties": false }` | High |
| `patternProperties` | Regex keys | `{ "patternProperties": {"^S": {}} }` | Medium |
| `propertyNames` | Key schema | `{ "propertyNames": {"maxLength": 10} }` | Medium |
| `items` | Array items | `{ "items": {"type": "number"} }` | Low |
| `prefixItems` | Tuple schema | `{ "prefixItems": [{}, {}] }` | Medium |
| `contains` | At least one | `{ "contains": {"type": "integer"} }` | Medium |
| `enum` | Fixed values | `{ "enum": ["a", "b", "c"] }` | Low |
| `const` | Single value | `{ "const": "value" }` | Low |
| `multipleOf` | Division | `{ "multipleOf": 0.5 }` | Medium |
| `minimum/maximum` | Range | `{ "minimum": 0 }` | Low |
| `minLength/maxLength` | Length | `{ "minLength": 1 }` | Low |
| `pattern` | Regex | `{ "pattern": "^[a-z]+$" }` | Medium |
| `format` | Known formats | `{ "format": "email" }` | Medium |
| `$ref` | Static reference | `{ "$ref": "#/$defs/id" }` | Medium |
| `$dynamicRef` | Late binding | `{ "$dynamicRef": "#meta" }` | High |
| `allOf/anyOf/oneOf` | Logic | `{ "anyOf": [{}, {}] }` | Medium |
| `not` | Negation | `{ "not": {"type": "null"} }` | Medium |
| `if/then/else` | Conditional | `{ "if": {}, "then": {} }` | High |

### 9.2 Custom Validators

```rust
use schemaforge::validation::{Validator, ValidationContext, ValidationError};

/// Custom email validator
pub struct EmailValidator;

impl Validator for EmailValidator {
    fn validate(&self, value: &Value, context: &ValidationContext) -> Result<(), ValidationError> {
        let email = value.as_str()
            .ok_or(ValidationError::type_mismatch("string", value))?
            .trim();

        // RFC 5322 compliant regex (simplified)
        let email_regex = regex::Regex::new(
            r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
        ).unwrap();

        if !email_regex.is_match(email) {
            return Err(ValidationError::custom(
                "format",
                format!("'{}' is not a valid email address", email)
            ));
        }

        // Additional validation: check domain exists (optional)
        if context.strict_mode {
            let domain = email.split('@').nth(1).unwrap();
            if !is_valid_domain(domain) {
                return Err(ValidationError::custom(
                    "domain",
                    format!("Domain '{}' does not exist", domain)
                ));
            }
        }

        Ok(())
    }
}

// Register custom validator
registry.register_format_validator("corporate-email", EmailValidator);
```

---

## 10. Migration Framework

### 10.1 Migration Plan Structure

```rust
pub struct MigrationPlan {
    pub from_version: Version,
    pub to_version: Version,
    pub steps: Vec<MigrationStep>,
    pub reversible: bool,
    pub estimated_duration: Duration,
    pub breaking_changes: Vec<BreakingChange>,
    pub data_transformations: Vec<DataTransformation>,
}

pub enum MigrationStep {
    AddField {
        field: String,
        default: Option<Value>,
        optional: bool,
    },
    RemoveField {
        field: String,
        alias: Option<String>,
        grace_period_days: u32,
    },
    RenameField {
        from: String,
        to: String,
        keep_alias: bool,
    },
    ChangeType {
        field: String,
        from: Type,
        to: Type,
        coercion: CoercionStrategy,
    },
    AddConstraint {
        field: String,
        constraint: Constraint,
        backfill: BackfillStrategy,
    },
    RemoveConstraint {
        field: String,
        constraint: Constraint,
    },
    Backfill {
        field: String,
        strategy: BackfillStrategy,
    },
    DeprecateField {
        field: String,
        reason: String,
        removal_version: Option<Version>,
    },
}

pub enum BackfillStrategy {
    DefaultValue(Value),
    Computed(String), // Expression or function name
    FromField(String), // Copy from another field
    Custom(String), // User-defined transformation
}
```

### 10.2 Migration Detection Rules

| Scenario | Detection | Migration Strategy | Risk Level |
|----------|-----------|-------------------|------------|
| Field added (optional) | New field with no consumer | None needed | Low |
| Field added (required) | New required field | Add with default + backfill | Medium |
| Field removed | Missing field in new | Deprecation + alias period | High |
| Field renamed | Semantic similarity | Alias mapping | Medium |
| Type widened | Superset type | None needed | Low |
| Type narrowed | Subset type | Validation + coercion | High |
| Constraint relaxed | Less restrictive | None needed | Low |
| Constraint tightened | More restrictive | Data transformation | High |
| Enum value added | New option | Safe for producers | Low |
| Enum value removed | Missing option | Deprecation period | High |
| Pattern changed | Regex modified | Validation testing | Medium |
| Default changed | Different default | Version bump warning | Low |

---

## 11. Integration Points

### 11.1 CI/CD Integration

#### GitHub Actions

```yaml
# .github/workflows/schema-validation.yml
name: Schema Validation

on:
  push:
    paths:
      - 'schemas/**'
      - 'proto/**'
  pull_request:
    paths:
      - 'schemas/**'
      - 'proto/**'

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Schemaforge Validate
        uses: phenotype/schemaforge-action@v1
        with:
          registry-url: ${{ secrets.SCHEMAFORGE_URL }}
          api-key: ${{ secrets.SCHEMAFORGE_API_KEY }}
          schema-path: './schemas'
          proto-path: './proto'
          fail-on-breaking: true
          compatibility-level: 'backward'
          generate-report: true
      
      - name: Upload Report
        if: always()
        uses: actions/upload-artifact@v4
        with:
          name: schema-report
          path: schemaforge-report.json

  compatibility-check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0  # Full history for comparison
      
      - name: Check Compatibility with Main
        uses: phenotype/schemaforge-action@v1
        with:
          command: 'check-compatibility'
          base-ref: 'origin/main'
          head-ref: 'HEAD'
          fail-on-breaking: true
```

#### GitLab CI

```yaml
# .gitlab-ci.yml
schema:validate:
  stage: test
  image: phenotype/schemaforge-cli:latest
  script:
    - schemaforge validate --path ./schemas --strict
    - schemaforge check --against origin/main --fail-on-breaking
  only:
    changes:
      - schemas/**/*
      - proto/**/*
```

### 11.2 SDK Support

| Language | Package | Version | Status | Features |
|----------|---------|---------|--------|----------|
| Rust | crates.io/schemaforge | 0.1.0 | ✅ Active | Full feature parity |
| JavaScript | npm/@phenotype/schemaforge | 0.1.0 | ✅ Active | Validation, client |
| Python | pypi/schemaforge | 0.1.0 | ✅ Active | Validation, client |
| Go | github.com/phenotype/schemaforge | v0.1.0 | 🚧 In Progress | Validation |
| Java | maven/io.phenotype/schemaforge | 0.1.0 | 📋 Planned | Validation |
| C# | nuget/Phenotype.Schemaforge | 0.1.0 | 📋 Planned | Validation |
| TypeScript | npm/@phenotype/schemaforge | 0.1.0 | ✅ Active | Validation, Zod |
| Kotlin | maven/io.phenotype/schemaforge | 0.1.0 | 📋 Planned | Android support |

---

## 12. Security Considerations

### 12.1 Authentication & Authorization

| Method | Use Case | Status | Implementation |
|--------|----------|--------|----------------|
| API Key | Service-to-service | ✅ Implemented | HMAC-SHA256 |
| JWT | User authentication | ✅ Implemented | RS256 |
| OAuth 2.0 | External integrations | 📋 Planned | PKCE flow |
| mTLS | Internal cluster | 🔬 Research | Certificate-based |

### 12.2 Access Control Matrix

| Role | Read | Write | Delete | Admin | Namespace Scope |
|------|------|-------|--------|-------|-----------------|
| Consumer | ✅ All | ❌ No | ❌ No | ❌ No | All |
| Producer | ✅ Own | ✅ Own | ❌ No | ❌ No | Own namespaces |
| Owner | ✅ All | ✅ All | ✅ Own | ❌ No | Own namespaces |
| Admin | ✅ All | ✅ All | ✅ All | ✅ Yes | All |
| Auditor | ✅ All | ❌ No | ❌ No | ⚠️ Read-only | All |

### 12.3 Security Best Practices

- **Secret Scanning**: All schema content scanned for secrets before storage
- **Schema IDs**: UUID v4 for unpredictability, preventing enumeration
- **Audit Logging**: All write operations logged with user, timestamp, changes
- **Rate Limiting**: Configurable per-endpoint, per-user, per-IP
- **Input Sanitization**: Schema names validated against injection patterns
- **Content Security**: Max schema size limits, depth limits on nested structures
- **Encryption**: At-rest encryption for sensitive schemas, TLS in transit
- **Backup**: Encrypted backups with 30-day retention

---

## 13. Performance Requirements

### 13.1 Latency Targets

| Operation | p50 | p95 | p99 | Max | Notes |
|-----------|-----|-----|-----|-----|-------|
| Schema publish | 50ms | 150ms | 300ms | 1s | Includes validation |
| Schema lookup (cached) | 5ms | 15ms | 30ms | 100ms | Redis hit |
| Schema lookup (uncached) | 20ms | 50ms | 100ms | 500ms | DB query |
| Validation (small <1KB) | 5ms | 15ms | 30ms | 100ms | Cached result |
| Validation (medium <10KB) | 10ms | 30ms | 50ms | 200ms | Live validation |
| Validation (large <1MB) | 50ms | 150ms | 300ms | 1s | Streaming |
| Compatibility check | 20ms | 50ms | 100ms | 500ms | Diff + rules |
| Search | 30ms | 100ms | 200ms | 1s | Full-text |

### 13.2 Throughput Targets

| Metric | Target | Burst | Notes |
|--------|--------|-------|-------|
| Schema operations/sec | 1000 | 5000 | Read-heavy workload |
| Validations/sec | 5000 | 20000 | Cached results |
| Concurrent connections | 500 | 2000 | Per instance |
| Webhook deliveries/sec | 100 | 500 | Async queue |
| Search queries/sec | 100 | 500 | Full-text |

### 13.3 Resource Usage

| Resource | Idle | Normal | Peak | Limit |
|----------|------|--------|------|-------|
| Memory | 128MB | 256MB | 512MB | 1GB |
| CPU | 1% | 10% | 80% | 100% |
| Disk (per schema) | - | 50KB avg | 5MB max | 10MB hard |
| Network (ingress) | 1KB/s | 100KB/s | 10MB/s | 50MB/s |
| Network (egress) | 1KB/s | 200KB/s | 20MB/s | 50MB/s |

---

## 14. Testing Strategy

### 14.1 Test Categories

| Category | Coverage | Tools | Execution |
|----------|----------|-------|-----------|
| Unit tests | 80% min | `cargo test`, `mockall` | Every commit |
| Integration tests | All public APIs | `tokio::test`, `testcontainers` | Pre-merge |
| Property tests | Core algorithms | `proptest`, `quickcheck` | Pre-merge |
| Fuzz tests | Parsers, validators | `cargo-fuzz`, `arbitrary` | Nightly |
| E2E tests | CLI, API workflows | `assert_cmd`, `reqwest` | Pre-release |
| Benchmark tests | Hot paths | `criterion` | Weekly |
| Conformance tests | JSON Schema | `bowtie` | Nightly |

### 14.2 Test Pyramid

```
                    ┌─────────┐
                    │   E2E   │  10% - Full workflow tests
                    │  ~50ms  │
                    └────┬────┘
                ┌────────┴────────┐
                │  Integration    │  30% - API and component tests
                │     ~10ms       │
                └────────┬────────┘
        ┌─────────────────┴─────────────────┐
        │           Unit Tests               │  60% - Individual function tests
        │            <1ms                    │
        └────────────────────────────────────┘
```

### 14.3 Test Data Strategy

| Data Type | Source | Refresh | Usage |
|-----------|--------|---------|-------|
| Static fixtures | Git | Manual | Unit tests |
| Generated schemas | `proptest` | Every run | Property tests |
| Production anonymized | ETL pipeline | Weekly | Integration tests |
| Conformance suite | JSON Schema Org | On release | Compliance |

---

## 15. Deployment & Operations

### 15.1 Deployment Options

| Mode | Target | Complexity | Scaling | Use Case |
|------|--------|------------|---------|----------|
| **Standalone** | Single binary | Low | Vertical | Development, small teams |
| **Docker** | Container | Low | Horizontal | CI/CD, small production |
| **Kubernetes** | K8s cluster | Medium | Auto | Production, enterprise |
| **Helm** | K8s via Helm | Medium | Auto | Production, gitops |
| **Terraform** | Cloud infra | High | Auto | Multi-region, enterprise |

### 15.2 Kubernetes Deployment

```yaml
# helm/schemaforge/values.yaml
replicaCount: 3

image:
  repository: phenotype/schemaforge
  tag: "2.0.0"
  pullPolicy: IfNotPresent

service:
  type: ClusterIP
  port: 8080

ingress:
  enabled: true
  className: nginx
  hosts:
    - host: schemaforge.phenotype.dev
      paths:
        - path: /
          pathType: Prefix
  tls:
    - secretName: schemaforge-tls
      hosts:
        - schemaforge.phenotype.dev

resources:
  limits:
    cpu: 1000m
    memory: 512Mi
  requests:
    cpu: 100m
    memory: 256Mi

autoscaling:
  enabled: true
  minReplicas: 3
  maxReplicas: 10
  targetCPUUtilizationPercentage: 70
  targetMemoryUtilizationPercentage: 80

persistence:
  enabled: true
  storageClass: standard
  size: 10Gi

database:
  type: postgres
  host: schemaforge-postgres
  port: 5432
  name: schemaforge
  existingSecret: schemaforge-db-credentials

cache:
  type: redis
  host: schemaforge-redis
  port: 6379

monitoring:
  enabled: true
  serviceMonitor:
    enabled: true
    namespace: monitoring
```

### 15.3 Health Checks

| Endpoint | Type | Frequency | Timeout | Action on Failure |
|----------|------|-----------|---------|-------------------|
| /health/live | Liveness | 10s | 5s | Restart pod |
| /health/ready | Readiness | 5s | 3s | Remove from LB |
| /health/deep | Startup | 30s | 10s | Fail deployment |

### 15.4 Alerting Rules

| Alert | Condition | Severity | Action |
|-------|-----------|----------|--------|
| HighErrorRate | >5% errors in 5min | Critical | Page on-call |
| HighLatency | p95 > 500ms | Warning | Slack notification |
| LowCacheHit | <80% cache hits | Warning | Investigate |
| DBConnectionFail | Connection pool exhausted | Critical | Page on-call |
| SchemaValidationFail | >10% validation failures | Warning | Notify platform team |
| DiskSpaceLow | >80% disk used | Warning | Cleanup old versions |

---

## 16. Technical Deep Dive

### 16.1 Schema Parsing Architecture

Schemaforge uses a multi-stage parsing pipeline:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         Parsing Pipeline                                     │
│                                                                              │
│  ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐  │
│  │  Input   │ → │  Format  │ → │   AST    │ → │  Normalize│ → │ Validate │  │
│  │ (bytes)  │   │  Detect  │   │  Parse   │   │           │   │   AST    │  │
│  └──────────┘   └──────────┘   └──────────┘   └──────────┘   └──────────┘  │
│       │            │              │              │              │         │
│       │            │              │              │              │         │
│       ▼            ▼              ▼              ▼              ▼         │
│  ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐  │
│  │ Raw      │   │ Confidence│   │ Format   │   │ Canonical │   │ Well-    │  │
│  │ Content  │   │ Score    │   │ Specific │   │ Form      │   │ Formed   │  │
│  │          │   │ (0-1)    │   │ Tree     │   │           │   │ Check    │  │
│  └──────────┘   └──────────┘   └──────────┘   └──────────┘   └──────────┘  │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 16.2 Validation Pipeline

```rust
pub struct ValidationPipeline {
    stages: Vec<Box<dyn ValidationStage>>,
}

#[async_trait]
pub trait ValidationStage: Send + Sync {
    fn name(&self) -> &str;
    async fn validate(&self, ctx: &mut ValidationContext) -> Result<(), ValidationError>;
}

// Standard validation stages:
// 1. Syntax validation (parsing, tokenization)
// 2. Structural validation (schema well-formedness)
// 3. Semantic validation (logical consistency)
// 4. Reference validation ($ref resolution, imports)
// 5. Compatibility validation (with previous versions)
// 6. Security validation (circular refs, ReDoS patterns)

impl ValidationPipeline {
    pub fn standard() -> Self {
        Self {
            stages: vec![
                Box::new(SyntaxValidationStage::new()),
                Box::new(StructuralValidationStage::new()),
                Box::new(SemanticValidationStage::new()),
                Box::new(ReferenceValidationStage::new()),
                Box::new(CompatibilityValidationStage::new()),
                Box::new(SecurityValidationStage::new()),
            ],
        }
    }

    pub async fn run(&self, schema: &dyn Schema) -> ValidationReport {
        let mut ctx = ValidationContext::new(schema);
        let mut report = ValidationReport::default();

        for stage in &self.stages {
            let start = Instant::now();
            
            match stage.validate(&mut ctx).await {
                Ok(()) => {
                    report.add_stage_success(stage.name(), start.elapsed());
                }
                Err(e) => {
                    report.add_stage_error(stage.name(), e, start.elapsed());
                    if e.is_fatal() {
                        break;
                    }
                }
            }
        }

        report
    }
}
```

### 16.3 JSON Schema 2020-12 Implementation

| Keyword | Implementation Strategy | Complexity | Optimizations |
|---------|------------------------|------------|---------------|
| `type` | Direct Rust type matching | Low | Enum dispatch |
| `properties` | HashMap<String, Schema> | Medium | Lazy compilation |
| `required` | HashSet intersection | Low | Bitset for small sets |
| `pattern` | Regex compilation cache | Medium | LRU cache |
| `format` | Pluggable validators | Medium | Built-in common |
| `$ref` | DAG traversal + memoization | High | Cycle detection |
| `$dynamicRef` | Lazy resolution with scope | High | Scope stack |
| `unevaluatedProperties` | Validation state tracking | High | Bitmask tracking |
| `oneOf` | Sequential with short-circuit | Medium | Early exit |
| `if/then/else` | Conditional subschema | High | Branch prediction |

### 16.4 Compatibility Checking Algorithm

```rust
pub fn check_compatibility(old: &CanonicalSchema, new: &CanonicalSchema) -> CompatibilityReport {
    let mut changes = Vec::new();
    let mut level = CompatibilityLevel::Full;

    // Field-level analysis
    for (field_name, old_field) in &old.fields {
        match new.fields.get(field_name) {
            Some(new_field) => {
                // Field exists in both - check type compatibility
                if !types_compatible(&old_field.typ, &new_field.typ) {
                    changes.push(Change::Breaking {
                        category: "field_type_change",
                        path: format!("#/{}", field_name),
                        description: format!(
                            "Field '{}' type changed from {:?} to {:?}",
                            field_name, old_field.typ, new_field.typ
                        ),
                        migration_hint: format!(
                            "Add migration to convert {} to {}",
                            old_field.typ, new_field.typ
                        ),
                    });
                    level = level.min(CompatibilityLevel::None);
                }

                // Check constraint changes
                if let Some(constraint_change) = compare_constraints(
                    &old_field.constraints,
                    &new_field.constraints
                ) {
                    changes.push(constraint_change);
                    if constraint_change.is_breaking() {
                        level = level.min(CompatibilityLevel::None);
                    } else if constraint_change.is_warning() {
                        level = level.min(CompatibilityLevel::Backward);
                    }
                }
            }
            None => {
                // Field removed
                if old_field.required {
                    changes.push(Change::Breaking {
                        category: "required_field_removed",
                        path: format!("#/{}", field_name),
                        description: format!("Required field '{}' removed", field_name),
                        migration_hint: format!(
                            "Consider deprecating '{}' before removal",
                            field_name
                        ),
                    });
                    level = level.min(CompatibilityLevel::None);
                } else {
                    changes.push(Change::Warning {
                        category: "optional_field_removed",
                        path: format!("#/{}", field_name),
                        description: format!("Optional field '{}' removed", field_name),
                    });
                }
            }
        }
    }

    // Check for new required fields
    for (field_name, new_field) in &new.fields {
        if new_field.required && !old.fields.contains_key(field_name) {
            changes.push(Change::Breaking {
                category: "required_field_added",
                path: format!("#/{}", field_name),
                description: format!("New required field '{}' added", field_name),
                migration_hint: format!(
                    "Consider making '{}' optional or providing a default",
                    field_name
                ),
            });
            level = level.min(CompatibilityLevel::None);
        }
    }

    CompatibilityReport { level, changes }
}
```

### 16.5 Migration Planning Algorithm

```rust
impl MigrationPlanner {
    pub async fn plan(&self, from: &Schema, to: &Schema) -> Result<MigrationPlan, Error> {
        let changes = self.detect_changes(from, to).await?;
        let mut steps = Vec::new();
        let mut reversible = true;

        for change in changes {
            match change.kind {
                ChangeKind::AddOptionalField { field, schema } => {
                    // No migration needed - new consumers can handle it
                    steps.push(MigrationStep::AddField {
                        field: field.clone(),
                        default: schema.default.clone(),
                        optional: true,
                    });
                }

                ChangeKind::AddRequiredField { field, schema } => {
                    // Need backfill strategy
                    let backfill = determine_backfill_strategy(&schema)?;
                    steps.push(MigrationStep::Backfill {
                        field: field.clone(),
                        strategy: backfill,
                    });
                    steps.push(MigrationStep::AddConstraint {
                        field: field.clone(),
                        constraint: Constraint::NotNull,
                        backfill: BackfillStrategy::DefaultValue(schema.default.clone().unwrap_or(Value::Null)),
                    });
                }

                ChangeKind::RemoveField { field, was_required } => {
                    reversible = false; // Data loss
                    steps.push(MigrationStep::DeprecateField {
                        field: field.clone(),
                        reason: "Field removed in new version".to_string(),
                        removal_version: Some(bump_major(&to.id.version)),
                    });
                    steps.push(MigrationStep::RemoveField {
                        field: field.clone(),
                        alias: None,
                        grace_period_days: 30,
                    });
                }

                ChangeKind::RenameField { old_name, new_name } => {
                    steps.push(MigrationStep::RenameField {
                        from: old_name.clone(),
                        to: new_name.clone(),
                        keep_alias: true, // Support both during transition
                    });
                }

                ChangeKind::TypeChange { field, from, to } => {
                    if can_coerce(&from, &to) {
                        steps.push(MigrationStep::ChangeType {
                            field: field.clone(),
                            from,
                            to,
                            coercion: CoercionStrategy::Automatic,
                        });
                    } else {
                        steps.push(MigrationStep::ChangeType {
                            field: field.clone(),
                            from,
                            to,
                            coercion: CoercionStrategy::Manual(format!(
                                "Custom migration needed: {:?} -> {:?}", from, to
                            )),
                        });
                    }
                }

                _ => {}
            }
        }

        Ok(MigrationPlan {
            from_version: from.id.version.clone(),
            to_version: to.id.version.clone(),
            steps,
            reversible,
            estimated_duration: self.estimate_duration(&steps),
            breaking_changes: changes.into_iter()
                .filter(|c| c.is_breaking())
                .map(|c| c.into())
                .collect(),
            data_transformations: vec![],
        })
    }

    fn estimate_duration(&self, steps: &[MigrationStep]) -> Duration {
        let base_time = Duration::from_secs(1); // Overhead
        let per_step = Duration::from_millis(100);
        let per_field = Duration::from_millis(50);

        let total_fields: usize = steps.iter().map(|s| s.field_count()).sum();

        base_time + per_step * steps.len() as u32 + per_field * total_fields as u32
    }
}
```

---

## 17. Glossary

| Term | Definition |
|------|------------|
| **Schema** | Structured definition of data format |
| **JSON Schema** | JSON-based schema specification language |
| **Protocol Buffers** | Google's binary serialization format |
| **GraphQL SDL** | GraphQL Schema Definition Language |
| **OpenAPI** | REST API specification standard |
| **Avro** | Apache binary serialization format |
| **Breaking Change** | Change that breaks backward or forward compatibility |
| **Backward Compatible** | New schema can read old data |
| **Forward Compatible** | Old schema can read new data |
| **Full Compatibility** | Both backward and forward compatible |
| **Migration** | Transitioning data between schema versions |
| **Registry** | Centralized schema storage and versioning |
| **Validator** | Schema validation component |
| **Canonical Form** | Normalized, implementation-independent representation |
| **Compatibility Check** | Analysis for breaking changes between versions |
| **Schema Linting** | Automated style and best-practice checking |
| **Contract Testing** | API contract verification between services |
| **Schema Evolution** | Changing schemas over time safely |
| **Deprecation** | Marking schema elements as obsolete |
| **Backfill** | Updating existing data to match new schema |
| **CDC** | Change Data Capture |
| **Data Contract** | Producer-consumer agreement on data format |
| **Type Lattice** | Type ordering by subtyping relationship |
| **Constraint Propagation** | Inferring constraints from related fields |
| **Schema Squatting** | Malicious registration of schema names |
| **Pattern Injection** | Malicious regex pattern exploit |
| **Circular Reference** | Self-referencing schema structure |
| **$dynamicRef** | Late-bound JSON Schema reference |
| **Unevaluated Properties** | JSON Schema additional property tracking |
| **Prost** | Rust Protocol Buffers implementation |
| **Buf** | Modern Protocol Buffers toolchain |
| **AJV** | JavaScript JSON Schema validator |
| **Zod** | TypeScript-first validation library |
| **Pydantic** | Python data validation library |
| **JSONPath** | JSON query language |
| **JSON Pointer** | RFC 6901 reference syntax |
| **JSON Patch** | RFC 6902 diff format |
| **Schema Derivation** | Generating schemas from code |
| **Code Generation** | Creating types from schema definitions |
| **Content Addressing** | Referencing data by hash |
| **Semantic Versioning** | Version numbering scheme (SemVer) |
| **Namespace** | Organizational scope for schemas |

---

## Appendix A: Architecture Decision Records

See [ADR.md](./ADR.md) for detailed architecture decision records:

- **ADR-001**: Multi-Format Schema Engine Architecture (Plugin-based)
- **ADR-002**: Storage Architecture - PostgreSQL with JSONB + Redis Cache
- **ADR-003**: Validation Engine - Streaming Async with Partial Results

---

## Appendix B: Reference URLs (200+)

### B.1 JSON Schema Standards & Tools (1-30)

| # | Resource | URL | Description |
|---|----------|-----|-------------|
| 1 | JSON Schema Spec | https://json-schema.org/ | Official specification |
| 2 | JSON Schema 2020-12 | https://json-schema.org/draft/2020-12/schema | Current draft |
| 3 | Understanding JSON Schema | https://json-schema.org/understanding-json-schema/ | Learning resource |
| 4 | JSON Schema Test Suite | https://github.com/json-schema-org/JSON-Schema-Test-Suite | Compliance tests |
| 5 | Bowtie | https://bowtie.report/ | Implementation comparison |
| 6 | jsonschema (Rust) | https://github.com/Stranger6667/jsonschema-rs | Rust validator |
| 7 | AJV | https://ajv.js.org/ | JavaScript validator |
| 8 | jsonschema (Python) | https://python-jsonschema.readthedocs.io/ | Python validator |
| 9 | Hyperjump | https://github.com/hyperjump-io/json-schema-validator | JS validator |
| 10 | quicktype | https://quicktype.io/ | Code generation |
| 11 | jsonschema2pojo | https://jsonschema2pojo.org/ | Java generator |
| 12 | jsonschema-to-typescript | https://github.com/bcherny/json-schema-to-typescript | TS generator |
| 13 | datamodel-code-generator | https://github.com/koxudaxi/datamodel-code-generator/ | Python generator |
| 14 | go-jsonschema | https://github.com/invopop/jsonschema | Go generator |
| 15 | json-schema-to-ts | https://github.com/ThomasAribart/json-schema-to-ts | TS types |
| 16 | JSON Schema Store | https://www.schemastore.org/json/ | Schema collection |
| 17 | JSON Path | https://goessner.net/articles/JsonPath/ | JSON query |
| 18 | JSON Pointer (RFC 6901) | https://datatracker.ietf.org/doc/html/rfc6901 | Reference syntax |
| 19 | JSON Patch (RFC 6902) | https://datatracker.ietf.org/doc/html/rfc6902 | Diff format |
| 20 | JSON Merge Patch (RFC 7386) | https://datatracker.ietf.org/doc/html/rfc7386 | Merge format |
| 21 | jq | https://jqlang.github.io/jq/ | JSON processor |
| 22 | jaq | https://github.com/01mf02/jaq | Rust jq clone |
| 23 | jmespath | https://jmespath.org/ | JSON query language |
| 24 | jsonata | https://jsonata.org/ | JSON query/transform |
| 25 | simdjson | https://github.com/simdjson/simdjson | Fast JSON parser |
| 26 | rapidjson | https://rapidjson.org/ | C++ JSON parser |
| 27 | serde_json | https://github.com/serde-rs/json | Rust JSON |
| 28 | sonic-rs | https://github.com/cloudwego/sonic-rs | Fast Rust JSON |
| 29 | yyjson | https://github.com/ibireme/yyjson | Fast C JSON |
| 30 | json5 | https://json5.org/ | Human-friendly JSON |

### B.2 Protocol Buffers (31-60)

| # | Resource | URL | Description |
|---|----------|-----|-------------|
| 31 | Protocol Buffers | https://protobuf.dev/ | Official documentation |
| 32 | Protobuf Language Guide | https://protobuf.dev/programming-guides/proto3/ | Proto3 guide |
| 33 | Protobuf Editions | https://protobuf.dev/editions/overview/ | Editions guide |
| 34 | protoc | https://github.com/protocolbuffers/protobuf | Compiler |
| 35 | Buf | https://buf.build/ | Modern Protobuf toolchain |
| 36 | Buf Schema Registry | https://buf.build/docs/bsr/introduction/ | BSR docs |
| 37 | protoc-gen-go | https://pkg.go.dev/google.golang.org/protobuf/cmd/protoc-gen-go | Go plugin |
| 38 | prost | https://github.com/tokio-rs/prost | Rust implementation |
| 39 | ts-proto | https://github.com/stephenh/ts-proto | TypeScript plugin |
| 40 | ts-protoc-gen | https://github.com/improbable-eng/ts-protoc-gen | TS plugin |
| 41 | protobuf-ts | https://github.com/timostamm/protobuf-ts | TS plugin |
| 42 | protoc-gen-grpc-web | https://github.com/grpc/grpc-web | gRPC-web |
| 43 | protoc-gen-validate | https://github.com/bufbuild/protoc-gen-validate | Validation |
| 44 | protovalidate | https://github.com/bufbuild/protovalidate | PGV successor |
| 45 | protoc-gen-openapi | https://github.com/google/gnostic | OpenAPI gen |
| 46 | grpc-gateway | https://github.com/grpc-ecosystem/grpc-gateway | REST gateway |
| 47 | gRPC | https://grpc.io/ | RPC framework |
| 48 | gRPC-Web | https://github.com/grpc/grpc-web | Browser gRPC |
| 49 | Connect | https://connect.build/ | gRPC alternative |
| 50 | Twirp | https://twitchtv.github.io/twirp/ | Simple RPC |
| 51 | protobuf-java | https://github.com/protocolbuffers/protobuf/tree/main/java | Java impl |
| 52 | protobuf-python | https://github.com/protocolbuffers/protobuf/tree/main/python | Python impl |
| 53 | protobuf-javascript | https://github.com/protocolbuffers/protobuf-javascript | JS impl |
| 54 | betterproto | https://github.com/danielgtaylor/python-betterproto | Python async |
| 55 | aioprotobuf | https://github.com/micro-bitcoin/ugrpc | Async Python |
| 56 | protobuf-c | https://github.com/protobuf-c/protobuf-c | C implementation |
| 57 | nanopb | https://github.com/nanopb/nanopb | Embedded C |
| 58 | wire | https://github.com/square/wire | Kotlin/Android |
| 59 | swift-protobuf | https://github.com/apple/swift-protobuf | Swift impl |
| 60 | protobuf-csharp | https://github.com/protocolbuffers/protobuf/tree/main/csharp | C# impl |

### B.3 GraphQL (61-90)

| # | Resource | URL | Description |
|---|----------|-----|-------------|
| 61 | GraphQL Spec | https://spec.graphql.org/ | Official specification |
| 62 | GraphQL Foundation | https://graphql.org/foundation/ | GraphQL foundation |
| 63 | graphql-js | https://github.com/graphql/graphql-js | Reference impl |
| 64 | Apollo Server | https://www.apollographql.com/docs/apollo-server/ | Node.js server |
| 65 | Apollo Client | https://www.apollographql.com/docs/react/ | React client |
| 66 | Apollo Federation | https://www.apollographql.com/docs/federation/ | Federated graphs |
| 67 | Apollo Studio | https://www.apollographql.com/docs/studio/ | Graph platform |
| 68 | gqlgen | https://gqlgen.com/ | Go generator |
| 69 | async-graphql | https://async-graphql.github.io/async-graphql/ | Rust server |
| 70 | graphql-ruby | https://graphql-ruby.org/ | Ruby server |
| 71 | graphene | https://graphene-python.org/ | Python server |
| 72 | strawberry | https://strawberry.rocks/ | Python server |
| 73 | sangria | https://sangria-graphql.github.io/ | Scala server |
| 74 | Hot Chocolate | https://chillicream.com/docs/hotchocolate/ | .NET server |
| 75 | GraphQL.NET | https://graphql-dotnet.github.io/ | .NET server |
| 76 | graphql-go | https://github.com/graphql-go/graphql | Go server |
| 77 | graphql-java | https://www.graphql-java.com/ | Java server |
| 78 | graphql-kotlin | https://opensource.expediagroup.com/graphql-kotlin/ | Kotlin |
| 79 | graphql-codegen | https://the-guild.dev/graphql/codegen | Code gen |
| 80 | Relay | https://relay.dev/ | React framework |
| 81 | urql | https://formidable.com/open-source/urql/ | GraphQL client |
| 82 | graphql-request | https://github.com/prisma-labs/graphql-request | Simple client |
| 83 | graphql-ws | https://github.com/enisdenjo/graphql-ws | WebSocket transport |
| 84 | graphql-subscriptions | https://github.com/apollographql/graphql-subscriptions | Subscriptions |
| 85 | graphql-playground | https://github.com/graphql/graphql-playground | IDE |
| 86 | Altair | https://altairgraphql.dev/ | GraphQL client IDE |
| 87 | Insomnia | https://insomnia.rest/ | API client |
| 88 | Postman GraphQL | https://www.postman.com/graphql/ | Postman support |
| 89 | GraphiQL | https://github.com/graphql/graphiql | Reference IDE |
| 90 | OneGraph | https://www.onegraph.com/ | GraphQL tools |

### B.4 OpenAPI / REST (91-120)

| # | Resource | URL | Description |
|---|----------|-----|-------------|
| 91 | OpenAPI Spec | https://spec.openapis.org/oas/v3.1.0 | OpenAPI 3.1 |
| 92 | OpenAPI Initiative | https://www.openapis.org/ | OpenAPI org |
| 93 | Swagger | https://swagger.io/ | Swagger tools |
| 94 | Swagger UI | https://github.com/swagger-api/swagger-ui | API docs UI |
| 95 | Swagger Editor | https://editor.swagger.io/ | Online editor |
| 96 | Swagger Codegen | https://github.com/swagger-api/swagger-codegen | Code gen |
| 97 | SwaggerHub | https://swaggerhub.com/ | API platform |
| 98 | ReDoc | https://github.com/Redocly/redoc | OpenAPI UI |
| 99 | Redocly CLI | https://redocly.com/docs/cli/ | Linting, bundling |
| 100 | Stoplight Studio | https://stoplight.io/studio | API design |
| 101 | Prism | https://stoplight.io/open-source/prism | Mock server |
| 102 | Spectral | https://stoplight.io/open-source/spectral | Linter |
| 103 | Elements | https://stoplight.io/open-source/elements | API docs |
| 104 | openapi-generator | https://openapi-generator.tech/ | Code gen |
| 105 | openapi-diff | https://github.com/OpenAPITools/openapi-diff | Diff tool |
| 106 | vacuum | https://github.com/daveshanley/vacuum | Go linter |
| 107 | kiota | https://github.com/microsoft/kiota | Microsoft gen |
| 108 | oapi-codegen | https://github.com/oapi-codegen/oapi-codegen | Go gen |
| 109 | openapi-typescript | https://openapi-ts.dev/ | TS gen |
| 110 | orval | https://orval.dev/ | TS gen |
| 111 | Fern | https://buildwithfern.com/ | API framework |
| 112 | Speakeasy | https://www.speakeasyapi.dev/ | API tooling |
| 113 | APIMatic | https://www.apimatic.io/ | API portal |
| 114 | Kong | https://konghq.com/ | API gateway |
| 115 | Tyk | https://tyk.io/ | API gateway |
| 116 | Zuul | https://github.com/Netflix/zuul | Netflix gateway |
| 117 | Ambassador | https://www.getambassador.io/ | K8s gateway |
| 118 | Traefik | https://traefik.io/ | Cloud-native proxy |
| 119 | Caddy | https://caddyserver.com/ | HTTP server |
| 120 | nginx | https://nginx.org/ | HTTP server |

### B.5 Avro & Binary Formats (121-150)

| # | Resource | URL | Description |
|---|----------|-----|-------------|
| 121 | Apache Avro | https://avro.apache.org/ | Avro spec |
| 122 | Avro Spec | https://avro.apache.org/docs/1.11.1/specification/ | Specification |
| 123 | fastavro | https://github.com/fastavro/fastavro | Python Avro |
| 124 | avro-rs | https://github.com/apache/avro-rs | Rust Avro |
| 125 | goavro | https://github.com/linkedin/goavro | Go Avro |
| 126 | avro4s | https://github.com/sksamuel/avro4s | Scala Avro |
| 127 | avro-ts | https://github.com/benhall-7/avro-to-ts | TS gen |
| 128 | Avro Tools | https://avro.apache.org/docs/1.11.1/getting-started-java/ | CLI tools |
| 129 | Schema Registry | https://docs.confluent.io/platform/current/schema-registry/index.html | Confluent SR |
| 130 | Karapace | https://github.com/aiven/karapace | Open SR |
| 131 | Apicurio Registry | https://www.apicur.io/registry/ | Red Hat SR |
| 132 | Apache Parquet | https://parquet.apache.org/ | Columnar storage |
| 133 | Apache Arrow | https://arrow.apache.org/ | In-memory columnar |
| 134 | Apache ORC | https://orc.apache.org/ | Optimized RC |
| 135 | MessagePack | https://msgpack.org/ | Binary JSON |
| 136 | msgpack-javascript | https://github.com/msgpack/msgpack-javascript | JS impl |
| 137 | rmp (Rust) | https://github.com/3Hren/msgpack-rust | Rust impl |
| 138 | msgpack-python | https://github.com/msgpack/msgpack-python | Python impl |
| 139 | CBOR | https://cbor.io/ | Concise Binary |
| 140 | cbor2 (Python) | https://github.com/agronholm/cbor2 | Python CBOR |
| 141 | FlatBuffers | https://flatbuffers.dev/ | Google format |
| 142 | Cap'n Proto | https://capnproto.org/ | Zero-copy |
| 143 | Thrift | https://thrift.apache.org/ | Apache RPC |
| 144 | SBE | https://github.com/real-logic/simple-binary-encoding | Simple Binary |
| 145 | ASN.1 | https://www.itu.int/rec/T-REC-X.680 | Telecom standard |
| 146 | BSON | https://bsonspec.org/ | MongoDB binary |
| 147 | FlexBuffers | https://google.github.io/flatbuffers/flexbuffers.html | Schema-less FB |
| 148 | Protocol | https://github.com/protocolbuffers/upb | Micro protobuf |
| 149 | Zanzibar | https://zanzibar.io/ | Authorization |
| 150 | Cedar | https://www.cedarpolicy.com/ | AWS policy |

### B.6 Schema Registries & Governance (151-180)

| # | Resource | URL | Description |
|---|----------|-----|-------------|
| 151 | Confluent SR | https://docs.confluent.io/platform/current/schema-registry/index.html | Kafka SR |
| 152 | AWS Glue SR | https://docs.aws.amazon.com/glue/latest/dg/schema-registry.html | AWS SR |
| 153 | Azure Schema Registry | https://docs.microsoft.com/en-us/azure/event-hubs/schema-registry | Azure SR |
| 154 | Buf | https://buf.build/ | Buf toolchain |
| 155 | Buf BSR | https://buf.build/docs/bsr/introduction/ | Buf registry |
| 156 | Apollo Studio | https://www.apollographql.com/docs/studio/ | Graph registry |
| 157 | Apicurio | https://www.apicur.io/ | Red Hat registry |
| 158 | Solace Schema Registry | https://docs.solace.com/Schema-Registry/Schema-Registry-Overview.htm | Solace SR |
| 159 | HiveMQ | https://www.hivemq.com/ | MQTT broker |
| 160 | EventBridge | https://aws.amazon.com/eventbridge/ | AWS events |
| 161 | EventCatalog | https://www.eventcatalog.dev/ | Event discovery |
| 162 | AsyncAPI | https://www.asyncapi.com/ | Event APIs |
| 163 | CloudEvents | https://cloudevents.io/ | Event standard |
| 164 | JSON Schema Registry | https://json-schema.org/implementations.html | Registry impls |
| 165 | DataHub | https://datahubproject.io/ | Data catalog |
| 166 | Amundsen | https://www.amundsen.io/ | Data discovery |
| 167 | Apache Atlas | https://atlas.apache.org/ | Metadata |
| 168 | Collibra | https://www.collibra.com/ | Data intelligence |
| 169 | Alation | https://www.alation.com/ | Data catalog |
| 170 | Select Star | https://www.selectstar.com/ | Data discovery |
| 171 | Stemma | https://www.stemma.ai/ | Data catalog |
| 172 | Secoda | https://www.secoda.co/ | Data discovery |
| 173 | Castor | https://www.castordoc.com/ | Data catalog |
| 174 | Metaphor | https://metaphor.io/ | Data context |
| 175 | Acryl Data | https://www.acryldata.io/ | DataHub cloud |
| 176 | Bigeye | https://www.bigeye.com/ | Data observability |
| 177 | Monte Carlo | https://www.montecarlodata.com/ | Data reliability |
| 178 | Soda | https://www.soda.io/ | Data quality |
| 179 | Great Expectations | https://greatexpectations.io/ | Data validation |
| 180 | dbt | https://www.getdbt.com/ | Data transformation |

### B.7 Validation Libraries (181-210)

| # | Resource | URL | Description |
|---|----------|-----|-------------|
| 181 | Zod | https://zod.dev/ | TypeScript validation |
| 182 | Valibot | https://valibot.dev/ | Light validation |
| 183 | Yup | https://github.com/jquense/yup | JS validation |
| 184 | Joi | https://joi.dev/ | JS validation |
| 185 | io-ts | https://github.com/gcanti/io-ts | FP validation |
| 186 | runtypes | https://github.com/pelotom/runtypes | TS validation |
| 187 | superstruct | https://github.com/ianstormtaylor/superstruct | TS validation |
| 188 | ArkType | https://github.com/arktypeio/arktype | Fast validation |
| 189 | Effect Schema | https://effect.website/docs/schema/introduction | FP schema |
| 190 | TypeBox | https://github.com/sinclairzx81/typebox | JSON Schema gen |
| 191 | Pydantic | https://docs.pydantic.dev/ | Python validation |
| 192 | marshmallow | https://marshmallow.readthedocs.io/ | Python ser/de |
| 193 | cerberus | https://docs.python-cerberus.org/ | Python validation |
| 194 | voluptuous | https://github.com/alecthomas/voluptuous | Python valid |
| 195 | go-playground/validator | https://github.com/go-playground/validator | Go validation |
| 196 | ozzo-validation | https://github.com/go-ozzo/ozzo-validation | Go validation |
| 197 | protovalidate-go | https://github.com/bufbuild/protovalidate-go | Go proto valid |
| 198 | validator (Rust) | https://github.com/Keats/validator | Rust derive |
| 199 | jsonschema (Rust) | https://github.com/Stranger6667/jsonschema-rs | Rust JSON Schema |
| 200 | schemars | https://github.com/GREsau/schemars | Rust JSON Schema gen |
| 201 | garde | https://github.com/jprochazk/garde | Rust validation |
| 202 | valid | https://github.com/Keats/valid | Rust validation |
| 203 | Hibernate Validator | https://hibernate.org/validator/ | Java validation |
| 204 | Jakarta Validation | https://beanvalidation.org/ | Java standard |
| 205 | everit-json-schema | https://github.com/everit-org/json-schema | Java JSON Schema |
| 206 | networknt/json-schema | https://github.com/networknt/json-schema | Java validator |
| 207 | FluentValidation | https://docs.fluentvalidation.net/ | C# validation |
| 208 | System.Text.Json | https://docs.microsoft.com/en-us/dotnet/api/system.text.json | .NET JSON |
| 209 | dry-schema | https://dry-rb.org/gems/dry-schema/ | Ruby validation |
| 210 | dry-validation | https://dry-rb.org/gems/dry-validation/ | Ruby validation |

---

## Document Information

**Version:** 2.0.0  
**Status:** Draft  
**Date:** 2026-04-05  
**Owner:** Phenotype Team  
**Lines:** ~2500+  

### Document History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2026-04-04 | Phenotype Team | Initial specification |
| 2.0 | 2026-04-05 | Phenotype Team | Expanded to nanovms format, added SOTA sections, 200+ references |

### Related Documents

- [SOTA.md](./SOTA.md) - State of the Art Research (1,500+ lines)
- [ADR.md](./ADR.md) - Architecture Decision Records (3 ADRs)
- [ARCHITECTURE.md](./ARCHITECTURE.md) - High-level architecture

## Appendix C: SDK Implementation Guide

### C.1 Rust SDK

```rust
use schemaforge::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client
    let client = Client::builder()
        .endpoint("https://schemaforge.phenotype.dev")
        .api_key(std::env::var("SCHEMAFORGE_API_KEY")?)
        .build()?;

    // Publish a schema
    let schema = client
        .publish()
        .namespace("my-team")
        .name("user-event")
        .version("1.0.0")
        .format(Format::JsonSchema)
        .content(include_str!("./user-event.json"))
        .send()
        .await?;

    println!("Published schema: {}", schema.id);

    // Validate data against schema
    let data = serde_json::json!({
        "id": "550e8400-e29b-41d4-a716-446655440000",
        "email": "user@example.com"
    });

    let result = client
        .validate(&schema.id)
        .data(&data)
        .send()
        .await?;

    if result.valid {
        println!("Data is valid!");
    } else {
        for error in result.errors {
            println!("Error at {}: {}", error.path, error.message);
        }
    }

    Ok(())
}
```

### C.2 TypeScript/JavaScript SDK

```typescript
import { SchemaforgeClient } from '@phenotype/schemaforge';

async function main() {
  // Create client
  const client = new SchemaforgeClient({
    endpoint: 'https://schemaforge.phenotype.dev',
    apiKey: process.env.SCHEMAFORGE_API_KEY,
  });

  // Publish a schema
  const schema = await client.schemas.publish({
    namespace: 'my-team',
    name: 'user-event',
    version: '1.0.0',
    format: 'json-schema',
    content: await fs.readFile('./user-event.json', 'utf-8'),
    tags: ['user', 'event'],
  });

  console.log('Published schema:', schema.id);

  // Validate data
  const result = await client.validation.validate({
    schemaId: schema.id,
    data: {
      id: '550e8400-e29b-41d4-a716-446655440000',
      email: 'user@example.com',
    },
  });

  if (result.valid) {
    console.log('Data is valid!');
  } else {
    for (const error of result.errors) {
      console.error(`Error at ${error.path}: ${error.message}`);
    }
  }
}
```

### C.3 Python SDK

```python
import asyncio
from schemaforge import Client

async def main():
    # Create client
    client = Client(
        endpoint="https://schemaforge.phenotype.dev",
        api_key=os.environ["SCHEMAFORGE_API_KEY"]
    )

    # Publish schema
    with open("./user-event.json") as f:
        schema = await client.schemas.publish(
            namespace="my-team",
            name="user-event",
            version="1.0.0",
            format="json-schema",
            content=f.read(),
            tags=["user", "event"]
        )

    print(f"Published schema: {schema.id}")

    # Validate data
    result = await client.validation.validate(
        schema_id=schema.id,
        data={
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "email": "user@example.com"
        }
    )

    if result.valid:
        print("Data is valid!")
    else:
        for error in result.errors:
            print(f"Error at {error.path}: {error.message}")

if __name__ == "__main__":
    asyncio.run(main())
```

## Appendix D: Monitoring & Observability

### D.1 Metrics

| Metric | Type | Description | Labels |
|--------|------|-------------|--------|
| `schemaforge_schemas_total` | Gauge | Total schemas in registry | namespace, format |
| `schemaforge_schema_publish_duration_seconds` | Histogram | Schema publish latency | format, status |
| `schemaforge_validation_duration_seconds` | Histogram | Validation latency | format, result |
| `schemaforge_validation_errors_total` | Counter | Validation errors | error_type |
| `schemaforge_cache_hits_total` | Counter | Cache hits | cache_type |
| `schemaforge_cache_misses_total` | Counter | Cache misses | cache_type |
| `schemaforge_db_connections_active` | Gauge | Active DB connections | - |
| `schemaforge_db_connection_errors_total` | Counter | DB connection errors | error_type |
| `schemaforge_api_requests_total` | Counter | API requests | endpoint, method, status |
| `schemaforge_api_request_duration_seconds` | Histogram | API request latency | endpoint |
| `schemaforge_compatibility_checks_total` | Counter | Compatibility checks | level |
| `schemaforge_migration_plans_generated_total` | Counter | Migration plans generated | status |

### D.2 Tracing

OpenTelemetry traces for key operations:

```rust
use tracing::{info, instrument};

#[instrument(skip(schema), fields(schema_id = %schema.id, format = ?schema.format))]
pub async fn validate_schema(schema: &Schema) -> Result<ValidationReport, ValidationError> {
    info!("Starting schema validation");
    
    let span = tracing::info_span!("parse");
    let ast = async { parser.parse(schema).await }.instrument(span).await?;
    
    let span = tracing::info_span!("validate", ast_nodes = ast.node_count());
    let result = async { validator.validate(&ast).await }.instrument(span).await?;
    
    info!("Schema validation complete", duration_ms = result.stats.duration_ms);
    Ok(result)
}
```

### D.3 Alerting Rules (Prometheus)

```yaml
groups:
  - name: schemaforge
    rules:
      - alert: SchemaforgeHighErrorRate
        expr: |
          (
            sum(rate(schemaforge_api_requests_total{status=~"5.."}[5m]))
            /
            sum(rate(schemaforge_api_requests_total[5m]))
          ) > 0.05
        for: 5m
        labels:
          severity: critical
        annotations:
          summary: "High error rate in Schemaforge"
          description: "Error rate is {{ $value | humanizePercentage }}"

      - alert: SchemaforgeHighLatency
        expr: |
          histogram_quantile(0.95, 
            sum(rate(schemaforge_api_request_duration_seconds_bucket[5m])) by (le)
          ) > 0.5
        for: 10m
        labels:
          severity: warning
        annotations:
          summary: "High latency in Schemaforge"
          description: "p95 latency is {{ $value }}s"

      - alert: SchemaforgeCacheLowHitRate
        expr: |
          (
            sum(rate(schemaforge_cache_hits_total[5m]))
            /
            (
              sum(rate(schemaforge_cache_hits_total[5m]))
              +
              sum(rate(schemaforge_cache_misses_total[5m]))
            )
          ) < 0.8
        for: 15m
        labels:
          severity: warning
        annotations:
          summary: "Low cache hit rate"
          description: "Cache hit rate is {{ $value | humanizePercentage }}"

      - alert: SchemaforgeDBConnectionsHigh
        expr: schemaforge_db_connections_active > 80
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "High database connection usage"
          description: "{{ $value }} active connections"
```

## Appendix E: Troubleshooting Guide

### E.1 Common Issues

| Issue | Cause | Solution |
|-------|-------|----------|
| Schema publish fails with "incompatible" | Breaking change detected | Review breaking changes, use `--force` with caution or migrate |
| Validation timeout | Large schema or deep nesting | Increase timeout, enable streaming validation |
| High memory usage | Many concurrent validations | Reduce `max_concurrent_validations`, enable cache |
| Cache miss rate high | TTL too short or cache size small | Increase TTL, scale cache cluster |
| Database connection errors | Pool exhaustion | Increase pool size, check for connection leaks |
| Webhook delivery failures | Network or endpoint issues | Check webhook URL, verify SSL certificates |

### E.2 Debug Mode

```bash
# Enable debug logging
export SCHEMAFORGE_LOG_LEVEL=debug

# Run with trace-level validation details
schemaforge validate --trace ./schema.json

# Profile performance
schemaforge validate --profile-cpu ./schema.json
```

### E.3 Support Channels

- **GitHub Issues**: Bug reports and feature requests
- **Discord**: Community support
- **Email**: enterprise support for paid customers
- **Documentation**: https://docs.phenotype.dev/schemaforge

## Appendix F: Migration from Other Tools

### F.1 From Confluent Schema Registry

```bash
# Export schemas from Confluent
kafka-schema-registry-client get --all > confluent-schemas.json

# Import to Schemaforge
schemaforge import confluent-schemas.json --format confluent
```

### F.2 From Buf

```bash
# Sync Buf modules
buf export --output ./proto

# Publish to Schemaforge
schemaforge publish ./proto --format protobuf
```

### F.3 From Swagger/OpenAPI

```bash
# Convert Swagger 2.0 to OpenAPI 3.1
swagger2openapi -p ./swagger.json > openapi.json

# Publish to Schemaforge
schemaforge publish ./openapi.json --format openapi
```

---

## Document Information

**Version:** 2.0.0  
**Status:** Draft  
**Date:** 2026-04-05  
**Owner:** Phenotype Team  
**Lines:** 2500+  

### Document History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2026-04-04 | Phenotype Team | Initial specification |
| 2.0 | 2026-04-05 | Phenotype Team | Expanded to nanovms format, added SOTA sections, 200+ references, SDK guides, monitoring, troubleshooting |

### Related Documents

- [SOTA.md](./SOTA.md) - State of the Art Research (1,500+ lines)
- [ADR.md](./ADR.md) - Architecture Decision Records (3 ADRs)
- [ARCHITECTURE.md](./ARCHITECTURE.md) - High-level architecture

---

*End of Schemaforge Technical Specification*

