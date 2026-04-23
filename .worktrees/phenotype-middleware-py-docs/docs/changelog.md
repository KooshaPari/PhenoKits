# Changelog

All notable changes to `phenotype-middleware-py` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- FR-INTEG-001: FastAPI adapter (planned)
- FR-INTEG-002: aiohttp adapter (planned)
- Enhanced W3C TraceContext support in TracingMiddleware

## [0.2.0] - 2026-04-02

### Added

#### New Middleware
- `AuthMiddleware` - Bearer token validation with custom validators
- `TracingMiddleware` - Correlation ID injection for distributed tracing
- `RetryMiddleware` - Exponential backoff with configurable jitter
- `RateLimitMiddleware` - Token bucket algorithm per-client
- `SyncMiddlewareAdapter` - Sync-to-async wrapper (FR-PROTO-002)
- `ConditionalMiddleware` - Predicate-based activation (FR-PIPE-003)

#### Documentation
- Complete API reference with all classes and methods
- Architecture guide with mermaid diagrams
- Advanced patterns guide with composite middleware, circuit breaker, caching
- Examples for FastAPI integration
- Updated VitePress configuration with search and navigation

#### Testing
- Unit tests for all domain models (15 tests)
- Error handling tests for middleware chain (13 tests)
- Built-in middleware tests (19 tests)
- 100% test coverage on implemented code

#### Quality
- All ruff lint checks pass
- All mypy type checks pass
- Fixed `datetime.utcnow()` deprecation warning
- Added `__all__` exports for clean public API

### Changed
- Updated SPEC.md with FR traceability matrix
- Updated AgilePlus specs with completed work tracking
- Enhanced TEST_COVERAGE_MATRIX with detailed mapping
- Refactored infrastructure module for cleaner imports

## [0.1.0] - 2026-03-25

### Added
- Initial release
- Core hexagonal architecture (domain/application/ports/infrastructure)
- `MiddlewareChain` orchestrator
- `Request`, `Response`, `MiddlewareResult` domain models
- `MiddlewarePort`, `LoggingPort`, `MetricsPort`, `AuthPort` interfaces
- `StdoutLoggingAdapter`, `PrometheusMetricsAdapter` implementations
- `LoggingMiddleware`, `MetricsMiddleware` built-in middleware
- Contract tests for port/adapter verification
- Basic VitePress documentation
- ADR-001: Architecture decision record

### Features
- Async/await support throughout
- Immutable request objects with context
- Result pattern for explicit error handling
- Error handler support in middleware chain
- Short-circuit capability for early responses

---

## Migration Guide

### From 0.1.0 to 0.2.0

#### New Imports Available
```python
# New middleware available
from phenotype_middleware.infrastructure import (
    AuthMiddleware,
    TracingMiddleware,
    RetryMiddleware,
    RateLimitMiddleware,
    SyncMiddlewareAdapter,
    ConditionalMiddleware,
)
```

#### Breaking Changes
None - 0.2.0 is fully backwards compatible with 0.1.0

#### New Capabilities
1. **Authentication**: Add `AuthMiddleware` to validate tokens
2. **Tracing**: Add `TracingMiddleware` for correlation IDs
3. **Rate Limiting**: Add `RateLimitMiddleware` to prevent abuse
4. **Retry Logic**: Add `RetryMiddleware` for resilient calls
5. **Conditional**: Use `ConditionalMiddleware` for route-specific logic
6. **Sync Support**: Wrap sync middleware with `SyncMiddlewareAdapter`

---

## Release Schedule

| Version | Target Date | Focus |
|---------|-------------|-------|
| 0.3.0 | Q2 2026 | FastAPI/aiohttp adapters, W3C TraceContext |
| 0.4.0 | Q3 2026 | Performance optimizations, caching enhancements |
| 1.0.0 | Q4 2026 | Stable release with full feature set |

---

## Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for how to propose changes.

## License

MIT - See [LICENSE](../LICENSE) for details.
