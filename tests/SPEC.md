# SPEC.md — tests/

## Overview

Shared test utilities, fixtures, and testing infrastructure for the Phenotype ecosystem. Provides reusable testing components that standardize testing patterns across Python, JavaScript, Rust, and Go projects.

| | |
|---|---|
| **Project Type** | Testing Infrastructure |
| **Stack** | Python + JS + Rust + Go (multi-language) |
| **Priority** | P0 |
| **Status** | Active Development |

---

## ASCII Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        tests/ DIRECTORY STRUCTURE                           │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                   SCHEMA DEFINITION LAYER                            │   │
│  │  fixtures/schemas/                                                   │   │
│  │  ├── user.schema.json         (JSON Schema + faker hints)           │   │
│  │  ├── order.schema.json                                              │   │
│  │  ├── organization.schema.json                                       │   │
│  │  └── common/                                                        │   │
│  │      ├── address.schema.json                                        │   │
│  │      └── money.schema.json                                          │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                    │                                         │
│                                    ▼                                         │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                   CODE GENERATION LAYER                              │   │
│  │                                                                      │   │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐              │   │
│  │  │   Python     │  │    Rust      │  │  TypeScript  │              │   │
│  │  │  Generator   │  │  Generator   │  │  Generator   │              │   │
│  │  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘              │   │
│  │         │                  │                  │                      │   │
│  │         ▼                  ▼                  ▼                      │   │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐              │   │
│  │  │UserFactory   │  │UserFactory   │  │userFactory   │              │   │
│  │  │.py           │  │.rs           │  │.ts           │              │   │
│  │  └──────────────┘  └──────────────┘  └──────────────┘              │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                    │                                         │
│                                    ▼                                         │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                   CORE INFRASTRUCTURE LAYER                          │   │
│  │                                                                      │   │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐   │   │
│  │  │ Fixtures │ │  Mocks   │ │ Factories│ │  Utils   │ │ Assertions│   │   │
│  │  │  (Data)  │ │ (Stubs)  │ │ (Gen)    │ │ (Helpers)│ │ (Custom) │   │   │
│  │  └────┬─────┘ └────┬─────┘ └────┬─────┘ └────┬─────┘ └────┬─────┘   │   │
│  │       │            │            │            │            │          │   │
│  │       └────────────┴────────────┴────────────┴────────────┘          │   │
│  │                          │                                         │   │
│  │                          ▼                                         │   │
│  │  ┌─────────────────────────────────────────────────────────────┐  │   │
│  │  │              TEST RUNNER INTEGRATION                         │  │   │
│  │  │  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────┐ │  │   │
│  │  │  │   pytest    │ │    jest     │ │ cargo test  │ │ testify │ │  │   │
│  │  │  │  (Python)   │ │  (Node.js)  │ │   (Rust)    │ │  (Go)   │ │  │   │
│  │  │  └─────────────┘ └─────────────┘ └─────────────┘ └─────────┘ │  │   │
│  │  └─────────────────────────────────────────────────────────────┘  │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                    │                                         │
│                                    ▼                                         │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │              CONTAINERIZED TESTING LAYER                             │   │
│  │                                                                      │   │
│  │  ┌──────────────────────────────────────────────────────────────┐   │   │
│  │  │              testcontainers Integration                       │   │   │
│  │  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐        │   │   │
│  │  │  │Postgres  │ │  Redis   │ │  Kafka   │ │LocalStack│        │   │   │
│  │  │  │Container │ │Container │ │Container │ │Container │        │   │   │
│  │  │  └──────────┘ └──────────┘ └──────────┘ └──────────┘        │   │   │
│  │  └──────────────────────────────────────────────────────────────┘   │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                    │                                         │
│                                    ▼                                         │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │              CONTRACT TESTING LAYER                                  │   │
│  │                                                                      │   │
│  │  ┌──────────────────────────────────────────────────────────────┐   │   │
│  │  │                     Pact Integration                          │   │   │
│  │  │  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐   │   │   │
│  │  │  │   Consumer   │───▶│ Pact Broker  │◀───│   Provider   │   │   │   │
│  │  │  │    Tests     │    │              │    │Verification│   │   │   │
│  │  │  └──────────────┘    └──────────────┘    └──────────────┘   │   │   │
│  │  └──────────────────────────────────────────────────────────────┘   │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Tenets (unless you know better ones)

1. **Fast Feedback:**

Unit tests must complete in < 100ms. Slow tests are skipped, not fixed. Integration tests must complete in < 10s per container set.

2. **Determinism:**

All fixtures use seeded random generation. No test may depend on external state. Time must be mockable.

3. **Isolation:**

Each test must be independently runnable. Shared state must be explicitly declared. No test ordering dependencies.

4. **Realism:**

Mocks are for unit tests only. Integration tests use real dependencies. Production-like data in all non-unit tests.

---

## Components Table

| Component | Path | Purpose | Language | Status |
|-----------|------|---------|----------|--------|
| Schema Definitions | `fixtures/schemas/` | JSON Schema fixture definitions | JSON | Planned |
| Python Factories | `fixtures/python/` | Generated factory_boy classes | Python | Planned |
| Rust Factories | `fixtures/rust/` | Generated fake-rs implementations | Rust | Planned |
| JS Factories | `fixtures/javascript/` | Generated faker-js factories | TypeScript | Planned |
| Container Utils | `containers.py` | Testcontainers configuration | Python | Planned |
| Contract Tests | `contracts/` | Pact contract definitions | Multi | Planned |
| pytest Fixtures | `conftest.py` | Shared pytest fixtures | Python | Planned |
| Custom Assertions | `assertions/` | Domain-specific assertions | Multi | Planned |

---

## Schema Definition System

### Schema Structure

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    SCHEMA DEFINITION SYSTEM                                  │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                 SCHEMA FILE STRUCTURE                                │   │
│  │                                                                      │   │
│  │  user.schema.json                                                    │   │
│  │  {                                                                    │   │
│  │    "$id": "https://phenotype.dev/schemas/user",                     │   │
│  │    "$schema": "http://json-schema.org/draft-07/schema#",            │   │
│  │    "type": "object",                                                  │   │
│  │    "title": "User",                                                   │   │
│  │    "description": "User entity for testing fixtures",               │   │
│  │                                                                      │   │
│  │    "definitions": {                                                   │   │
│  │      "userRole": {                                                    │   │
│  │        "type": "string",                                              │   │
│  │        "enum": ["admin", "user", "guest"]                             │   │
│  │      }                                                                │   │
│  │    },                                                                 │   │
│  │                                                                      │   │
│  │    "properties": {                                                    │   │
│  │      "id": {                                                          │   │
│  │        "type": "string",                                              │   │
│  │        "format": "uuid",                                              │   │
│  │        "faker": "string.uuid",                                        │   │
│  │        "description": "Unique identifier"                             │   │
│  │      },                                                               │   │
│  │      "email": {                                                       │   │
│  │        "type": "string",                                              │   │
│  │        "format": "email",                                             │   │
│  │        "faker": "internet.email",                                     │   │
│  │        "description": "User email address"                            │   │
│  │      },                                                               │   │
│  │      "name": {                                                        │   │
│  │        "type": "string",                                              │   │
│  │        "faker": "person.fullName",                                      │   │
│  │        "minLength": 1,                                                │   │
│  │        "maxLength": 100                                               │   │
│  │      },                                                               │   │
│  │      "role": {                                                        │   │
│  │        "$ref": "#/definitions/userRole",                              │   │
│  │        "default": "user"                                              │   │
│  │      },                                                               │   │
│  │      "created_at": {                                                  │   │
│  │        "type": "string",                                              │   │
│  │        "format": "date-time",                                         │   │
│  │        "faker": "date.recent",                                        │   │
│  │        "description": "Account creation timestamp"                    │   │
│  │      },                                                               │   │
│  │      "is_active": {                                                   │   │
│  │        "type": "boolean",                                             │   │
│  │        "default": true                                              │   │
│  │      }                                                                │   │
│  │    },                                                                 │   │
│  │                                                                      │   │
│  │    "required": ["id", "email"],                                       │   │
│  │                                                                      │   │
│  │    "traits": {                                                        │   │
│  │      "admin": {                                                       │   │
│  │        "role": "admin"                                                │   │
│  │      },                                                               │   │
│  │      "inactive": {                                                    │   │
│  │        "is_active": false                                             │   │
│  │      },                                                               │   │
│  │      "verified": {                                                    │   │
│  │        "email_verified_at": {                                        │   │
│  │          "faker": "date.past"                                         │   │
│  │        }                                                              │   │
│  │      }                                                                │   │
│  │    }                                                                  │   │
│  │  }                                                                    │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    PHENOTYPE EXTENSIONS                                │   │
│  │                                                                      │   │
│  │  Extension Keywords:                                                 │   │
│  │                                                                      │   │
│  │  ┌─────────────┬────────────────────────────────────────────────────┐│   │
│  │  │ Keyword     │ Description                                        ││   │
│  │  ├─────────────┼────────────────────────────────────────────────────┤│   │
│  │  │ faker       │ Faker method for value generation                   ││   │
│  │  │ faker_args  │ Arguments passed to faker method                    ││   │
│  │  │ unique      │ Ensure uniqueness across batch                      ││   │
│  │  │ sequenced   │ Generate sequential values (1, 2, 3...)            ││   │
│  │  │ computed    │ Expression for derived values                       ││   │
│  │  │ reference   │ Reference to another schema                         ││   │
│  │  │ traits      │ Named attribute sets for variations                 ││   │
│  │  │ builder     │ Builder pattern method definitions                  ││   │
│  │  └─────────────┴────────────────────────────────────────────────────┘│   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Core Entity Schemas

| Entity | File | Relationships | Priority |
|--------|------|---------------|----------|
| User | `user.schema.json` | has_many: organizations, orders | P0 |
| Organization | `organization.schema.json` | has_many: users, teams | P0 |
| Team | `team.schema.json` | belongs_to: organization, has_many: users | P0 |
| Project | `project.schema.json` | belongs_to: organization | P0 |
| Order | `order.schema.json` | belongs_to: user, has_many: line_items | P1 |
| LineItem | `line_item.schema.json` | belongs_to: order | P1 |
| Address | `address.schema.json` | embedded in: user, order | P1 |
| Money | `money.schema.json` | embedded in: line_item | P1 |

---

## Code Generation Pipeline

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    CODE GENERATION PIPELINE                                   │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  Input: Schema Change                                                        │
│       │                                                                      │
│       ▼                                                                      │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │              1. SCHEMA VALIDATION                                   │   │
│  │                                                                      │   │
│  │  - Validate JSON Schema syntax                                      │   │
│  │  - Check faker method exists                                        │   │
│  │  - Validate trait references                                        │   │
│  │  - Check circular references                                        │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│       │                                                                      │
│       ▼                                                                      │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │              2. LANGUAGE GENERATION                                 │   │
│  │                                                                      │   │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐                │   │
│  │  │   Python    │  │    Rust     │  │     JS      │                │   │
│  │  │ Generator   │  │  Generator  │  │  Generator  │                │   │
│  │  └─────────────┘  └─────────────┘  └─────────────┘                │   │
│  │                                                                      │   │
│  │  Parallel execution per language                                     │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│       │                                                                      │
│       ▼                                                                      │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │              3. OUTPUT GENERATION                                   │   │
│  │                                                                      │   │
│  │  fixtures/python/user_factory.py                                    │   │
│  │  fixtures/rust/user_factory.rs                                        │   │
│  │  fixtures/javascript/user_factory.ts                                  │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│       │                                                                      │
│       ▼                                                                      │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │              4. POST-GENERATION                                     │   │
│  │                                                                      │   │
│  │  - Format with language-specific formatter                           │   │
│  │  - Run type checker (mypy, rustc, tsc)                              │   │
│  │  - Generate tests for generated code                                │   │
│  │  - Update documentation                                              │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│       │                                                                      │
│       ▼                                                                      │
│  Output: Generated Factories                                                │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Python Generation Output

```python
# fixtures/python/user_factory.py
# Auto-generated from user.schema.json
# DO NOT EDIT MANUALLY

import factory
from faker import Faker
from datetime import datetime
from typing import Optional, List
from .models import User, UserRole

faker = Faker()
Faker.seed(12345)  # Deterministic generation

class UserFactory(factory.Factory):
    """
    Factory for generating User test fixtures.
    
    Generated from: user.schema.json
    Schema version: 1.0.0
    Generated at: 2026-04-04T10:30:00Z
    """
    
    class Meta:
        model = User
        sqlalchemy_session_persistence = "commit"
    
    # Core attributes
    id: str = factory.LazyFunction(lambda: faker.uuid4())
    email: str = factory.LazyFunction(lambda: faker.email())
    name: str = factory.LazyFunction(lambda: faker.name())
    role: UserRole = factory.LazyFunction(lambda: UserRole.USER)
    created_at: datetime = factory.LazyFunction(lambda: faker.date_time())
    is_active: bool = factory.LazyFunction(lambda: True)
    
    # Traits for variations
    class Params:
        admin = factory.Trait(
            role=UserRole.ADMIN,
            name=factory.LazyFunction(lambda: f"Admin {faker.last_name()}")
        )
        
        inactive = factory.Trait(
            is_active=False,
            email=faker.email(domain="inactive.example.com")
        )
        
        verified = factory.Trait(
            email_verified_at=factory.LazyFunction(lambda: faker.past_date())
        )
    
    # Builder methods
    @classmethod
    def with_organization(cls, **kwargs) -> "UserFactory":
        """Create user with associated organization."""
        return cls(organization=factory.SubFactory("tests.fixtures.OrganizationFactory"), **kwargs)
    
    @classmethod
    def with_orders(cls, count: int = 3, **kwargs) -> "UserFactory":
        """Create user with N orders."""
        return cls(
            orders=factory.RelatedFactoryList(
                "tests.fixtures.OrderFactory",
                factory_related_name="user",
                size=count
            ),
            **kwargs
        )

# Convenience functions
def build_user(**kwargs) -> User:
    """Build a single user instance (not persisted)."""
    return UserFactory.build(**kwargs)

def create_user(**kwargs) -> User:
    """Create and persist a single user."""
    return UserFactory.create(**kwargs)

def build_users(count: int, **kwargs) -> List[User]:
    """Build N user instances."""
    return UserFactory.build_batch(count, **kwargs)

def create_users(count: int, **kwargs) -> List[User]:
    """Create and persist N users."""
    return UserFactory.create_batch(count, **kwargs)
```

### Rust Generation Output

```rust
// fixtures/rust/user_factory.rs
// Auto-generated from user.schema.json
// DO NOT EDIT MANUALLY

use fake::{Dummy, Fake, Faker, StringFaker};
use fake::faker::internet::en::FreeEmail;
use fake::faker::name::en::Name;
use fake::faker::chrono::en::DateTimeBefore;
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};

/// User entity for testing fixtures.
/// 
/// Generated from: user.schema.json
/// Schema version: 1.0.0
/// Generated at: 2026-04-04T10:30:00Z
#[derive(Debug, Clone, Dummy)]
pub struct User {
    #[dummy(faker = "Uuid::new_v4()")]
    pub id: Uuid,
    
    #[dummy(faker = "FreeEmail()")]
    pub email: String,
    
    #[dummy(faker = "Name()")]
    pub name: String,
    
    #[dummy(faker = "(0..=2).fake::<usize>().into()")]
    pub role: UserRole,
    
    #[dummy(faker = "DateTimeBefore(Utc::now())")]
    pub created_at: DateTime<Utc>,
    
    #[dummy(faker = "true")]
    pub is_active: bool,
}

#[derive(Debug, Clone)]
pub enum UserRole {
    Admin,
    User,
    Guest,
}

impl From<usize> for UserRole {
    fn from(value: usize) -> Self {
        match value {
            0 => UserRole::Admin,
            1 => UserRole::User,
            _ => UserRole::Guest,
        }
    }
}

/// Factory for generating User test fixtures.
pub struct UserFactory;

impl UserFactory {
    /// Build a single user instance.
    pub fn build() -> User {
        Faker.fake()
    }
    
    /// Build a single user with trait overrides.
    pub fn admin() -> User {
        let mut user = Self::build();
        user.role = UserRole::Admin;
        user.name = format!("Admin {}", user.name.split_whitespace().last().unwrap_or("User"));
        user
    }
    
    /// Build a single inactive user.
    pub fn inactive() -> User {
        let mut user = Self::build();
        user.is_active = false;
        user.email = format!("inactive-{}", user.email);
        user
    }
    
    /// Build a single verified user.
    pub fn verified() -> User {
        let mut user = Self::build();
        user.email_verified_at = Some(Faker.fake::<DateTime<Utc>>() - Duration::days(1));
        user
    }
    
    /// Build N user instances.
    pub fn batch(n: usize) -> Vec<User> {
        (0..n).map(|_| Self::build()).collect()
    }
    
    /// Build with custom overrides.
    pub fn build_with<F>(f: F) -> User 
    where 
        F: FnOnce(&mut User)
    {
        let mut user = Self::build();
        f(&mut user);
        user
    }
}

/// Builder pattern for complex user construction.
pub struct UserBuilder {
    user: User,
}

impl UserBuilder {
    pub fn new() -> Self {
        Self {
            user: UserFactory::build()
        }
    }
    
    pub fn with_email(mut self, email: &str) -> Self {
        self.user.email = email.to_string();
        self
    }
    
    pub fn with_role(mut self, role: UserRole) -> Self {
        self.user.role = role;
        self
    }
    
    pub fn with_organization(mut self, org: Organization) -> Self {
        self.user.organization_id = Some(org.id);
        self
    }
    
    pub fn build(self) -> User {
        self.user
    }
}
```

### TypeScript/JavaScript Generation Output

```typescript
// fixtures/javascript/user_factory.ts
// Auto-generated from user.schema.json
// DO NOT EDIT MANUALLY

import { faker } from '@faker-js/faker';
import { v4 as uuidv4 } from 'uuid';

export type UserRole = 'admin' | 'user' | 'guest';

export interface User {
  id: string;
  email: string;
  name: string;
  role: UserRole;
  createdAt: string;
  isActive: boolean;
  emailVerifiedAt?: string;
  organizationId?: string;
}

export interface UserBuildOptions {
  overrides?: Partial<User>;
  trait?: 'admin' | 'inactive' | 'verified';
  withOrganization?: boolean;
  withOrders?: number;
}

/**
 * Factory for generating User test fixtures.
 * 
 * Generated from: user.schema.json
 * Schema version: 1.0.0
 * Generated at: 2026-04-04T10:30:00Z
 */
export const userFactory = {
  /**
   * Build a single user instance.
   */
  build(options: UserBuildOptions = {}): User {
    const base: User = {
      id: uuidv4(),
      email: faker.internet.email(),
      name: faker.person.fullName(),
      role: faker.helpers.arrayElement<UserRole>(['admin', 'user', 'guest']),
      createdAt: faker.date.recent().toISOString(),
      isActive: true,
    };

    let user = { ...base, ...options.overrides };

    // Apply trait overrides
    if (options.trait) {
      user = this.applyTrait(user, options.trait);
    }

    return user;
  },

  /**
   * Build an admin user.
   */
  admin(overrides: Partial<User> = {}): User {
    return this.build({
      overrides: { role: 'admin', ...overrides },
    });
  },

  /**
   * Build an inactive user.
   */
  inactive(overrides: Partial<User> = {}): User {
    return this.build({
      overrides: { isActive: false, ...overrides },
    });
  },

  /**
   * Build a verified user.
   */
  verified(overrides: Partial<User> = {}): User {
    return this.build({
      overrides: {
        emailVerifiedAt: faker.date.past().toISOString(),
        ...overrides,
      },
    });
  },

  /**
   * Build N user instances.
   */
  batch(n: number, options: Omit<UserBuildOptions, 'trait'> & { traits?: Array<'admin' | 'inactive' | 'verified'> } = {}): User[] {
    const { traits, ...rest } = options;
    return Array.from({ length: n }, (_, i) => {
      const trait = traits?.[i];
      return this.build({ ...rest, trait });
    });
  },

  /**
   * Apply trait overrides.
   */
  applyTrait(user: User, trait: 'admin' | 'inactive' | 'verified'): User {
    switch (trait) {
      case 'admin':
        return {
          ...user,
          role: 'admin',
          name: `Admin ${user.name.split(' ').pop() || 'User'}`,
        };
      case 'inactive':
        return {
          ...user,
          isActive: false,
          email: `inactive-${user.email}`,
        };
      case 'verified':
        return {
          ...user,
          emailVerifiedAt: faker.date.past().toISOString(),
        };
      default:
        return user;
    }
  },

  /**
   * Create builder for complex user construction.
   */
  builder(): UserBuilder {
    return new UserBuilder();
  },
};

/**
 * Builder class for complex user construction.
 */
export class UserBuilder {
  private user: User;

  constructor() {
    this.user = userFactory.build();
  }

  withEmail(email: string): this {
    this.user.email = email;
    return this;
  }

  withRole(role: UserRole): this {
    this.user.role = role;
    return this;
  }

  withName(name: string): this {
    this.user.name = name;
    return this;
  }

  withOrganization(orgId: string): this {
    this.user.organizationId = orgId;
    return this;
  }

  withOrders(count: number): this {
    // Would integrate with order factory
    return this;
  }

  build(): User {
    return { ...this.user };
  }
}

// Convenience exports
export const buildUser = userFactory.build.bind(userFactory);
export const buildAdmin = userFactory.admin.bind(userFactory);
export const buildBatch = userFactory.batch.bind(userFactory);
```

---

## Data Models

### Base Fixture Interface

```python
# Core fixture protocol
typing.Protocol
class FixtureProtocol(typing.Protocol):
    """Protocol for all test fixtures."""
    
    @classmethod
    def build(cls, **overrides) -> typing.Any:
        """Build instance without persistence."""
        ...
    
    @classmethod
    def create(cls, **overrides) -> typing.Any:
        """Build and persist instance."""
        ...
    
    @classmethod
    def build_batch(cls, count: int, **overrides) -> typing.List[typing.Any]:
        """Build multiple instances."""
        ...
    
    @classmethod
    def create_batch(cls, count: int, **overrides) -> typing.List[typing.Any]:
        """Build and persist multiple instances."""
        ...
```

### Fixture Scope Model

```python
class FixtureScope:
    """Defines fixture lifecycle scope."""
    
    FUNCTION = "function"    # New per test function
    CLASS = "class"          # New per test class  
    MODULE = "module"        # New per test module
    PACKAGE = "package"      # New per test package
    SESSION = "session"    # Once per test session

class FixtureConfig:
    """Configuration for fixture generation."""
    
    scope: FixtureScope = FixtureScope.FUNCTION
    autouse: bool = False
    params: typing.Optional[typing.List[typing.Any]] = None
    ids: typing.Optional[typing.Callable] = None
    name: typing.Optional[str] = None
```

### Mock Configuration Model

```python
class MockConfig:
    """Configuration for mock adapters."""
    
    adapter_type: str
    responses: typing.List[MockResponse]
    strict_mode: bool = True
    record_calls: bool = True
    passthrough: bool = False
    match_on: typing.List[str] = None  # ['method', 'path', 'query', 'body']

class MockResponse:
    """Defines a mock response pattern."""
    
    match_pattern: MatchPattern
    response: typing.Any
    status_code: int = 200
    headers: typing.Dict[str, str] = None
    delay_ms: int = 0
    error_rate: float = 0.0
    callback: typing.Optional[typing.Callable] = None

class MatchPattern:
    """Pattern for matching requests."""
    
    method: str  # GET, POST, etc.
    path: typing.Union[str, typing.Pattern]
    query: typing.Optional[typing.Dict[str, typing.Any]] = None
    body: typing.Optional[typing.Any] = None
    headers: typing.Optional[typing.Dict[str, str]] = None
```

### Test Result Model

```python
class TestResult:
    """Standardized test result format."""
    
    name: str
    node_id: str
    status: TestStatus  # passed | failed | skipped | error | xfail
    duration_ms: float
    setup_duration_ms: float
    teardown_duration_ms: float
    call_duration_ms: float
    
    # Assertion details
    assertions: int
    assertions_passed: int
    assertions_failed: int
    
    # Failure details
    failures: typing.List[TestFailure]
    errors: typing.List[TestError]
    
    # Metadata
    markers: typing.List[str]
    fixture_names: typing.List[str]
    metadata: typing.Dict[str, typing.Any]

class TestFailure:
    """Failure details for test reporting."""
    
    message: str
    assertion_type: str
    expected: typing.Any
    actual: typing.Any
    context: typing.Dict[str, typing.Any]
    traceback: str
    filename: str
    lineno: int

class TestError:
    """Error details for test reporting."""
    
    message: str
    exception_type: str
    traceback: str
    filename: str
    lineno: int
```

---

## Testcontainers Integration

### Container Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    TESTCONTAINERS INTEGRATION                                 │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    CONTAINER HIERARCHY                             │   │
│  │                                                                      │   │
│  │  BaseContainer (testcontainers.core)                                │   │
│  │       │                                                              │   │
│  │       ├──► PhenotypePostgresContainer                               │   │
│  │       │       - Auto-init with migrations                           │   │
│  │       │       - Transaction isolation helper                        │   │
│  │       │       - Fixture loading helper                              │   │
│  │       │                                                              │   │
│  │       ├──► PhenotypeRedisContainer                                  │   │
│  │       │       - Key prefix isolation                                │   │
│  │       │       - Pub/sub test helpers                                │   │
│  │       │                                                              │   │
│  │       ├──► PhenotypeKafkaContainer                                  │   │
│  │       │       - Topic auto-creation                                 │   │
│  │       │       - Consumer group helpers                              │   │
│  │       │                                                              │   │
│  │       └──► PhenotypeLocalStackContainer                             │   │
│  │               - S3 bucket initialization                            │   │
│  │               - SQS queue setup                                     │   │
│  │               - DynamoDB table creation                             │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    SESSION MANAGEMENT                               │   │
│  │                                                                      │   │
│  │  SessionScopedContainer                                             │   │
│  │  - Container reuse across test session                              │   │
│  │  - Parallel test coordination                                       │   │
│  │  - Health check aggregation                                         │   │
│  │  - Cleanup on session end                                           │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Container Configuration

```python
# containers.py - Standardized container definitions

from testcontainers.postgres import PostgresContainer
from testcontainers.redis import RedisContainer
from testcontainers.kafka import KafkaContainer
from testcontainers.localstack import LocalStackContainer
from testcontainers.core.waiting_utils import wait_for_logs
import os

class PhenotypePostgresContainer(PostgresContainer):
    """
    PostgreSQL container configured for Phenotype tests.
    
    Features:
    - Automatic migration execution
    - Test transaction isolation
    - Fixture data loading
    - Connection pooling configuration
    """
    
    def __init__(
        self,
        image: str = "postgres:15-alpine",
        database: str = "test",
        username: str = "test",
        password: str = "test",
        migrations_path: str = "./migrations"
    ):
        super().__init__(image)
        self.database = database
        self.username = username
        self.password = password
        
        # Environment configuration
        self.with_env("POSTGRES_DB", database)
        self.with_env("POSTGRES_USER", username)
        self.with_env("POSTGRES_PASSWORD", password)
        
        # Mount migrations if path exists
        if os.path.exists(migrations_path):
            self.with_volume_mapping(
                os.path.abspath(migrations_path),
                "/docker-entrypoint-initdb.d"
            )
        
        # Expose standard port
        self.with_exposed_ports(5432)
        
        # Wait for ready
        self.with_wait_for(wait_for_logs("database system is ready to accept connections"))
    
    def get_connection_url(self, driver: str = "postgresql+psycopg2") -> str:
        """Get SQLAlchemy-compatible connection URL."""
        host = self.get_container_host_ip()
        port = self.get_exposed_port(5432)
        return f"{driver}://{self.username}:{self.password}@{host}:{port}/{self.database}"
    
    def get_async_connection_url(self) -> str:
        """Get asyncpg-compatible connection URL."""
        return self.get_connection_url(driver="postgresql+asyncpg")
    
    def load_fixtures(self, fixtures_path: str):
        """Load SQL fixture files into database."""
        import psycopg2
        conn = psycopg2.connect(self.get_connection_url(driver="postgresql"))
        cursor = conn.cursor()
        
        for filename in sorted(os.listdir(fixtures_path)):
            if filename.endswith(".sql"):
                with open(os.path.join(fixtures_path, filename)) as f:
                    cursor.execute(f.read())
        
        conn.commit()
        cursor.close()
        conn.close()


class PhenotypeRedisContainer(RedisContainer):
    """
    Redis container configured for Phenotype tests.
    
    Features:
    - Key prefix isolation per test
    - Pub/sub channel management
    - Lua script testing support
    """
    
    def __init__(
        self,
        image: str = "redis:7-alpine",
        maxmemory: str = "64mb"
    ):
        super().__init__(image)
        
        # Configuration for testing
        self.with_command(
            f"redis-server "
            f"--appendonly no "
            f"--maxmemory {maxmemory} "
            f"--maxmemory-policy allkeys-lru"
        )
        
        self.with_exposed_ports(6379)
        self.with_wait_for(wait_for_logs("Ready to accept connections"))
    
    def get_connection_url(self) -> str:
        """Get Redis connection URL."""
        host = self.get_container_host_ip()
        port = self.get_exposed_port(6379)
        return f"redis://{host}:{port}/0"
    
    def get_key_prefix(self, test_id: str) -> str:
        """Generate isolated key prefix for test."""
        return f"test:{test_id}:"


class PhenotypeKafkaContainer(KafkaContainer):
    """
    Kafka container configured for Phenotype tests.
    
    Features:
    - Auto topic creation
    - Consumer group testing
    - Message format validation
    """
    
    def __init__(
        self,
        image: str = "confluentinc/cp-kafka:latest",
        topics: list = None
    ):
        super().__init__(image)
        self.topics = topics or []
        
        # Kafka listeners
        self.with_env("KAFKA_BROKER_ID", "1")
        self.with_env("KAFKA_OFFSETS_TOPIC_REPLICATION_FACTOR", "1")
        
        self.with_exposed_ports(9092, 9093)
    
    def get_bootstrap_servers(self) -> str:
        """Get Kafka bootstrap servers."""
        host = self.get_container_host_ip()
        port = self.get_exposed_port(9092)
        return f"{host}:{port}"
    
    def create_topics(self, admin_client):
        """Create configured topics."""
        from kafka.admin import NewTopic
        
        new_topics = [
            NewTopic(name, num_partitions=1, replication_factor=1)
            for name in self.topics
        ]
        admin_client.create_topics(new_topics)


class PhenotypeLocalStackContainer(LocalStackContainer):
    """
    LocalStack container for AWS service testing.
    
    Features:
    - S3 bucket initialization
    - SQS queue setup
    - DynamoDB table creation
    - Secrets Manager integration
    """
    
    def __init__(
        self,
        image: str = "localstack/localstack:latest",
        services: list = None
    ):
        services = services or ["s3", "sqs", "dynamodb", "secretsmanager"]
        super().__init__(image)
        
        self.with_env("SERVICES", ",".join(services))
        self.with_env("DEFAULT_REGION", "us-east-1")
        
        self.with_exposed_ports(4566)
    
    def get_endpoint_url(self, service: str) -> str:
        """Get LocalStack endpoint URL for service."""
        host = self.get_container_host_ip()
        port = self.get_exposed_port(4566)
        return f"http://{host}:{port}"
    
    def init_s3_buckets(self, buckets: list):
        """Initialize S3 buckets."""
        import boto3
        s3 = boto3.client(
            "s3",
            endpoint_url=self.get_endpoint_url("s3"),
            aws_access_key_id="test",
            aws_secret_access_key="test",
            region_name="us-east-1"
        )
        for bucket in buckets:
            s3.create_bucket(Bucket=bucket)
```

### pytest Fixtures for Containers

```python
# conftest.py - Container fixtures

import pytest
import pytest_asyncio
from .containers import (
    PhenotypePostgresContainer,
    PhenotypeRedisContainer,
    PhenotypeKafkaContainer,
)

# Session-scoped containers for reuse
@pytest.fixture(scope="session")
def postgres_container():
    """
    PostgreSQL container shared across test session.
    
    Yields:
        PhenotypePostgresContainer: Configured Postgres container
    """
    container = PhenotypePostgresContainer()
    container.start()
    
    # Export for subprocess tests
    os.environ["TEST_DATABASE_URL"] = container.get_connection_url()
    
    yield container
    
    container.stop()

@pytest.fixture(scope="session")
def redis_container():
    """Redis container shared across test session."""
    container = PhenotypeRedisContainer()
    container.start()
    
    os.environ["TEST_CACHE_URL"] = container.get_connection_url()
    
    yield container
    container.stop()

# Function-scoped database sessions
@pytest.fixture
def db_engine(postgres_container):
    """SQLAlchemy engine with transaction isolation."""
    from sqlalchemy import create_engine
    
    engine = create_engine(postgres_container.get_connection_url())
    yield engine
    engine.dispose()

@pytest.fixture
def db_connection(db_engine):
    """Database connection with transaction."""
    connection = db_engine.connect()
    transaction = connection.begin()
    
    yield connection
    
    transaction.rollback()
    connection.close()

@pytest.fixture
def db_session(db_connection):
    """SQLAlchemy session with automatic rollback."""
    from sqlalchemy.orm import Session
    
    session = Session(bind=db_connection)
    yield session
    session.close()

# Async variants
@pytest_asyncio.fixture
async def async_db_engine(postgres_container):
    """Async SQLAlchemy engine."""
    from sqlalchemy.ext.asyncio import create_async_engine
    
    engine = create_async_engine(postgres_container.get_async_connection_url())
    yield engine
    await engine.dispose()

@pytest_asyncio.fixture
async def async_db_session(async_db_engine):
    """Async SQLAlchemy session."""
    from sqlalchemy.ext.asyncio import AsyncSession
    
    async with AsyncSession(async_db_engine) as session:
        yield session
        await session.rollback()

# Redis client fixtures
@pytest.fixture
def redis_client(redis_container):
    """Redis client with test isolation."""
    import redis
    
    client = redis.from_url(redis_container.get_connection_url())
    
    # Clear all test keys before test
    for key in client.scan_iter(match="test:*"):
        client.delete(key)
    
    yield client
    
    # Cleanup after test
    for key in client.scan_iter(match="test:*"):
        client.delete(key)
    
    client.close()
```

---

## Performance Targets

### Execution Time Budgets

| Test Type | Target Time | Maximum Time | Parallel Workers |
|-----------|-------------|--------------|------------------|
| Unit Test | < 10ms | 100ms | All cores |
| Component Test | < 50ms | 200ms | All cores |
| Contract Test | < 100ms | 500ms | All cores |
| Integration Test | < 1s | 10s | 4 workers |
| E2E Test | < 5s | 30s | 2 workers |
| Fixture Setup | < 10ms | 50ms | N/A |
| Container Startup | < 5s | 15s | Session-scoped |

### Test Suite Performance

| Metric | Target | Critical Threshold |
|--------|--------|-------------------|
| Total suite time | < 5 minutes | 10 minutes |
| Tests per second | > 50 | 20 |
| Flaky test rate | < 1% | 5% |
| Coverage target | > 80% | 70% |
| Mutation score | > 70% | 60% |

### Memory Budgets

| Component | Target | Maximum |
|-----------|--------|---------|
| Per test | < 10MB | 50MB |
| Container (Postgres) | 100MB | 200MB |
| Container (Redis) | 10MB | 64MB |
| Container (Kafka) | 500MB | 1GB |
| Test session total | < 2GB | 4GB |

---

## Contract Testing Specifications

### Pact Configuration

```python
# contracts/config.py

from pact import Consumer, Provider
import os

# Pact Broker configuration
PACT_BROKER_URL = os.environ.get("PACT_BROKER_URL", "http://localhost:9292")
PACT_BROKER_TOKEN = os.environ.get("PACT_BROKER_TOKEN")

# Consumer/Provider definitions
CONSUMERS = {
    "phench": Consumer("phench"),
    "heliosApp": Consumer("heliosApp"),
}

PROVIDERS = {
    "heliosCLI": Provider("heliosCLI"),
    "external-api": Provider("external-api"),
}

# Contract versions
PACT_SPEC_VERSION = "3.0.0"

# Publication settings
PACT_PUBLISH_VERIFICATION_RESULTS = True
PACT_PROVIDER_APP_VERSION = os.environ.get("GIT_COMMIT", "dev")
```

### Consumer Contract Patterns

```python
# contracts/test_helioscli_consumer.py

import pytest
import requests
from pact import Consumer, Provider, Like, Term, Format

@pytest.fixture
def pact():
    """Pact fixture for heliosCLI consumer tests."""
    return Consumer("phench").has_pact_with(
        Provider("heliosCLI"),
        pact_dir="./pacts",
        version=PACT_SPEC_VERSION
    )

class TestHeliosCLIConsumerContracts:
    """
    Consumer contract tests for heliosCLI API.
    
    These tests define the expectations that phench has of heliosCLI.
    They generate pact files that heliosCLI must satisfy.
    """
    
    def test_get_user_by_id(self, pact, user_factory):
        """
        Contract: GET /api/v1/users/{id}
        
        Expected behavior:
        - Returns 200 with user details for existing user
        - User has required fields: id, email, name, role
        """
        user = user_factory.build()
        
        expected = {
            "id": Like(user.id),
            "email": Like(user.email),
            "name": Like(user.name),
            "role": Term("^(admin|user|guest)$", user.role),
            "created_at": Format().timestamp(),
            "is_active": Like(True)
        }
        
        (pact
         .given(f"user with id '{user.id}' exists")
         .upon_receiving("a request for user by id")
         .with_request("GET", f"/api/v1/users/{user.id}")
         .will_respond_with(200, body=expected))
        
        with pact:
            result = requests.get(f"{pact.uri}/api/v1/users/{user.id}")
            assert result.status_code == 200
            assert result.json()["id"] == user.id
    
    def test_create_user(self, pact, user_factory):
        """
        Contract: POST /api/v1/users
        
        Expected behavior:
        - Returns 201 with created user
        - Assigned id is UUID format
        """
        new_user = user_factory.build()
        request_body = {
            "email": new_user.email,
            "name": new_user.name,
            "role": new_user.role
        }
        
        expected_response = {
            "id": Format().uuid(),
            "email": new_user.email,
            "name": new_user.name,
            "role": new_user.role,
            "created_at": Format().timestamp(),
            "is_active": True
        }
        
        (pact
         .given(f"email '{new_user.email}' is available")
         .upon_receiving("a request to create user")
         .with_request("POST", "/api/v1/users", body=request_body)
         .will_respond_with(201, body=expected_response))
        
        with pact:
            result = requests.post(
                f"{pact.uri}/api/v1/users",
                json=request_body
            )
            assert result.status_code == 201
            assert "id" in result.json()
    
    def test_get_user_not_found(self, pact):
        """
        Contract: GET /api/v1/users/{id} for non-existent user
        
        Expected behavior:
        - Returns 404 with error details
        """
        non_existent_id = "user-99999"
        
        expected_error = {
            "error": Like("User not found"),
            "code": Like("USER_NOT_FOUND"),
            "user_id": non_existent_id
        }
        
        (pact
         .given(f"user with id '{non_existent_id}' does not exist")
         .upon_receiving("a request for non-existent user")
         .with_request("GET", f"/api/v1/users/{non_existent_id}")
         .will_respond_with(404, body=expected_error))
        
        with pact:
            result = requests.get(f"{pact.uri}/api/v1/users/{non_existent_id}")
            assert result.status_code == 404
```

### Provider Verification Patterns

```python
# contracts/test_helioscli_provider.py

import pytest
from pact.verifier import Verifier
from helioscli.app import create_app

class TestHeliosCLIProviderVerification:
    """
    Provider verification tests for heliosCLI.
    
    These tests verify that heliosCLI satisfies all consumer contracts.
    """
    
    @pytest.fixture
    def app(self, db_session):
        """Create test application with real dependencies."""
        return create_app(
            database_session=db_session,
            testing=True
        )
    
    @pytest.fixture
    def provider_url(self, app):
        """Get test server URL."""
        # Start test server
        import threading
        from werkzeug.serving import make_server
        
        server = make_server("localhost", 0, app)
        thread = threading.Thread(target=server.serve_forever)
        thread.daemon = True
        thread.start()
        
        yield f"http://localhost:{server.server_port}"
        
        server.shutdown()
    
    def test_honours_pact_with_phench(self, provider_url):
        """Verify against phench consumer contracts."""
        verifier = Verifier(
            provider="heliosCLI",
            provider_base_url=provider_url
        )
        
        output, return_code = verifier.verify_pacts(
            "./pacts/phench-helioscli.json",
            provider_states_setup_url=f"{provider_url}/_pact/setup",
            verbose=False
        )
        
        assert return_code == 0, f"Provider verification failed:\n{output}"
    
    def test_honours_all_consumer_pacts_from_broker(self, provider_url):
        """Verify against all consumers from Pact Broker."""
        verifier = Verifier(
            provider="heliosCLI",
            provider_base_url=provider_url
        )
        
        output, return_code = verifier.verify_with_broker(
            broker_url=PACT_BROKER_URL,
            broker_token=PACT_BROKER_TOKEN,
            provider_states_setup_url=f"{provider_url}/_pact/setup",
            publish_verification_results=PACT_PUBLISH_VERIFICATION_RESULTS,
            provider_app_version=PACT_PROVIDER_APP_VERSION,
            verbose=True
        )
        
        assert return_code == 0, f"Broker verification failed:\n{output}"
```

### Provider State Handlers

```python
# contracts/provider_states.py

from flask import Blueprint, request, jsonify
from tests.fixtures import UserFactory, OrganizationFactory

pact_states = Blueprint('pact_states', __name__)

@pact_states.route('/_pact/setup', methods=['POST'])
def setup_provider_state():
    """
    Setup provider state for pact verification.
    
    Expected body:
    {
        "state": "user with id {id} exists",
        "params": {"id": "user-123"}
    }
    """
    data = request.get_json()
    state = data.get('state')
    params = data.get('params', {})
    
    # Route to appropriate handler
    handlers = {
        'user with id {id} exists': _create_user,
        'user with id {id} does not exist': _delete_user,
        'email {email} is available': _ensure_email_available,
        'no users exist': _delete_all_users,
        'organization with id {id} exists': _create_organization,
    }
    
    handler = handlers.get(state)
    if handler:
        handler(**params)
    
    return jsonify({"status": "ok"})

def _create_user(id: str, **kwargs):
    """Create test user with given ID."""
    UserFactory.create(id=id, **kwargs)

def _delete_user(id: str):
    """Delete user by ID if exists."""
    from helioscli.models import User
    User.query.filter_by(id=id).delete()
    db.session.commit()

def _ensure_email_available(email: str):
    """Remove any user with given email."""
    from helioscli.models import User
    User.query.filter_by(email=email).delete()
    db.session.commit()

def _delete_all_users():
    """Delete all users."""
    from helioscli.models import User
    User.query.delete()
    db.session.commit()

def _create_organization(id: str, **kwargs):
    """Create organization with given ID."""
    OrganizationFactory.create(id=id, **kwargs)
```

---

## Dependencies

### External Dependencies

| Package | Version | Purpose | Tier |
|---------|---------|---------|------|
| pytest | ^8.0 | Test framework | P0 |
| pytest-asyncio | ^0.23 | Async test support | P0 |
| pytest-xdist | ^3.5 | Parallel execution | P0 |
| pytest-cov | ^4.1 | Coverage reporting | P0 |
| factory_boy | ^3.3 | Fixture generation | P0 |
| faker | ^24.0 | Fake data generation | P0 |
| testcontainers | ^4.0 | Container integration | P0 |
| pact-python | ^2.2 | Contract testing | P1 |
| hypothesis | ^6.100 | Property testing | P2 |
| syrupy | ^4.6 | Snapshot testing | P2 |
| responses | ^0.25 | HTTP mocking | P0 |
| moto | ^5.0 | AWS mocking | P1 |
| freezegun | ^1.4 | Time mocking | P1 |
| mutmut | ^2.4 | Mutation testing | P3 |

### Internal Dependencies

| Component | Path | Integration Type |
|-----------|------|------------------|
| Core Types | `src/` | Type definitions |
| Tooling | `tooling/` | CI/CD hooks |
| Config | `config/` | Test configuration |

---

## Integration Points

### Consumer Projects

| Project | Fixtures Used | Container Usage | Contract Participation |
|---------|---------------|-----------------|----------------------|
| phench | All | PostgreSQL, Redis | Consumer (heliosCLI) |
| src/ | Core | PostgreSQL | Consumer (heliosCLI) |
| heliosCLI | Core fixtures | PostgreSQL, Redis | Provider (phench, heliosApp) |
| heliosApp | Core fixtures | None | Consumer (heliosCLI) |
| omniroute-temp | Mocks, containers | Kafka | None |

### CI/CD Integration

```yaml
# GitHub Actions workflow integration
name: Test Integration

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    services:
      docker:
        image: docker:dind
        options: --privileged
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Run tests with fixtures
        run: |
          # Generate fixtures if schemas changed
          task fixtures:generate
          
          # Run with testcontainers
          pytest --testcontainers \
                 --cov=src \
                 --cov-report=xml \
                 --cov-report=html
      
      - name: Run contract tests
        if: github.event_name == 'pull_request'
        run: |
          pytest tests/contracts/ -v
          
          # Verify can-i-deploy
          pact-broker can-i-deploy \
            --pacticipant phench \
            --version ${{ github.sha }} \
            --to-environment staging
      
      - name: Publish contracts
        if: github.ref == 'refs/heads/main'
        run: |
          pact-broker publish pacts/ \
            --consumer-app-version ${{ github.sha }} \
            --broker-base-url ${{ secrets.PACT_BROKER_URL }} \
            --broker-token ${{ secrets.PACT_BROKER_TOKEN }}
```

---

## Directory Structure

```
tests/
├── README.md                      # Quick start guide
├── SPEC.md                        # This specification
├── SOTA.md                        # State of the art research
├── PLAN.md                        # Implementation plan
├── ADR-001-*.md                   # Architecture decision records
├── ADR-002-*.md
├── ADR-003-*.md
│
├── fixtures/                      # Generated fixture code
│   ├── schemas/                   # JSON Schema definitions
│   │   ├── user.schema.json
│   │   ├── order.schema.json
│   │   ├── organization.schema.json
│   │   └── common/
│   │       ├── address.schema.json
│   │       └── money.schema.json
│   │
│   ├── python/                    # Generated Python factories
│   │   ├── __init__.py
│   │   ├── user_factory.py
│   │   ├── order_factory.py
│   │   └── ...
│   │
│   ├── rust/                      # Generated Rust factories
│   │   ├── mod.rs
│   │   ├── user_factory.rs
│   │   └── ...
│   │
│   └── javascript/                # Generated JS/TS factories
│       ├── index.ts
│       ├── user_factory.ts
│       └── ...
│
├── containers.py                  # Testcontainers configuration
├── conftest.py                    # Shared pytest fixtures
│
├── contracts/                     # Contract testing
│   ├── config.py
│   ├── consumer_tests/            # Consumer contract tests
│   │   ├── test_helioscli_consumer.py
│   │   └── ...
│   ├── provider_tests/            # Provider verification tests
│   │   ├── test_helioscli_provider.py
│   │   └── ...
│   └── provider_states.py           # Provider state handlers
│
├── assertions/                    # Custom assertions
│   ├── __init__.py
│   ├── api_assertions.py
│   └── db_assertions.py
│
├── mocks/                         # Mock adapters
│   ├── __init__.py
│   ├── http_mocks.py
│   └── aws_mocks.py
│
├── utils/                         # Test utilities
│   ├── __init__.py
│   ├── timing.py                  # Timing helpers
│   ├── random.py                  # Seeded random
│   └── json.py                    # JSON comparison
│
└── generators/                    # Schema-to-code generators
    ├── __init__.py
    ├── base.py                    # Base generator
    ├── python_gen.py              # Python generator
    ├── rust_gen.py                # Rust generator
    └── javascript_gen.py          # JS generator
```

---

## Planned Features

### Phase 1: Core Infrastructure (Weeks 1-2)

| Feature | Description | Acceptance Criteria |
|---------|-------------|---------------------|
| Schema system | JSON Schema with phenotype extensions | 5 core schemas defined |
| Python factories | factory_boy generation | UserFactory working in phench |
| Container setup | PostgreSQL, Redis containers | Integration tests < 5s |
| pytest fixtures | Shared session-scoped fixtures | conftest.py in tests/ |

### Phase 2: Multi-Language (Weeks 3-4)

| Feature | Description | Acceptance Criteria |
|---------|-------------|---------------------|
| Rust factories | fake-rs generation | heliosCLI uses UserFactory |
| JS factories | faker-js generation | heliosApp uses userFactory |
| Container reuse | Session-scoped optimization | Suite time < 3 minutes |
| Generator CI | Auto-generation on schema change | CI validates generated code |

### Phase 3: Contract Testing (Weeks 5-6)

| Feature | Description | Acceptance Criteria |
|---------|-------------|---------------------|
| Pact setup | Pact Broker integration | Consumer tests generate pacts |
| Provider verification | heliosCLI validates contracts | 100% contract verification |
| CI gates | can-i-deploy integration | Breaking changes blocked |
| Bi-directional | OpenAPI + Pact validation | API spec validated |

### Phase 4: Advanced Testing (Weeks 7-8)

| Feature | Description | Acceptance Criteria |
|---------|-------------|---------------------|
| Property tests | Hypothesis integration | 10% of tests use properties |
| Snapshot testing | syrupy integration | API responses snapshotted |
| Chaos testing | Fault injection | Circuit breaker tests |
| Mutation testing | mutmut integration | Mutation score > 70% |

---

## API Reference

### Factory API

```python
# Universal factory interface across all languages

class FactoryInterface:
    """
    Standard factory interface.
    
    All generated factories implement this interface.
    """
    
    @classmethod
    def build(cls, **overrides) -> Model:
        """
        Build instance without persistence.
        
        Args:
            **overrides: Attribute overrides
            
        Returns:
            Model instance (not persisted)
        """
        pass
    
    @classmethod
    def create(cls, **overrides) -> Model:
        """
        Build and persist instance.
        
        Args:
            **overrides: Attribute overrides
            
        Returns:
            Model instance (persisted)
        """
        pass
    
    @classmethod
    def build_batch(cls, count: int, **overrides) -> List[Model]:
        """
        Build multiple instances.
        
        Args:
            count: Number of instances
            **overrides: Attribute overrides (applied to all)
            
        Returns:
            List of Model instances
        """
        pass
    
    @classmethod
    def create_batch(cls, count: int, **overrides) -> List[Model]:
        """
        Build and persist multiple instances.
        
        Args:
            count: Number of instances
            **overrides: Attribute overrides (applied to all)
            
        Returns:
            List of persisted Model instances
        """
        pass
    
    @classmethod
    def trait(cls, name: str, **overrides) -> "FactoryTrait":
        """
        Create factory with trait applied.
        
        Args:
            name: Trait name
            **overrides: Additional overrides
            
        Returns:
            Factory with trait configured
        """
        pass
```

### Container API

```python
# Container lifecycle management

class ContainerManager:
    """
    Manages container lifecycle for tests.
    """
    
    @staticmethod
    def get_or_create(
        container_class: Type[Container],
        scope: str = "session"
    ) -> Container:
        """
        Get existing container or create new one.
        
        Args:
            container_class: Container class to instantiate
            scope: Lifecycle scope (function/class/module/session)
            
        Returns:
            Container instance
        """
        pass
    
    @staticmethod
    def cleanup(scope: str = None):
        """
        Cleanup containers for scope.
        
        Args:
            scope: Scope to cleanup (None = all)
        """
        pass
```

### Contract API

```python
# Contract testing API

class ContractTest:
    """
    Base class for contract tests.
    """
    
    consumer: str
    provider: str
    
    def given(self, state: str) -> "ContractTest":
        """Set provider state precondition."""
        pass
    
    def upon_receiving(self, description: str) -> "ContractTest":
        """Describe the interaction."""
        pass
    
    def with_request(
        self,
        method: str,
        path: str,
        body: dict = None,
        headers: dict = None,
        query: dict = None
    ) -> "ContractTest":
        """Define request."""
        pass
    
    def will_respond_with(
        self,
        status: int,
        body: dict = None,
        headers: dict = None
    ) -> "ContractTest":
        """Define expected response."""
        pass
```

---

## Testing Patterns

### Unit Test Pattern

```python
# tests/unit/test_user_service.py

import pytest
from unittest.mock import Mock, patch
from src.user_service import UserService
from tests.fixtures import build_user

class TestUserService:
    """Unit tests for UserService (mocked dependencies)."""
    
    @pytest.fixture
    def user_repo(self):
        return Mock()
    
    @pytest.fixture
    def service(self, user_repo):
        return UserService(user_repo)
    
    def test_get_user_returns_user(self, service, user_repo):
        # Arrange
        user = build_user()
        user_repo.get_by_id.return_value = user
        
        # Act
        result = service.get_user(user.id)
        
        # Assert
        assert result == user
        user_repo.get_by_id.assert_called_once_with(user.id)
    
    def test_get_user_not_found_raises(self, service, user_repo):
        user_repo.get_by_id.return_value = None
        
        with pytest.raises(UserNotFoundError):
            service.get_user("non-existent")
```

### Component Test Pattern

```python
# tests/component/test_user_repository.py

import pytest
from tests.fixtures import create_user
from src.repositories import UserRepository

class TestUserRepository:
    """Component tests for UserRepository (real in-memory DB)."""
    
    @pytest.fixture
    def repo(self, db_session):
        return UserRepository(db_session)
    
    def test_create_user_persists(self, repo, db_session):
        # Arrange
        user_data = {"email": "test@example.com", "name": "Test User"}
        
        # Act
        user = repo.create(user_data)
        db_session.commit()
        
        # Assert
        found = repo.get_by_id(user.id)
        assert found.email == user_data["email"]
    
    def test_find_by_email_returns_user(self, repo, db_session):
        # Arrange
        user = create_user(email="find@example.com")
        db_session.commit()
        
        # Act
        found = repo.find_by_email("find@example.com")
        
        # Assert
        assert found.id == user.id
```

### Integration Test Pattern

```python
# tests/integration/test_user_api.py

import pytest
from tests.fixtures import create_user
from tests.containers import PhenotypePostgresContainer

class TestUserAPI:
    """Integration tests for User API (real containers)."""
    
    @pytest.fixture
    def api_client(self, postgres_container):
        """API client with real database."""
        from src.app import create_app
        
        app = create_app(
            database_url=postgres_container.get_connection_url()
        )
        return app.test_client()
    
    def test_create_user_endpoint(self, api_client):
        response = api_client.post("/api/users", json={
            "email": "new@example.com",
            "name": "New User"
        })
        
        assert response.status_code == 201
        assert response.json["email"] == "new@example.com"
        assert "id" in response.json
    
    def test_get_user_endpoint(self, api_client, db_session):
        # Arrange
        user = create_user()
        db_session.commit()
        
        # Act
        response = api_client.get(f"/api/users/{user.id}")
        
        # Assert
        assert response.status_code == 200
        assert response.json["id"] == user.id
```

### Contract Test Pattern

```python
# tests/contracts/test_helioscli_consumer.py

import pytest
from pact import Consumer, Provider, Like

class TestHeliosCLIContracts:
    """Consumer contract tests for heliosCLI API."""
    
    @pytest.fixture
    def pact(self):
        return Consumer("phench").has_pact_with(
            Provider("heliosCLI"),
            pact_dir="./pacts"
        )
    
    def test_get_user_contract(self, pact, user_factory):
        expected = {
            "id": Like("user-123"),
            "email": Like("user@example.com"),
            "role": Like("user")
        }
        
        (pact
         .given("user exists")
         .upon_receiving("get user request")
         .with_request("GET", "/api/users/user-123")
         .will_respond_with(200, body=expected))
        
        with pact:
            # Test against mock provider
            response = requests.get(f"{pact.uri}/api/users/user-123")
            assert response.status_code == 200
```

### Property Test Pattern

```python
# tests/property/test_sorting.py

import pytest
from hypothesis import given, strategies as st

class TestSortingProperties:
    """Property-based tests for sorting operations."""
    
    @given(st.lists(st.integers(), min_size=1))
    def test_sort_idempotent(self, elements):
        """Sorting twice equals sorting once."""
        assert sorted(sorted(elements)) == sorted(elements)
    
    @given(st.lists(st.integers()))
    def test_sort_preserves_length(self, elements):
        """Sorting doesn't change list length."""
        assert len(sorted(elements)) == len(elements)
    
    @given(st.lists(st.integers(), min_size=2))
    def test_sort_orders_elements(self, elements):
        """Sorted list is ordered."""
        sorted_list = sorted(elements)
        for i in range(len(sorted_list) - 1):
            assert sorted_list[i] <= sorted_list[i + 1]
```

---

## Quality Gates

### Pre-Commit Checks

```yaml
# .pre-commit-config.yaml
repos:
  - repo: local
    hooks:
      - id: fixtures-generate
        name: Generate fixtures from schemas
        entry: task fixtures:generate
        language: system
        files: fixtures/schemas/.*\.json$
      
      - id: fixture-tests
        name: Run fixture tests
        entry: pytest tests/fixtures/ -v
        language: system
        pass_filenames: false
      
      - id: contract-validation
        name: Validate contracts
        entry: pact-broker validate
        language: system
        files: pacts/.*\.json$
```

### CI Gates

| Gate | Condition | Action on Failure |
|------|-----------|-------------------|
| Lint | ruff/mypy pass | Block merge |
| Unit Tests | 100% pass | Block merge |
| Coverage | > 80% | Block merge |
| Integration Tests | 100% pass | Block merge |
| Contract Verification | All pass | Block deploy |
| can-i-deploy | Compatible | Block deploy |

---

## Troubleshooting

### Common Issues

#### Container Startup Failures

```
Problem: PostgreSQL container fails to start
Solution:
1. Check Docker is running: docker ps
2. Verify image exists: docker pull postgres:15-alpine
3. Check port conflicts: lsof -i :5432
4. Review logs: pytest --testcontainers-debug
```

#### Fixture Generation Failures

```
Problem: Generated factories have syntax errors
Solution:
1. Validate schema: task fixtures:validate
2. Check faker method names are valid
3. Review generator logs: task fixtures:generate --verbose
4. Manually inspect generated file
```

#### Contract Test Failures

```
Problem: Provider verification fails
Solution:
1. Check provider is running: curl localhost:5000/health
2. Verify provider state handler: /_pact/setup
3. Check pact file version matches provider
4. Review verification logs: pytest tests/contracts/ -v
```

### Debug Mode

```python
# Enable debug output for tests
import os
os.environ["TEST_DEBUG"] = "1"
os.environ["TESTCONTAINERS_DEBUG"] = "1"
os.environ["PACT_DEBUG"] = "1"
```

---

## Migration Guide

### From Manual Fixtures

```python
# Before: Manual fixture definition
def test_user():
    return {
        "id": str(uuid.uuid4()),
        "email": "test@example.com",
        "name": "Test User",
        "created_at": datetime.now().isoformat()
    }

# After: Generated factory
from tests.fixtures import build_user

user = build_user()  # Fully populated, deterministic
user = build_user(email="specific@example.com")  # With overrides
```

### From Mock-Based Integration Tests

```python
# Before: Mock-based integration test
@mock.patch("src.db.execute")
def test_user_creation(mock_execute):
    mock_execute.return_value = [{"id": "123"}]
    result = create_user(...)
    # May pass but fail in production

# After: Container-based integration test
def test_user_creation(db_session, postgres_container):
    result = create_user(...)
    db_session.commit()
    found = db_session.get(User, result.id)
    assert found is not None  # Verified against real DB
```

---

## Notes

### Design Decisions

1. **Schema-First Approach:**
   - Single source of truth for fixture structure
   - Enables code generation across languages
   - Version controlled alongside code

2. **Session-Scoped Containers:**
   - Minimizes container startup overhead
   - Transaction isolation per test
   - Automatic cleanup via ryuk

3. **Consumer-Driven Contracts:**
   - Consumers define expectations
   - Providers verify compatibility
   - Enables independent deployment

### Known Limitations

1. **Generator Complexity:**
   - Complex nested relationships require manual tuning
   - Custom faker providers need per-language implementation

2. **Container Resource Usage:**
   - Requires Docker in CI environment
   - Memory overhead for multiple containers

3. **Contract Test Scope:**
   - HTTP APIs only (no message queue support yet)
   - Requires Pact Broker infrastructure

### Future Enhancements

1. **GraphQL Support:**
   - Schema-based fixture generation
   - GraphQL contract testing

2. **Event-Driven Testing:**
   - Kafka message contract testing
   - Event sourcing test patterns

3. **Visual Testing:**
   - Screenshot comparison
   - Visual regression testing

4. **Performance Contracts:**
   - Response time SLAs
   - Throughput validation

---

## Appendix A: Error Code Reference

| Code | Description | Resolution |
|------|-------------|------------|
| TEST001 | Fixture generation failed | Check schema syntax |
| TEST002 | Container startup timeout | Verify Docker is running |
| TEST003 | Contract verification failed | Check provider state handlers |
| TEST004 | Database connection refused | Verify container health |
| TEST005 | Schema validation error | Validate JSON Schema |
| TEST006 | Missing faker provider | Install faker extension |
| TEST007 | Pact broker unreachable | Check network/broker URL |
| TEST008 | Coverage threshold not met | Add tests for uncovered code |

## Appendix B: Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `TEST_DATABASE_URL` | `postgresql://...` | Override database URL |
| `TEST_CACHE_URL` | `redis://...` | Override cache URL |
| `TEST_DEBUG` | `0` | Enable debug output |
| `TESTCONTAINERS_DEBUG` | `0` | Enable container debug |
| `PACT_BROKER_URL` | `http://localhost:9292` | Pact Broker endpoint |
| `PACT_BROKER_TOKEN` | `None` | Pact Broker auth token |
| `PACT_PUBLISH_VERIFICATION_RESULTS` | `True` | Publish verification results |
| `PYTEST_CURRENT_TEST` | N/A | Current test identifier |

## Appendix C: CLI Commands

### Fixture Management

```bash
# Generate fixtures from schemas
task fixtures:generate

# Validate schema files
task fixtures:validate

# Clean generated files
task fixtures:clean

# Watch and regenerate
task fixtures:watch
```

### Contract Testing

```bash
# Run consumer contract tests
pytest tests/contracts/consumer/ -v

# Run provider verification
pytest tests/contracts/provider/ -v

# Publish contracts to broker
pact-broker publish pacts/ --consumer-app-version $VERSION

# Check deployment readiness
pact-broker can-i-deploy --pacticipant phench --version $VERSION
```

### Container Testing

```bash
# Run with testcontainers
pytest --testcontainers

# Run specific container test
pytest -k "postgres"

# Debug container startup
pytest --testcontainers-debug

# Reuse containers (faster)
pytest --testcontainers-reuse
```

## Appendix D: Testing Checklist

### New Feature Checklist

- [ ] Unit tests for business logic
- [ ] Component tests for data access
- [ ] Integration tests with containers
- [ ] Contract tests if API changes
- [ ] Documentation updated
- [ ] Performance regression check

### Code Review Checklist

- [ ] Tests are deterministic
- [ ] No sleep-based waiting
- [ ] Fixtures used appropriately
- [ ] Mock usage justified
- [ ] Error cases tested
- [ ] Edge cases covered

### Release Checklist

- [ ] All tests passing
- [ ] Contract verification complete
- [ ] Coverage above threshold
- [ ] No new flaky tests
- [ ] Performance benchmarks acceptable
- [ ] can-i-deploy check passed

## Appendix E: Third-Party Integrations

### GitHub Actions Integration

```yaml
name: Test Suite

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Set up Python
        uses: actions/setup-python@v5
        with:
          python-version: '3.11'
      
      - name: Install dependencies
        run: pip install -r requirements-test.txt
      
      - name: Run test suite
        run: task test:all
      
      - name: Upload coverage
        uses: codecov/codecov-action@v4
        with:
          files: ./coverage.xml
```

### GitLab CI Integration

```yaml
test:
  image: python:3.11
  services:
    - docker:dind
  variables:
    DOCKER_DRIVER: overlay2
    TESTCONTAINERS_DOCKER_SOCKET_OVERRIDE: /var/run/docker.sock
  script:
    - pip install -r requirements-test.txt
    - pytest --cov=src --cov-report=xml
  coverage: '/TOTAL.*\s+(\d+%)$/'
```

## Appendix F: Performance Tuning Guide

### Slow Test Diagnosis

```bash
# Identify slowest tests
pytest --durations=10

# Profile test execution
pytest --profile

# Run with timing
time pytest
```

### Optimization Strategies

1. **Container Reuse:** Use session-scoped containers
2. **Parallel Execution:** Use pytest-xdist
3. **Selective Testing:** Run only changed tests
4. **Async Tests:** Use pytest-asyncio for IO-bound tests
5. **Lazy Fixtures:** Defer fixture setup until needed
6. **Database Transactions:** Rollback instead of truncate
7. **Mock Isolation:** Avoid shared mock state

---

*End of SPEC.md — tests/ Specification*

*Status: Complete*
*Version: 1.0.0*
*Last Updated: 2026-04-04*



