# ADR-001: Multi-Language Test Fixture Strategy

## Status

**Accepted** — 2026-04-04

## Context

The Phenotype ecosystem comprises multiple services written in different languages:

| Project | Language | Test Framework |
|---------|----------|----------------|
| phench | Python | pytest |
| heliosCLI | Rust | cargo test |
| heliosApp | TypeScript | jest/vitest |
| src/ | Python | pytest |
| tooling/ | Go | testing |

Each project currently maintains its own test fixtures, leading to:
- **Duplication:** Same user/order fixtures defined 4+ times
- **Drift:** Fixtures become inconsistent across projects
- **Maintenance burden:** Update required in N locations
- **Onboarding friction:** Different fixture patterns per project

## Decision

We will implement a **schema-first, code-generated fixture strategy** with the following architecture:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    MULTI-LANGUAGE FIXTURE ARCHITECTURE                       │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    SCHEMA DEFINITION LAYER                           │   │
│  │                                                                      │   │
│  │  fixtures/schemas/                                                   │   │
│  │  ├── user.schema.json           (JSON Schema)                     │   │
│  │  ├── order.schema.json                                               │   │
│  │  ├── organization.schema.json                                        │   │
│  │  └── common/                                                         │   │
│  │      ├── address.schema.json                                         │   │
│  │      └── money.schema.json                                           │   │
│  │                                                                      │   │
│  │  Schema Example (user.schema.json):                                  │   │
│  │  {                                                                    │   │
│  │    "$id": "https://phenotype.dev/schemas/user",                       │   │
│  │    "type": "object",                                                  │   │
│  │    "properties": {                                                     │   │
│  │      "id": {                                                           │   │
│  │        "type": "string",                                                │   │
│  │        "format": "uuid",                                                │   │
│  │        "faker": "uuid4"                                                 │   │
│  │      },                                                                 │   │
│  │      "email": {                                                        │   │
│  │        "type": "string",                                                │   │
│  │        "format": "email",                                               │   │
│  │        "faker": "email"                                                 │   │
│  │      },                                                                 │   │
│  │      "role": {                                                         │   │
│  │        "type": "string",                                                │   │
│  │        "enum": ["admin", "user", "guest"],                              │   │
│  │        "default": "user"                                                 │   │
│  │      },                                                                 │   │
│  │      "created_at": {                                                   │   │
│  │        "type": "string",                                                │   │
│  │        "format": "date-time",                                           │   │
│  │        "faker": "iso8601"                                               │   │
│  │      }                                                                   │   │
│  │    },                                                                  │   │
│  │    "required": ["id", "email"]                                         │   │
│  │  }                                                                      │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                    │                                         │
│                                    ▼                                         │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                   CODE GENERATION LAYER                              │   │
│  │                                                                      │   │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐              │   │
│  │  │   Python     │  │    Rust      │  │     JS       │              │   │
│  │  │  Generator   │  │  Generator   │  │  Generator   │              │   │
│  │  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘              │   │
│  │         │                  │                  │                    │   │
│  │         ▼                  ▼                  ▼                    │   │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐              │   │
│  │  │UserFactory   │  │UserFactory   │  │userFactory   │              │   │
│  │  │.py           │  │.rs           │  │.ts           │              │   │
│  │  └──────────────┘  └──────────────┘  └──────────────┘              │   │
│  │                                                                      │   │
│  │  Trigger: CI on schema change, or manual: `task fixtures:generate`   │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                    │                                         │
│                                    ▼                                         │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                   CONSUMER PROJECTS                                   │   │
│  │                                                                      │   │
│  │  phench/           heliosCLI/        heliosApp/        src/          │   │
│  │    │                   │                  │              │            │   │
│  │    ▼                   ▼                  ▼              ▼            │   │
│  │  imports            uses             imports         imports        │   │
│  │  tests.fixtures     test_fixtures    @phenotype/fixtures tests       │   │
│  │    │                   │                  │              │            │   │
│  │    ▼                   ▼                  ▼              ▼            │   │
│  │  UserFactory      UserFactory      userFactory()    UserFactory     │   │
│  │  .build()         ::build()       .build()         .build()         │   │
│  │                                                                      │   │
│  │  Same API pattern across all languages: build(), create(), batch()    │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Schema Extensions

We extend JSON Schema with phenotype-specific keywords:

| Keyword | Type | Description |
|---------|------|-------------|
| `faker` | string | Faker method to generate value |
| `faker_args` | object | Arguments passed to faker |
| `unique` | boolean | Ensure uniqueness across batch |
| `sequenced` | boolean | Generate sequential values |
| `computed` | string | Expression for derived values |
| `reference` | string | Reference to another schema |

### Language-Specific Generation

#### Python (factory_boy)

```python
# Generated from user.schema.json
import factory
from faker import Faker
from .models import User

faker = Faker()

class UserFactory(factory.Factory):
    class Meta:
        model = User
    
    id = factory.LazyFunction(lambda: faker.uuid4())
    email = factory.LazyFunction(lambda: faker.email())
    role = factory.Iterator(["admin", "user", "guest"])
    created_at = factory.LazyFunction(lambda: faker.iso8601())
    
    class Params:
        admin = factory.Trait(role="admin")
        verified = factory.Trait(email_verified=True)
```

#### Rust (fake + Dummy trait)

```rust
// Generated from user.schema.json
use fake::{Dummy, Fake, Faker};
use fake::faker::internet::en::FreeEmail;
use fake::faker::name::en::Name;
use uuid::Uuid;

#[derive(Debug, Dummy)]
pub struct User {
    #[dummy(faker = "Uuid::new_v4()")]
    pub id: Uuid,
    
    #[dummy(faker = "FreeEmail()")]
    pub email: String,
    
    #[dummy(faker = "(1..=3).fake()")]  // Maps to enum index
    pub role: UserRole,
    
    #[dummy(faker = "chrono::Utc::now()")]
    pub created_at: DateTime<Utc>,
}

impl UserFactory {
    pub fn build() -> User {
        Faker.fake()
    }
    
    pub fn admin() -> User {
        let mut user = Self::build();
        user.role = UserRole::Admin;
        user
    }
    
    pub fn batch(n: usize) -> Vec<User> {
        (0..n).map(|_| Self::build()).collect()
    }
}
```

#### JavaScript/TypeScript (faker-js)

```typescript
// Generated from user.schema.json
import { faker } from '@faker-js/faker';
import { v4 as uuidv4 } from 'uuid';

export interface User {
  id: string;
  email: string;
  role: 'admin' | 'user' | 'guest';
  createdAt: string;
}

export const userFactory = {
  build(overrides: Partial<User> = {}): User {
    return {
      id: uuidv4(),
      email: faker.internet.email(),
      role: faker.helpers.arrayElement(['admin', 'user', 'guest']),
      createdAt: new Date().toISOString(),
      ...overrides
    };
  },
  
  admin(overrides: Partial<User> = {}): User {
    return this.build({ role: 'admin', ...overrides });
  },
  
  batch(n: number, overrides: Partial<User> = {}): User[] {
    return Array.from({ length: n }, () => this.build(overrides));
  }
};
```

## Consequences

### Positive

1. **Single Source of Truth:** Schema changes propagate to all languages automatically
2. **Consistency:** Same fixture structure, naming, and behavior across projects
3. **Type Safety:** Generated code is type-safe in each language
4. **Reduced Maintenance:** Update one schema, regenerate everywhere
5. **Onboarding:** One fixture pattern to learn, applies to all projects

### Negative

1. **Build Dependency:** CI must run generator before tests
2. **Generator Maintenance:** Custom tooling to maintain
3. **Schema Complexity:** JSON Schema learning curve
4. **Language Gaps:** Some faker features differ across languages

### Mitigations

| Risk | Mitigation |
|------|------------|
| Generator bugs | Generated code is checked into git; review diff on PR |
| Schema drift | CI validation that schemas are valid JSON Schema |
| Language feature gaps | Document limitations per language; use manual factories for edge cases |
| Build time | Generator runs only when schemas change (cached) |

## Alternatives Considered

### Alternative 1: Shared YAML Fixtures

Static YAML files loaded by all languages.

**Rejected:**
- No type generation
- Limited faker integration
- Manual synchronization required

### Alternative 2: Proto/GraphQL Schema

Use existing IDL for fixtures.

**Rejected:**
- Too coupled to API schema
- Fixture needs often differ from API
- Complex faker integration

### Alternative 3: Runtime Translation Service

HTTP service that generates fixtures on demand.

**Rejected:**
- Network dependency for tests
- Latency impacts test speed
- Adds infrastructure complexity

## Implementation

### Phase 1: Schema + Python (Week 1)

- [ ] Define 5 core schemas (user, order, org, team, project)
- [ ] Implement Python generator
- [ ] Integrate with phench and src/

### Phase 2: Rust + JS (Week 2-3)

- [ ] Implement Rust generator
- [ ] Implement TypeScript generator
- [ ] Integrate with heliosCLI and heliosApp

### Phase 3: Validation + Docs (Week 4)

- [ ] CI validation for schema changes
- [ ] Documentation for adding new schemas
- [ ] Migration guide for existing fixtures

### Success Criteria

| Metric | Target |
|--------|--------|
| Schema coverage | 10+ entity types |
| Language coverage | Python, Rust, TypeScript |
| Duplication reduction | 50%+ fewer fixture definitions |
| Generation time | < 5s for all schemas |
| CI integration | Automatic on schema PRs |

## References

- [factory_boy documentation](https://factoryboy.readthedocs.io/)
- [fake-rs documentation](https://docs.rs/fake/)
- [faker-js documentation](https://fakerjs.dev/)
- [JSON Schema specification](https://json-schema.org/)

---

*ADR-001 — Multi-Language Test Fixture Strategy*
