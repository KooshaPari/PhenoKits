# Architecture Decision Records — phenotype-middleware-py

## ADR-001 — Protocol-Based Middleware Interface

**Status:** Accepted  
**Date:** 2026-03-27

### Context
Python lacks a standard middleware protocol. Django, Starlette, and aiohttp each define their own. A unified protocol avoids coupling to any framework.

### Decision
Define a `Middleware` protocol using `typing.Protocol`. The `call(ctx, next)` signature is framework-agnostic. Framework-specific adapters wrap it.

### Consequences
- Middleware implementations are portable across frameworks.
- Framework adapters are thin shims, testable independently.

---

## ADR-002 — Context Object for Cross-Middleware State

**Status:** Accepted  
**Date:** 2026-03-27

### Context
Middleware often needs to pass state downstream (e.g., authenticated user, trace ID). Thread-local storage is unsafe in async contexts.

### Decision
A `MiddlewareContext` object is created per request and passed explicitly through the chain. It is a typed dataclass with an `extras: dict` escape hatch.

### Consequences
- No global state; safe for asyncio.
- Downstream middleware and handlers access context via the context object.

---

## ADR-003 — Hexagonal Boundary Enforcement

**Status:** Accepted  
**Date:** 2026-03-27

### Context
Middleware implements cross-cutting infrastructure concerns. Placing it in the domain or application layer would violate hexagonal boundaries.

### Decision
Middleware is an infrastructure-layer concern. It attaches to port adapters (inbound adapters) only. Domain and application layers have no middleware imports.

### Consequences
- `import-linter` rules enforce that domain/application layers cannot import middleware.
- This rule is checked in CI.
