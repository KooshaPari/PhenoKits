# PhenoForge

Code generation, scaffolding, and project initialization framework for the Phenotype ecosystem. Provides CLI tools for bootstrapping new Rust crates, TypeScript packages, and Python projects with best-practice defaults, governance templates, and integrated quality gates.

## Overview

**PhenoForge** eliminates boilerplate and accelerates project initialization by providing opinionated code generators, project templates, and scaffolding tools aligned with Phenotype architecture standards. New repositories emerge pre-configured with CLAUDE.md governance, AGENTS.md operating contracts, CI/CD pipelines, and local quality gates.

**Core Mission**: Enable instant, policy-compliant project initialization with zero manual governance setup, automatic toolchain configuration, and standards-aligned templates.

## Technology Stack

- **CLI Framework**: Rust with clap for command parsing
- **Template Engine**: Handlebars-based project generation
- **Project Types**: Rust workspaces, TypeScript monorepos, Python packages, Go modules
- **Governance Integration**: Automatic CLAUDE.md, PRD.md, AGENTS.md generation
- **CI/CD Templates**: GitHub Actions, Forgejo/Woodpecker workflow generation
- **Linting & Formatting**: Pre-configured Ruff, Clippy, prettier, eslint defaults

## Key Features

- **Project Scaffolding**: Create complete Rust/TS/Python projects in seconds
- **Template Customization**: Extensible Handlebars templates for domain-specific boilerplate
- **Governance Automation**: Auto-generate CLAUDE.md, PRD.md, AGENTS.md based on project type
- **Workspace Setup**: Cargo workspaces, npm/pnpm monorepo configuration, Python venv setup
- **Quality Gate Integration**: Pre-configured test runners, linters, formatters, pre-commit hooks
- **CI/CD Pipeline Generation**: GitHub Actions and Woodpecker workflows with passing defaults
- **Documentation Templates**: README.md structure, docs/ directory scaffolding, ADR templates

## Quick Start

```bash
# Clone and explore
git clone <repo-url>
cd phenoForge

# Review governance and architecture
cat CLAUDE.md          # Project governance
cat PRD.md             # Product requirements

# Build the CLI
cargo build --release
./target/release/phenoforge --help

# Create a new Rust workspace project
./target/release/phenoforge new --type rust --name my-service --template workspace

# Generate TypeScript package
./target/release/phenoforge new --type typescript --name my-package --template lib

# Run tests
cargo test --workspace
```

## Project Structure

```
phenoForge/
├── src/
│   ├── cli/                   # CLI command handling
│   ├── templates/             # Handlebars project templates
│   │   ├── rust/              # Rust workspace templates
│   │   ├── typescript/        # TypeScript monorepo templates
│   │   └── python/            # Python package templates
│   ├── generator/             # Core generation logic
│   ├── governance/            # CLAUDE.md and AGENTS.md generation
│   └── main.rs
├── templates/                 # Template files (Handlebars)
│   ├── Cargo.toml.hbs
│   ├── CLAUDE.md.hbs
│   ├── .github/workflows.hbs
│   └── ...
├── tests/
│   ├── integration/           # End-to-end scaffolding tests
│   └── templates/             # Template correctness tests
└── Cargo.toml
```

## Related Phenotype Projects

- **AgilePlus**: Work tracking (newly created projects register with AgilePlus)
- **PhenoPlugins**: Plugin scaffolding and extensibility system
- **phenotype-tooling**: Development tools and CLI utilities
- **Tracera**: Project telemetry and metrics (tracks scaffold usage)
