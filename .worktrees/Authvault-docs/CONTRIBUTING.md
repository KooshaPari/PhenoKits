# Contributing to Authvault

First off, thank you for considering contributing to **Authvault**! It's people like you who make this authentication framework better for everyone.

## Code of Conduct

By participating in this project, you agree to abide by our Code of Conduct.

## How Can I Contribute?

### Reporting Bugs

- Use the Bug Report issue template
- Provide a clear and descriptive title
- Describe the exact steps to reproduce the problem
- Include your Rust version, OS, and Authvault version

### Suggesting Enhancements

- Check the Issues to see if the enhancement has already been suggested
- Use the Feature Request issue template
- Describe the use case and why it would benefit the project

### Pull Requests

1. Fork the repo and create your branch from `main`
2. If you've added code that should be tested, add tests
3. If you've changed APIs, update the documentation
4. Ensure the test suite passes (`cargo test`)
5. Make sure your code lints (`cargo clippy -- -D warnings`)
6. Format your code (`cargo fmt`)

#### Branch Naming

```
feat/<feature-name>     # New features
fix/<bug-description>  # Bug fixes
docs/<topic>           # Documentation
refactor/<area>        # Code refactoring
```

#### Commit Messages

Follow conventional commits:

```
feat(auth): add OAuth2 client credentials flow
fix(jwt): correct token expiry calculation
docs(readme): update installation instructions
```

## Development Setup

```bash
# Clone the repository
git clone https://github.com/phenotype-dev/authvault.git
cd authvault

# Install dependencies
cargo fetch

# Run tests
cargo test

# Run linter
cargo clippy -- -D warnings

# Format code
cargo fmt
```

## Architecture Guidelines

Authvault follows hexagonal architecture principles:

- **Domain Layer**: Pure business logic, zero external dependencies
- **Application Layer**: Use cases and orchestration
- **Adapters Layer**: External integrations (REST, gRPC, etc.)
- **Infrastructure Layer**: Cross-cutting concerns (error handling, logging)

### Key Principles

1. **Dependency Rule**: Source code dependencies only point inward
2. **Pure Domain**: Domain has no external dependencies
3. **Port Interfaces**: Define interfaces in domain, implement in adapters
4. **Testability**: Business logic must be unit testable without mocks

## Security Considerations

- Never commit secrets or credentials
- All auth-related code must be reviewed for security implications
- Use `cargo audit` to check for vulnerable dependencies
- Follow secure coding practices for authentication flows

## License

By contributing, you agree that your contributions will be licensed under the MIT or Apache-2.0 license.

---

Thank you for your contributions!
