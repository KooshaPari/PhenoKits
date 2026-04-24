# Phenotype Contracts Specification

> Hexagonal Architecture Contracts — Ports, Adapters, and Domain Models

**Version**: 1.0  
**Status**: Draft  
**Last Updated**: 2026-04-04  
**Owner**: Phenotype Architecture Team

---

## Table of Contents

1. [Overview](#overview)
2. [Architecture Principles](#architecture-principles)
3. [SOTA: API Contract Landscape](#sota-api-contract-landscape)
4. [SOTA: Schema Validation](#sota-schema-validation)
5. [SOTA: OpenAPI Ecosystem](#sota-openapi-ecosystem)
6. [Core Concepts](#core-concepts)
7. [Port Interface Design](#port-interface-design)
8. [Inbound Ports](#inbound-ports)
9. [Outbound Ports](#outbound-ports)
10. [Domain Models](#domain-models)
11. [Plugin System](#plugin-system)
12. [Contract-First Development](#contract-first-development)
13. [Code Generation](#code-generation)
14. [Testing Strategy](#testing-strategy)
15. [Security Model](#security-model)
16. [Performance Considerations](#performance-considerations)
17. [Implementation Guide](#implementation-guide)
18. [Migration Path](#migration-path)
19. [Related Documents](#related-documents)
20. [Appendices](#appendices)

---

## Overview

### 1.1 Purpose

The Phenotype Contracts system defines a comprehensive set of interfaces, models, and contracts for implementing hexagonal architecture (ports and adapters) across the Phenotype ecosystem. It provides:

- **Type-safe port interfaces** for driving and driven adapters
- **Domain models** for cross-service communication
- **Plugin contracts** for extensibility
- **Validation schemas** for data integrity
- **Code generation** for multi-language support

### 1.2 Scope

This specification covers:

| Component | Description | Languages |
|-----------|-------------|-----------|
| **Port Interfaces** | Driving (inbound) and driven (outbound) ports | Go, Rust, TypeScript, Python |
| **Domain Models** | DTOs, events, commands, queries | Go, Rust, TypeScript, Python |
| **Plugin System** | Extension points and lifecycle | Go, Rust, WASM |
| **Validation** | Schema validation and type checking | All |
| **Code Generation** | Contract-to-code tooling | All |

### 1.3 Target Audience

- **Service Developers**: Implementing hexagonal architecture
- **Adapter Authors**: Creating new adapters for ports
- **Plugin Developers**: Extending the system with plugins
- **Platform Engineers**: Managing contract evolution
- **API Designers**: Defining cross-service contracts

### 1.4 Design Goals

| Goal | Priority | Description |
|------|----------|-------------|
| **Type Safety** | Critical | Compile-time type checking across languages |
| **Testability** | Critical | Easy to mock and test in isolation |
| **Composability** | High | Components compose like functions |
| **Performance** | High | Minimal overhead at boundaries |
| **Evolvability** | High | Backward compatible changes supported |
| **Discoverability** | Medium | Self-documenting interfaces |

---

## Architecture Principles

### 2.1 Hexagonal Architecture

The contracts implement hexagonal architecture (ports and adapters) as described by Alistair Cockburn:

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                         Hexagonal Architecture                                    │
│                                                                                 │
│                         ┌───────────────────────────────────────┐              │
│                         │           Driving Adapters              │              │
│                         │  (REST, gRPC, CLI, Message Handlers)   │              │
│                         └─────────────────────┬─────────────────┘              │
│                                               │                                 │
│                         ┌─────────────────────▼─────────────────┐              │
│                         │         Inbound Ports (Driving)        │              │
│                         │  UseCase, CommandHandler, QueryHandler  │              │
│                         └─────────────────────┬─────────────────┘              │
│                                               │                                 │
│ ┌─────────────────────────────────────────────┼─────────────────────────────┐ │
│ │                                             │                             │ │
│ │                   ┌────────────────────────▼────────────────────────┐       │ │
│ │                   │               Domain Core                          │       │ │
│ │                   │  Entities, Value Objects, Domain Services, Events   │       │ │
│ │                   └────────────────────────▲────────────────────────┘       │ │
│ │                                             │                             │ │
│ └─────────────────────────────────────────────┼─────────────────────────────┘ │
│                                               │                                 │
│                         ┌─────────────────────▼─────────────────┐              │
│                         │         Outbound Ports (Driven)         │              │
│                         │  Repository, Cache, EventBus, External  │              │
│                         └─────────────────────┬─────────────────┘              │
│                                               │                                 │
│                         ┌─────────────────────▼─────────────────┐              │
│                         │           Driven Adapters              │              │
│                         │  (Postgres, Redis, Kafka, HTTP Client)  │              │
│                         └───────────────────────────────────────┘              │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### 2.2 SOLID Principles in Contracts

| Principle | Application | Example |
|-----------|-------------|---------|
| **Single Responsibility** | Each port has one purpose | `Repository` for persistence only |
| **Open/Closed** | Extend via new ports, don't modify | Add `CacheRepository` without changing `Repository` |
| **Liskov Substitution** | Adapter substitution | Swap Postgres for MySQL adapter |
| **Interface Segregation** | Small, focused interfaces | `QueryRepository` separate from `Repository` |
| **Dependency Inversion** | Depend on ports, not implementations | Service depends on `Repository`, not `*sql.DB` |

### 2.3 Domain-Driven Design Patterns

The contracts support DDD patterns:

| Pattern | Contract Support | Implementation |
|---------|-----------------|----------------|
| **Entity** | Generic interfaces | `Repository[T, ID]` |
| **Value Object** | Immutable models | Structs with no setters |
| **Domain Event** | Event interfaces | `EventHandler[E any]` |
| **Aggregate** | Repository per aggregate | One repo per aggregate root |
| **CQRS** | Command/Query separation | `CommandHandler`, `QueryHandler` |
| **Event Sourcing** | Event store interface | `EventStore` port |

---

## SOTA: API Contract Landscape

### 3.1 Contract Format Evolution

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                        API Contract Format Evolution                            │
│                                                                                 │
│  2011 ────────┐                                                                 │
│  Swagger 1.0  │  XML-based, limited                                           │
│               │                                                                 │
│  2014 ────────┼───┐                                                             │
│  Swagger 2.0  │   │  JSON-based, schemas                                        │
│               │   │                                                             │
│  2017 ────────┼───┼───┐                                                         │
│  OpenAPI 3.0  │   │   │  Components, callbacks                                  │
│               │   │   │                                                         │
│  2021 ────────┼───┼───┼───┐                                                     │
│  OpenAPI 3.1  │   │   │   │  JSON Schema 2020-12, webhooks                      │
│               │   │   │   │                                                     │
│  2024+ ───────┴───┴───┴───┴──► OpenAPI 4.0 (Moonwalk)                            │
│                                                                                 │
│  Alternatives:                                                                  │
│  ├── AsyncAPI ──────► Event-driven APIs (Kafka, MQTT)                          │
│  ├── GraphQL SDL ───► GraphQL type system                                       │
│  ├── Protocol Buffers ──► gRPC services                                         │
│  ├── Smithy ────────► AWS service definitions                                   │
│  └── TypeSpec ──────► Microsoft API DSL                                        │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### 3.2 Format Comparison Matrix

| Format | Use Case | Human Readable | Tooling | Code Gen | Phenotype Status |
|--------|----------|----------------|---------|----------|------------------|
| **OpenAPI 3.1** | REST APIs | ✅ YAML/JSON | Excellent | Excellent | **Primary** |
| **AsyncAPI** | Events | ✅ YAML/JSON | Good | Good | Planned |
| **GraphQL SDL** | GraphQL | ✅ | Excellent | Excellent | Supported |
| **Protocol Buffers** | gRPC | ❌ (binary) | Excellent | Excellent | Internal RPC |
| **Smithy** | AWS APIs | ✅ | Good | Good | Research |
| **TypeSpec** | Modern APIs | ✅ | Growing | Good | Research |
| **JSON Schema** | Validation | ✅ | Excellent | Good | Validation |
| **WSDL** | SOAP | ✅ XML | Legacy | Legacy | Avoid |

### 3.3 Industry Adoption (2024-2026)

| Format | Public APIs | Internal APIs | Events | Documentation |
|--------|-------------|---------------|--------|---------------|
| **OpenAPI 3.1** | 78% | 65% | 12% | 85% |
| **OpenAPI 3.0** | 15% | 20% | 5% | 10% |
| **AsyncAPI** | 3% | 8% | 45% | 3% |
| **GraphQL** | 4% | 7% | 35% | 2% |
| **Protobuf/gRPC** | 0% | 25% | 3% | 0% |

---

## SOTA: Schema Validation

### 4.1 Validation Technology Landscape

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                     Schema Validation Technology Stack                            │
│                                                                                 │
│  ┌──────────────────────────────────────────────────────────────────────────┐  │
│  │                        Validation Layers                                  │  │
│  │                                                                           │  │
│  │   Edge (Gateway)    │   Application         │   Domain                     │  │
│  │   ─────────────     │   ───────────         │   ──────                     │  │
│  │   JSON Schema       │   Native Types        │   Business Rules             │  │
│  │   (Fast rejection)  │   + Validation Libs   │   (Complex logic)            │  │
│  │                     │                       │                              │  │
│  │   • Network-level   │   • go-validator      │   • Custom validators        │  │
│  │   • Early rejection │   • Zod               │   • Cross-field checks       │  │
│  │   • DDoS protection │   • Pydantic          │   • External validation      │  │
│  │                     │   • Validator.rs      │                              │  │
│  └──────────────────────────────────────────────────────────────────────────┘  │
│                                                                                 │
│  Performance Comparison (requests/sec):                                       │
│                                                                                 │
│  JSON Schema (compiled):    1,000,000+                                          │
│  Native Go validator:       500,000+                                          │
│  Native Rust validator:   2,000,000+                                          │
│  Zod (TypeScript):            50,000+                                          │
│  Pydantic V2:                100,000+                                          │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### 4.2 Validator Implementations by Language

| Language | Library | Performance | Features | Status |
|----------|---------|-------------|----------|--------|
| **Go** | go-playground/validator | Fast | Struct tags, custom | Production |
| **Go** | ozzo-validation | Fast | Fluent API | Production |
| **Rust** | validator | Very Fast | Derive macros | Production |
| **Rust** | garde | Very Fast | Derive macros | Emerging |
| **TypeScript** | Zod | Medium | Type inference | Production |
| **TypeScript** | io-ts | Medium | Codec-based | Mature |
| **TypeScript** | valibot | Medium | Tree-shakeable | Emerging |
| **Python** | Pydantic V2 | Fast | Type hints | Production |
| **Python** | marshmallow | Medium | Schema-based | Mature |
| **Python** | attrs + cattrs | Fast | Class-based | Mature |

### 4.3 JSON Schema Performance

| Implementation | Language | Cold Start | Warm Validation | Large Schema |
|----------------|----------|------------|-----------------|--------------|
| **ajv** | JavaScript | 15ms | 0.001ms | 12ms |
| **jsonschema-rs** | Rust | 5ms | 0.0005ms | 3ms |
| **go-jsonschema** | Go | 8ms | 0.002ms | 7ms |
| **fastjsonschema** | Python | 25ms | 0.01ms | 20ms |
| **jsonschema (Python)** | Python | 50ms | 0.1ms | 45ms |

---

## SOTA: OpenAPI Ecosystem

### 5.1 Tool Ecosystem Overview

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                        OpenAPI Tool Ecosystem                                     │
│                                                                                 │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │
│  │   Design     │  │   Generate   │  │    Test      │  │   Document   │       │
│  │              │  │              │  │              │  │              │       │
│  │ • Stoplight  │  │ • oapi-codegen│  │ • Pact      │  │ • Redoc      │       │
│  │ • Postman    │  │ • openapi-gen │  │ • Schemathesis│  │ • Swagger UI │       │
│  │ • Insomnia   │  │ • openapi-ts  │  │ • Prism     │  │ • Elements   │       │
│  │ • SwaggerHub │  │ • orval       │  │ • Portman   │  │ • Bump.sh    │       │
│  └──────────────┘  └──────────────┘  └──────────────┘  └──────────────┘       │
│                                                                                 │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐                         │
│  │   Validate   │  │    Mock      │  │   Monitor    │                         │
│  │              │  │              │  │              │                         │
│  │ • Spectral   │  │ • Prism      │  │ • Optic      │                         │
│  │ • Schemathesis│  │ • WireMock   │  │ • Akita      │                         │
│  │ • Dredd      │  │ • Mockoon    │  │ • Treblle    │                         │
│  └──────────────┘  └──────────────┘  └──────────────┘                         │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### 5.2 Code Generator Comparison

| Generator | Languages | Server | Client | Quality | Maintenance |
|-----------|-----------|--------|--------|---------|-------------|
| **openapi-generator** | 50+ | ✅ | ✅ | High | Active |
| **oapi-codegen** | Go | ✅ | ✅ | Very High | Active |
| **openapi-typescript** | TypeScript | ❌ | ✅ | Very High | Active |
| **orval** | TypeScript | ❌ | ✅ | High | Active |
| **progenitor** | Rust | ❌ | ✅ | Very High | Active |
| **datamodel-code-generator** | Python | ❌ | ✅ | High | Active |
| **ng-openapi-gen** | TypeScript/Angular | ❌ | ✅ | High | Active |

### 5.3 Documentation Tools

| Tool | OpenAPI 3.1 | Interactive | Theming | Hosting | Best For |
|------|-------------|-------------|---------|---------|----------|
| **Redoc** | ✅ | ❌ | ✅ | Self/Cloud | Beautiful static docs |
| **Swagger UI** | ✅ | ✅ | ✅ | Self | Interactive testing |
| **Stoplight Elements** | ✅ | ✅ | ✅ | Both | Embedded docs |
| **ReadMe** | ✅ | ✅ | ✅ | Cloud | Developer portals |
| **Mintlify** | ✅ | ✅ | ✅ | Cloud | Modern dev docs |
| **Bump.sh** | ✅ | ❌ | ✅ | Cloud | Changelog focus |

---

## Core Concepts

### 6.1 Port Definition

A **port** is an interface that defines how the application core interacts with the outside world:

```go
// Port interface characteristics:
// 1. Language-agnostic concept (interface in Go, trait in Rust)
// 2. Defines what, not how (DIP - Dependency Inversion Principle)
// 3. Testable via mocks
// 4. Multiple adapters can implement same port
```

### 6.2 Port Categories

| Category | Direction | Implemented By | Example |
|----------|-----------|----------------|---------|
| **Driving (Inbound)** | Outside → Core | Adapters | `UseCase`, `CommandHandler` |
| **Driven (Outbound)** | Core → Outside | Infrastructure | `Repository`, `Cache` |

### 6.3 Port Naming Conventions

```
Inbound Ports (Driving):
├── UseCase              # Standard use case
├── CommandHandler       # CQRS command handler
├── QueryHandler         # CQRS query handler
├── EventHandler         # Domain event handler
├── InputPort            # Generic input
├── Validator            # Input validation
└── Interceptor          # Request/response middleware

Outbound Ports (Driven):
├── Repository           # Data persistence
├── QueryRepository      # Read-only queries
├── EventStore           # Event sourcing
├── EventPublisher       # Event distribution
├── Cache                # Caching
├── ExternalService      # HTTP clients
├── SecretStore          # Secrets management
├── MetricsCollector     # Metrics
├── Logger               # Logging
└── ConfigProvider       # Configuration
```

### 6.4 Contract Types

| Contract Type | Purpose | Format | Location |
|---------------|---------|--------|----------|
| **Port Interface** | Go/Rust interfaces | Code | `contracts/ports/` |
| **Domain Model** | Data structures | Code | `contracts/models/` |
| **API Contract** | REST API spec | OpenAPI | `contracts/openapi/` |
| **Event Contract** | Event schemas | AsyncAPI | `contracts/asyncapi/` |
| **Plugin Contract** | Plugin interfaces | Code + YAML | `contracts/plugins/` |

---

## Port Interface Design

### 7.1 Generic Port Patterns

The contracts use Go generics for type-safe ports:

```go
// Generic UseCase interface
type UseCase[In any, Out any] interface {
    Execute(ctx context.Context, input In) (Out, error)
}

// Generic Repository interface
type Repository[T any, ID any] interface {
    Create(ctx context.Context, entity T) (T, error)
    GetByID(ctx context.Context, id ID) (T, error)
    Update(ctx context.Context, entity T) (T, error)
    Delete(ctx context.Context, id ID) error
    List(ctx context.Context, filter *QueryFilter) ([]T, error)
}

// Generic CommandHandler
type CommandHandler[C any] interface {
    Handle(ctx context.Context, cmd C) (*CommandResult, error)
}

// Generic QueryHandler
type QueryHandler[Q any, R any] interface {
    Handle(ctx context.Context, query Q) (R, error)
}
```

### 7.2 Port Method Design Principles

| Principle | Rule | Example |
|-----------|------|---------|
| **Context First** | `ctx context.Context` as first param | `Execute(ctx, input)` |
| **Error Last** | Return error as last result | `(Result, error)` |
| **No Surprises** | Behavior matches naming | `Delete` actually deletes |
| **Complete Operations** | Single method does complete operation | `Create` returns created entity |
| **Idempotency** | Safe to retry | `Create` with idempotency key |

### 7.3 Port Granularity

| Granularity | Pros | Cons | Use When |
|-------------|------|------|----------|
| **Fine-grained** | Reusable, testable | More interfaces | Shared operations |
| **Coarse-grained** | Fewer interfaces, simpler | Less reusable | Use case specific |
| **CRUD** | Standard patterns | May not fit domain | Simple entities |
| **Domain-specific** | Matches domain | Less consistent | Complex domains |

---

## Inbound Ports

### 8.1 UseCase Interface

The fundamental inbound port for application operations:

```go
// contracts/ports/inbound/ports.go
package inbound

import "context"

// UseCase defines the interface for application use cases.
// Following SRP: Each use case has a single responsibility.
type UseCase interface {
    // Execute runs the use case with the given context and input.
    // Returns the output or error.
    Execute(ctx context.Context, input interface{}) (interface{}, error)
}

// Generic version for type safety
type UseCase[In any, Out any] interface {
    Execute(ctx context.Context, input In) (Out, error)
}

// Implementing a use case
type CreateOrderUseCase struct {
    orderRepo outbound.Repository[Order, string]
    eventBus  outbound.EventPublisher
}

func (uc *CreateOrderUseCase) Execute(
    ctx context.Context,
    input CreateOrderInput,
) (CreateOrderOutput, error) {
    // 1. Validate input
    if err := input.Validate(); err != nil {
        return CreateOrderOutput{}, err
    }
    
    // 2. Create order entity
    order, err := domain.NewOrder(input.CustomerID, input.Items)
    if err != nil {
        return CreateOrderOutput{}, err
    }
    
    // 3. Persist via outbound port
    saved, err := uc.orderRepo.Create(ctx, order)
    if err != nil {
        return CreateOrderOutput{}, err
    }
    
    // 4. Publish event
    event := domain.NewOrderCreatedEvent(saved)
    if err := uc.eventBus.Publish(ctx, "orders", []models.DomainEvent{event}); err != nil {
        // Log but don't fail
    }
    
    return CreateOrderOutput{OrderID: saved.ID}, nil
}
```

### 8.2 CQRS Handler Interfaces

```go
// CommandHandler handles commands (CQRS pattern).
// Commands are operations that change state.
type CommandHandler[C any] interface {
    // Handle processes a command and returns a result.
    Handle(ctx context.Context, cmd C) (*models.CommandResult, error)
}

// QueryHandler handles queries (CQRS pattern).
// Queries are operations that read state without side effects.
type QueryHandler[Q any, R any] interface {
    // Handle processes a query and returns results.
    Handle(ctx context.Context, query Q) (R, error)
}

// EventHandler handles domain events.
type EventHandler[E any] interface {
    // Handle processes an event.
    Handle(ctx context.Context, event E) error
}
```

### 8.3 InputPort and Processing Interfaces

```go
// InputPort is a generic interface for input ports.
// Used by driving adapters to interact with the application.
type InputPort[In any, Out any] interface {
    // Execute runs the port with input and returns output.
    Execute(ctx context.Context, in In) (Out, error)
}

// Validator defines interface for input validation.
// Following ISP: Separate interfaces for different validation concerns.
type Validator[In any] interface {
    // Validate checks if the input is valid.
    Validate(in In) error
}

// PreProcessor defines interface for preprocessing input.
// Applies cross-cutting concerns before use case execution.
type PreProcessor[In any] interface {
    // PreProcess runs preprocessing on input.
    PreProcess(ctx context.Context, in In) (In, error)
}

// PostProcessor defines interface for postprocessing output.
// Applies cross-cutting concerns after use case execution.
type PostProcessor[Out any] interface {
    // PostProcess runs postprocessing on output.
    PostProcess(ctx context.Context, out Out) (Out, error)
}

// Interceptor defines middleware interface for use cases.
// Combines PreProcessor and PostProcessor for full request/response interception.
type Interceptor[In any, Out any] interface {
    PreProcessor[In]
    PostProcessor[Out]
}
```

### 8.4 Auth Command/Query Interfaces

```go
// contracts/ports/inbound/auth.go
package inbound

import "time"

// AuthCommands defines CQRS commands for authentication operations.
type AuthCommands struct{}

// LoginCommand represents user login.
type LoginCommand struct {
    Email    string
    Password string
}

// LoginHandler handles LoginCommand.
type LoginHandler func(cmd LoginCommand) (*AuthResponse, error)

// RefreshTokenCommand represents token refresh.
type RefreshTokenCommand struct {
    RefreshToken string
}

// RefreshTokenHandler handles RefreshTokenCommand.
type RefreshTokenHandler func(cmd RefreshTokenCommand) (*AuthResponse, error)

// LogoutCommand represents user logout.
type LogoutCommand struct {
    AccessToken string
}

// LogoutHandler handles LogoutCommand.
type LogoutHandler func(cmd LogoutCommand) error

// CreateAPIKeyCommand represents API key creation.
type CreateAPIKeyCommand struct {
    UserID    string
    Name      string
    Scopes    []string
    RateLimit int
}

// CreateAPIKeyHandler handles CreateAPIKeyCommand.
type CreateAPIKeyHandler func(cmd CreateAPIKeyCommand) (*CreateAPIKeyResponse, error)

// RevokeAPIKeyCommand represents API key revocation.
type RevokeAPIKeyCommand struct {
    KeyID string
}

// RevokeAPIKeyHandler handles RevokeAPIKeyCommand.
type RevokeAPIKeyHandler func(cmd RevokeAPIKeyCommand) error

// AuthResponse contains the response for auth operations.
type AuthResponse struct {
    AccessToken  string   `json:"access_token"`
    RefreshToken string   `json:"refresh_token,omitempty"`
    ExpiresIn    int64    `json:"expires_in"`
    TokenType    string   `json:"token_type"`
    UserID       string   `json:"user_id"`
    Email        string   `json:"email"`
    Roles        []string `json:"roles"`
}

// CreateAPIKeyResponse contains the response for API key creation.
type CreateAPIKeyResponse struct {
    Key       string    `json:"key"`
    KeyID     string    `json:"key_id"`
    Name      string    `json:"name"`
    CreatedAt time.Time `json:"created_at"`
}
```

---

## Outbound Ports

### 9.1 Repository Interface

The primary outbound port for data persistence:

```go
// contracts/ports/outbound/ports.go
package outbound

import (
    "context"
    "contracts/models"
)

// Repository defines the interface for data persistence.
// Following DIP: Depend on abstraction, not concrete implementation.
type Repository[T any, ID any] interface {
    // Create persists a new entity and returns the created entity with ID.
    Create(ctx context.Context, entity T) (T, error)

    // GetByID retrieves an entity by its ID.
    GetByID(ctx context.Context, id ID) (T, error)

    // Update modifies an existing entity.
    Update(ctx context.Context, entity T) (T, error)

    // Delete removes an entity by its ID.
    Delete(ctx context.Context, id ID) error

    // List returns all entities with optional filtering.
    List(ctx context.Context, filter *models.QueryFilter) ([]T, error)

    // Exists checks if an entity with the given ID exists.
    Exists(ctx context.Context, id ID) (bool, error)

    // Count returns the total number of entities.
    Count(ctx context.Context, filter *models.QueryFilter) (int64, error)
}

// QueryRepository defines read-only repository operations.
// Following CQRS: Separate read and write models for optimized queries.
type QueryRepository[T any] interface {
    // Find retrieves entities based on query criteria.
    Find(ctx context.Context, criteria *models.QueryCriteria) ([]T, error)

    // FindOne retrieves a single entity based on criteria.
    FindOne(ctx context.Context, criteria *models.QueryCriteria) (*T, error)

    // Aggregate performs aggregation operations (count, sum, avg, etc.).
    Aggregate(ctx context.Context, criteria *models.QueryCriteria) (*models.AggregationResult, error)
}
```

### 9.2 Event Store and Publisher

```go
// EventStore defines interface for event persistence.
// Following Event Sourcing: Store events, derive state.
type EventStore interface {
    // Append adds events to the store.
    Append(ctx context.Context, aggregateID string, events []models.DomainEvent) error

    // GetEvents retrieves all events for an aggregate.
    GetEvents(ctx context.Context, aggregateID string) ([]models.DomainEvent, error)

    // GetEventsSince retrieves events since a specific version.
    GetEventsSince(ctx context.Context, aggregateID string, version int64) ([]models.DomainEvent, error)

    // GetAllEvents retrieves all events with optional filtering.
    GetAllEvents(ctx context.Context, filter *models.EventFilter) ([]models.DomainEvent, error)
}

// EventPublisher defines interface for publishing domain events.
type EventPublisher interface {
    // Publish sends events to the message bus.
    Publish(ctx context.Context, topic string, events []models.DomainEvent) error

    // PublishAsync sends events asynchronously.
    PublishAsync(ctx context.Context, topic string, events []models.DomainEvent) (<-chan error, error)

    // Subscribe registers a handler for events on a topic.
    Subscribe(ctx context.Context, topic string, handler models.EventHandler) error
}

// EventBusPort defines the minimal event bus contract used by application services.
type EventBusPort interface {
    Publish(ctx context.Context, topic string, event any) error
}
```

### 9.3 Cache Interface

```go
// Cache defines interface for caching operations.
type Cache interface {
    // Get retrieves a value from cache.
    Get(ctx context.Context, key string) ([]byte, error)

    // Set stores a value in cache with optional TTL.
    Set(ctx context.Context, key string, value []byte, ttl *models.Duration) error

    // Delete removes a key from cache.
    Delete(ctx context.Context, key string) error

    // Exists checks if a key exists in cache.
    Exists(ctx context.Context, key string) (bool, error)

    // Clear removes all entries from cache.
    Clear(ctx context.Context) error

    // GetOrSet retrieves from cache or compute and store.
    GetOrSet(ctx context.Context, key string, compute func() ([]byte, error), ttl *models.Duration) ([]byte, error)
}
```

### 9.4 External Service and Infrastructure

```go
// ExternalService defines interface for calling external HTTP services.
type ExternalService interface {
    // Call makes an HTTP request to an external service.
    Call(ctx context.Context, request *models.ExternalRequest) (*models.ExternalResponse, error)

    // CallWithRetry makes an HTTP request with retry logic.
    CallWithRetry(ctx context.Context, request *models.ExternalRequest, retryConfig *models.RetryConfig) (*models.ExternalResponse, error)
}

// SecretStore defines interface for secret management.
type SecretStore interface {
    // Get retrieves a secret value.
    Get(ctx context.Context, key string) (string, error)

    // Set stores a secret value.
    Set(ctx context.Context, key string, value string) error

    // Delete removes a secret.
    Delete(ctx context.Context, key string) error

    // List returns all secret keys.
    List(ctx context.Context, path string) ([]string, error)
}

// MetricsCollector defines interface for collecting metrics.
type MetricsCollector interface {
    // Counter records a counter metric.
    Counter(ctx context.Context, name string, value float64, labels map[string]string) error

    // Gauge records a gauge metric.
    Gauge(ctx context.Context, name string, value float64, labels map[string]string) error

    // Histogram records a histogram metric.
    Histogram(ctx context.Context, name string, value float64, labels map[string]string) error

    // Summary records a summary metric.
    Summary(ctx context.Context, name string, value float64, labels map[string]string) error
}

// Logger defines interface for logging operations.
type Logger interface {
    // Debug logs a debug message.
    Debug(ctx context.Context, msg string, args ...any)

    // Info logs an info message.
    Info(ctx context.Context, msg string, args ...any)

    // Warn logs a warning message.
    Warn(ctx context.Context, msg string, args ...any)

    // Error logs an error message.
    Error(ctx context.Context, msg string, args ...any)

    // Fatal logs a fatal message and exits.
    Fatal(ctx context.Context, msg string, args ...any)
}

// ObservabilityPort records errors and operational signals.
type ObservabilityPort interface {
    RecordError(ctx context.Context, operation string, err error)
}

// ConfigProvider defines interface for configuration management.
type ConfigProvider interface {
    // Get retrieves a configuration value.
    Get(ctx context.Context, key string) (any, error)

    // Set sets a configuration value.
    Set(ctx context.Context, key string, value any) error

    // GetAll retrieves all configuration.
    GetAll(ctx context.Context) (map[string]any, error)

    // Subscribe registers a callback for configuration changes.
    Subscribe(ctx context.Context, key string, callback func(any)) error
}
```

### 9.5 Database Operations

```go
// contracts/ports/outbound/db.go
package outbound

import "context"

// QueryResult represents the result of a database query with pagination.
type QueryResult[T any] struct {
    Data       []T
    TotalCount int64
    Page       int
    PageSize   int
    HasNext    bool
    HasPrev    bool
}

// PaginationParams holds pagination parameters.
type PaginationParams struct {
    Page     int
    PageSize int
}

// NewPaginationParams creates pagination params with defaults.
func NewPaginationParams(page, pageSize int) PaginationParams {
    if page < 1 {
        page = 1
    }
    if pageSize < 1 {
        pageSize = 20
    }
    if pageSize > 100 {
        pageSize = 100
    }
    return PaginationParams{Page: page, PageSize: pageSize}
}

// Offset returns the SQL offset.
func (p PaginationParams) Offset() int {
    return (p.Page - 1) * p.PageSize
}

// QueryOptions holds optional query parameters.
type QueryOptions struct {
    Timeout       int // milliseconds, 0 = default
    SlowThreshold int // milliseconds, 0 = disabled
    MaxRows       int // 0 = unlimited
    ReadOnly      bool
    NoWait        bool
}

// QueryExecutor defines the interface for executing queries.
// This is the primary outbound port for database operations.
type QueryExecutor interface {
    // Query executes a SELECT query and returns rows.
    Query(ctx context.Context, query string, args ...any) (Rows, error)

    // QueryRow executes a query that returns a single row.
    QueryRow(ctx context.Context, query string, args ...any) Row

    // Exec executes a query that doesn't return rows (INSERT, UPDATE, DELETE).
    Exec(ctx context.Context, query string, args ...any) (Result, error)

    // BeginTx starts a new transaction with options.
    BeginTx(ctx context.Context, opts TxOptions) (Transaction, error)

    // Ping checks database connectivity.
    Ping(ctx context.Context) error

    // Stats returns connection pool statistics.
    Stats() PoolStats
}

// Transaction represents a database transaction.
type Transaction interface {
    QueryExecutor

    // Commit commits the transaction.
    Commit(ctx context.Context) error

    // Rollback rolls back the transaction.
    Rollback(ctx context.Context) error

    // CommitAsync commits asynchronously.
    CommitAsync() error
}

// TxOptions holds transaction options.
type TxOptions struct {
    Isolation IsolationLevel
    ReadOnly  bool
    NoWait    bool
    Immediate bool
}

// IsolationLevel represents SQL isolation levels.
type IsolationLevel int

const (
    IsolationDefault         IsolationLevel = 0
    IsolationReadUncommitted IsolationLevel = 1
    IsolationReadCommitted   IsolationLevel = 2
    IsolationWriteCommitted  IsolationLevel = 3
    IsolationRepeatableRead  IsolationLevel = 4
    IsolationSnapshot        IsolationLevel = 5
    IsolationSerializable    IsolationLevel = 6
    IsolationLinearizable    IsolationLevel = 7
)

// Row represents a single row returned by QueryRow.
type Row interface {
    Scan(dest ...any) error
    Err() error
}

// Rows represents a set of rows returned by Query.
type Rows interface {
    Row

    // Next advances to the next row.
    Next() bool

    // Close closes the rows.
    Close() error

    // Columns returns the column names.
    Columns() ([]string, error)

    // ScanSlice scans the current row into a slice.
    ScanSlice(dest any) error

    // ScanMap scans the current row into a map.
    ScanMap(dest map[string]any) error
}

// Result represents the result of an Exec operation.
type Result interface {
    LastInsertId() (int64, error)
    RowsAffected() (int64, error)
}

// PoolStats holds connection pool statistics.
type PoolStats struct {
    MaxOpenConnections int
    OpenConnections    int
    InUseConnections   int
    IdleConnections    int
    WaitCount          int64
    WaitDuration       int64 // nanoseconds
    MaxIdleClosed      int64
    MaxLifetimeClosed  int64
}

// ConnectionPool provides connection pool management.
type ConnectionPool interface {
    // ConfigurePool sets up the connection pool.
    ConfigurePool(config PoolConfig) error

    // GetPoolStats returns current pool statistics.
    GetPoolStats(ctx context.Context) (PoolStats, error)

    // HealthCheck verifies database health.
    HealthCheck(ctx context.Context) error

    // Close closes all connections.
    Close() error
}

// PoolConfig holds database connection pool configuration.
type PoolConfig struct {
    MaxOpenConns      int  // default: 25
    MaxIdleConns      int  // default: 5
    ConnMaxLifetime   int  // milliseconds, default: 5 minutes
    ConnMaxIdleTime   int  // milliseconds, default: 1 minute
    DialTimeout       int  // milliseconds, default: 5 seconds
    QueryTimeout      int  // milliseconds, default: 30 seconds
    EnableHealthCheck bool // default: true
}

// DefaultPoolConfig returns the default pool configuration.
func DefaultPoolConfig() PoolConfig {
    return PoolConfig{
        MaxOpenConns:      25,
        MaxIdleConns:      5,
        ConnMaxLifetime:   300000, // 5 minutes
        ConnMaxIdleTime:   60000,  // 1 minute
        DialTimeout:       5000,   // 5 seconds
        QueryTimeout:      30000,  // 30 seconds
        EnableHealthCheck: true,
    }
}

// IndexManager defines the interface for managing database indexes.
type IndexManager interface {
    // CreateIndex creates an index.
    CreateIndex(ctx context.Context, def IndexDefinition) error

    // DropIndex drops an index.
    DropIndex(ctx context.Context, name string) error

    // ListIndexes returns all indexes for a table.
    ListIndexes(ctx context.Context, table string) ([]IndexDefinition, error)

    // CreateIndexes creates multiple indexes efficiently.
    CreateIndexes(ctx context.Context, defs []IndexDefinition) error
}

// IndexDefinition represents a database index definition.
type IndexDefinition struct {
    Name         string
    Table        string
    Columns      []string
    Unique       bool
    Partial      string // SQL WHERE clause for partial index
    Concurrently bool   // CREATE INDEX CONCURRENTLY
}

// MigrationExecutor defines the interface for running migrations.
type MigrationExecutor interface {
    // Up runs pending migrations.
    Up(ctx context.Context) error

    // Down rolls back the last migration.
    Down(ctx context.Context) error

    // Version returns the current migration version.
    Version(ctx context.Context) (int, error)

    // Pending returns pending migrations.
    Pending(ctx context.Context) ([]Migration, error)
}

// Migration represents a database migration.
type Migration struct {
    Version   int
    Name      string
    AppliedAt int64
}
```

### 9.6 Adapter Manifest

```go
// AdapterManifest provides metadata about an adapter implementation.
// Following PoLA: Adapters declare their capabilities explicitly.
type AdapterManifest struct {
    // Name is the unique name of the adapter.
    Name string

    // Version is the semantic version of the adapter.
    Version string

    // Description describes what the adapter does.
    Description string

    // Provides lists the port interfaces this adapter implements.
    Provides []string
}

// Adapter is implemented by all adapters to provide self-description.
// Following SoC: Adapters self-document their capabilities.
type Adapter interface {
    // Manifest returns the adapter's manifest.
    Manifest() *AdapterManifest
}
```

---

## Domain Models

### 10.1 Domain Event Model

```go
// contracts/models/events.go
package models

import (
    "encoding/json"
    "time"
)

// DomainEvent represents a domain event following Event Sourcing patterns.
type DomainEvent struct {
    // ID is the unique identifier of the event.
    ID string `json:"id"`

    // AggregateID is the ID of the aggregate that generated this event.
    AggregateID string `json:"aggregate_id"`

    // AggregateType is the type of the aggregate.
    AggregateType string `json:"aggregate_type"`

    // EventType is the type of the event.
    EventType string `json:"event_type"`

    // EventData contains the event payload.
    EventData json.RawMessage `json:"event_data"`

    // Version is the version of the aggregate at the time of the event.
    Version int64 `json:"version"`

    // OccurredAt is when the event occurred.
    OccurredAt time.Time `json:"occurred_at"`

    // Metadata contains additional event metadata.
    Metadata map[string]string `json:"metadata,omitempty"`
}

// EventHandler is a function type for handling events.
type EventHandler func(ctx interface{}, event *DomainEvent) error

// EventFilter contains filtering criteria for event queries.
type EventFilter struct {
    // AggregateID filters by aggregate ID.
    AggregateID string

    // AggregateType filters by aggregate type.
    AggregateType string

    // EventType filters by event type.
    EventType string

    // From filters events after this time.
    From *time.Time

    // To filters events before this time.
    To *time.Time

    // Limit limits the number of events returned.
    Limit int

    // Offset offsets the results.
    Offset int
}
```

### 10.2 Command and Query Models

```go
// CommandResult represents the result of a command execution.
type CommandResult struct {
    // Success indicates if the command succeeded.
    Success bool `json:"success"`

    // Error contains error information if the command failed.
    Error *CommandError `json:"error,omitempty"`

    // Data contains command result data.
    Data any `json:"data,omitempty"`

    // Metadata contains additional result metadata.
    Metadata map[string]string `json:"metadata,omitempty"`
}

// CommandError represents a command execution error.
type CommandError struct {
    // Code is the error code.
    Code string `json:"code"`

    // Message is the human-readable error message.
    Message string `json:"message"`

    // Details contains additional error details.
    Details map[string]any `json:"details,omitempty"`
}

// QueryFilter contains filtering criteria for entity queries.
type QueryFilter struct {
    // IDs filters by entity IDs.
    IDs []string

    // Limit limits the number of results.
    Limit int

    // Offset offsets the results.
    Offset int

    // SortBy specifies the field to sort by.
    SortBy string

    // SortOrder specifies the sort order (asc/desc).
    SortOrder string
}

// QueryCriteria contains advanced query criteria.
type QueryCriteria struct {
    // Filter contains field-value filters.
    Filter map[string]any

    // Pagination for the query.
    Limit  int
    Offset int

    // Sort specifies sorting criteria.
    Sort []SortCriterion

    // Include specifies related entities to include.
    Include []string

    // Exclude specifies fields to exclude.
    Exclude []string
}

// SortCriterion represents a sorting criterion.
type SortCriterion struct {
    // Field is the field to sort by.
    Field string

    // Order is the sort order (asc/desc).
    Order string
}

// AggregationResult represents the result of an aggregation query.
type AggregationResult struct {
    // Count is the count of matching entities.
    Count int64 `json:"count,omitempty"`

    // Sum is the sum of a numeric field.
    Sum float64 `json:"sum,omitempty"`

    // Average is the average of a numeric field.
    Average float64 `json:"average,omitempty"`

    // Min is the minimum value.
    Min float64 `json:"min,omitempty"`

    // Max is the maximum value.
    Max float64 `json:"max,omitempty"`

    // Groups contains grouped aggregation results.
    Groups []AggregationGroup `json:"groups,omitempty"`
}

// AggregationGroup represents a group in aggregation results.
type AggregationGroup struct {
    // Key is the group key value.
    Key any `json:"key"`

    // Count is the count of items in the group.
    Count int64 `json:"count"`

    // Sum is the sum of values in the group.
    Sum float64 `json:"sum,omitempty"`

    // Average is the average of values in the group.
    Average float64 `json:"average,omitempty"`
}
```

### 10.3 Duration and External Request/Response

```go
// Duration represents a time duration with JSON support.
type Duration struct {
    // Duration is the duration value.
    time.Duration
}

// MarshalJSON serializes the duration to JSON.
func (d Duration) MarshalJSON() ([]byte, error) {
    return json.Marshal(d.String())
}

// UnmarshalJSON deserializes the duration from JSON.
func (d *Duration) UnmarshalJSON(data []byte) error {
    var v any
    if err := json.Unmarshal(data, &v); err != nil {
        return err
    }

    switch val := v.(type) {
    case string:
        p, err := time.ParseDuration(val)
        if err != nil {
            return err
        }
        d.Duration = p
    case float64:
        d.Duration = time.Duration(val)
    case int64:
        d.Duration = time.Duration(val)
    default:
        return nil
    }
    return nil
}

// ExternalRequest represents an HTTP request to an external service.
type ExternalRequest struct {
    // Method is the HTTP method.
    Method string

    // URL is the request URL.
    URL string

    // Headers are the request headers.
    Headers map[string]string

    // Body is the request body.
    Body []byte

    // Timeout is the request timeout.
    Timeout time.Duration

    // RetryConfig contains retry configuration.
    RetryConfig *RetryConfig
}

// ExternalResponse represents an HTTP response from an external service.
type ExternalResponse struct {
    // StatusCode is the HTTP status code.
    StatusCode int

    // Headers are the response headers.
    Headers map[string]string

    // Body is the response body.
    Body []byte

    // Error contains error information if the request failed.
    Error string
}

// RetryConfig contains configuration for retry logic.
type RetryConfig struct {
    // MaxAttempts is the maximum number of retry attempts.
    MaxAttempts int

    // InitialInterval is the initial retry interval.
    InitialInterval time.Duration

    // MaxInterval is the maximum retry interval.
    MaxInterval time.Duration

    // Multiplier is the backoff multiplier.
    Multiplier float64

    // Jitter adds randomness to retry intervals.
    Jitter bool
}

// NewDefaultRetryConfig returns a default retry configuration.
func NewDefaultRetryConfig() *RetryConfig {
    return &RetryConfig{
        MaxAttempts:     3,
        InitialInterval: 100 * time.Millisecond,
        MaxInterval:     30 * time.Second,
        Multiplier:      2.0,
        Jitter:          true,
    }
}
```

---

## Plugin System

### 11.1 Plugin Types

```go
// contracts/plugins/plugin.go
package plugins

import "context"

// PluginType defines the type/category of a plugin.
type PluginType string

const (
    // PluginTypeBus represents a message bus plugin.
    PluginTypeBus PluginType = "bus"
    // PluginTypeCache represents a cache plugin.
    PluginTypeCache PluginType = "cache"
    // PluginTypeDatabase represents a database plugin.
    PluginTypeDatabase PluginType = "database"
    // PluginTypeAuth represents an authentication plugin.
    PluginTypeAuth PluginType = "auth"
    // PluginTypeStorage represents a storage plugin.
    PluginTypeStorage PluginType = "storage"
    // PluginTypeLogger represents a logging plugin.
    PluginTypeLogger PluginType = "logger"
    // PluginTypeMetrics represents a metrics plugin.
    PluginTypeMetrics PluginType = "metrics"
    // PluginTypeTracer represents a tracing plugin.
    PluginTypeTracer PluginType = "tracer"
    // PluginTypeCustom represents a custom plugin type.
    PluginTypeCustom PluginType = "custom"
)

// PluginState represents the current state of a plugin.
type PluginState string

const (
    // PluginStateRegistered indicates the plugin is registered but not loaded.
    PluginStateRegistered PluginState = "registered"
    // PluginStateLoaded indicates the plugin is loaded.
    PluginStateLoaded PluginState = "loaded"
    // PluginStateInitialized indicates the plugin is initialized and ready.
    PluginStateInitialized PluginState = "initialized"
    // PluginStateRunning indicates the plugin is running.
    PluginStateRunning PluginState = "running"
    // PluginStateStopping indicates the plugin is stopping.
    PluginStateStopping PluginState = "stopping"
    // PluginStateStopped indicates the plugin has stopped.
    PluginStateStopped PluginState = "stopped"
    // PluginStateError indicates the plugin encountered an error.
    PluginStateError PluginState = "error"
)

// Plugin is the base interface that all plugins must implement.
// Following SRP: Single responsibility for plugin lifecycle.
type Plugin interface {
    // Metadata returns the plugin metadata.
    Metadata() *Metadata

    // Init initializes the plugin with the given configuration.
    Init(ctx context.Context, config map[string]any) error

    // Start starts the plugin.
    Start(ctx context.Context) error

    // Stop stops the plugin gracefully.
    Stop(ctx context.Context) error

    // Health returns the plugin health status.
    Health(ctx context.Context) (*HealthStatus, error)
}

// Metadata contains plugin metadata information.
type Metadata struct {
    // ID is the unique identifier of the plugin.
    ID string `json:"id"`

    // Name is the human-readable name of the plugin.
    Name string `json:"name"`

    // Version is the version of the plugin (semver).
    Version string `json:"version"`

    // Description describes what the plugin does.
    Description string `json:"description"`

    // Type is the type of the plugin.
    Type PluginType `json:"type"`

    // Author is the plugin author.
    Author string `json:"author,omitempty"`

    // License is the plugin license.
    License string `json:"license,omitempty"`

    // Homepage is the plugin homepage URL.
    Homepage string `json:"homepage,omitempty"`

    // Repository is the plugin repository URL.
    Repository string `json:"repository,omitempty"`

    // Dependencies are the plugin dependencies.
    Dependencies []Dependency `json:"dependencies,omitempty"`

    // Tags are the plugin tags for categorization.
    Tags []string `json:"tags,omitempty"`
}

// Dependency represents a plugin dependency.
type Dependency struct {
    // ID is the dependency plugin ID.
    ID string `json:"id"`

    // Version specifies the version constraint.
    Version string `json:"version"`

    // Optional indicates if the dependency is optional.
    Optional bool `json:"optional,omitempty"`
}

// HealthStatus represents the health status of a plugin.
type HealthStatus struct {
    // State is the current state of the plugin.
    State PluginState `json:"state"`

    // Healthy indicates if the plugin is healthy.
    Healthy bool `json:"healthy"`

    // Message contains a status message.
    Message string `json:"message,omitempty"`

    // Error contains error information if unhealthy.
    Error string `json:"error,omitempty"`

    // LastCheck is when the health was last checked.
    LastCheck string `json:"last_check,omitempty"`
}
```

### 11.2 Plugin Factory and Registry

```go
// PluginFactory creates plugin instances.
// Following Factory pattern for plugin instantiation.
type PluginFactory interface {
    // Create creates a new plugin instance.
    Create(ctx context.Context, config map[string]any) (Plugin, error)
}

// PluginRegistry manages plugin registration and discovery.
// Following Registry pattern for plugin lifecycle management.
type PluginRegistry interface {
    // Register registers a plugin factory.
    Register(pluginType PluginType, factory PluginFactory) error

    // Unregister unregisters a plugin factory.
    Unregister(pluginType PluginType) error

    // GetFactory returns the factory for a plugin type.
    GetFactory(pluginType PluginType) (PluginFactory, error)

    // List returns all registered plugin types.
    List() []PluginType

    // Create creates a new plugin instance by type.
    Create(ctx context.Context, pluginType PluginType, config map[string]any) (Plugin, error)
}

// PluginLoader loads plugins from various sources.
type PluginLoader interface {
    // Load loads plugins from the given source.
    Load(ctx context.Context, source string) ([]Plugin, error)

    // LoadFromFile loads plugins from a plugin manifest file.
    LoadFromFile(ctx context.Context, path string) ([]Plugin, error)

    // LoadFromDir loads plugins from a directory.
    LoadFromDir(ctx context.Context, dir string) ([]Plugin, error)

    // LoadFromRegistry loads plugins from a registry.
    LoadFromRegistry(ctx context.Context, registryURL string, names []string) ([]Plugin, error)
}

// PluginValidator validates plugin configurations and dependencies.
type PluginValidator interface {
    // Validate validates a plugin configuration.
    Validate(config map[string]any) error

    // ValidateDependencies checks if dependencies are satisfied.
    ValidateDependencies(deps []Dependency, available map[string]bool) error
}
```

### 11.3 Plugin Manifest

```go
// contracts/plugins/manifest.go
package plugins

// Manifest represents a plugin manifest file.
// Used for declarative plugin configuration.
type Manifest struct {
    // Version is the manifest version.
    Version string `json:"manifest_version"`

    // Plugins contains the list of plugins to load.
    Plugins []ManifestPlugin `json:"plugins"`
}

// ManifestPlugin represents a plugin in the manifest.
type ManifestPlugin struct {
    // ID is the unique identifier of the plugin.
    ID string `json:"id"`

    // Type is the type of the plugin.
    Type PluginType `json:"type"`

    // Source is the source of the plugin (file path, URL, or registry reference).
    Source string `json:"source"`

    // Version is the version constraint.
    Version string `json:"version,omitempty"`

    // Enabled indicates if the plugin is enabled.
    Enabled bool `json:"enabled,omitempty"`

    // Config contains the plugin configuration.
    Config map[string]any `json:"config,omitempty"`

    // Dependencies are the plugin dependencies.
    Dependencies []Dependency `json:"dependencies,omitempty"`

    // Priority is the plugin priority for loading order.
    Priority int `json:"priority,omitempty"`
}

// DefaultManifest returns a default manifest structure.
func DefaultManifest() *Manifest {
    return &Manifest{
        Version: "1.0.0",
        Plugins: []ManifestPlugin{},
    }
}

// Validate validates the manifest structure.
func (m *Manifest) Validate() error {
    if m.Version == "" {
        m.Version = "1.0.0"
    }

    for i, p := range m.Plugins {
        if p.ID == "" {
            return &ManifestError{
                Field:   "plugins",
                Index:   i,
                Message: "plugin ID is required",
            }
        }
        if p.Type == "" {
            return &ManifestError{
                Field:   "type",
                Index:   i,
                Message: "plugin type is required",
            }
        }
    }

    return nil
}

// ManifestError represents a manifest validation error.
type ManifestError struct {
    Field   string
    Index   int
    Message string
}

func (e *ManifestError) Error() string {
    if e.Index >= 0 {
        return "manifest: " + e.Field + "[" + string(rune(e.Index)) + "]: " + e.Message
    }
    return "manifest: " + e.Field + ": " + e.Message
}
```

### 11.4 Plugin YAML Manifest Format

```yaml
# plugin.yaml - Declarative plugin configuration
apiVersion: phenotype.io/v1
kind: Plugin

metadata:
  id: com.example.my-plugin
  name: My Example Plugin
  version: 1.2.3
  author: Example Inc
  license: Apache-2.0
  
spec:
  type: cache
  runtime: wasm  # native | grpc | wasm
  
  # For WASM plugins
  wasm:
    module: plugin.wasm
    memory: 128Mi
    timeout: 30s
    
  # For gRPC plugins  
  grpc:
    address: localhost:50051
    tls: true
    
  # For native plugins
  native:
    package: github.com/example/my-plugin
    symbol: NewPlugin
  
  # Configuration schema
  config:
    schema:
      type: object
      required: [endpoint]
      properties:
        endpoint:
          type: string
          format: uri
        timeout:
          type: integer
          default: 5000
        retries:
          type: integer
          default: 3
          maximum: 10
  
  # Dependencies
  dependencies:
    - id: phenotype.logger
      version: ">=1.0.0"
      optional: false
    - id: phenotype.metrics
      version: ">=1.0.0"
      optional: true
  
  # Capabilities
  capabilities:
    - read
    - write
    - streaming
  
  # Resource limits
  resources:
    cpu: "100m"
    memory: "256Mi"
    requestsPerSecond: 1000
```

---

## Contract-First Development

### 12.1 Contract-First Workflow

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                    Contract-First Development Workflow                            │
│                                                                                 │
│  1. Design Phase                                                                │
│     ├── Define use cases and user stories                                       │
│     ├── Identify commands and queries                                            │
│     ├── Design domain events                                                     │
│     └── Create OpenAPI/AsyncAPI drafts                                          │
│                                                                                 │
│  2. Review Phase                                                                │
│     ├── API design review with stakeholders                                     │
│     ├── Contract validation (Spectral)                                         │
│     ├── Breaking change analysis                                               │
│     └── Approval and versioning                                                │
│                                                                                 │
│  3. Generation Phase                                                            │
│     ├── Generate Go interfaces from OpenAPI                                     │
│     ├── Generate TypeScript clients                                            │
│     ├── Generate Rust types                                                    │
│     └── Generate Python models                                                 │
│                                                                                 │
│  4. Implementation Phase                                                        │
│     ├── Implement ports (interfaces)                                           │
│     ├── Create adapters                                                        │
│     ├── Implement domain logic                                                 │
│     └── Add business validation                                              │
│                                                                                 │
│  5. Testing Phase                                                               │
│     ├── Contract tests (Pact)                                                  │
│     ├── Property-based tests (Schemathesis)                                    │
│     ├── Unit tests for adapters                                                │
│     └── Integration tests                                                      │
│                                                                                 │
│  6. Deployment Phase                                                            │
│     ├── Deploy to staging                                                     │
│     ├── Consumer contract verification                                          │
│     ├── Performance testing                                                   │
│     └── Production deployment                                                 │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### 12.2 Contract Versioning

| Version Level | When to Bump | Example | Compatibility |
|---------------|--------------|---------|---------------|
| **Major** | Breaking changes | `1.x.x` → `2.0.0` | Breaking |
| **Minor** | New features | `1.1.x` → `1.2.0` | Backward compatible |
| **Patch** | Bug fixes | `1.1.1` → `1.1.2` | Fully compatible |

### 12.3 Breaking Change Detection

```go
// Breaking change detection in CI/CD
func DetectBreakingChanges(oldContract, newContract *OpenAPI) []BreakingChange {
    var changes []BreakingChange
    
    // Check removed operations
    for path, oldOps := range oldContract.Paths {
        newOps := newContract.Paths[path]
        for method := range oldOps {
            if _, exists := newOps[method]; !exists {
                changes = append(changes, BreakingChange{
                    Type:     "removed_operation",
                    Path:     path,
                    Method:   method,
                    Breaking: true,
                })
            }
        }
    }
    
    // Check required parameter additions
    // Check response schema changes
    // Check enum value removals
    
    return changes
}
```

---

## Code Generation

### 13.1 Generation Pipeline

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                      Code Generation Pipeline                                     │
│                                                                                 │
│  OpenAPI 3.1 Schema                                                             │
│       │                                                                         │
│       ▼                                                                         │
│  ┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐           │
│  │   Parse &       │───►│   Validate      │───►│   Transform     │           │
│  │   Normalize     │    │   (Spectral)    │    │   (Custom)      │           │
│  └─────────────────┘    └─────────────────┘    └─────────────────┘           │
│                                                         │                       │
│                                                         ▼                       │
│                              ┌─────────────────────────────────────┐           │
│                              │     Multi-Target Generation         │           │
│                              │                                     │           │
│                              │  ┌─────────┐ ┌─────────┐ ┌────────┐ │           │
│                              │  │   Go    │ │  Rust   │ │Python  │ │           │
│                              │  │ Generator│ │Generator│ │Generator│ │           │
│                              │  └────┬────┘ └────┬────┘ └───┬────┘ │           │
│                              │       │           │          │       │           │
│                              │       ▼           ▼          ▼       │           │
│                              │  ports.go    types.rs    models.py   │           │
│                              └─────────────────────────────────────┘           │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### 13.2 Generated Go Interfaces

```go
// Generated from OpenAPI by oapi-codegen
package generated

// CreateOrderRequest is generated from OpenAPI schema
type CreateOrderRequest struct {
    CustomerID string      `json:"customerId" validate:"required,uuid"`
    Items      []OrderItem `json:"items" validate:"required,min=1,dive"`
    Total      Money       `json:"total" validate:"required"`
}

// Validate implements the Validator interface
func (r CreateOrderRequest) Validate() error {
    return validate.Struct(r)
}

// Server interface generated from OpenAPI
 type OrdersServer interface {
    // CreateOrder operation
    CreateOrder(ctx context.Context, request CreateOrderRequest) (CreateOrderResponse, error)
    
    // GetOrder operation
    GetOrder(ctx context.Context, orderID string) (Order, error)
    
    // ListOrders operation
    ListOrders(ctx context.Context, params ListOrdersParams) (OrderList, error)
}
```

### 13.3 Generated TypeScript Types

```typescript
// Generated from OpenAPI by openapi-typescript
export interface CreateOrderRequest {
  customerId: string;
  items: OrderItem[];
  total: Money;
}

export interface Order {
  id: string;
  status: 'pending' | 'confirmed' | 'shipped' | 'delivered';
  customerId: string;
  items: OrderItem[];
  total: Money;
  createdAt: string;
}

// With Zod validation
export const CreateOrderRequestSchema = z.object({
  customerId: z.string().uuid(),
  items: z.array(OrderItemSchema).min(1),
  total: MoneySchema,
});

export type CreateOrderRequest = z.infer<typeof CreateOrderRequestSchema>;
```

---

## Testing Strategy

### 14.1 Testing Pyramid for Contracts

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                      Contract Testing Pyramid                                     │
│                                                                                 │
│                           ┌─────────────┐                                      │
│                           │   E2E       │  < 5%                                 │
│                           │  Tests      │  Full system                          │
│                           └──────┬──────┘                                      │
│                                  │                                              │
│                      ┌───────────▼───────────┐                                  │
│                      │   Integration         │  15-20%                          │
│                      │   Tests               │  Adapters + Ports                  │
│                      └───────────┬───────────┘                                  │
│                                  │                                              │
│          ┌───────────────────────▼───────────────────────┐                      │
│          │           Contract Tests                      │  20-25%              │
│          │  (Pact, Schemathesis, Provider Tests)        │                      │
│          └───────────────────────┬───────────────────────┘                      │
│                                  │                                              │
│  ┌───────────────────────────────▼───────────────────────────────┐              │
│  │                      Unit Tests                               │  50-60%      │
│  │  (Port implementations, domain logic, validation)             │              │
│  └───────────────────────────────────────────────────────────────┘              │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### 14.2 Contract Testing with Pact

```go
// Consumer test
func TestConsumer(t *testing.T) {
    pact := &dsl.Pact{
        Consumer: "order-service-client",
        Provider: "order-service",
    }
    defer pact.Teardown()
    
    pact.AddInteraction().
        Given("an order exists").
        UponReceiving("a request for the order").
        WithRequest(dsl.Request{
            Method: "GET",
            Path:   dsl.String("/orders/123"),
        }).
        WillRespondWith(dsl.Response{
            Status: 200,
            Body:   dsl.Match(&Order{ID: "123", Status: "pending"}),
        })
    
    err := pact.Verify(func() error {
        client := NewOrderServiceClient(pact.ServerURI)
        order, err := client.GetOrder("123")
        assert.NoError(t, err)
        assert.Equal(t, "123", order.ID)
        return nil
    })
    
    assert.NoError(t, err)
}

// Provider test
func TestProvider(t *testing.T) {
    pact := &dsl.Pact{
        Provider: "order-service",
    }
    
    _, err := pact.VerifyProvider(t, types.VerifyRequest{
        ProviderBaseURL: "http://localhost:8080",
        PactURLs:        []string{"./pacts/order-service-client-order-service.json"},
    })
    
    assert.NoError(t, err)
}
```

### 14.3 Property-Based Testing

```python
# Schemathesis for property-based API testing
import schemathesis

schema = schemathesis.from_path("openapi.yaml")

@schema.parametrize()
def test_api_contract(case):
    """All API responses must match OpenAPI schema."""
    response = case.call()
    case.validate_response(response)

@schema.given(data=st.data())
def test_validation_edge_cases(data):
    """Generated edge cases should be handled."""
    # Schemathesis generates edge cases automatically
    pass
```

---

## Security Model

### 15.1 Input Validation Layers

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                      Input Validation Defense in Depth                            │
│                                                                                 │
│  Layer 1: Network                                                             │
│  ├── Rate limiting                                                             │
│  ├── DDoS protection                                                           │
│  └── IP allowlisting                                                           │
│                                                                                 │
│  Layer 2: API Gateway                                                           │
│  ├── JWT/OAuth2 validation                                                     │
│  ├── JSON Schema validation (fast rejection)                                  │
│  └── Request size limits                                                       │
│                                                                                 │
│  Layer 3: Application                                                           │
│  ├── Port interface validation                                                 │
│  ├── Business rule validation                                                  │
│  └── Sanitization                                                              │
│                                                                                 │
│  Layer 4: Domain                                                                │
│  ├── Entity invariants                                                         │
│  ├── Cross-field validation                                                    │
│  └── Domain constraints                                                        │
│                                                                                 │
│  Layer 5: Data Access                                                           │
│  ├── Parameterized queries (SQL injection prevention)                         │
│  ├── ORM validation                                                            │
│  └── Field-level encryption                                                    │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### 15.2 Security Headers and Contracts

```yaml
# OpenAPI security definitions
components:
  securitySchemes:
    bearerAuth:
      type: http
      scheme: bearer
      bearerFormat: JWT
    
    apiKey:
      type: apiKey
      in: header
      name: X-API-Key
    
    oauth2:
      type: oauth2
      flows:
        authorizationCode:
          authorizationUrl: https://auth.phenotype.io/oauth/authorize
          tokenUrl: https://auth.phenotype.io/oauth/token
          scopes:
            read: Read access
            write: Write access
            admin: Admin access

security:
  - bearerAuth: []

paths:
  /admin/users:
    get:
      security:
        - bearerAuth: []
          oauth2: [admin]
```

---

## Performance Considerations

### 16.1 Port Interface Performance

| Operation | Latency Target | Throughput Target | Notes |
|-----------|----------------|-------------------|-------|
| **In-memory call** | < 1μs | > 1M ops/s | Direct interface call |
| **gRPC call** | < 5ms | > 10K ops/s | Same datacenter |
| **HTTP REST** | < 50ms | > 1K ops/s | External API |
| **Event publish** | < 10ms | > 5K ops/s | Async |
| **DB query (cached)** | < 1ms | > 50K ops/s | Redis cache |
| **DB query (uncached)** | < 100ms | > 1K ops/s | PostgreSQL |

### 16.2 Validation Performance Budget

| Validation Type | Max Time | Percentage of Request |
|-----------------|----------|----------------------|
| **JSON Schema (edge)** | 1ms | 2% |
| **Native type validation** | 5ms | 10% |
| **Business validation** | 20ms | 40% |
| **Total validation** | 26ms | 52% |
| **Business logic** | 24ms | 48% |
| **Total request** | 50ms | 100% |

### 16.3 Caching Strategy for Ports

```go
// Cached repository decorator
type CachedRepository[T any, ID any] struct {
    inner  Repository[T, ID]
    cache  Cache
    ttl    time.Duration
}

func (r *CachedRepository[T, ID]) GetByID(ctx context.Context, id ID) (T, error) {
    var zero T
    
    // Try cache
    key := fmt.Sprintf("repo:%T:%v", zero, id)
    cached, err := r.cache.Get(ctx, key)
    if err == nil {
        var entity T
        if err := json.Unmarshal(cached, &entity); err == nil {
            return entity, nil
        }
    }
    
    // Cache miss - fetch from inner
    entity, err := r.inner.GetByID(ctx, id)
    if err != nil {
        return zero, err
    }
    
    // Store in cache
    data, _ := json.Marshal(entity)
    r.cache.Set(ctx, key, data, &models.Duration{Duration: r.ttl})
    
    return entity, nil
}
```

---

## Implementation Guide

### 17.1 Creating a New Port

```go
// 1. Define the port interface
package ports

import "context"

// NotificationPort defines the interface for sending notifications.
type NotificationPort interface {
    // Send sends a notification to a recipient.
    Send(ctx context.Context, notification Notification) error
    
    // SendBatch sends multiple notifications efficiently.
    SendBatch(ctx context.Context, notifications []Notification) (BatchResult, error)
    
    // GetStatus retrieves the status of a sent notification.
    GetStatus(ctx context.Context, notificationID string) (NotificationStatus, error)
}

// 2. Define the input/output types
type Notification struct {
    ID        string
    Type      NotificationType // email, sms, push
    Recipient string
    Content   Content
    Metadata  map[string]string
    Priority  Priority
}

type NotificationType string

const (
    NotificationTypeEmail NotificationType = "email"
    NotificationTypeSMS   NotificationType = "sms"
    NotificationTypePush  NotificationType = "push"
)

type Priority int

const (
    PriorityLow Priority = iota
    PriorityNormal
    PriorityHigh
    PriorityCritical
)

type Content struct {
    Subject string
    Body    string
    HTML    string
    Data    map[string]any // Template variables
}

type NotificationStatus struct {
    ID        string
    State     State // pending, sent, delivered, failed
    SentAt    *time.Time
    DeliveredAt *time.Time
    Error     string
}

type State string

const (
    StatePending    State = "pending"
    StateSent       State = "sent"
    StateDelivered  State = "delivered"
    StateFailed     State = "failed"
)

type BatchResult struct {
    Total     int
    Succeeded int
    Failed    int
    Errors    map[string]error
}
```

### 17.2 Creating an Adapter

```go
// 3. Implement an adapter
package adapters

import (
    "context"
    "contracts/ports"
)

// EmailAdapter implements NotificationPort for email.
type EmailAdapter struct {
    client EmailClient
    config EmailConfig
}

// NewEmailAdapter creates a new email adapter.
func NewEmailAdapter(client EmailClient, config EmailConfig) *EmailAdapter {
    return &EmailAdapter{client: client, config: config}
}

// Send implements NotificationPort.
func (a *EmailAdapter) Send(ctx context.Context, notification ports.Notification) error {
    if notification.Type != ports.NotificationTypeEmail {
        return fmt.Errorf("email adapter only handles email notifications, got %s", notification.Type)
    }
    
    email := Email{
        To:      notification.Recipient,
        Subject: notification.Content.Subject,
        Body:    notification.Content.Body,
        HTML:    notification.Content.HTML,
    }
    
    return a.client.Send(ctx, email)
}

// SendBatch implements NotificationPort.
func (a *EmailAdapter) SendBatch(ctx context.Context, notifications []ports.Notification) (ports.BatchResult, error) {
    result := ports.BatchResult{
        Total:  len(notifications),
        Errors: make(map[string]error),
    }
    
    for _, n := range notifications {
        if err := a.Send(ctx, n); err != nil {
            result.Errors[n.ID] = err
            result.Failed++
        } else {
            result.Succeeded++
        }
    }
    
    return result, nil
}

// GetStatus implements NotificationPort.
func (a *EmailAdapter) GetStatus(ctx context.Context, notificationID string) (ports.NotificationStatus, error) {
    status, err := a.client.GetStatus(ctx, notificationID)
    if err != nil {
        return ports.NotificationStatus{}, err
    }
    
    return ports.NotificationStatus{
        ID:          notificationID,
        State:       mapEmailState(status.State),
        SentAt:      status.SentAt,
        DeliveredAt: status.DeliveredAt,
        Error:       status.Error,
    }, nil
}

// Manifest implements the Adapter interface.
func (a *EmailAdapter) Manifest() *ports.AdapterManifest {
    return &ports.AdapterManifest{
        Name:        "email",
        Version:     "1.0.0",
        Description: "Email notification adapter using SMTP",
        Provides:    []string{"NotificationPort"},
    }
}
```

### 17.3 Using Ports in Services

```go
// 4. Use the port in a service
package services

type OrderService struct {
    orderRepo      ports.Repository[Order, string]
    notification   ports.NotificationPort
    eventPublisher ports.EventPublisher
}

func NewOrderService(
    orderRepo ports.Repository[Order, string],
    notification ports.NotificationPort,
    eventPublisher ports.EventPublisher,
) *OrderService {
    return &OrderService{
        orderRepo:      orderRepo,
        notification:   notification,
        eventPublisher: eventPublisher,
    }
}

func (s *OrderService) ConfirmOrder(ctx context.Context, orderID string) error {
    // 1. Fetch order
    order, err := s.orderRepo.GetByID(ctx, orderID)
    if err != nil {
        return fmt.Errorf("fetch order: %w", err)
    }
    
    // 2. Update status
    order.Status = StatusConfirmed
    if _, err := s.orderRepo.Update(ctx, order); err != nil {
        return fmt.Errorf("update order: %w", err)
    }
    
    // 3. Send notification (via port)
    notification := ports.Notification{
        Type:      ports.NotificationTypeEmail,
        Recipient: order.CustomerEmail,
        Content: ports.Content{
            Subject: "Your order has been confirmed",
            Body:    fmt.Sprintf("Order %s is confirmed and will ship soon.", order.ID),
        },
    }
    
    if err := s.notification.Send(ctx, notification); err != nil {
        // Log but don't fail the operation
        slog.Error("failed to send notification", "error", err)
    }
    
    // 4. Publish event
    event := domain.NewOrderConfirmedEvent(order)
    if err := s.eventPublisher.Publish(ctx, "orders", []models.DomainEvent{event}); err != nil {
        return fmt.Errorf("publish event: %w", err)
    }
    
    return nil
}
```

---

## Migration Path

### 18.1 From Layered Architecture

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                    Migration: Layered → Hexagonal                                 │
│                                                                                 │
│  Before (Layered)                    After (Hexagonal)                         │
│                                                                                 │
│  ┌──────────────┐                    ┌──────────────┐                            │
│  │ Controller   │  ──────────────►  │ HTTP Adapter │                            │
│  │ (handles     │                    │ (driving)    │                            │
│  │  HTTP)       │                    └──────┬───────┘                            │
│  └──────┬───────┘                           │                                     │
│         │                                   ▼                                     │
│  ┌──────▼───────┐                    ┌──────────────┐                            │
│  │ Service      │  ──────────────►  │ UseCase Port │                            │
│  │ (business    │                    │ (inbound)    │                            │
│  │  logic)      │                    └──────┬───────┘                            │
│  └──────┬───────┘                           │                                     │
│         │                                   ▼                                     │
│  ┌──────▼───────┐                    ┌──────────────┐                            │
│  │ Repository   │  ──────────────►  │ Repository   │                            │
│  │ (data access)│                    │ Port (outbound)│                            │
│  └──────┬───────┘                    └──────┬───────┘                            │
│         │                                   │                                     │
│  ┌──────▼───────┐                    ┌──────▼───────┐                            │
│  │ Database     │  ──────────────►  │ Postgres     │                            │
│  │ (PostgreSQL) │                    │ Adapter      │                            │
│  └──────────────┘                    └──────────────┘                            │
│                                                                                 │
│  Key Changes:                                                                   │
│  1. Extract interfaces from concrete implementations                           │
│  2. Move HTTP handling to adapter layer                                         │
│  3. Define ports for all external interactions                                   │
│  4. Test with mock adapters                                                     │
│  5. Swap adapters without changing core logic                                   │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### 18.2 Step-by-Step Migration

| Step | Action | Risk | Duration |
|------|--------|------|----------|
| 1 | Identify all external dependencies | Low | 1-2 days |
| 2 | Define port interfaces | Low | 2-3 days |
| 3 | Create adapter implementations | Low | 3-5 days |
| 4 | Refactor services to use ports | Medium | 5-7 days |
| 5 | Add contract tests | Low | 2-3 days |
| 6 | Migrate database layer | Medium | 3-5 days |
| 7 | Add integration tests | Low | 2-3 days |
| 8 | Remove old code | Low | 1-2 days |

---

## Related Documents

### ADRs

1. [ADR-001: API Contract Format Selection](./adr/ADR-001-api-contract-format.md)
2. [ADR-002: Schema Validation Strategy](./adr/ADR-002-schema-validation.md)
3. [ADR-003: Plugin System Contract Design](./adr/ADR-003-plugin-system-contracts.md)

### Research

- [SOTA Research: API Contracts, Schema Validation, OpenAPI](./SOTA-RESEARCH.md)

### External References

- [OpenAPI 3.1 Specification](https://spec.openapis.org/oas/3.1.0)
- [JSON Schema 2020-12](https://json-schema.org/draft/2020-12/schema)
- [Hexagonal Architecture](https://alistair.cockburn.us/hexagonal-architecture/)
- [Domain-Driven Design](https://domainlanguage.com/ddd/reference/)

---

## Appendices

### Appendix A: Complete Port Reference

#### A.1 Inbound Ports

| Port | Purpose | Generic | Methods |
|------|---------|---------|---------|
| `UseCase` | Application operation | ✅ | Execute(ctx, in) (out, error) |
| `CommandHandler` | CQRS command | ✅ | Handle(ctx, cmd) (*CommandResult, error) |
| `QueryHandler` | CQRS query | ✅ | Handle(ctx, query) (result, error) |
| `EventHandler` | Domain event | ✅ | Handle(ctx, event) error |
| `InputPort` | Generic input | ✅ | Execute(ctx, in) (out, error) |
| `Validator` | Input validation | ✅ | Validate(in) error |
| `PreProcessor` | Pre-processing | ✅ | PreProcess(ctx, in) (in, error) |
| `PostProcessor` | Post-processing | ✅ | PostProcess(ctx, out) (out, error) |
| `Interceptor` | Middleware | ✅ | PreProcess + PostProcess |

#### A.2 Outbound Ports

| Port | Purpose | Generic | Methods |
|------|---------|---------|---------|
| `Repository` | CRUD operations | ✅ | Create, GetByID, Update, Delete, List, Exists, Count |
| `QueryRepository` | Read operations | ✅ | Find, FindOne, Aggregate |
| `EventStore` | Event sourcing | ❌ | Append, GetEvents, GetEventsSince, GetAllEvents |
| `EventPublisher` | Event distribution | ❌ | Publish, PublishAsync, Subscribe |
| `Cache` | Caching | ❌ | Get, Set, Delete, Exists, Clear, GetOrSet |
| `ExternalService` | HTTP clients | ❌ | Call, CallWithRetry |
| `SecretStore` | Secrets | ❌ | Get, Set, Delete, List |
| `MetricsCollector` | Metrics | ❌ | Counter, Gauge, Histogram, Summary |
| `Logger` | Logging | ❌ | Debug, Info, Warn, Error, Fatal |
| `ConfigProvider` | Configuration | ❌ | Get, Set, GetAll, Subscribe |
| `QueryExecutor` | Database | ❌ | Query, QueryRow, Exec, BeginTx, Ping, Stats |
| `Transaction` | DB transactions | ❌ | QueryExecutor + Commit, Rollback |
| `ConnectionPool` | Pool management | ❌ | ConfigurePool, GetPoolStats, HealthCheck, Close |
| `IndexManager` | DB indexes | ❌ | CreateIndex, DropIndex, ListIndexes, CreateIndexes |
| `MigrationExecutor` | Migrations | ❌ | Up, Down, Version, Pending |

### Appendix B: Glossary

| Term | Definition |
|------|------------|
| **Port** | An interface defining how the application core interacts with the outside world |
| **Adapter** | Implementation of a port for a specific technology |
| **Driving Adapter** | Adapter that drives the application (REST, CLI, etc.) |
| **Driven Adapter** | Adapter that the application drives (database, cache, etc.) |
| **Use Case** | Single application operation with a specific responsibility |
| **CQRS** | Command Query Responsibility Segregation - separate read/write models |
| **Event Sourcing** | Store events as the source of truth, derive state from events |
| **Domain Event** | Something that happened in the domain that other parts may need to know about |
| **DTO** | Data Transfer Object - data structure for cross-boundary communication |
| **Plugin** | Dynamically loaded module that extends functionality |

### Appendix C: Error Codes

| Code | Description | HTTP Status |
|------|-------------|-------------|
| `VALIDATION_ERROR` | Input validation failed | 400 |
| `NOT_FOUND` | Resource not found | 404 |
| `ALREADY_EXISTS` | Resource already exists | 409 |
| `UNAUTHORIZED` | Authentication required | 401 |
| `FORBIDDEN` | Insufficient permissions | 403 |
| `RATE_LIMITED` | Too many requests | 429 |
| `INTERNAL_ERROR` | Internal server error | 500 |
| `NOT_IMPLEMENTED` | Feature not implemented | 501 |
| `SERVICE_UNAVAILABLE` | Service temporarily unavailable | 503 |
| `CONTRACT_VIOLATION` | Contract validation failed | 400 |

---

*End of Specification*

*Version 1.0 | Last Updated: 2026-04-04*
