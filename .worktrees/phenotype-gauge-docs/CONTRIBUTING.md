# Contributing to phenotype-gauge

Thank you for your interest in contributing!

## Development Setup

```bash
# Clone the repository
git clone git@github.com:KooshaPari/phenotype-gauge.git
cd phenotype-gauge

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Run tests
cargo test --all-features

# Run linter
cargo clippy -- -D warnings
```

## Code Style

- Format code with `cargo fmt`
- Lint with `cargo clippy -- -D warnings`
- Maximum line length: 100 characters
- Add tests for new functionality
- Update documentation when applicable

## Pull Request Process

1. Fork the repository
2. Create a feature branch (`feat/your-feature`)
3. Make your changes
4. Run `cargo test` and `cargo clippy` locally
5. Submit a pull request

## Reporting Issues

Please report security vulnerabilities via GitHub Security Advisories, not public issues.
