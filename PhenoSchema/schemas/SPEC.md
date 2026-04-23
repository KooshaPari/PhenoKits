# SPEC.md - schemas

## Schema Management and Validation System for the Phenotype Ecosystem

**Version:** 1.0  
**Status:** Specification  
**Last Updated:** 2026-04-05

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Architecture Overview](#architecture-overview)
3. [System Architecture](#system-architecture)
4. [Schema Definition Layer](#schema-definition-layer)
5. [Schema Registry](#schema-registry)
6. [Code Generation](#code-generation)
7. [Runtime Validation](#runtime-validation)
8. [CI/CD Integration](#cicd-integration)
9. [Multi-Language Support](#multi-language-support)
10. [Schema Evolution](#schema-evolution)
11. [Security Model](#security-model)
12. [Performance Requirements](#performance-requirements)
13. [Error Handling](#error-handling)
14. [Testing Strategy](#testing-strategy)
15. [Deployment](#deployment)
16. [Migration Guide](#migration-guide)
17. [Appendices](#appendices)

---

## Executive Summary

The `schemas` system provides comprehensive schema management, validation, and code generation for the Phenotype ecosystem. It addresses the fundamental challenge of maintaining data consistency across polyglot services written in Rust, TypeScript, Python, and Go.

### Problem Statement

The Phenotype ecosystem faces critical data consistency challenges:

1. **Schema Drift**: Services evolve independently, leading to incompatible data formats
2. **Type Duplication**: Same types defined separately in each language
3. **Validation Inconsistency**: Same data validated differently across services
4. **Documentation Gaps**: Schema definitions not synchronized with implementations
5. **Version Management**: No systematic approach to schema versioning

### Solution Overview

The `schemas` system provides:

1. **Polyglot Schema Authoring**: TypeSpec as primary authoring language
2. **Schema Registry**: Confluent Schema Registry for versioning and compatibility
3. **Automated Code Generation**: Type generation for all target languages
4. **Runtime Validation**: Consistent validation across services
5. **CI/CD Integration**: Automated schema validation and generation

### Key Features

| Feature | Description | Status |
|---------|-------------|--------|
| TypeSpec Authoring | Modern schema definition language | Planned |
| JSON Schema Export | Canonical interchange format | Planned |
| Protobuf Export | Binary serialization | Planned |
| Schema Registry | Versioning and compatibility | Planned |
| TypeScript Generation | Zod schemas with inference | Planned |
| Rust Generation | Serde derives + validation | Planned |
| Python Generation | Pydantic models | Planned |
| Go Generation | Structs with validation tags | Planned |
| CI Integration | Automated validation | Planned |

### Target Users

- **Primary**: Phenotype service developers (Rust, TypeScript, Python, Go)
- **Secondary**: API consumers and client library users
- **Tertiary**: DevOps engineers managing schema deployments

---

## Architecture Overview

### System Context

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         Phenotype Ecosystem                                   │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌────────────┐        │
│  │  heliosCLI  │  │  pheno-cli  │  │  platforms  │  │  services  │        │
│  │             │  │             │  │             │  │            │        │
│  │  Rust       │  │  Rust       │  │  Rust/Go    │  │  Polyglot  │        │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘  └──────┬─────┘        │
│         │                │                │                │              │
│         └────────────────┼────────────────┼────────────────┘              │
│                          │                │                               │
│  ┌───────────────────────┴────────────────┴───────────────────┐         │
│  │                      schemas System                          │         │
│  │                                                            │         │
│  │  ┌──────────────────────────────────────────────────────┐  │         │
│  │  │                 Authoring Layer                      │  │         │
│  │  │  • TypeSpec (.tsp)                                   │  │         │
│  │  │  • CUE (.cue)                                        │  │         │
│  │  │  • JSON Schema (.json)                               │  │         │
│  │  └──────────────────────────────────────────────────────┘  │         │
│  │                                                            │         │
│  │  ┌──────────────────────────────────────────────────────┐  │         │
│  │  │                 Compilation Layer                    │  │         │
│  │  │  • TypeSpec Compiler                                 │  │         │
│  │  │  • JSON Schema Generator                             │  │         │
│  │  │  • Protobuf Generator                                  │  │         │
│  │  └──────────────────────────────────────────────────────┘  │         │
│  │                                                            │         │
│  │  ┌──────────────────────────────────────────────────────┐  │         │
│  │  │                 Registry Layer                       │  │         │
│  │  │  • Schema Registry (Confluent/Apicurio)              │  │         │
│  │  │  • Version Management                                │  │         │
│  │  │  • Compatibility Checking                            │  │         │
│  │  └──────────────────────────────────────────────────────┘  │         │
│  │                                                            │         │
│  │  ┌──────────────────────────────────────────────────────┐  │         │
│  │  │                 Generation Layer                     │  │         │
│  │  │  • TypeScript (Zod)                                  │  │         │
│  │  │  • Rust (Serde)                                      │  │         │
│  │  │  • Python (Pydantic)                                 │  │         │
│  │  │  • Go (Structs + Tags)                               │  │         │
│  │  └──────────────────────────────────────────────────────┘  │         │
│  │                                                            │         │
│  └────────────────────────────────────────────────────────────┘         │
│                                                                          │
│  ┌──────────────────────────────────────────────────────────────────┐   │
│  │                     Integration Points                            │   │
│  │                                                                    │   │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐              │   │
│  │  │  CI/CD      │  │  IDEs       │  │  Docs       │              │   │
│  │  │  • GitHub   │  │  • VS Code  │  │  • OpenAPI  │              │   │
│  │  │  Actions    │  │  • JetBrains│  │  • Generated│              │   │
│  │  └─────────────┘  └─────────────┘  └─────────────┘              │   │
│  │                                                                    │   │
│  └──────────────────────────────────────────────────────────────────┘   │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

### Design Principles

1. **Single Source of Truth**: One schema definition generates all artifacts
2. **Schema-First**: Define schemas before implementation
3. **Validation at Boundaries**: Validate all incoming/outgoing data
4. **Fail Fast**: Detect schema violations early in development
5. **Backward Compatibility**: Schema changes don't break existing consumers
6. **Developer Experience**: Excellent tooling, IDE support, error messages
7. **Polyglot Native**: First-class support for all target languages
8. **Automation**: CI/CD handles validation and generation automatically

---

## System Architecture

### Layer Diagram

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              CLI Layer                                       │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌────────────┐         │
│  │   schemas   │  │   schemas   │  │   schemas   │  │   schemas  │         │
│  │   validate  │  │   generate  │  │   registry  │  │   sync     │         │
│  │             │  │             │  │             │  │            │         │
│  │  check      │  │  typescript│  │  list       │  │  push      │         │
│  │  lint       │  │  rust       │  │  register   │  │  pull      │         │
│  │  diff       │  │  python     │  │  versions   │  │  status    │         │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘  └──────┬─────┘         │
│         │                │                │                │               │
│         └────────────────┼────────────────┼────────────────┘               │
│                          │                │                                  │
├──────────────────────────┼────────────────┼──────────────────────────────────┤
│                          │                │                                  │
│  ┌───────────────────────┴────────────────┴───────────────────┐            │
│  │                   API Layer                               │            │
│  │                                                           │            │
│  │  • TypeSpec compiler integration                          │            │
│  │  • Schema registry client                                   │            │
│  │  • Code generation orchestration                            │            │
│  │  • Validation engine                                        │            │
│  └───────────────────────────────────────────────────────────┘            │
│                                                                            │
├────────────────────────────────────────────────────────────────────────────┤
│                          Service Layer                                     │
│  ┌────────────────┐  ┌────────────────┐  ┌────────────────┐                │
│  │  Schema        │  │  Code          │  │  Registry      │                │
│  │  Compiler      │  │  Generator     │  │  Client        │                │
│  │                │  │                │  │                │                │
│  │  • parse()     │  │  • typescript()│  │  • register()  │                │
│  │  • validate()  │  │  • rust()      │  │  • lookup()    │                │
│  │  • compile()   │  │  • python()    │  │  • check()     │                │
│  │  • export()    │  │  • go()        │  │  • versions()  │                │
│  └────────┬───────┘  └────────┬───────┘  └────────┬───────┘                │
│           │                   │                   │                          │
│           └───────────────────┼───────────────────┘                          │
│                               │                                              │
├───────────────────────────────┼──────────────────────────────────────────────┤
│                               │                                              │
│  ┌────────────────────────────┴────────────────────────────┐                  │
│  │                   Core Layer                           │                  │
│  │                                                        │                  │
│  │  ┌────────────┐  ┌────────────┐  ┌────────────┐      │                  │
│  │  │ TypeSpec   │  │ JSON       │  │ Protobuf   │      │                  │
│  │  │ Parser     │  │ Schema     │  │ Generator  │      │                  │
│  │  └────────────┘  └────────────┘  └────────────┘      │                  │
│  │                                                        │                  │
│  │  ┌────────────┐  ┌────────────┐  ┌────────────┐      │                  │
│  │  │ Validation │  │ Diff       │  │ Migration  │      │                  │
│  │  │ Engine     │  │ Engine     │  │ Engine     │      │                  │
│  │  └────────────┘  └────────────┘  └────────────┘      │                  │
│  └────────────────────────────────────────────────────────┘                  │
│                                                                                │
├────────────────────────────────────────────────────────────────────────────────┤
│                          Data Layer                                            │
│  ┌────────────────┐  ┌────────────────┐  ┌────────────────┐                  │
│  │  Git           │  │  Schema        │  │  Generated     │                  │
│  │  Repository    │  │  Registry      │  │  Artifacts     │                  │
│  │                │  │                │  │                │                  │
│  │  *.tsp         │  │  Confluent     │  │  *.ts          │                  │
│  │  *.cue         │  │  Apicurio      │  │  *.rs          │                  │
│  │  *.json        │  │                │  │  *.py          │                  │
│  └────────────────┘  └────────────────┘  └────────────────┘                  │
│                                                                                │
└────────────────────────────────────────────────────────────────────────────────┘
```

### Component Interaction Flow

```
Developer: Edit TypeSpec schema

     ┌─────────┐
     │   IDE   │
     └────┬────┘
          │ Type checking, autocomplete
          ▼
     ┌─────────┐
     │  Git    │
     └────┬────┘
          │ Commit & Push
          ▼
     ┌─────────┐
     │   CI    │
     └────┬────┘
          │ Trigger pipeline
          ▼
     ┌─────────────┐
     │   Service   │
     │  Compiler   │
     └──────┬──────┘
            │ Compile TypeSpec
            ▼
     ┌────────────┐
     │   Check    │
     │  Registry  │
     └──────┬─────┘
            │ Check compatibility
            ▼
     ┌────────────┐
     │  Generate  │
     │   Code     │
     └──────┬─────┘
            │ Generate TypeScript, Rust, Python, Go
            ▼
     ┌────────────┐
     │   Verify   │
     └──────┬─────┘
            │ Compile generated code
            ▼
     ┌────────────┐
     │   Commit   │
     └────────────┘
            │
            ├──► Registry: Register new schema version
            ├──► Git: Commit generated code
            └──► Artifacts: Publish packages
```

---

## Schema Definition Layer

### TypeSpec as Primary Authoring Language

TypeSpec is the primary language for defining schemas:

```typespec
import "@typespec/http";
import "@typespec/rest";
import "@typespec/openapi3";
import "@typespec/json-schema";
import "@typespec/protobuf";

using TypeSpec.Http;
using TypeSpec.Rest;

@service({
  title: "Phenotype User Service",
  version: "1.0.0"
})
namespace Phenotype.Users;

@jsonSchema
namespace Schemas {
  enum Role {
    Admin,
    User,
    Guest
  }

  model Address {
    @maxLength(200)
    street: string;
    
    @maxLength(100)
    city: string;
    
    @maxLength(20)
    postalCode: string;
    
    @maxLength(100)
    country: string;
  }

  model User {
    @key
    @format("uuid")
    id: string;
    
    @format("email")
    email: string;
    
    @minLength(1)
    @maxLength(100)
    name?: string;
    
    role: Role = Role.User;
    
    metadata: Record<string>;
    
    @doc("Creation timestamp (ISO 8601)")
    createdAt: utcDateTime;
    
    @maxItems(10)
    @uniqueItems
    tags: string[];
    
    billingAddress?: Address;
    shippingAddress?: Address;
    
    @visibility("read")
    @doc("Computed field - total orders")
    totalOrders?: int32;
  }

  model CreateUserRequest {
    @format("email")
    email: string;
    
    @minLength(1)
    @maxLength(100)
    name?: string;
    
    role?: Role = Role.User;
  }

  model UpdateUserRequest {
    @minLength(1)
    @maxLength(100)
    name?: string;
    
    role?: Role;
    
    metadata?: Record<string>;
  }

  @error
  model ValidationError {
    code: "VALIDATION_ERROR";
    message: string;
    fieldErrors: FieldError[];
  }

  model FieldError {
    field: string;
    message: string;
    value?: unknown;
  }

  @error
  model NotFoundError {
    @statusCode _: 404;
    code: "NOT_FOUND";
    message: string;
    resource: string;
  }
}

@route("/api/v1/users")
@tag("Users")
interface UserService {
  @doc("List all users")
  @get
  list(
    @query filter?: string,
    @query page?: int32 = 1,
    @query pageSize?: int32 = 20
  ): PaginatedResponse<User>;

  @doc("Get a user by ID")
  @get
  get(@path id: string): User | NotFoundError;

  @doc("Create a new user")
  @post
  create(@body body: CreateUserRequest): User | ValidationError;

  @doc("Update an existing user")
  @patch
  update(
    @path id: string,
    @body body: UpdateUserRequest
  ): User | NotFoundError | ValidationError;

  @doc("Delete a user")
  @delete
  delete(@path id: string): void | NotFoundError;
}

model PaginatedResponse<T> {
  items: T[];
  page: int32;
  pageSize: int32;
  totalCount: int32;
  hasNextPage: boolean;
  nextPageToken?: string;
}
```

### CUE for Configuration Schemas

CUE is used for configuration and policy schemas:

```cue
package config

// Definition schemas
#Environment: "development" | "staging" | "production"

#LogLevel: "debug" | "info" | "warn" | "error"

#DatabaseConfig: {
  host: string & =~"^[a-zA-Z0-9.-]+$")
  port: int & >=1 & <=65535
  database: string & =~"^[a-zA-Z_][a-zA-Z0-9_]*$")
  username: string
  password: string & strings.MinRunes(8)
  poolSize: int & >=1 & <=100 | *10
  timeout: string & =~"^[0-9]+[smhd]$" | *"30s"
  ssl: {
    enabled: bool | *true
    caCert?: string
    clientCert?: string
    clientKey?: string
  }
}

#ServerConfig: {
  host: string | *"0.0.0.0"
  port: int & >=1 & <=65535 | *8080
  tls: {
    enabled: bool | *false
    certFile?: string
    keyFile?: string
  }
  cors: {
    enabled: bool | *true
    origins: [...string] | ["*"]
    methods: [...string] | ["GET", "POST", "PUT", "DELETE", "PATCH"]
  }
  rateLimit: {
    enabled: bool | *true
    requestsPerMinute: int & >=1 | *60
  }
}

// Top-level configuration
#PhenotypeConfig: {
  environment: #Environment
  logLevel: #LogLevel | *"info"
  
  server: #ServerConfig
  
  database: #DatabaseConfig
  
  cache?: {
    enabled: bool | *true
    backend: "memory" | "redis"
    redis?: {
      url: string
      ttl: string | *"5m"
    }
  }
  
  features: {
    [string]: bool
  }
}

// Validation example
config: #PhenotypeConfig & {
  environment: "production"
  logLevel: "info"
  
  server: {
    port: 8443
    tls: {
      enabled: true
      certFile: "/etc/certs/server.crt"
      keyFile: "/etc/certs/server.key"
    }
  }
  
  database: {
    host: "db.phenotype.internal"
    port: 5432
    database: "phenotype_prod"
    username: "phenotype_app"
    password: "${DB_PASSWORD}"  // Template for env substitution
  }
  
  features: {
    "new-dashboard": true
    "beta-api": false
  }
}
```

### Raw JSON Schema

Direct JSON Schema for simple cases or external integration:

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://phenotype.dev/schemas/user.json",
  "title": "User",
  "description": "A user in the Phenotype system",
  "type": "object",
  "required": ["id", "email", "createdAt"],
  "properties": {
    "id": {
      "type": "string",
      "format": "uuid",
      "description": "Unique identifier"
    },
    "email": {
      "type": "string",
      "format": "email"
    },
    "name": {
      "type": "string",
      "minLength": 1,
      "maxLength": 100
    },
    "role": {
      "type": "string",
      "enum": ["admin", "user", "guest"],
      "default": "user"
    },
    "metadata": {
      "type": "object",
      "additionalProperties": { "type": "string" },
      "default": {}
    },
    "createdAt": {
      "type": "string",
      "format": "date-time"
    },
    "tags": {
      "type": "array",
      "items": { "type": "string" },
      "uniqueItems": true,
      "maxItems": 10,
      "default": []
    },
    "address": {
      "type": "object",
      "properties": {
        "street": { "type": "string" },
        "city": { "type": "string" },
        "postalCode": { "type": "string" },
        "country": { "type": "string" }
      },
      "required": ["street", "city", "postalCode", "country"]
    }
  },
  "additionalProperties": false
}
```

---

## Schema Registry

### Registry Interface

```rust
/// Schema registry client trait
trait SchemaRegistry {
  /// Register a new schema version
  async fn register(
    &self,
    subject: &str,
    schema: &Schema,
    metadata: SchemaMetadata,
  ) -> Result<SchemaVersion, RegistryError>;

  /// Look up a schema by subject and version
  async fn lookup(
    &self,
    subject: &str,
    version: VersionSpec,
  ) -> Result<RegisteredSchema, RegistryError>;

  /// List all versions of a subject
  async fn list_versions(
    &self,
    subject: &str,
  ) -> Result<Vec<SchemaVersion>, RegistryError>;

  /// Check compatibility with existing schemas
  async fn check_compatibility(
    &self,
    subject: &str,
    schema: &Schema,
    level: CompatibilityLevel,
  ) -> Result<CompatibilityResult, RegistryError>;

  /// Get global or subject-specific config
  async fn get_config(
    &self,
    subject: Option<&str>,
  ) -> Result<RegistryConfig, RegistryError>;

  /// Update global or subject-specific config
  async fn update_config(
    &self,
    subject: Option<&str>,
    config: RegistryConfig,
  ) -> Result<(), RegistryError>;

  /// Delete a specific schema version
  async fn delete_version(
    &self,
    subject: &str,
    version: SchemaVersion,
  ) -> Result<(), RegistryError>;

  /// Search schemas by criteria
  async fn search(
    &self,
    query: &SearchQuery,
  ) -> Result<SearchResults, RegistryError>;
}

struct Schema {
  schema_type: SchemaType,
  content: String,
  references: Vec<SchemaReference>,
}

enum SchemaType {
  JsonSchema,
  Protobuf,
  Avro,
}

enum VersionSpec {
  Latest,
  Specific(u32),
  ById(i32),
}

enum CompatibilityLevel {
  None,
  Backward,
  Forward,
  Full,
  Transitive,
}
```

### Subject Naming

```
Format: {organization}.{domain}.{entity}-{format}

Examples:
- phenotype.users.User-jsonschema
- phenotype.users.User-protobuf
- phenotype.events.UserCreated-avro
- phenotype.config.Server-cue
- phenotype.common.Address-jsonschema

Rules:
1. Organization: lowercase, no spaces
2. Domain: lowercase, plural (users, events, config)
3. Entity: PascalCase (User, UserCreated, ServerConfig)
4. Format: lowercase (jsonschema, protobuf, avro, cue)
5. Separators: dots between components, hyphen before format
```

### Versioning Strategy

```rust
/// Semantic versioning for schemas
struct SchemaVersion {
  major: u32,
  minor: u32,
  patch: u32,
}

impl SchemaVersion {
  /// Parse from string "MAJOR.MINOR.PATCH"
  fn parse(s: &str) -> Result<Self, VersionParseError> {
    // ...
  }

  /// Determine compatibility impact of changes
  fn compatibility_impact(
    old: &Self,
    changes: &[SchemaChange],
  ) -> CompatibilityImpact {
    for change in changes {
      match change {
        SchemaChange::AddRequiredField =>
          return CompatibilityImpact::Major,
        SchemaChange::RemoveField =>
          return CompatibilityImpact::Major,
        SchemaChange::AddOptionalField =>
          continue, // Minor
        SchemaChange::TightenConstraint =>
          return CompatibilityImpact::Major,
        SchemaChange::LoosenConstraint =>
          continue, // Minor
        SchemaChange::DocumentationOnly =>
          continue, // Patch
        _ => {}
      }
    }
    CompatibilityImpact::Minor
  }
}
```

### Compatibility Rules

| Change Type | Backward | Forward | Full | Version |
|-------------|----------|---------|------|---------|
| Add optional field | Yes | Yes | Yes | Minor |
| Add required field with default | Yes | No | No | Major |
| Add required field without default | No | Yes | No | Major |
| Remove field (not used) | Yes | No | No | Major |
| Make field optional | Yes | No | No | Major |
| Make field required | No | Yes | No | Major |
| Widen type (int -> number) | Yes | No | No | Major |
| Narrow type (number -> int) | No | Yes | No | Major |
| Loosen constraint | Yes | No | No | Major |
| Tighten constraint | No | Yes | No | Major |
| Rename field | No | No | No | Major |
| Change field order | Yes | Yes | Yes | Patch |
| Documentation only | Yes | Yes | Yes | Patch |

---

## Code Generation

### Generation Pipeline

```rust
/// Code generation orchestrator
struct CodeGenerator {
  registry: Arc<dyn SchemaRegistry>,
  templates: TemplateEngine,
  cache: GenerationCache,
}

impl CodeGenerator {
  /// Generate code for all target languages
  async fn generate_all(
    &self,
    subject: &str,
    version: VersionSpec,
  ) -> Result<GeneratedArtifacts, GenerationError> {
    let schema = self.registry.lookup(subject, version).await?;

    Ok(GeneratedArtifacts {
      typescript: self.generate_typescript(&schema).await?,
      rust: self.generate_rust(&schema).await?,
      python: self.generate_python(&schema).await?,
      go: self.generate_go(&schema).await?,
      protobuf: self.generate_protobuf(&schema).await?,
      openapi: self.generate_openapi(&schema).await?,
    })
  }

  /// Generate TypeScript with Zod
  async fn generate_typescript(
    &self,
    schema: &RegisteredSchema,
  ) -> Result<String, GenerationError> {
    let template = self.templates.load("typescript/zod.ts.j2")?;

    let context = json!({
      "schema": schema,
      "imports": self.collect_typescript_imports(schema),
      "types": self.generate_zod_types(schema),
      "inference": self.generate_type_inference(schema),
    });

    self.templates.render(&template, &context)
  }

  /// Generate Rust with Serde
  async fn generate_rust(
    &self,
    schema: &RegisteredSchema,
  ) -> Result<String, GenerationError> {
    let template = self.templates.load("rust/serde.rs.j2")?;

    let context = json!({
      "schema": schema,
      "imports": self.collect_rust_imports(schema),
      "structs": self.generate_rust_structs(schema),
      "derives": ["Debug", "Clone", "Serialize", "Deserialize", "JsonSchema"],
    });

    self.templates.render(&template, &context)
  }

  /// Generate Python with Pydantic
  async fn generate_python(
    &self,
    schema: &RegisteredSchema,
  ) -> Result<String, GenerationError> {
    let template = self.templates.load("python/pydantic.py.j2")?;

    let context = json!({
      "schema": schema,
      "imports": self.collect_python_imports(schema),
      "models": self.generate_pydantic_models(schema),
      "config": self.generate_model_config(schema),
    });

    self.templates.render(&template, &context)
  }

  /// Generate Go with validation tags
  async fn generate_go(
    &self,
    schema: &RegisteredSchema,
  ) -> Result<String, GenerationError> {
    let template = self.templates.load("go/structs.go.j2")?;

    let context = json!({
      "schema": schema,
      "package": self.go_package_name(schema),
      "imports": self.collect_go_imports(schema),
      "structs": self.generate_go_structs(schema),
      "validators": self.generate_go_validators(schema),
    });

    self.templates.render(&template, &context)
  }
}
```

### TypeScript Output

```typescript
// Generated from TypeSpec - DO NOT EDIT
// Source: phenotype.users.User
// Version: 1.2.0
// Generated: 2026-04-05T12:00:00Z

import { z } from 'zod';

/**
 * User role enumeration
 */
export const RoleSchema = z.enum(['admin', 'user', 'guest']);
export type Role = z.infer<typeof RoleSchema>;

/**
 * Address structure
 */
export const AddressSchema = z.object({
  street: z.string().max(200),
  city: z.string().max(100),
  postalCode: z.string().max(20),
  country: z.string().max(100),
});
export type Address = z.infer<typeof AddressSchema>;

/**
 * A user in the Phenotype system
 */
export const UserSchema = z.object({
  /** Unique identifier */
  id: z.string().uuid(),

  /** Email address */
  email: z.string().email(),

  /** Display name */
  name: z.string().min(1).max(100).optional(),

  /** User role */
  role: RoleSchema.default('user'),

  /** Arbitrary string metadata */
  metadata: z.record(z.string()).default({}),

  /** Creation timestamp (ISO 8601) */
  createdAt: z.string().datetime(),

  /** User tags */
  tags: z.array(z.string()).max(10).refine(
    (items) => new Set(items).size === items.length,
    { message: 'Tags must be unique' }
  ).default([]),

  billingAddress: AddressSchema.optional(),
  shippingAddress: AddressSchema.optional(),
});
export type User = z.infer<typeof UserSchema>;

/**
 * Create user request
 */
export const CreateUserRequestSchema = z.object({
  email: z.string().email(),
  name: z.string().min(1).max(100).optional(),
  role: RoleSchema.default('user'),
});
export type CreateUserRequest = z.infer<typeof CreateUserRequestSchema>;

/**
 * Update user request
 */
export const UpdateUserRequestSchema = z.object({
  name: z.string().min(1).max(100).optional(),
  role: RoleSchema.optional(),
  metadata: z.record(z.string()).optional(),
}).partial();
export type UpdateUserRequest = z.infer<typeof UpdateUserRequestSchema>;

/**
 * Field error structure
 */
export const FieldErrorSchema = z.object({
  field: z.string(),
  message: z.string(),
  value: z.unknown().optional(),
});
export type FieldError = z.infer<typeof FieldErrorSchema>;

/**
 * Validation error
 */
export const ValidationErrorSchema = z.object({
  code: z.literal('VALIDATION_ERROR'),
  message: z.string(),
  fieldErrors: z.array(FieldErrorSchema),
});
export type ValidationError = z.infer<typeof ValidationErrorSchema>;

/**
 * Not found error
 */
export const NotFoundErrorSchema = z.object({
  code: z.literal('NOT_FOUND'),
  message: z.string(),
  resource: z.string(),
});
export type NotFoundError = z.infer<typeof NotFoundErrorSchema>;
```

### Rust Output

```rust
// Generated from TypeSpec - DO NOT EDIT
// Source: phenotype.users.User
// Version: 1.2.0
// Generated: 2026-04-05T12:00:00Z

use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use validator::{Validate, ValidationError};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// User role enumeration
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Admin,
    User,
    Guest,
}

impl Default for Role {
    fn default() -> Self {
        Role::User
    }
}

/// Address structure
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Validate)]
pub struct Address {
    #[validate(length(max = 200))]
    pub street: String,

    #[validate(length(max = 100))]
    pub city: String,

    #[serde(rename = "postalCode")]
    #[validate(length(max = 20))]
    pub postal_code: String,

    #[validate(length(max = 100))]
    pub country: String,
}

/// A user in the Phenotype system
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Validate)]
pub struct User {
    /// Unique identifier
    #[schemars(with = "String")]
    pub id: Uuid,

    /// Email address
    #[validate(email)]
    pub email: String,

    /// Display name
    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,

    /// User role
    #[serde(default)]
    pub role: Role,

    /// Arbitrary string metadata
    #[serde(default)]
    pub metadata: HashMap<String, String>,

    /// Creation timestamp
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,

    /// User tags
    #[validate(length(max = 10))]
    #[serde(default)]
    pub tags: Vec<String>,

    #[serde(rename = "billingAddress")]
    pub billing_address: Option<Address>,

    #[serde(rename = "shippingAddress")]
    pub shipping_address: Option<Address>,
}

impl User {
    /// Validate with custom logic
    pub fn validate_custom(&self) -> Result<(), ValidationError> {
        // Check unique tags
        let unique_count = self.tags.iter().collect::<std::collections::HashSet<_>>().len();
        if unique_count != self.tags.len() {
            return Err(ValidationError::new("tags must be unique"));
        }
        Ok(())
    }
}

/// Create user request
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Validate)]
pub struct CreateUserRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,

    #[serde(default)]
    pub role: Role,
}

/// Update user request
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Validate, Default)]
pub struct UpdateUserRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,

    pub role: Option<Role>,

    pub metadata: Option<HashMap<String, String>>,
}

/// Field error structure
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FieldError {
    pub field: String,
    pub message: String,
    pub value: Option<serde_json::Value>,
}

/// Validation error
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ValidationError {
    pub code: String,
    pub message: String,
    pub field_errors: Vec<FieldError>,
}

/// Not found error
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct NotFoundError {
    pub code: String,
    pub message: String,
    pub resource: String,
}
```

### Python Output

```python
# Generated from TypeSpec - DO NOT EDIT
# Source: phenotype.users.User
# Version: 1.2.0
# Generated: 2026-04-05T12:00:00Z

from __future__ import annotations

from datetime import datetime
from typing import Optional, Dict, List
from uuid import UUID
from enum import Enum

from pydantic import BaseModel, EmailStr, Field, ConfigDict, field_validator


class Role(str, Enum):
    """User role enumeration"""
    ADMIN = "admin"
    USER = "user"
    GUEST = "guest"


class Address(BaseModel):
    """Address structure"""
    model_config = ConfigDict(
        strict=True,
        populate_by_name=True,
    )

    street: str = Field(..., max_length=200)
    city: str = Field(..., max_length=100)
    postal_code: str = Field(..., alias="postalCode", max_length=20)
    country: str = Field(..., max_length=100)


class User(BaseModel):
    """A user in the Phenotype system"""
    model_config = ConfigDict(
        strict=True,
        populate_by_name=True,
    )

    id: UUID = Field(..., description="Unique identifier")
    email: EmailStr = Field(..., description="Email address")
    name: Optional[str] = Field(None, min_length=1, max_length=100, description="Display name")
    role: Role = Field(default=Role.USER, description="User role")
    metadata: Dict[str, str] = Field(default_factory=dict, description="Arbitrary string metadata")
    created_at: datetime = Field(..., alias="createdAt", description="Creation timestamp (ISO 8601)")
    tags: List[str] = Field(default_factory=list, max_length=10, description="User tags")
    billing_address: Optional[Address] = Field(None, alias="billingAddress")
    shipping_address: Optional[Address] = Field(None, alias="shippingAddress")

    @field_validator('tags')
    @classmethod
    def validate_unique_tags(cls, v: List[str]) -> List[str]:
        if len(v) != len(set(v)):
            raise ValueError('Tags must be unique')
        return v


class CreateUserRequest(BaseModel):
    """Create user request"""
    model_config = ConfigDict(
        strict=True,
        populate_by_name=True,
    )

    email: EmailStr
    name: Optional[str] = Field(None, min_length=1, max_length=100)
    role: Role = Field(default=Role.USER)


class UpdateUserRequest(BaseModel):
    """Update user request"""
    model_config = ConfigDict(
        strict=True,
        populate_by_name=True,
    )

    name: Optional[str] = Field(None, min_length=1, max_length=100)
    role: Optional[Role] = None
    metadata: Optional[Dict[str, str]] = None


class FieldError(BaseModel):
    """Field error structure"""
    field: str
    message: str
    value: Optional[object] = None


class ValidationErrorResponse(BaseModel):
    """Validation error"""
    code: str = "VALIDATION_ERROR"
    message: str
    field_errors: List[FieldError] = Field(..., alias="fieldErrors")


class NotFoundError(BaseModel):
    """Not found error"""
    code: str = "NOT_FOUND"
    message: str
    resource: str
```

### Go Output

```go
// Generated from TypeSpec - DO NOT EDIT
// Source: phenotype.users.User
// Version: 1.2.0
// Generated: 2026-04-05T12:00:00Z

package users

import (
	"time"

	"github.com/go-playground/validator/v10"
	"github.com/google/uuid"
)

// Role represents user role enumeration
type Role string

const (
	RoleAdmin Role = "admin"
	RoleUser  Role = "user"
	RoleGuest Role = "guest"
)

// Address represents address structure
type Address struct {
	Street     string `json:"street" validate:"required,max=200"`
	City       string `json:"city" validate:"required,max=100"`
	PostalCode string `json:"postalCode" validate:"required,max=20"`
	Country    string `json:"country" validate:"required,max=100"`
}

// User represents a user in the Phenotype system
type User struct {
	ID               uuid.UUID         `json:"id" validate:"required,uuid"`
	Email            string            `json:"email" validate:"required,email"`
	Name             string            `json:"name,omitempty" validate:"omitempty,min=1,max=100"`
	Role             Role              `json:"role" validate:"required,oneof=admin user guest"`
	Metadata         map[string]string `json:"metadata"`
	CreatedAt        time.Time         `json:"createdAt" validate:"required"`
	Tags             []string          `json:"tags" validate:"max=10,dive,required"`
	BillingAddress   *Address          `json:"billingAddress,omitempty"`
	ShippingAddress  *Address          `json:"shippingAddress,omitempty"`
}

// Validate performs custom validation
func (u User) Validate() error {
	validate := validator.New()

	if err := validate.Struct(u); err != nil {
		return err
	}

	// Check unique tags
	tagSet := make(map[string]struct{})
	for _, tag := range u.Tags {
		if _, exists := tagSet[tag]; exists {
			return validator.ValidationErrors{
				{
					Field: "Tags",
					Tag:   "unique",
					Param: "",
				},
			}
		}
		tagSet[tag] = struct{}{}
	}

	return nil
}

// CreateUserRequest represents create user request
type CreateUserRequest struct {
	Email string `json:"email" validate:"required,email"`
	Name  string `json:"name,omitempty" validate:"omitempty,min=1,max=100"`
	Role  Role   `json:"role" validate:"omitempty,oneof=admin user guest"`
}

// UpdateUserRequest represents update user request
type UpdateUserRequest struct {
	Name     string            `json:"name,omitempty" validate:"omitempty,min=1,max=100"`
	Role     Role              `json:"role,omitempty" validate:"omitempty,oneof=admin user guest"`
	Metadata map[string]string `json:"metadata,omitempty"`
}

// FieldError represents field error structure
type FieldError struct {
	Field   string      `json:"field"`
	Message string      `json:"message"`
	Value   interface{} `json:"value,omitempty"`
}

// ValidationError represents validation error
type ValidationError struct {
	Code        string       `json:"code"`
	Message     string       `json:"message"`
	FieldErrors []FieldError `json:"fieldErrors"`
}

// NotFoundError represents not found error
type NotFoundError struct {
	Code     string `json:"code"`
	Message  string `json:"message"`
	Resource string `json:"resource"`
}
```

---

## Runtime Validation

### Validation Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                     Runtime Validation Flow                           │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  Input Data                                                          │
│     │                                                                │
│     ▼                                                                │
│  ┌─────────────────┐                                                │
│  │ Parse (JSON/etc)│                                                │
│  └────────┬────────┘                                                │
│           │                                                          │
│           ▼                                                          │
│  ┌─────────────────┐                                                │
│  │ Type Validation │  Check types match schema                       │
│  │ (primitive)     │  String, number, boolean, etc                  │
│  └────────┬────────┘                                                │
│           │                                                          │
│           ▼                                                          │
│  ┌─────────────────┐                                                │
│  │ Constraint Check│  min/max, pattern, enum, etc                   │
│  │ (business rules)│                                                 │
│  └────────┬────────┘                                                │
│           │                                                          │
│           ▼                                                          │
│  ┌─────────────────┐                                                │
│  │ Cross-Field     │  Relationships between fields                    │
│  │ Validation      │  e.g., end > start                             │
│  └────────┬────────┘                                                │
│           │                                                          │
│           ▼                                                          │
│  ┌─────────────────┐                                                │
│  │ Custom Validators│  Domain-specific rules                        │
│  │                 │  e.g., email uniqueness                        │
│  └────────┬────────┘                                                │
│           │                                                          │
│           ▼                                                          │
│    Validated Data                                                    │
│    OR ValidationError                                                │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### Validation by Language

**TypeScript (Zod):**
```typescript
import { UserSchema, ValidationError } from './schemas';

class UserService {
  async createUser(input: unknown): Promise<User> {
    // Full validation
    const result = UserSchema.safeParse(input);

    if (!result.success) {
      const validationError: ValidationError = {
        code: 'VALIDATION_ERROR',
        message: 'Input validation failed',
        fieldErrors: result.error.errors.map(err => ({
          field: err.path.join('.'),
          message: err.message,
          value: err.code,
        })),
      };
      throw new ValidationException(validationError);
    }

    return result.data;
  }

  async partialUpdate(input: unknown): Promise<Partial<User>> {
    // Partial validation (for PATCH)
    const PartialSchema = UserSchema.partial();
    const result = PartialSchema.safeParse(input);
    // ...
  }
}
```

**Rust (Pydantic-style with validator):**
```rust
use validator::Validate;

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<User>, AppError> {
    // Validate
    payload.validate().map_err(|e| {
        AppError::Validation(ValidationError {
            code: "VALIDATION_ERROR".to_string(),
            message: "Input validation failed".to_string(),
            field_errors: e.field_errors().into_iter().map(|(field, errors)| {
                FieldError {
                    field: field.to_string(),
                    message: errors.iter().map(|e| e.message.clone().unwrap_or_default()).collect::<Vec<_>>().join(", "),
                    value: None,
                }
            }).collect(),
        })
    })?;

    // Custom validation
    if payload.email.ends_with("@tempmail.com") {
        return Err(AppError::Validation(ValidationError {
            code: "INVALID_EMAIL".to_string(),
            message: "Temporary email addresses not allowed".to_string(),
            field_errors: vec![FieldError {
                field: "email".to_string(),
                message: "Temporary email addresses not allowed".to_string(),
                value: Some(payload.email.into()),
            }],
        }));
    }

    // Create user
    let user = state.user_repo.create(payload).await?;
    Ok(Json(user))
}
```

**Python (Pydantic):**
```python
from fastapi import FastAPI, HTTPException
from pydantic import ValidationError
from schemas import User, CreateUserRequest, ValidationErrorResponse

app = FastAPI()

@app.post("/users", response_model=User)
async def create_user(request: CreateUserRequest):
    # Validation happens automatically via FastAPI/Pydantic
    # But we can add custom validation:

    # Custom business logic
    if "temp" in request.email:
        raise HTTPException(
            status_code=400,
            detail=ValidationErrorResponse(
                message="Temporary emails not allowed",
                field_errors=[{
                    "field": "email",
                    "message": "Temporary email addresses not allowed",
                    "value": request.email
                }]
            ).model_dump()
        )

    user = await user_service.create(request)
    return user

@app.exception_handler(ValidationError)
async def validation_exception_handler(request, exc: ValidationError):
    return JSONResponse(
        status_code=400,
        content=ValidationErrorResponse(
            message="Input validation failed",
            field_errors=[
                {
                    "field": ".".join(str(x) for x in error["loc"]),
                    "message": error["msg"],
                    "value": error.get("input")
                }
                for error in exc.errors()
            ]
        ).model_dump()
    )
```

**Go (go-playground/validator):**
```go
package main

import (
	"net/http"

	"github.com/go-playground/validator/v10"
	"github.com/labstack/echo/v4"
)

var validate = validator.New()

func createUserHandler(c echo.Context) error {
	var req CreateUserRequest
	if err := c.Bind(&req); err != nil {
		return c.JSON(http.StatusBadRequest, map[string]string{
			"error": "Invalid request body",
		})
	}

	// Validate
	if err := validate.Struct(req); err != nil {
		validationErrors := err.(validator.ValidationErrors)

		fieldErrors := make([]FieldError, 0, len(validationErrors))
		for _, err := range validationErrors {
			fieldErrors = append(fieldErrors, FieldError{
				Field:   err.Field(),
				Message: err.Error(),
			})
		}

		return c.JSON(http.StatusBadRequest, ValidationError{
			Code:        "VALIDATION_ERROR",
			Message:     "Input validation failed",
			FieldErrors: fieldErrors,
		})
	}

	// Custom validation
	if strings.Contains(req.Email, "tempmail") {
		return c.JSON(http.StatusBadRequest, ValidationError{
			Code:    "INVALID_EMAIL",
			Message: "Temporary email addresses not allowed",
			FieldErrors: []FieldError{{
				Field:   "email",
				Message: "Temporary email addresses not allowed",
				Value:   req.Email,
			}},
		})
	}

	// Create user
	user, err := userService.Create(req)
	if err != nil {
		return err
	}

	return c.JSON(http.StatusCreated, user)
}
```

---

## CI/CD Integration

### GitHub Actions Workflow

```yaml
# .github/workflows/schemas.yml
name: Schema CI/CD

on:
  push:
    branches: [main]
    paths:
      - 'schemas/**'
  pull_request:
    paths:
      - 'schemas/**'

jobs:
  validate:
    name: Validate Schemas
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'

      - name: Install TypeSpec
        run: |
          npm install -g @typespec/compiler
          npm install

      - name: Lint TypeSpec
        run: |
          cd schemas
          tsp compile . --emit @typespec/lint || true

      - name: Compile TypeSpec
        run: |
          cd schemas
          tsp compile . --emit @typespec/json-schema --output ./generated/json-schema
          tsp compile . --emit @typespec/protobuf --output ./generated/protobuf
          tsp compile . --emit @typespec/openapi3 --output ./generated/openapi

      - name: Validate JSON Schema
        run: |
          npm install -g ajv-cli
          for schema in schemas/generated/json-schema/**/*.json; do
            ajv compile -s "$schema" || exit 1
          done

  compatibility:
    name: Check Compatibility
    runs-on: ubuntu-latest
    needs: validate
    services:
      schema-registry:
        image: confluentinc/cp-schema-registry:7.6.0
        ports:
          - 8081:8081
        env:
          SCHEMA_REGISTRY_HOST_NAME: schema-registry
          SCHEMA_REGISTRY_KAFKASTORE_BOOTSTRAP_SERVERS: dummy:9092
          SCHEMA_REGISTRY_AVRO_COMPATIBILITY_LEVEL: FULL
    steps:
      - uses: actions/checkout@v4

      - name: Download Schemas
        uses: actions/download-artifact@v4
        with:
          name: compiled-schemas
          path: schemas/generated

      - name: Check Compatibility
        run: |
          for schema in schemas/generated/json-schema/**/*.json; do
            subject=$(basename $schema .json)
            echo "Checking compatibility for $subject"

            response=$(curl -s -X POST \
              http://localhost:8081/compatibility/subjects/phenotype.${subject}-jsonschema/versions/latest \
              -H "Content-Type: application/vnd.schemaregistry.v1+json" \
              -d "{\"schema\": $(jq -Rs . < $schema)}")

            is_compatible=$(echo $response | jq -r '.is_compatible')
            if [ "$is_compatible" != "true" ]; then
              echo "Compatibility check failed for $subject"
              echo $response | jq .
              exit 1
            fi
          done

  generate:
    name: Generate Code
    runs-on: ubuntu-latest
    needs: [validate, compatibility]
    strategy:
      matrix:
        language: [typescript, rust, python, go]
    steps:
      - uses: actions/checkout@v4

      - name: Setup for ${{ matrix.language }}
        uses: ./.github/actions/setup-${{ matrix.language }}

      - name: Generate ${{ matrix.language }}
        run: |
          ./scripts/generate-${{ matrix.language }}.sh \
            --input schemas/generated/json-schema \
            --output schemas/generated/${{ matrix.language }}

      - name: Verify Compilation
        run: |
          cd schemas/generated/${{ matrix.language }}
          ${{ matrix.language == 'typescript' && 'npx tsc --noEmit' || '' }}
          ${{ matrix.language == 'rust' && 'cargo check' || '' }}
          ${{ matrix.language == 'python' && 'python -c "import user"' || '' }}
          ${{ matrix.language == 'go' && 'go build ./...' || '' }}

      - name: Upload Generated Code
        uses: actions/upload-artifact@v4
        with:
          name: generated-${{ matrix.language }}
          path: schemas/generated/${{ matrix.language }}

  register:
    name: Register Schemas
    runs-on: ubuntu-latest
    needs: generate
    if: github.ref == 'refs/heads/main'
    environment: production
    steps:
      - uses: actions/checkout@v4

      - name: Download All Generated Code
        uses: actions/download-artifact@v4
        with:
          path: schemas/generated
          pattern: generated-*

      - name: Register with Schema Registry
        env:
          REGISTRY_URL: ${{ secrets.SCHEMA_REGISTRY_URL }}
          REGISTRY_API_KEY: ${{ secrets.SCHEMA_REGISTRY_API_KEY }}
        run: |
          for schema in schemas/generated/json-schema/**/*.json; do
            subject=$(basename $schema .json)
            echo "Registering $subject"

            curl -X POST \
              "$REGISTRY_URL/subjects/phenotype.${subject}-jsonschema/versions" \
              -H "Authorization: Bearer $REGISTRY_API_KEY" \
              -H "Content-Type: application/vnd.schemaregistry.v1+json" \
              -d "{\"schema\": $(jq -Rs . < $schema)}"
          done

  commit:
    name: Commit Generated Code
    runs-on: ubuntu-latest
    needs: generate
    if: github.ref == 'refs/heads/main'
    steps:
      - uses: actions/checkout@v4
        with:
          token: ${{ secrets.GITHUB_TOKEN }}

      - name: Download Generated Code
        uses: actions/download-artifact@v4
        with:
          path: schemas/generated
          pattern: generated-*

      - name: Commit Changes
        run: |
          git config --local user.email "action@github.com"
          git config --local user.name "GitHub Action"

          git add schemas/generated/

          if git diff --staged --quiet; then
            echo "No changes to commit"
            exit 0
          fi

          git commit -m "chore: regenerate code from schemas [skip ci]"
          git push
```

---

## Multi-Language Support

### Language Matrix

| Feature | TypeScript | Rust | Python | Go |
|---------|------------|------|--------|-----|
| **Schema Library** | Zod | Serde + Schemars | Pydantic | Structs + Tags |
| **Validation** | Full | Partial | Full | Partial |
| **Type Inference** | Yes | Yes | Yes | Limited |
| **JSON Support** | Native | Native | Native | Native |
| **Protobuf Support** | Via lib | Prost | protobuf | protoc-gen-go |
| **Avro Support** | Limited | Limited | Via pydantic-avro | Limited |
| **Bundle Size** | ~20KB | Zero-cost | ~100KB | Minimal |
| **Performance** | Good | Excellent | Good | Good |

### Validation Comparison

| Capability | TypeScript (Zod) | Rust (validator) | Python (Pydantic) | Go (validator) |
|------------|------------------|------------------|-------------------|----------------|
| Type checking | Yes | Yes | Yes | Partial |
| Required fields | Yes | Yes | Yes | Yes |
| Min/max length | Yes | Yes | Yes | Yes |
| Pattern/regex | Yes | Yes | Yes | Yes |
| Email format | Yes | Yes | Yes | Yes |
| UUID format | Yes | Yes | Yes | Yes |
| Date/time | Yes | Limited | Yes | Limited |
| Enum validation | Yes | Yes | Yes | Yes |
| Custom validators | Yes | Yes | Yes | Yes |
| Cross-field validation | Yes | Yes | Yes | Manual |
| Conditional validation | Yes | Limited | Yes | Limited |
| Nested validation | Yes | Yes | Yes | Yes |
| Array validation | Yes | Yes | Yes | Yes |
| Unique items | Yes | Manual | Yes | Manual |

---

## Schema Evolution

### Evolution Workflow

```
┌─────────────────────────────────────────────────────────────────────┐
│                     Schema Evolution Workflow                       │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  1. Developer makes schema change                                   │
│     │                                                                │
│     ▼                                                                │
│  ┌─────────────────┐                                                │
│  │ Local Testing   │  Validate locally with --compat=FULL          │
│  └────────┬────────┘                                                │
│           │                                                          │
│           ▼                                                          │
│  ┌─────────────────┐                                                │
│  │ Create PR       │  Include schema changes                        │
│  └────────┬────────┘                                                │
│           │                                                          │
│           ▼                                                          │
│  ┌─────────────────┐                                                │
│  │ CI Validation   │  Check compatibility with registry            │
│  │                 │  FAIL if breaking change without MAJOR bump   │
│  └────────┬────────┘                                                │
│           │                                                          │
│           ▼                                                          │
│  ┌─────────────────┐                                                │
│  │ Code Review     │  Reviewer checks:                             │
│  │                 │  - Semantic version bump                       │
│  │                 │  - Migration guide if breaking                 │
│  │                 │  - Consumer impact analysis                    │
│  └────────┬────────┘                                                │
│           │                                                          │
│           ▼                                                          │
│  ┌─────────────────┐                                                │
│  │ Merge & Deploy  │  Register new schema version                   │
│  │                 │  Generate & publish client code                │
│  └────────┬────────┘                                                │
│           │                                                          │
│           ▼                                                          │
│  ┌─────────────────┐                                                │
│  │ Monitor         │  Track schema usage                            │
│  │                 │  Alert on breaking consumer usage             │
│  └─────────────────┘                                                │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### Breaking Change Detection

```rust
/// Detect breaking changes between schemas
struct BreakingChangeDetector;

impl BreakingChangeDetector {
  fn detect(
    old: &Schema,
    new: &Schema,
  ) -> Vec<BreakingChange> {
    let mut changes = Vec::new();

    // Check for removed fields
    for field in old.required_fields() {
      if !new.has_field(&field.name) {
        changes.push(BreakingChange::RemovedRequiredField {
          field: field.name.clone(),
        });
      }
    }

    // Check for type narrowing
    for field in old.fields() {
      if let Some(new_field) = new.get_field(&field.name) {
        if !new_field.type.is_compatible(&field.type) {
          changes.push(BreakingChange::IncompatibleTypeChange {
            field: field.name.clone(),
            old_type: field.type.clone(),
            new_type: new_field.type.clone(),
          });
        }
      }
    }

    // Check for constraint tightening
    for field in old.fields() {
      if let Some(new_field) = new.get_field(&field.name) {
        if new_field.constraints.is_tighter_than(&field.constraints) {
          changes.push(BreakingChange::TightenedConstraint {
            field: field.name.clone(),
            old_constraints: field.constraints.clone(),
            new_constraints: new_field.constraints.clone(),
          });
        }
      }
    }

    changes
  }
}

enum BreakingChange {
  RemovedRequiredField { field: String },
  IncompatibleTypeChange { field: String, old_type: Type, new_type: Type },
  TightenedConstraint { field: String, old_constraints: Constraints, new_constraints: Constraints },
  ChangedDefaultValue { field: String, old_default: Value, new_default: Value },
}
```

---

## Security Model

### Schema Security Considerations

| Threat | Mitigation |
|--------|------------|
| Injection via schema | Validate schemas before registration |
| DoS via deep nesting | Limit schema complexity (max depth, refs) |
| Information disclosure | Mark sensitive fields with @secret |
| Schema poisoning | Access control on registry writes |
| Replay attacks | Version pinning in clients |

### Access Control

```yaml
# Registry ACL
subjects:
  "phenotype.users.*":
    read:
      - "role:developer"
      - "role:service"
    write:
      - "role:schema-admin"
      - "team:platform"

  "phenotype.events.*":
    read:
      - "role:developer"
      - "role:analyst"
    write:
      - "role:event-admin"
      - "team:data"

  "phenotype.config.*":
    read:
      - "role:developer"
      - "role:ops"
    write:
      - "role:ops-admin"
      - "team:platform"
```

### Sensitive Data Handling

```typespec
model User {
  id: string;
  email: string;

  @secret
  passwordHash: string;  // Never logged or exposed

  @secret
  apiKey: string;  // Masked in debug output

  @visibility("create", "read")
  ssn: string;  // Only visible on create, not update or list
}
```

---

## Performance Requirements

### Throughput Targets

| Operation | Target | Notes |
|-----------|--------|-------|
| Schema compilation | < 5s | For typical schema (100 types) |
| Code generation | < 10s | All languages |
| JSON Schema validation | > 10K ops/sec | Simple objects |
| Protobuf serialization | > 1M ops/sec | Per core |
| Registry lookup | < 50ms | Cached |
| Compatibility check | < 100ms | With remote registry |

### Resource Limits

```yaml
schema_limits:
  max_depth: 10
  max_properties: 1000
  max_enum_values: 1000
  max_string_length: 10000
  max_array_items: 10000
  max_file_size: 1MB

generation_limits:
  max_types_per_file: 100
  max_output_size: 10MB
  max_parallel_jobs: 4
```

---

## Error Handling

### Error Categories

| Category | Description | Example |
|----------|-------------|---------|
| ParseError | Invalid schema syntax | Missing brace in TypeSpec |
| ValidationError | Schema is invalid | Circular reference |
| CompatibilityError | Breaking change | Removed required field |
| GenerationError | Code gen failed | Unsupported type mapping |
| RegistryError | Registry operation failed | Connection timeout |

### Error Format

```json
{
  "error": {
    "code": "SCHEMA_VALIDATION_FAILED",
    "message": "Schema validation failed with 3 errors",
    "details": [
      {
        "code": "INVALID_TYPE",
        "message": "Unknown type 'UUUID'",
        "location": {
          "file": "users/User.tsp",
          "line": 15,
          "column": 10
        },
        "suggestion": "Did you mean 'UUID'?"
      },
      {
        "code": "MISSING_REQUIRED",
        "message": "Field 'email' is required but has no type",
        "location": {
          "file": "users/User.tsp",
          "line": 18,
          "column": 3
        }
      }
    ]
  }
}
```

---

## Testing Strategy

### Test Levels

| Level | Purpose | Tools |
|-------|---------|-------|
| Unit | Schema parsing, validation | Jest, pytest, cargo test |
| Integration | End-to-end generation | Docker, testcontainers |
| Compatibility | Schema evolution | Custom harness |
| Performance | Throughput benchmarks | k6, criterion |
| E2E | Full pipeline | GitHub Actions |

### Test Examples

```rust
#[cfg(test)]
mod schema_tests {
    use super::*;

    #[test]
    fn test_user_schema_validates_valid_input() {
        let schema = load_schema("User.json");
        let input = json!({
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "email": "user@example.com",
            "createdAt": "2024-01-01T00:00:00Z",
            "role": "user"
        });

        let result = schema.validate(&input);
        assert!(result.is_valid());
    }

    #[test]
    fn test_user_schema_rejects_invalid_email() {
        let schema = load_schema("User.json");
        let input = json!({
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "email": "not-an-email",
            "createdAt": "2024-01-01T00:00:00Z"
        });

        let result = schema.validate(&input);
        assert!(!result.is_valid());
        assert!(result.errors().iter().any(|e| e.field == "email"));
    }

    #[test]
    fn test_backward_compatibility() {
        let v1 = load_schema("User-v1.0.0.json");
        let v2 = load_schema("User-v1.1.0.json");

        let check = CompatibilityChecker::new(v1, v2);
        assert!(check.is_backward_compatible());
    }
}
```

---

## Deployment

### Infrastructure

```yaml
# docker-compose.yml for local development
version: '3.8'

services:
  schema-registry:
    image: confluentinc/cp-schema-registry:7.6.0
    ports:
      - "8081:8081"
    environment:
      SCHEMA_REGISTRY_HOST_NAME: schema-registry
      SCHEMA_REGISTRY_KAFKASTORE_BOOTSTRAP_SERVERS: kafka:9092
      SCHEMA_REGISTRY_AVRO_COMPATIBILITY_LEVEL: FULL
      SCHEMA_REGISTRY_JSON_SCHEMA_COMPATIBILITY_LEVEL: FULL
      SCHEMA_REGISTRY_PROTOBUF_COMPATIBILITY_LEVEL: FULL

  # Optional: Web UI for schema browsing
  schema-registry-ui:
    image: landoop/schema-registry-ui:latest
    ports:
      - "8000:8000"
    environment:
      SCHEMAREGISTRY_URL: http://schema-registry:8081
```

### Production Checklist

- [ ] Schema registry deployed with HA configuration
- [ ] Access control configured
- [ ] Backup strategy for schema history
- [ ] Monitoring and alerting set up
- [ ] CI/CD pipeline configured
- [ ] Documentation published
- [ ] Team training completed
- [ ] Migration guide for existing services

---

## Migration Guide

### From Hand-Written Types

1. **Inventory existing types**
   ```bash
   schemas migrate inventory --language typescript --path ./src
   ```

2. **Generate initial TypeSpec**
   ```bash
   schemas migrate generate --from-typescript ./src/types/*.ts --output ./schemas/
   ```

3. **Review and refine schemas**
   - Add constraints (min/max, patterns)
   - Add documentation
   - Define relationships

4. **Set up CI pipeline**
   - Add validation step
   - Add generation step

5. **Gradual adoption**
   - New services use generated types
   - Existing services migrate incrementally

### Breaking Change Process

1. **Analyze impact**
   ```bash
   schemas analyze-impact --schema User --from 1.0.0 --to 2.0.0
   ```

2. **Create migration guide**
   - Document breaking changes
   - Provide code examples
   - Define timeline

3. **Coordinate with consumers**
   - Notify teams
   - Schedule migration window
   - Provide support

4. **Deploy with monitoring**
   - Watch for validation errors
   - Monitor consumer lag
   - Rollback if needed

---

## Appendices

### Appendix A: TypeSpec Quick Reference

See `docs/reference/TYPESPEC_QUICK_REFERENCE.md`

### Appendix B: JSON Schema Patterns

See `docs/reference/JSON_SCHEMA_PATTERNS.md`

### Appendix C: Migration Scripts

See `scripts/migration/`

### Appendix D: Troubleshooting

See `docs/guides/TROUBLESHOOTING.md`

### Appendix E: API Reference

See `docs/reference/API_REFERENCE.md`

---

## References

- [ADR-001: Schema System Architecture](./docs/adr/ADR-001-schema-system-architecture.md)
- [ADR-002: Schema Registry and Versioning](./docs/adr/ADR-002-schema-registry-versioning.md)
- [ADR-003: Code Generation and Runtime Validation](./docs/adr/ADR-003-code-generation-validation.md)
- [SCHEMA_SYSTEMS_SOTA.md](./docs/research/SCHEMA_SYSTEMS_SOTA.md)

### External References

- [TypeSpec Documentation](https://typespec.io/docs)
- [JSON Schema Specification](https://json-schema.org/draft/2020-12/schema)
- [Confluent Schema Registry](https://docs.confluent.io/platform/current/schema-registry/)
- [Protocol Buffers](https://protobuf.dev)
- [Apache Avro](https://avro.apache.org)
- [CUE Language](https://cuelang.org)
- [Zod Documentation](https://zod.dev)
- [Pydantic Documentation](https://docs.pydantic.dev)
- [Serde Documentation](https://serde.rs)

---

**Document Status:** Specification Complete  
**Next Review:** 2026-07-05  
**Maintainer:** Phenotype Architecture Team  
**Questions:** Open an issue in the schemas repository
