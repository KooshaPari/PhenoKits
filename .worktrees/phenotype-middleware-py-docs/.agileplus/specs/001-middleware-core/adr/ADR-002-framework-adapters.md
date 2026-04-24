# ADR-002: Framework Adapter Postponement

## Status

Proposed

## Context

The Middleware Core library implements a hexagonal architecture with ports and adapters. Two requirements (FR-INTEG-001 and FR-INTEG-002) specify FastAPI and aiohttp adapters.

## Decision

We will defer implementation of framework-specific adapters (FastAPI and aiohttp) until Q2 2026, prioritizing:

1. Core middleware implementations (Auth, Tracing, Retry, RateLimit)
2. Comprehensive documentation and examples
3. Performance validation and benchmarking

## Rationale

### Why Postpone?

1. **Clean Architecture Principle**: Framework adapters belong in a separate layer from core. They should be optional extensions, not core dependencies.

2. **Dependency Management**: Framework adapters require adding framework-specific dependencies (FastAPI, aiohttp, Starlette, etc.) which increase the library's footprint and maintenance burden.

3. **Community Contribution Opportunity**: These are well-defined integration tasks that don't require deep knowledge of the middleware core. They can be developed in parallel by contributors.

4. **User Empowerment**: The current `MiddlewareChain` API is simple enough for users to integrate directly with their frameworks using just a few lines of code.

### Current State

Users can already integrate today:

```python
# FastAPI example (current capability)
from fastapi import Request, Response
from phenotype_middleware import MiddlewareChain

@app.middleware("http")
async def middleware_chain(request: Request, call_next):
    pm_request = Request(
        path=str(request.url.path),
        method=request.method,
        headers=dict(request.headers),
        body=await request.body()
    )
    result = await chain.execute(pm_request)
    if result.response:
        return Response(
            content=result.response.body,
            status_code=result.response.status_code,
            headers=result.response.headers
        )
    return await call_next(request)
```

## Consequences

### Positive

- Core library stays lightweight
- No forced framework dependencies
- Clear extension point for community

### Negative

- Slightly more boilerplate for users
- Need to maintain separate adapter packages (eventually)

## Timeline

| Milestone | Target |
|-----------|--------|
| Propose FastAPI adapter | Q2 2026 |
| Propose aiohttp adapter | Q2 2026 |
| Community contribution window | Q2 2026 |
| Official adapter packages | Q3 2026 |

## Related

- FR-INTEG-001: FastAPI integration requirement
- FR-INTEG-002: aiohttp integration requirement
- WP-010: FastAPI work package
- WP-011: aiohttp work package
