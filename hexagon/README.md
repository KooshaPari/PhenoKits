# hexagon

**Phenotype Hexagonal Architecture Pattern Library**

A foundational reference library implementing hexagonal (ports-and-adapters) architecture patterns for the Phenotype ecosystem. Hexagon provides reusable abstractions for decoupling domain logic from infrastructure concerns, enabling modular, testable, and framework-independent application design.

## Overview

Hexagon codifies the hexagonal architecture pattern across the Phenotype organization, providing:
- **Port definitions** — Clear contracts for domain-to-infrastructure boundaries
- **Adapter implementations** — Standard adapters for databases, queues, web frameworks, and observability
- **Testing harnesses** — Mocks and in-memory implementations for isolated unit testing
- **Documentation** — Reference implementations and architectural decision records

The library supports polyrepo decomposition and shared infrastructure without creating central dependencies.

## Technology Stack

- **Language**: Rust, Go, Python, TypeScript (multi-language reference)
- **Core Abstraction**: Port/Adapter pattern
- **Key Dependencies**: serde, tokio (Rust); standard lib (Go); pydantic (Python)
- **Architecture Style**: Domain-Driven Design (DDD) with ports-and-adapters separation

## Key Features

- Port trait definitions and lifecycle management
- Reference adapter implementations (SQLite, PostgreSQL, HTTP, gRPC, Kafka)
- In-memory test doubles for all adapters
- Observability integration (tracing, metrics, logging via ports)
- Configuration-driven adapter selection
- Multi-language support (Rust primary, Go/Python/TS secondary)
- Comprehensive testing patterns and fixtures

## Quick Start

```bash
# Clone and navigate
git clone https://github.com/KooshaPari/phenotype-infrakit.git
cd phenotype-infrakit

# Review architecture principles
cat docs/HEXAGON_ARCHITECTURE.md

# Check reference implementations
ls crates/phenotype-*-core/

# Run tests (Rust)
cargo test --workspace --lib

# Review related projects
cat RELATED_PROJECTS.md
```

## Project Structure

```
hexagon/
├── docs/
│   ├── HEXAGON_ARCHITECTURE.md     # Pattern overview and rationale
│   ├── PORT_DESIGN_GUIDE.md        # How to design new ports
│   ├── ADAPTER_CHECKLIST.md        # Adapter implementation requirements
│   └── examples/                   # Reference implementations
├── reference-ports/
│   ├── storage-port.md             # Database port contract
│   ├── async-port.md               # Async execution port
│   └── observability-port.md       # Tracing/metrics port
├── test-doubles/
│   ├── in-memory-store/            # Test storage implementation
│   ├── mock-http-client/           # Mock HTTP adapter
│   └── fixtures/                   # Shared test data
└── GOVERNANCE.md                   # Maintenance and evolution policy
```

## Related Phenotype Projects

- **HexaKit** — Comprehensive toolkit wrapping hexagon reference patterns into reusable crates
- **phenotype-infrakit** — Monorepo housing core hexagon implementations and adapters
- **phenotype-shared** — Shared generic infrastructure extracted from hexagon reference

## Governance & Maintenance

Hexagon is maintained as a shared reference in the Phenotype ecosystem. All changes must preserve backward compatibility with active consumers. See `GOVERNANCE.md` for architectural evolution policy.

- **Specification**: See `FUNCTIONAL_REQUIREMENTS.md` for port/adapter contracts
- **Worklog**: Tracking decisions and evolution in `/docs/worklogs/HEXAGON.md`
- **Decision Records**: Architecture decisions in `docs/adr/` (ADR-001, ADR-002, ...)

For new work, check AgilePlus:
```bash
cd /repos/AgilePlus && agileplus status --project hexagon
```

---

**Last Updated**: 2026-04-25
