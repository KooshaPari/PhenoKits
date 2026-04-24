# PRD — phenotype-middleware-py

## Overview

`phenotype-middleware-py` provides composable Python middleware patterns aligned with hexagonal architecture. It supplies request/response interceptors, context propagation, retry logic, and pipeline builders for use in Phenotype services.

## Goals

- Provide reusable middleware primitives that work across HTTP, gRPC, and message-bus adapters.
- Enable cross-cutting concerns (auth, logging, tracing, retry) to be applied declaratively.
- Keep domain layers free of infrastructure concerns by injecting middleware at the adapter boundary.

## Epics

### E1 — Middleware Protocol
- E1.1 Define a `Middleware` protocol: `async def call(ctx, next) -> Response`.
- E1.2 Support synchronous middleware via a sync wrapper.
- E1.3 Allow middleware to short-circuit the chain and return early.

### E2 — Built-in Middleware
- E2.1 Authentication: validate bearer tokens or API keys.
- E2.2 Logging: structured request/response logging with correlation IDs.
- E2.3 Tracing: inject and propagate W3C TraceContext headers.
- E2.4 Retry: configurable retry with exponential backoff and jitter.
- E2.5 Rate limiting: token-bucket rate limiter.

### E3 — Pipeline Builder
- E3.1 Fluent API: `Pipeline().use(AuthMiddleware).use(LoggingMiddleware).build()`.
- E3.2 Middleware ordering is explicit and deterministic.
- E3.3 Pipelines are serializable to a config representation.

### E4 — Hexagonal Integration
- E4.1 Middleware pipelines attach to port adapters, not domain services.
- E4.2 Provide adapter base classes for FastAPI, aiohttp, and message consumers.

## Acceptance Criteria

- A pipeline with auth + logging + tracing middleware processes a request end-to-end.
- Middleware can be added/removed without changing domain or application layer code.
- All built-in middleware have unit tests with no real I/O.
