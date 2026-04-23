# ADR-001: phenotype-middleware-py Architecture

**Status**: Accepted
**Date**: 2026-03-25

## Context

We need a Python library for reusable middleware patterns that can be shared across Phenotype ecosystem projects. The library must follow hexagonal/clean architecture principles and support xDD methodologies.

## Decision

Create `phenotype-middleware-py` with the following architecture:

### Layered Structure

```
src/phenotype_middleware/
├── domain/           # Pure business logic (no dependencies)
│   └── models.py    # Request, Response, MiddlewareResult, Errors
├── application/      # Use cases
│   └── __init__.py  # MiddlewareChain orchestrator
├── ports/           # Interface definitions (ABC)
│   └── __init__.py  # MiddlewarePort, LoggingPort, MetricsPort, AuthPort
└── infrastructure/  # External adapters
    └── __init__.py  # StdoutLoggingAdapter, PrometheusMetricsAdapter
```

### xDD Methodologies

| Category | Methodology | Implementation |
|----------|-------------|----------------|
| **Development** | TDD | Contract tests before adapters |
| **Development** | BDD | Given-When-Then scenario naming |
| **Development** | DDD | Domain models with bounded context |
| **Development** | CDD | Port/Adapter contract verification |
| **Design** | SOLID | Interface segregation, dependency inversion |
| **Design** | KISS | Simple port interfaces |
| **Design** | PoLA | Descriptive error messages with context |

## Consequences

### Positive
- Reusable middleware across Python projects
- Clear separation of concerns
- Testable domain logic
- Contract tests ensure adapter compliance

### Negative
- Abstract base classes add complexity
- Requires Python 3.10+ for modern typing

## References

- [Hexagonal Architecture](http://alistair.cockburn.us/hexagonal+architecture)
- [Python ABC](https://docs.python.org/3/library/abc.html)
- [Hypothesis Property Testing](https://hypothesis.readthedocs.io/)
