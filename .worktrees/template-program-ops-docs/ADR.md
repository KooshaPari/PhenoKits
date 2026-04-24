# ADR — template-program-ops

## ADR-001: Python Version
**Status:** Accepted
**Context:** Python version for operations tooling.
**Decision:** Target Python 3.11+ for all generated projects.
**Rationale:** Python 3.11 has performance improvements and better async support.

## ADR-002: CLI Framework
**Status:** Accepted
**Context:** Multiple CLI frameworks exist.
**Decision:** Use Typer for CLI applications. Rich for formatted output.
**Rationale:** Typer provides excellent type safety; Rich makes beautiful output.

## ADR-003: Configuration
**Status:** Accepted
**Context:** Configuration must be flexible and secure.
**Decision:** Use TOML for config files. Environment variables for secrets. Pydantic for validation.
**Rationale:** TOML is human-readable; Pydantic validates automatically.

## ADR-004: Async Operations
**Status:** Accepted
**Context:** Operations often involve multiple concurrent tasks.
**Decision:** Use asyncio for I/O-bound operations. ThreadPoolExecutor for CPU-bound.
**Rationale:** Async enables efficient concurrency; thread pool for blocking operations.

## ADR-005: Logging
**Status:** Accepted
**Context:** Operations tooling needs structured logging.
**Decision:** Use structlog for structured JSON logs. Log levels for different severity.
**Rationale:** Structured logs enable easy parsing; structlog is fast and flexible.

## ADR-006: Testing
**Status:** Accepted
**Context:** Operations tooling needs reliable tests.
**Decision:** Use pytest with pytest-asyncio. pytest-mock for mocking.
**Rationale:** pytest is the standard; pytest-asyncio for async tests.
