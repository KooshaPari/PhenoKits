# ADR-003: Code Generation and Runtime Validation Strategy

**Date:** 2026-04-05

**Status:** ACCEPTED

**Author:** Phenotype Architecture Team

---

## Context

Following ADR-001 and ADR-002, we need to define how schemas translate into runtime code across our polyglot environment (TypeScript, Rust, Python, Go). This includes:

1. **Type definitions**: Structs, classes, interfaces
2. **Validation logic**: Runtime data validation
3. **Serialization**: Converting to/from wire formats
4. **Documentation**: Type hints, docstrings

### Requirements by Language

| Language | Primary Use | Type Generation | Validation | Serialization |
|----------|-------------|-----------------|------------|---------------|
| TypeScript | Frontend, Node.js | Native | Zod | JSON |
| Rust | Backend services | Serde derives | Limited | JSON, Protobuf |
| Python | ML, scripts, APIs | Pydantic | Pydantic | JSON, Avro |
| Go | Microservices | Structs | Manual/Tags | JSON, Protobuf |

---

## Decision

### TypeScript

**Strategy:** Zod + Type Inference

```typescript
// Schema definition (from TypeSpec or hand-written)
import { z } from 'zod';

export const UserSchema = z.object({
  id: z.string().uuid(),
  email: z.string().email(),
  name: z.string().min(1).max(100).optional(),
  role: z.enum(['admin', 'user', 'guest']).default('user'),
  metadata: z.record(z.string()).default({}),
  createdAt: z.string().datetime(),
  tags: z.array(z.string()).max(10)
});

// Type inference
type User = z.infer<typeof UserSchema>;

// Usage
const result = UserSchema.safeParse(unknownData);
if (result.success) {
  // result.data is typed as User
  console.log(result.data.email);
}
```

**Generation Approach:**
1. TypeSpec compiles to JSON Schema
2. `json-schema-to-zod` or custom generator creates Zod schemas
3. Type inference provides TypeScript types

### Rust

**Strategy:** Serde + Schemars

```rust
// Schema definition
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use validator::{Validate, ValidationError};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Validate)]
pub struct User {
    #[schemars(with = "String")]
    pub id: Uuid,
    
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,
    
    #[serde(default = "default_role")]
    pub role: Role,
    
    #[serde(default)]
    pub metadata: HashMap<String, String>,
    
    pub created_at: DateTime<Utc>,
    
    #[validate(length(max = 10))]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum Role {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "guest")]
    Guest,
}

fn default_role() -> Role {
    Role::User
}

// Usage
let user: User = serde_json::from_str(json_str)?;
user.validate()?;  // Runtime validation

// Generate JSON Schema
let schema = schema_for!(User);
```

**Generation Approach:**
1. TypeSpec compiles to JSON Schema
2. `typify` or custom generator creates Rust structs
3. Add Serde derives, schemars, validator annotations
4. Or: Define in Rust with `schemars`, export JSON Schema to registry

**For gRPC (Protobuf):**
```rust
// prost generates this from .proto
pub struct User {
    pub id: String,
    pub email: String,
    // ...
}
```

### Python

**Strategy:** Pydantic V2

```python
from pydantic import BaseModel, EmailStr, Field, ConfigDict
from datetime import datetime
from typing import Optional, Dict, List
from uuid import UUID

class Address(BaseModel):
    street: str
    city: str
    postal_code: str = Field(..., alias="postalCode")
    country: str

class User(BaseModel):
    model_config = ConfigDict(
        strict=True,
        populate_by_name=True,
    )
    
    id: UUID
    email: EmailStr
    name: Optional[str] = Field(None, min_length=1, max_length=100)
    role: str = Field(default="user", pattern="^(admin|user|guest)$")
    metadata: Dict[str, str] = Field(default_factory=dict)
    created_at: datetime = Field(..., alias="createdAt")
    tags: List[str] = Field(default_factory=list, max_length=10)
    address: Optional[Address] = None

# Usage
user = User.model_validate(data)  # From dict
user = User.model_validate_json(json_str)  # From JSON

# Generate JSON Schema
schema = User.model_json_schema()

# For Avro
from pydantic_avro import AvroBase
class UserAvro(AvroBase, User):
    pass

avro_schema = UserAvro.avro_schema()
```

**Generation Approach:**
1. TypeSpec compiles to JSON Schema
2. `datamodel-code-generator` creates Pydantic models
3. Or: Define in Pydantic, export JSON Schema to registry

### Go

**Strategy:** Structs + Validation Tags

```go
package users

import (
    "github.com/go-playground/validator/v10"
    "github.com/google/uuid"
    "time"
)

type Role string

const (
    RoleAdmin Role = "admin"
    RoleUser  Role = "user"
    RoleGuest Role = "guest"
)

type Address struct {
    Street     string `json:"street" validate:"required"`
    City       string `json:"city" validate:"required"`
    PostalCode string `json:"postalCode" validate:"required"`
    Country    string `json:"country" validate:"required"`
}

type User struct {
    ID        uuid.UUID         `json:"id" validate:"required,uuid"`
    Email     string            `json:"email" validate:"required,email"`
    Name      string            `json:"name,omitempty" validate:"omitempty,min=1,max=100"`
    Role      Role              `json:"role" validate:"required,oneof=admin user guest"`
    Metadata  map[string]string `json:"metadata"`
    CreatedAt time.Time         `json:"createdAt" validate:"required"`
    Tags      []string          `json:"tags" validate:"max=10"`
    Address   *Address          `json:"address,omitempty"`
}

// Usage
var validate = validator.New()

func (u *User) Validate() error {
    return validate.Struct(u)
}

// Generate JSON Schema (using go-jsonschema)
//go:generate gojsonschema -p users schema/user.json
```

**Generation Approach:**
1. TypeSpec compiles to JSON Schema
2. `go-jsonschema` or `quicktype` generates Go structs
3. Add validation tags manually or via generator
4. For Protobuf: `protoc-gen-go` generates structs

---

## Rationale

### Why Different Strategies Per Language?

Each language has its own ecosystem and idioms:

| Language | Best Practice | Rationale |
|----------|---------------|-----------|
| TypeScript | Zod | Type inference, excellent DX, 35k stars |
| Rust | Serde + Schemars | De-facto standard, zero-cost abstractions |
| Python | Pydantic V2 | 20k stars, FastAPI standard, Rust core |
| Go | Structs + Tags | Idiomatic, minimal, reflection-based |

### Why Code Generation?

1. **Single source of truth**: Schema changes propagate automatically
2. **Type safety**: Generated types match schema exactly
3. **DRY**: No manual duplication of schema in code
4. **Consistency**: Same validation across services

### Why Runtime Validation?

1. **Defense in depth**: Validate at boundaries
2. **Fail fast**: Catch errors early
3. **Clear errors**: Good validation libraries provide actionable messages
4. **Security**: Prevent injection/malformed data

---

## Implementation Details

### Generation Pipeline

```
TypeSpec Source (.tsp)
       │
       ▼
┌──────────────┐
│ TypeSpec     │
│ Compiler     │
└──────────────┘
       │
       ├──▶ JSON Schema (.json)
       │         │
       │         ├──▶ TypeScript: zod
       │         ├──▶ Rust: serde+schemars
       │         ├──▶ Python: pydantic
       │         └──▶ Go: structs+tags
       │
       └──▶ Protobuf (.proto)
                 │
                 ├──▶ Rust: prost
                 ├──▶ Go: protoc-gen-go
                 └──▶ Python: grpcio-tools
```

### Directory Structure

```
schemas/
├── src/
│   ├── users/
│   │   ├── User.tsp          # TypeSpec source
│   │   └── Address.tsp
│   └── events/
│       └── UserCreated.tsp
├── generated/
│   ├── json-schema/
│   │   └── users/
│   │       └── User.json
│   ├── protobuf/
│   │   └── users/
│   │       └── user.proto
│   ├── typescript/
│   │   └── users/
│   │       └── User.ts       # Zod schemas
│   ├── rust/
│   │   └── users/
│   │       └── user.rs       # Serde structs
│   ├── python/
│   │   └── users/
│   │       └── user.py       # Pydantic models
│   └── go/
│       └── users/
│           └── user.go       # Structs
└── registry/                 # Synced to schema registry
    └── users/
        └── User.jsonschema
```

### CI Integration

```yaml
# .github/workflows/codegen.yml
name: Code Generation

on:
  push:
    branches: [main]
    paths:
      - 'schemas/src/**'
  pull_request:
    paths:
      - 'schemas/src/**'

jobs:
  generate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Node
        uses: actions/setup-node@v4
        with:
          node-version: '20'
      
      - name: Setup Rust
        uses: dtolnay/rust-action@stable
      
      - name: Setup Python
        uses: actions/setup-python@v5
        with:
          python-version: '3.12'
      
      - name: Setup Go
        uses: actions/setup-go@v5
        with:
          go-version: '1.22'
      
      - name: Install TypeSpec
        run: npm install -g @typespec/compiler
      
      - name: Compile TypeSpec
        run: |
          cd schemas
          tsp compile . --emit @typespec/json-schema --output ./generated/json-schema
          tsp compile . --emit @typespec/protobuf --output ./generated/protobuf
      
      - name: Generate TypeScript
        run: |
          npm install -g json-schema-to-zod
          json-schema-to-zod -i ./schemas/generated/json-schema -o ./schemas/generated/typescript
      
      - name: Generate Rust
        run: |
          cargo install typify-cli
          typify ./schemas/generated/json-schema/*.json --output ./schemas/generated/rust/
      
      - name: Generate Python
        run: |
          pip install datamodel-code-generator
          datamodel-codegen --input ./schemas/generated/json-schema --output ./schemas/generated/python
      
      - name: Generate Go
        run: |
          go install github.com/atombender/go-jsonschema/cmd/gojsonschema@latest
          gojsonschema -p users ./schemas/generated/json-schema/*.json --output ./schemas/generated/go/
      
      - name: Verify Compilation
        run: |
          # TypeScript
          cd ./schemas/generated/typescript && npm install typescript && npx tsc --noEmit
          
          # Rust
          cd ./schemas/generated/rust && cargo check
          
          # Python
          cd ./schemas/generated/python && pip install pydantic && python -c "import user"
          
          # Go
          cd ./schemas/generated/go && go build ./...
      
      - name: Commit Changes
        if: github.ref == 'refs/heads/main'
        run: |
          git config --local user.email "action@github.com"
          git config --local user.name "GitHub Action"
          git add ./schemas/generated/
          git diff --staged --quiet || git commit -m "chore: regenerate code from schemas"
          git push
```

---

## Validation Strategies

### Input Validation

**TypeScript (Zod):**
```typescript
import { UserSchema } from './schemas';

app.post('/users', (req, res) => {
  const result = UserSchema.safeParse(req.body);
  if (!result.success) {
    return res.status(400).json({ errors: result.error.errors });
  }
  const user = result.data;  // Fully typed
  // ...
});
```

**Rust (Validator):**
```rust
use validator::Validate;

pub async fn create_user(Json(payload): Json<User>) -> Result<Json<User>, AppError> {
    payload.validate().map_err(|e| AppError::Validation(e))?;
    // ...
}
```

**Python (Pydantic):**
```python
from fastapi import FastAPI
from schemas import User

app = FastAPI()

@app.post("/users")
async def create_user(user: User) -> User:
    # Already validated by FastAPI/Pydantic
    return user
```

**Go (Validator):**
```go
func CreateUser(w http.ResponseWriter, r *http.Request) {
    var user User
    if err := json.NewDecoder(r.Body).Decode(&user); err != nil {
        http.Error(w, err.Error(), 400)
        return
    }
    
    if err := user.Validate(); err != nil {
        http.Error(w, err.Error(), 400)
        return
    }
    // ...
}
```

### Output Serialization

All languages use native serialization:
- TypeScript: `JSON.stringify()` with Zod
- Rust: `serde_json::to_string()`
- Python: `model_dump_json()` (Pydantic)
- Go: `json.Marshal()`

---

## Consequences

### Positive

- **Type safety**: Generated types prevent runtime errors
- **Consistency**: Same validation across all services
- **Developer experience**: IDE autocomplete, type hints
- **Single source**: Schema changes propagate automatically
- **Documentation**: Generated types include docstrings

### Negative

- **Build complexity**: Generation step adds build time
- **Git noise**: Generated files create PR noise
- **Merge conflicts**: Generated files can conflict
- **Debugging**: Generated code harder to debug
- **Dependency**: Build requires schema compiler

### Mitigations

| Risk | Mitigation |
|------|------------|
| Build time | Cache generation, incremental builds |
| Git noise | .gitattributes to hide diffs, or commit only sources |
| Merge conflicts | Regenerate in CI, don't manually edit generated files |
| Debugging | Source maps, good error messages |
| Dependency | Pin compiler versions, vendor if needed |

---

## Alternatives Considered

### A1: Hand-Written Types Only

**Pros:** Full control, no build complexity
**Cons:** Duplication, drift, more errors
**Rejected:** Doesn't meet consistency requirements

### A2: Single Language Generator

**Pros:** Simpler toolchain
**Cons:** Not all languages equally supported
**Rejected:** Each language needs native support

### A3: JSON Schema Direct

**Pros:** Direct use of canonical format
**Cons:** Poor DX compared to native types
**Rejected:** Native types provide better experience

---

## Validation

### Acceptance Criteria

- [ ] TypeScript Zod schemas generate and compile
- [ ] Rust Serde structs generate and compile
- [ ] Python Pydantic models generate and work
- [ ] Go structs generate and compile
- [ ] Validation works in all languages
- [ ] CI pipeline regenerates on schema changes
- [ ] Documentation updated

### Test Plan

1. **Generation tests**: Verify output for each language
2. **Compilation tests**: Ensure generated code compiles
3. **Validation tests**: Confirm validation rules match schema
4. **Integration tests**: End-to-end data flow

---

## References

- [ADR-001: Schema System Architecture](./ADR-001-schema-system-architecture.md)
- [ADR-002: Schema Registry and Versioning](./ADR-002-schema-registry-versioning.md)
- [Zod Documentation](https://zod.dev)
- [Pydantic Documentation](https://docs.pydantic.dev)
- [Serde Documentation](https://serde.rs)
- [Go Validator](https://github.com/go-playground/validator)
- [TypeSpec Emitters](https://typespec.io/docs)

---

**Decision Delta:**
- TypeScript: Zod with type inference
- Rust: Serde + Schemars + Validator
- Python: Pydantic V2
- Go: Structs with validation tags
- Automated generation from TypeSpec via JSON Schema

**Review Date:** 2026-07-05
