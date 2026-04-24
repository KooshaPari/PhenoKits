# ADR-001: API Contract Format Selection for Phenotype Contracts

**Date**: 2026-04-04  
**Status**: Proposed  
**Deciders**: Phenotype Architecture Team  
**Related**: [SOTA Research](./SOTA-RESEARCH.md)

## Context

The Phenotype Contracts system requires a canonical format for defining API contracts between services. These contracts define:

1. **Port interfaces** for hexagonal architecture
2. **Data transfer objects** (DTOs) for cross-service communication
3. **Event schemas** for asynchronous messaging
4. **Plugin interfaces** for extensibility

The format must support:
- Code generation for Go, Rust, TypeScript, and Python
- Documentation generation
- Validation at build time and runtime
- Versioning and evolution
- Tool ecosystem integration

## Decision Drivers

| Driver | Weight | Description |
|--------|--------|-------------|
| **Ecosystem Maturity** | High | Large tool ecosystem, community support |
| **Code Generation** | High | Must generate idiomatic code for all target languages |
| **Validation** | High | Schema validation for runtime safety |
| **Documentation** | Medium | Automatic API documentation generation |
| **Human Readability** | Medium | Should be readable by humans for review |
| **Extensibility** | Medium | Support for custom extensions and annotations |

## Options Considered

### Option A: OpenAPI 3.1

**Description**: Industry standard REST API specification with JSON Schema 2020-12 alignment.

**Pros**:
- Industry standard with massive ecosystem (Redoc, Swagger UI, Postman, etc.)
- Full JSON Schema 2020-12 support
- Excellent code generation (oapi-codegen, openapi-generator, etc.)
- Strong tooling (Spectral for linting, Prism for mocking)
- Human-readable YAML/JSON format
- First-class webhook support in 3.1
- Bi-directional contract testing support

**Cons**:
- Primarily designed for REST/HTTP
- Can be verbose for simple contracts
- Event-driven contracts require AsyncAPI supplement

**Code Generation Quality**:

| Language | Tool | Quality |
|----------|------|---------|
| Go | oapi-codegen | Excellent |
| Rust | progenitor | Good |
| TypeScript | openapi-typescript | Excellent |
| Python | openapi-python-client | Good |

### Option B: Protocol Buffers (gRPC)

**Description**: Google's binary serialization format with strong typing and code generation.

**Pros**:
- Binary serialization (efficient)
- Strong backward compatibility
- Excellent Go support (protoc-gen-go)
- Native gRPC for internal services
- Buf ecosystem for schema management

**Cons**:
- Not human-readable (binary)
- Limited REST/HTTP support without grpc-gateway
- Smaller documentation tool ecosystem
- Requires proto compilation step
- Less suitable for public APIs

### Option C: AsyncAPI

**Description**: Specification for event-driven APIs and asynchronous messaging.

**Pros**:
- Purpose-built for events (Kafka, MQTT, WebSockets)
- Growing ecosystem
- Good code generation for event consumers
- Supports multiple protocols

**Cons**:
- Not suitable for REST/HTTP APIs
- Smaller ecosystem than OpenAPI
- Less mature tooling

### Option D: Smithy

**Description**: AWS's interface definition language for defining services and SDKs.

**Pros**:
- Clean separation of model and protocol
- Strong code generation
- AWS backing

**Cons**:
- AWS-centric ecosystem
- Smaller community than OpenAPI
- Learning curve

### Option E: TypeSpec

**Description**: Microsoft's TypeScript-like language for API definitions.

**Pros**:
- Familiar syntax for TypeScript developers
- Excellent type system
- Multiple emitters (OpenAPI, gRPC, etc.)

**Cons**:
- Relatively new (less mature)
- Smaller ecosystem
- Microsoft-centric

### Option F: GraphQL SDL

**Description**: GraphQL's schema definition language.

**Pros**:
- Excellent for GraphQL APIs
- Strong type system
- Introspection

**Cons**:
- GraphQL-specific
- Not suitable for REST/HTTP
- Over-fetching/under-fetching concerns

## Decision

**Adopt OpenAPI 3.1 as the primary contract format**, with AsyncAPI for event-driven contracts and Protocol Buffers for internal gRPC services.

### Rationale

1. **Ecosystem Dominance**: OpenAPI 3.1 is the de facto industry standard
2. **Tooling Breadth**: Widest range of tools for documentation, testing, and generation
3. **Human Readable**: YAML/JSON is reviewable in PRs
4. **Future Proof**: Active development with Moonwalk (4.0) on the horizon
5. **Multi-Language**: Best code generation across Go, Rust, TypeScript, Python

### Hybrid Approach

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    Phenotype Contract Format Strategy                         │
│                                                                             │
│  ┌─────────────────────┐  ┌─────────────────────┐  ┌─────────────────────┐ │
│  │    OpenAPI 3.1      │  │     AsyncAPI        │  │ Protocol Buffers    │ │
│  │                     │  │                     │  │                     │ │
│  │  • REST/HTTP APIs   │  │  • Event contracts  │  │  • Internal gRPC    │ │
│  │  • Port interfaces  │  │  • Kafka topics     │  │  • High-perf RPC    │ │
│  │  • Public APIs      │  │  • WebSocket events │  │  • Service mesh     │ │
│  └─────────────────────┘  └─────────────────────┘  └─────────────────────┘ │
│                                                                             │
│  Primary (80%)           Events (15%)           Internal (5%)            │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Implementation Plan

### Phase 1: Foundation (Weeks 1-2)
- [ ] Create OpenAPI 3.1 templates for common patterns
- [ ] Set up oapi-codegen for Go services
- [ ] Configure Spectral linting in CI/CD
- [ ] Deploy Redoc for documentation

### Phase 2: Tooling (Weeks 3-4)
- [ ] Add openapi-typescript for TypeScript clients
- [ ] Set up contract validation in CI/CD
- [ ] Create breaking change detection
- [ ] Document contract authoring guidelines

### Phase 3: Advanced (Weeks 5-8)
- [ ] Implement AsyncAPI for event contracts
- [ ] Add Pact for consumer-driven testing
- [ ] Create custom codegen for Phenotype patterns
- [ ] Set up contract registry (SwaggerHub or custom)

### Phase 4: Optimization (Ongoing)
- [ ] AI-assisted contract review
- [ ] Automated documentation improvement
- [ ] Performance optimization for validation
- [ ] Cross-language consistency checks

## Consequences

### Positive
- Industry-standard format ensures interoperability
- Rich ecosystem reduces custom tooling needs
- Clear documentation path with Redoc/Swagger UI
- Strong type generation across all target languages
- Easy adoption by external developers

### Negative
- Some verbosity compared to DSL approaches
- Need for multiple formats (OpenAPI + AsyncAPI + Protobuf)
- Learning curve for OpenAPI 3.1 specifics
- Tool fragmentation (many tools, varying quality)

## Migration Path

For existing contracts:

1. **Current Go interfaces** → Extract to OpenAPI 3.1
2. **Event structs** → Create AsyncAPI specifications
3. **Internal gRPC** → Keep Protobuf, add OpenAPI gateway

```bash
# Migration workflow
./scripts/extract-openapi.sh contracts/ports/inbound/ports.go
./scripts/validate-contracts.sh contracts/openapi/
./scripts/generate-types.sh contracts/openapi/ contracts/generated/
```

## References

- [OpenAPI 3.1 Specification](https://spec.openapis.org/oas/3.1.0)
- [SOTA Research: API Contract Landscape](./SOTA-RESEARCH.md#api-contract-landscape)
- [oapi-codegen](https://github.com/deepmap/oapi-codegen)
- [AsyncAPI Specification](https://www.asyncapi.com/docs/reference/specification/v2.6.0)

---

*This ADR will be updated as implementation progresses.*
