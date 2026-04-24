# AGENTS.md — contracts

## Project Overview

- **Name**: contracts (Software Contracts & Specifications)
- **Description**: Interface contracts, API specifications, and service level agreements for Phenotype ecosystem integration
- **Location**: `/Users/kooshapari/CodeProjects/Phenotype/repos/contracts`
- **Language Stack**: OpenAPI, Protocol Buffers, JSON Schema, CUE
- **Published**: Private (Phenotype org)

## Quick Start

```bash
# Navigate to project
cd /Users/kooshapari/CodeProjects/Phenotype/repos/contracts

# Validate contracts
make validate

# Generate code from specs
make generate

# Check for breaking changes
make breaking
```

## Architecture

### Contract System Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                     Contract Definitions                           │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐│
│  │   OpenAPI       │  │   Protocol      │  │   JSON Schema   ││
│  │   (REST APIs)   │  │   Buffers       │  │   (Validation)  ││
│  │                 │  │   (gRPC)        │  │                 ││
│  └────────┬────────┘  └────────┬────────┘  └────────┬────────┘│
└───────────┼───────────────────┼───────────────────┼───────────┘
            │                   │                   │
            └───────────────────┼───────────────────┘
                                │
┌───────────────────────────────▼───────────────────────────────┐
│                     Contract Registry                            │
│  ┌──────────────────────────────────────────────────────────┐│
│  │                   Version Management                        ││
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐  ││
│  │  │ Version  │  │ Breaking │  │ Deprecate│  │ Migration│  ││
│  │  │ Control  │  │ Change   │  │ Schedule │  │ Guides   │  ││
│  │  └──────────┘  └──────────┘  └──────────┘  └──────────┘  ││
│  └──────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────┘
            │                   │
            └───────────────────┼───────────────────┐
                                │                   │
┌───────────────────────────────▼───────┐  ┌───────▼───────────┐
│                     Code Generation      │  │  Documentation    │
│  ┌─────────────────┐  ┌─────────────────┐│  │  Generation      │
│  │   Go Types      │  │   Rust Types    ││  │                  │
│  │   (oapi-codegen)│  │   (prost)       ││  │  • API Docs      │
│  ├─────────────────┤  ├─────────────────┤│  │  • SDK Docs      │
│  │   TypeScript    │  │   Python        ││  │  • Changelogs    │
│  │   (openapi-ts)  │  │   (datamodel)   ││  │                  │
│  └─────────────────┘  └─────────────────┘│  └──────────────────┘
└─────────────────────────────────────────┘
```

### Contract Lifecycle

```
┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐
│  Draft   │───▶│  Review  │───▶│  Approve │───▶│ Publish  │───▶│ Version  │
│          │    │          │    │          │    │          │    │          │
└──────────┘    └──────────┘    └──────────┘    └──────────┘    └──────────┘
     │               │               │               │               │
     ▼               ▼               ▼               ▼               ▼
  Propose        RFC Phase      Final Review    Semantic        Deprecate
  Contract       Comments       Approval        Versioning      Schedule
```

## Quality Standards

### Contract Validation

- **OpenAPI**: Spectral linting
- **Protobuf**: `buf lint`
- **JSON Schema**: Draft 2020-12
- **Breaking Changes**: `buf breaking`

### Code Generation

```bash
# Generate Go code
make generate-go

# Generate TypeScript
make generate-ts

# Generate Rust
make generate-rust

# Generate Python
make generate-python
```

## Git Workflow

### Branch Naming

Format: `<type>/<contract>/<description>`

Types: `spec`, `breaking`, `docs`, `fix`

Examples:
- `spec/api/add-user-endpoints`
- `breaking/v2/remove-deprecated-fields`
- `fix/schema/correct-date-format`

### Commit Messages

Format: `<type>(<scope>): <description>`

Examples:
- `spec(api): define authentication endpoints`
- `breaking(proto): remove legacy message types`
- `fix(openapi): correct response schema`

## File Structure

```
contracts/
├── docs/                      # Documentation
│   ├── SPEC.md               # Master specification
│   └── reconcile.rules.yaml  # Validation rules
├── models/                    # Data models
│   ├── user.yaml
│   ├── organization.yaml
│   └── common/
├── ports/                     # Interface definitions
│   ├── http.yaml
│   └── grpc.yaml
├── plugins/                   # Plugin contracts
├── openapi/                   # OpenAPI specs
│   ├── api.yaml
│   └── components/
├── proto/                     # Protocol buffers
│   ├── phenotype/
│   │   ├── api/
│   │   └── common/
│   └── buf.yaml
├── schema/                    # JSON schemas
│   └── types/
├── cue/                       # CUE definitions
├── generated/                 # Generated code
├── Makefile
└── AGENTS.md                  # This file
```

## CLI Commands

```bash
# Validation
make validate               # Validate all specs
make validate-openapi       # OpenAPI only
make validate-proto         # Protobuf only
make validate-schema        # JSON Schema only

# Generation
make generate               # Generate all code
make generate-go            # Go code
make generate-ts            # TypeScript code

# Breaking changes
make breaking               # Check for breaking changes
make breaking-against=main  # Compare against branch

# Documentation
make docs                   # Generate documentation
make serve-docs             # Serve docs locally
```

## Configuration

### buf.yaml

```yaml
version: v1
name: buf.build/phenotype/contracts
deps:
  - buf.build/googleapis/googleapis
breaking:
  use:
    - FILE
lint:
  use:
    - DEFAULT
```

### OpenAPI Extension

```yaml
openapi: 3.1.0
info:
  title: Phenotype API
  version: 1.0.0
  x-api-id: phenotype-api-v1
  x-audience: external-public

paths:
  /users:
    get:
      operationId: listUsers
      x-sdk-method: users.list
      responses:
        200:
          description: Success
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/UserList'
```

## Troubleshooting

### Validation errors

```bash
# Run spectral on OpenAPI
spectral lint openapi/api.yaml

# Run buf lint
buf lint

# Check with more detail
buf lint --error-format=json
```

### Breaking change detection

```bash
# Compare against main
buf breaking --against '.git#branch=main'

# Ignore specific paths
buf breaking --against '.git#branch=main' \
  --exclude-path legacy/
```

### Code generation failures

```bash
# Regenerate all
make clean && make generate

# Check generator versions
oapi-codegen --version
buf --version
```

## Resources

- [OpenAPI Specification](https://spec.openapis.org/)
- [Protocol Buffers Guide](https://protobuf.dev/)
- [JSON Schema](https://json-schema.org/)
- [Buf Documentation](https://buf.build/docs/)
- [Phenotype Registry](https://github.com/KooshaPari/phenotype-registry)

## Agent Notes

**Critical Implementation Details:**
- All contracts must be versioned
- Breaking changes require major version bump
- Backward compatibility is mandatory
- Document all deprecated fields

**Known Gotchas:**
- Optional fields vs nullable differ across languages
- Timestamps need timezone specification
- Enum additions are backward compatible
- Field number reuse in protobuf is dangerous

**Testing Strategy:**
- Validate contracts in CI
- Test generated code compiles
- Verify backward compatibility
- Check documentation generation
