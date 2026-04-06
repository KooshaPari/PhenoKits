# authkit

**Authentication and authorization framework with OAuth2, JWT, and RBAC/ABAC support.**

A hexagonal architecture-based authentication framework supporting:

- **Multiple Auth Methods**: JWT, OAuth2, API Keys, Session tokens
- **Authorization Models**: RBAC, ABAC, PBAC with policy engine
- **User Management**: Registration, login, password reset
- **Session Management**: Token refresh, revocation, rotation
- **Multi-tenancy**: Tenant isolation, domain-based routing
- **Audit Logging**: Complete auth event logging

## Architecture

```
authkit/
├── src/
│   ├── domain/          # Core domain logic (pure)
│   │   ├── identity.rs # User entities
│   │   ├── auth.rs     # Authentication logic
│   │   ├── policy.rs   # Authorization policies
│   │   ├── session.rs  # Session management
│   │   ├── ports.rs    # Interface definitions
│   │   └── errors.rs   # Domain errors
│   ├── application/     # Application services
│   ├── adapters/        # Infrastructure adapters
│   └── infrastructure/  # Cross-cutting concerns
```

## Features

- [x] JWT tokens with RS256/HS256
- [x] OAuth2 client credentials flow
- [x] Password hashing (argon2, bcrypt)
- [x] RBAC with role hierarchy
- [x] ABAC with attribute-based policies
- [x] Session management with refresh tokens
- [x] Rate limiting
- [x] Audit logging
- [ ] Multi-factor authentication
- [ ] Passwordless authentication
- [ ] Delegation and federation

## Installation

```toml
[dependencies]
authkit = "0.1"
```

## Quick Start

```rust
use authkit::{Authenticator, Claims};

let auth = Authenticator::new(secret_key)?;
let token = auth.generate_token(user_id, roles)?;
let claims = auth.verify_token(&token)?;
```

## Development

```bash
cargo build
cargo test
cargo clippy -- -D warnings
cargo fmt
```

## License

MIT OR Apache-2.0
