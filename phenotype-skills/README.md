# phenotype-skills

Skills registry, execution engine, and scaffolding tools for agent-driven action systems. Provides a framework for defining, registering, versioning, and executing agent skills across the Phenotype platform.

## Overview

**phenotype-skills** is the foundational skills framework enabling agents to extend their capabilities through composable, reusable action modules. Skills are versioned, discoverable, and sandboxed, allowing agents to safely execute third-party actions while maintaining security boundaries and resource limits.

**Core Mission**: Enable safe, extensible agent capabilities through a versioned, discoverable skills registry with automatic dependency resolution and sandboxed execution.

## Technology Stack

- **Skill Runtime**: Rust-based execution engine with WASM support
- **Registry**: Distributed skills registry with versioning and discovery
- **Definitions**: TOML-based skill metadata and interface definitions
- **Execution**: Sandbox isolation, resource limits, timeout management
- **Observability**: Skill execution tracing, error logging, performance metrics
- **Languages**: Rust (core), Python (SDK), JavaScript/TypeScript (SDK)

## Key Features

- **Skill Definitions**: Declarative TOML format for skill interface and metadata
- **Versioning**: Semantic versioning with compatibility checking
- **Execution Sandboxing**: WASM-based isolation, resource limits, timeouts
- **Dependency Management**: Automatic dependency resolution and conflict detection
- **Skill Discovery**: Registry-based discovery with metadata search
- **SDK Support**: Multi-language SDKs for skill implementation
- **Error Handling**: Rich error types with debugging context
- **Telemetry**: Execution tracing, metrics collection, performance insights

## Quick Start

```bash
# Clone and explore
git clone <repo-url>
cd phenotype-skills

# Review governance and architecture
cat CLAUDE.md          # Project governance
cat AGENTS.md          # Agent operating contract

# Build core runtime
cargo build --workspace

# Run tests
cargo test --workspace

# Create a new skill from template
./scripts/new-skill.sh --name "my-skill" --language python

# Register skill
cargo run --bin skill-register -- --skill my-skill --version 1.0.0

# View available skills
cargo run --bin skill-list -- --filter ai
```

## Project Structure

```
phenotype-skills/
├── src/
│   ├── runtime/
│   │   ├── executor.rs         # WASM executor and sandboxing
│   │   ├── environment.rs      # Execution environment setup
│   │   ├── sandbox.rs          # Resource limits and isolation
│   │   └── timeout.rs          # Execution timeout management
│   ├── registry/
│   │   ├── skill.rs            # Skill metadata and interface
│   │   ├── discovery.rs        # Skill discovery and search
│   │   ├── storage.rs          # Registry persistence
│   │   └── versioning.rs       # Version management and compatibility
│   ├── definition/
│   │   ├── parser.rs           # TOML skill definition parser
│   │   ├── schema.rs           # Skill definition schema
│   │   └── validator.rs        # Skill validation
│   ├── observability/
│   │   ├── tracing.rs          # Execution tracing
│   │   ├── metrics.rs          # Performance metrics
│   │   └── logging.rs          # Structured logging
│   └── lib.rs
├── sdks/
│   ├── rust/                   # Rust SDK for skill development
│   │   ├── src/
│   │   └── Cargo.toml
│   ├── python/                 # Python SDK
│   │   ├── phenotype_skills/
│   │   └── pyproject.toml
│   └── typescript/             # TypeScript SDK
│       ├── src/
│       └── package.json
├── skills/
│   ├── templates/              # Skill templates by language
│   │   ├── rust/
│   │   ├── python/
│   │   └── typescript/
│   └── builtins/               # Built-in skills
│       ├── http/               # HTTP request skill
│       ├── file_io/            # File I/O skill
│       ├── data_transform/     # Data transformation skill
│       └── ai_call/            # AI model calling skill
├── bin/
│   ├── skill-register/         # Register skills in registry
│   ├── skill-list/             # List available skills
│   ├── skill-execute/          # Execute a skill
│   └── skill-validate/         # Validate skill definitions
├── examples/
│   ├── http_skill.toml         # HTTP request skill example
│   ├── data_transform.toml     # Data transform skill
│   └── custom_skill.py         # Custom Python skill
├── tests/
│   ├── integration/
│   │   ├── execution_test.rs
│   │   └── registry_test.rs
│   └── fixtures/
├── docs/
│   ├── ARCHITECTURE.md         # Runtime and registry design
│   ├── SKILL_DEFINITION.md     # Skill definition syntax
│   ├── CREATING_SKILLS.md      # Guide to creating skills
│   ├── SDK_GUIDE.md            # SDK reference
│   └── EXECUTION.md            # Execution model and sandboxing
├── benches/
│   ├── execution_perf.rs
│   └── registry_lookup.rs
└── Cargo.toml
```

## Skill Definition Example

```toml
# skills/http-fetch.toml
[skill]
name = "http-fetch"
version = "1.0.0"
description = "Fetch data from HTTP endpoints"
author = "Phenotype Team"
license = "MIT"

[[inputs]]
name = "url"
type = "string"
required = true
description = "URL to fetch from"

[[inputs]]
name = "method"
type = "string"
required = false
default = "GET"
description = "HTTP method (GET, POST, etc)"

[[outputs]]
name = "response"
type = "string"
description = "HTTP response body"

[[outputs]]
name = "status_code"
type = "integer"
description = "HTTP status code"

[requirements]
timeout = "30s"
memory_limit = "256MB"
network_access = true
```

## Related Phenotype Projects

- **phenotype-task-engine**: Task scheduling (uses skills for action execution)
- **Tracera**: Observability (skill execution tracing and metrics)
- **AgilePlus**: Work tracking (can use skills for automation)
- **phenotype-ops-mcp**: MCP server (skill registry and execution endpoints)