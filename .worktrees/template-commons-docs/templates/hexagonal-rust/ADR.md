# Architecture Decision Record: Hexagonal Architecture Template

## Status

**Accepted** - 2026-03-25

## Context

Phenotype ecosystem needs a consistent architectural pattern across all Rust crates to ensure:
- Clear separation of concerns
- Testability
- Maintainability
- Extensibility
- Technology swap capability

## Decision

Adopt **Hexagonal Architecture** (Ports and Adapters) as the standard pattern for all Phenotype Rust crates.

## Architecture Layers

### 1. Domain Layer (Innermost)
- **Location**: `src/domain/`
- **Dependencies**: NONE (pure Rust)
- **Contains**:
  - Entities with identity
  - Value objects (immutable types)
  - Aggregates (consistency boundaries)
  - Domain services
  - Domain events
  - Port interfaces (traits)

### 2. Application Layer (Middle)
- **Location**: `src/application/`
- **Dependencies**: Domain only
- **Contains**:
  - Use cases (orchestration)
  - DTOs (data transfer)
  - Command handlers
  - Query handlers

### 3. Adapters Layer (Outermost)
- **Location**: `src/adapters/`
- **Dependencies**: Domain ports, external libraries
- **Contains**:
  - Inbound: HTTP controllers, CLI
  - Outbound: DB, cache, external services

## Dependency Rule

```
Adapters ──implements──► Domain Ports
Application ──uses──────► Domain Ports
Application ──depends───► Domain
```

## Consequences

### Positive
- Domain logic is isolated and testable without infrastructure
- Easy to swap implementations (e.g., SQL to NoSQL)
- Clear boundaries enable parallel development
- Business rules are not tied to frameworks

### Negative
- More indirection (may increase cognitive overhead)
- Additional boilerplate for small projects
- Requires discipline to maintain boundaries

### Mitigations
- Provide templates for common patterns
- Use code generation where helpful
- Document anti-patterns to avoid

## Alternatives Considered

| Alternative | Why Not Chosen |
|-------------|----------------|
| Layered Architecture | Less explicit boundaries, harder to test |
| Clean Architecture | Similar but heavier (use cases mandatory) |
| Onion Architecture | Similar to hexagonal, different terminology |

## References

- [Alistair Cockburn - Hexagonal Architecture](https://alistair.cockburn.us/hexagonal-architecture/)
- [Ports and Adapters - ThoughtBot](https://thoughtbot.com/blog/hexagonal-rails)
- [Implementing Domain-Driven Design - Vaughn Vernon](https://www.amazon.com/dp/0321834577)
