# phenotype-middleware-py

[![PyPI](https://img.shields.io/badge/pypi-phenotype--middleware-blue)](https://pypi.org/project/phenotype-middleware/)
[![Python](https://img.shields.io/badge/python-3.10%2B-blue)](./pyproject.toml)
[![License](https://img.shields.io/badge/license-MIT-green)](./LICENSE)
[![Tests](https://img.shields.io/badge/tests-138%20passing-brightgreen)](./tests/)
[![Coverage](https://img.shields.io/badge/coverage-96%25-brightgreen)](./TEST_COVERAGE_MATRIX.md)
[![Ruff](https://img.shields.io/badge/ruff-passing-brightgreen)](./pyproject.toml)
[![MyPy](https://img.shields.io/badge/mypy-passing-brightgreen)](./pyproject.toml)

Python **middleware** patterns aligned with **hexagonal** architecture (domain / application / ports / infrastructure).

## Features

- **Hexagonal Architecture**: Clean separation between domain logic, application services, ports (interfaces), and infrastructure (implementations)
- **Async-First**: Built for modern Python with full async/await support
- **Type-Safe**: Fully typed with MyPy compliance
- **Composable**: Chain multiple middleware with predictable execution order
- **Production-Ready**: Includes authentication, tracing, retry, rate-limiting, caching, and compression middleware

## Installation

```bash
pip install phenotype-middleware
```

## Quick Start

```python
from phenotype_middleware import MiddlewareChain, Request, Response
from phenotype_middleware.infrastructure import (
    LoggingMiddleware, MetricsMiddleware, 
    AuthMiddleware, CacheMiddleware, CompressionMiddleware
)

# Create a chain
chain = MiddlewareChain([
    LoggingMiddleware(),
    AuthMiddleware(validator=validate_token),
    CacheMiddleware(ttl_seconds=300),
    CompressionMiddleware(min_size=1024),
    MetricsMiddleware()
])

# Execute
request = Request(path="/api/users", method="GET")
result = await chain.execute(request)

if result.response:
    print(f"Status: {result.response.status_code}")
```

## Documentation

- **[Getting Started](./docs/getting-started.md)** — Installation, development setup, and first steps
- **[API Reference](./docs/api-reference.md)** — Complete API documentation
- **[Examples](./docs/examples.md)** — Usage patterns and common scenarios
- **[Architecture](./docs/architecture.md)** — Hexagonal design and mermaid diagrams
- **[Advanced Patterns](./docs/advanced-patterns.md)** — Custom middleware, error handling, testing

Live documentation site: (run `npm run docs:dev` locally)

## Project Structure

```
src/phenotype_middleware/
├── domain/          # Core business logic (models, errors)
├── application/     # Use cases (chain orchestration)
├── ports/           # Abstract interfaces (ABC definitions)
└── infrastructure/  # Concrete implementations
```

| Layer | Purpose | Example |
|-------|---------|---------|
| **Domain** | Business entities and rules | `Request`, `Response`, `MiddlewareResult` |
| **Application** | Orchestrate use cases | `MiddlewareChain` execution flow |
| **Ports** | Define interfaces | `MiddlewarePort`, `LoggingPort` |
| **Infrastructure** | Concrete implementations | `LoggingMiddleware`, `StdoutLoggingAdapter` |

## Built-in Middleware

| Middleware | Purpose | Key Features |
|------------|---------|--------------|
| `LoggingMiddleware` | Request logging | Configurable log levels, custom formatters |
| `MetricsMiddleware` | Request metrics | Counts, durations, customizable labels |
| `AuthMiddleware` | Bearer token validation | Custom validators, configurable headers |
| `TracingMiddleware` | Distributed tracing | Correlation IDs, W3C TraceContext format |
| `RetryMiddleware` | Resilient requests | Exponential backoff, jitter, max attempts |
| `RateLimitMiddleware` | Request throttling | Token bucket, per-client tracking |
| `CacheMiddleware` | Response caching | In-memory, TTL, customizable keys |
| `CompressionMiddleware` | Response compression | Gzip/deflate, min size threshold |
| `ConditionalMiddleware` | Predicate-based | Dynamic middleware inclusion |
| `SyncMiddlewareAdapter` | Sync wrapper | Use sync middleware in async chain |

## Development

```bash
# Clone and setup
git clone <repo>
cd phenotype-middleware-py
pip install -e ".[dev]"

# Run tests
pytest

# Run with coverage
pytest --cov=src --cov-report=term

# Quality checks
ruff check src tests
mypy src

# Documentation
cd docs && npm install && npm run docs:dev
```

## Architecture

See:
- [ADR-001: Architecture Decision](./adr/ADR-001-architecture.md) — Hexagonal design rationale
- [ADR-002: Framework Adapters](./.agileplus/specs/001-middleware-core/adr/ADR-002-framework-adapters.md) — Postponement rationale
- [FUNCTIONAL_REQUIREMENTS.md](./FUNCTIONAL_REQUIREMENTS.md) — Full requirement specifications
- [TEST_COVERAGE_MATRIX.md](./TEST_COVERAGE_MATRIX.md) — Test coverage (96%)

## Package Info

- **PyPI name**: `phenotype-middleware` (see `pyproject.toml`)
- **Python**: ≥ 3.10
- **License**: MIT
- **Test Coverage**: 96% (138 tests)

## Contributing

Contributions welcome! See [Work Packages](./.agileplus/specs/001-middleware-core/work-packages.md) for active and planned work.

## License

MIT
