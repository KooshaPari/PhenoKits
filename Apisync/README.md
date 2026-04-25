# Apisync

**API Synchronization & Orchestration Framework** — A Phenotype ecosystem component for decoupled API state management and cross-service synchronization.

## Overview

Apisync provides a lightweight, asynchronous API synchronization framework designed for distributed systems requiring reliable state propagation across service boundaries. Built as part of the Phenotype ecosystem, it enables event-driven architecture patterns with minimal configuration and maximum observability.

**Core Mission**: Enable services to maintain eventual consistency through declarative sync rules and automated conflict resolution.

## Technology Stack

- **Language**: Python 3.11+
- **Key Dependencies**: AsyncIO, Pydantic, SQLAlchemy
- **Architecture**: Event-driven, hexagonal ports & adapters
- **Testing**: pytest with async support, coverage tracking
- **Deployment**: Container-native (Docker, Kubernetes ready)

## Key Features

- **Declarative Sync Rules**: Define synchronization behavior in configuration, not code
- **Event-Driven Architecture**: Pub/sub patterns with configurable delivery guarantees
- **Conflict Resolution**: Automatic and manual strategies for concurrent updates
- **State Tracking**: Full audit log of all mutations with timestamp and source tracking
- **Rate Limiting & Backoff**: Built-in resilience with exponential backoff strategies
- **Observability**: Structured logging, metrics export, distributed tracing support
- **Testing Utilities**: Fixtures, mocks, and factories for rapid integration testing

## Quick Start

```bash
# Clone and setup
git clone https://github.com/KooshaPari/Phenotype repos/Apisync
cd Apisync

# Review governance context
cat CLAUDE.md

# Install dependencies
pip install -e .

# Run tests
pytest tests/ -v --cov=apisync

# Validate governance & quality
python3 validate_governance.py
task quality
```

## Project Structure

```
apisync/
├── core/                      # Core sync engine and primitives
│   ├── engine.py             # Main sync orchestrator
│   ├── rules.py              # Sync rule definitions
│   └── models.py             # Data models
├── adapters/                 # Port implementations
│   ├── storage/              # State persistence (SQLAlchemy)
│   ├── transport/            # Message transport (RabbitMQ, Kafka)
│   └── conflict/             # Conflict resolution strategies
├── cli/                      # Command-line interface
├── tests/                    # Comprehensive test suite
└── docs/                     # Architecture, guides, reference
```

## Related Phenotype Projects

- **Tracera** — Distributed tracing and observability platform
- **phenotype-bus** — Event bus and messaging infrastructure
- **PhenoObservability** — Monitoring and alerting stack

## Governance & Documentation

- [CLAUDE.md](./CLAUDE.md) - AI context and development guidelines
- [PRD.md](./PRD.md) - Product requirements and user stories
- [ARCHITECTURE.md](./ARCHITECTURE.md) - System design and component interactions
- [FUNCTIONAL_REQUIREMENTS.md](./FUNCTIONAL_REQUIREMENTS.md) - FR tracability and acceptance criteria

## License

Apache 2.0 — Part of the Phenotype organization ecosystem.
