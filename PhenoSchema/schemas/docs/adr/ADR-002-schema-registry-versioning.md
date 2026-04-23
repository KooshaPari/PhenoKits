# ADR-002: Schema Registry and Versioning Strategy

**Date:** 2026-04-05

**Status:** ACCEPTED

**Author:** Phenotype Architecture Team

---

## Context

Following ADR-001's decision to adopt a polyglot schema architecture, we need a systematic approach to schema versioning, storage, and compatibility management. The schema registry is the central component that enables:

1. **Schema discovery**: Services can discover available schemas
2. **Version management**: Track schema evolution over time
3. **Compatibility enforcement**: Prevent breaking changes
4. **Ownership tracking**: Know who maintains each schema

### Requirements

| Requirement | Priority | Description |
|-------------|----------|-------------|
| Multi-format support | P0 | Store JSON Schema, Protobuf, Avro |
| Version management | P0 | Semantic versioning with history |
| Compatibility checking | P0 | BACKWARD/FORWARD/FULL modes |
| REST API | P0 | HTTP API for schema operations |
| Git integration | P1 | Sync with Git repository |
| Access control | P1 | Schema ownership and permissions |
| Audit logging | P2 | Track all schema changes |
| Search/discovery | P2 | Find schemas by name, tags, domain |

---

## Decision

### Registry Selection: Confluent Schema Registry

**Primary choice:** Confluent Schema Registry (open source)

**Alternative:** Apicurio Registry (if Confluent doesn't meet needs)

### Key Decisions

1. **Format Support**: JSON Schema (primary), Avro, Protobuf
2. **Versioning**: Semantic versioning (MAJOR.MINOR.PATCH)
3. **Subject Naming**: `{organization}.{domain}.{entity}-{format}`
4. **Compatibility Mode**: FULL by default (backward + forward)
5. **Storage**: Git repository as source of truth, registry as cache

### Subject Naming Convention

```
{organization}.{domain}.{entity}-{format}

Examples:
- phenotype.users.User-jsonschema
- phenotype.users.User-protobuf
- phenotype.events.UserCreated-avro
- phenotype.config.Database-cue
```

### Schema Structure

```json
{
  "schema": "{...}",
  "schemaType": "JSON",
  "metadata": {
    "owner": "team-platform",
    "domain": "users",
    "tags": ["public", "stable"],
    "description": "User entity schema",
    "source": "https://github.com/KooshaPari/schemas/tree/main/users/User.tsp"
  }
}
```

### Versioning Strategy

| Change Type | Version Impact | Compatibility |
|-------------|----------------|---------------|
| Add optional field | MINOR | BACKWARD, FORWARD |
| Add required field with default | MINOR | BACKWARD |
| Add required field without default | MAJOR | FORWARD only |
| Remove field | MAJOR | Breaking |
| Rename field | MAJOR | Breaking |
| Change type | MAJOR | Breaking |
| Change constraint (tighter) | MAJOR | Breaking |
| Change constraint (looser) | MINOR | BACKWARD |
| Documentation only | PATCH | No change |

### Compatibility Modes

| Mode | Old Schema Reads New Data | New Schema Reads Old Data | Use Case |
|------|---------------------------|---------------------------|----------|
| BACKWARD | Yes | No | Rolling updates (old consumers first) |
| FORWARD | No | Yes | Rolling updates (new consumers first) |
| FULL | Yes | Yes | Default, safest option |
| NONE | No | No | Emergency fixes, temporary |
| TRANSITIVE | Yes (all versions) | Yes (all versions) | Strictest, checks chain |

---

## Rationale

### Why Confluent Schema Registry?

1. **Industry standard**: Widely adopted, especially in Kafka ecosystem
2. **Multi-format support**: JSON Schema, Avro, Protobuf
3. **Mature ecosystem**: Client libraries, documentation, community
4. **Compatibility rules**: Well-tested compatibility checking
5. **Reference provider**: Can store references between schemas

### Why Semantic Versioning?

1. **Familiar**: Industry standard understood by developers
2. **Clear semantics**: Major = breaking, Minor = feature, Patch = fix
3. **Tooling support**: Existing tools understand semver
4. **Compatibility mapping**: Easy to map to compatibility modes

### Why Git as Source of Truth?

1. **Review process**: PR-based schema changes
2. **History**: Full audit trail in Git
3. **CI integration**: Validate before merge
4. **Branching**: Feature branch schema development
5. **Backup**: Git provides inherent backup

### Why FULL Compatibility by Default?

1. **Safety**: Prevents accidental breaking changes
2. **Flexibility**: Both old and new consumers work
3. **Rolling deployments**: No coordination required
4. **Rollback safety**: Can always roll back

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                     Schema Management Workflow                      │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  Developer                                                          │
│     │                                                               │
│     │  1. Edit TypeSpec/CUE                                         │
│     ▼                                                               │
│  ┌─────────────────┐                                                │
│  │  Git Repository │                                                │
│  │  (Source of Truth)│                                               │
│  │                 │                                                │
│  │  schemas/       │                                                │
│  │  ├── users/     │                                                │
│  │  │   └── User.tsp│                                               │
│  │  └── events/    │                                                │
│  │      └── UserCreated.tsp│                                        │
│  └────────┬────────┘                                                │
│           │                                                         │
│           │  2. Pull Request                                        │
│           ▼                                                         │
│  ┌─────────────────┐                                                │
│  │  CI Pipeline    │                                                │
│  │                 │                                                │
│  │  ┌───────────┐ │                                                │
│  │  │ Compile   │ │  TypeSpec → JSON Schema                        │
│  │  │ (tsp)     │ │  TypeSpec → Protobuf                         │
│  │  └───────────┘ │                                                │
│  │        │      │                                                │
│  │        ▼      │                                                │
│  │  ┌───────────┐ │                                                │
│  │  │ Check     │ │  Compare with registry                         │
│  │  │ Compat    │ │  Verify BACKWARD/FORWARD/FULL                  │
│  │  └───────────┘ │                                                │
│  │        │      │                                                │
│  │        ▼      │                                                │
│  │  ┌───────────┐ │                                                │
│  │  │ Generate  │ │  TypeScript, Rust, Python, Go                 │
│  │  │ Code      │ │  Verify generated code compiles                │
│  │  └───────────┘ │                                                │
│  │        │      │                                                │
│  │        ▼      │                                                │
│  │  ┌───────────┐ │                                                │
│  │  │ Register  │ │  Push to Schema Registry                       │
│  │  │ (on merge)│ │                                                │
│  │  └───────────┘ │                                                │
│  └────────┬────────┘                                                │
│           │                                                         │
│           ▼                                                         │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                  Schema Registry                               │   │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐   │   │
│  │  │ JSON Schema │  │ Protobuf    │  │ Avro                │   │   │
│  │  │ (subjects)  │  │ (subjects)  │  │ (subjects)          │   │   │
│  │  └─────────────┘  └─────────────┘  └─────────────────────┘   │   │
│  │                                                                │   │
│  │  Features:                                                     │   │
│  │  • Version lookup by subject + version                         │   │
│  │  • Latest version lookup                                       │   │
│  │  • Compatibility checking                                      │   │
│  │  • Schema references                                           │   │
│  └────────────────────────────────────────────────────────────────┘   │
│                                                                          │
│  4. Consumers use schemas from registry                                 │
│     • Validate incoming data                                            │
│     • Serialize/deserialize                                             │
│     • Generate client code                                              │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## Implementation

### Registry Deployment

**Option A: Confluent Schema Registry (Docker)**

```yaml
# docker-compose.yml
version: '3'
services:
  schema-registry:
    image: confluentinc/cp-schema-registry:7.6.0
    hostname: schema-registry
    ports:
      - "8081:8081"
    environment:
      SCHEMA_REGISTRY_HOST_NAME: schema-registry
      SCHEMA_REGISTRY_KAFKASTORE_BOOTSTRAP_SERVERS: kafka:9092
      SCHEMA_REGISTRY_AVRO_COMPATIBILITY_LEVEL: FULL
      SCHEMA_REGISTRY_JSON_SCHEMA_COMPATIBILITY_LEVEL: FULL
      SCHEMA_REGISTRY_PROTOBUF_COMPATIBILITY_LEVEL: FULL
```

**Option B: Apicurio Registry**

```yaml
services:
  apicurio:
    image: apicurio/apicurio-registry-mem:latest
    ports:
      - "8080:8080"
```

### API Usage Examples

**Register a Schema:**
```bash
curl -X POST http://localhost:8081/subjects/phenotype.users.User-jsonschema/versions \
  -H "Content-Type: application/vnd.schemaregistry.v1+json" \
  -d '{
    "schema": "{\"type\":\"object\",\"properties\":{...}}",
    "schemaType": "JSON"
  }'
```

**Get Latest Schema:**
```bash
curl http://localhost:8081/subjects/phenotype.users.User-jsonschema/versions/latest
```

**Check Compatibility:**
```bash
curl -X POST http://localhost:8081/compatibility/subjects/phenotype.users.User-jsonschema/versions/latest \
  -H "Content-Type: application/vnd.schemaregistry.v1+json" \
  -d '{
    "schema": "{...new schema...}",
    "schemaType": "JSON"
  }'
```

### CI Integration

```yaml
# .github/workflows/schemas.yml
name: Schema CI

on:
  pull_request:
    paths:
      - 'schemas/**'

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Node
        uses: actions/setup-node@v4
        with:
          node-version: '20'
      
      - name: Install TypeSpec
        run: npm install -g @typespec/compiler
      
      - name: Compile Schemas
        run: |
          cd schemas
          tsp compile . --emit @typespec/json-schema --output ./generated/json-schema
          tsp compile . --emit @typespec/protobuf --output ./generated/protobuf
      
      - name: Check Compatibility
        run: |
          for schema in ./generated/json-schema/**/*.json; do
            subject=$(basename $schema .json)
            curl -s -X POST \
              http://schema-registry:8081/compatibility/subjects/phenotype.${subject}-jsonschema/versions/latest \
              -d "{\"schema\": $(jq -Rs . < $schema)}"
          done
      
      - name: Generate Code
        run: |
          # TypeScript
          quicktype -s schema ./generated/json-schema/**/*.json -o ./generated/typescript/
          
          # Rust
          schemafy-rs ./generated/json-schema/ -o ./generated/rust/
          
          # Verify compilation
          cd ./generated/typescript && tsc --noEmit
          cd ./generated/rust && cargo check
```

---

## Consequences

### Positive

- **Centralized management**: Single place to find all schemas
- **Compatibility guarantees**: CI prevents breaking changes
- **Audit trail**: Git history + registry history
- **Discovery**: Services can discover schemas at runtime
- **Standardization**: Consistent naming and versioning

### Negative

- **Operational complexity**: Another service to run
- **Network dependency**: Services depend on registry availability
- **Caching complexity**: Need to cache schemas locally
- **Migration effort**: Existing schemas need registration

### Mitigations

| Risk | Mitigation |
|------|------------|
| Registry downtime | Local caching with TTL |
| Network issues | Fallback to embedded schemas |
| Performance | Cache compiled schemas |
| Security | Access control, audit logging |

---

## Alternatives Considered

### A1: Git-Only (No Registry)

**Pros:** Simple, no additional infrastructure
**Cons:** No runtime discovery, no compatibility checking, no versioning API
**Rejected:** Doesn't meet runtime requirements

### A2: Custom Registry

**Pros:** Perfect fit for needs
**Cons:** Development effort, maintenance burden
**Rejected:** Opportunity cost too high

### A3: Cloud Registry (AWS Glue, etc.)

**Pros:** Managed service
**Cons:** Vendor lock-in, cost, may not support all formats
**Rejected:** Prefer open source solution

---

## Validation

### Acceptance Criteria

- [ ] Schema registry deployed and accessible
- [ ] All schemas registered with proper naming
- [ ] CI pipeline checks compatibility
- [ ] Generated code compiles in all languages
- [ ] Documentation updated
- [ ] Runbook created for registry operations

### Success Metrics

| Metric | Target |
|--------|--------|
| Schema registration time | < 5 seconds |
| Schema lookup time | < 100ms |
| Compatibility check time | < 1 second |
| Registry uptime | > 99.9% |
| Breaking changes caught | 100% in CI |

---

## References

- [ADR-001: Schema System Architecture](./ADR-001-schema-system-architecture.md)
- [Confluent Schema Registry Documentation](https://docs.confluent.io/platform/current/schema-registry/)
- [Apicurio Registry](https://www.apicur.io/registry/)
- [Semantic Versioning](https://semver.org/)
- [Schema Registry API Reference](https://docs.confluent.io/platform/current/schema-registry/develop/api.html)

---

**Decision Delta:**
- Confluent Schema Registry as registry implementation
- Semantic versioning with FULL compatibility by default
- Git as source of truth, registry as runtime cache
- Subject naming: `{organization}.{domain}.{entity}-{format}`

**Review Date:** 2026-07-05
