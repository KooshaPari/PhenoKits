# Functional Requirements — phenotype-middleware-py

## FR-PROTO — Protocol

| ID | Requirement |
|----|-------------|
| FR-PROTO-001 | The system SHALL define a Middleware protocol with async call(ctx, next) -> Response signature. |
| FR-PROTO-002 | The system SHALL provide a sync-to-async wrapper for synchronous middleware. |
| FR-PROTO-003 | Middleware SHALL be able to short-circuit the chain and return a response early. |
| FR-PROTO-004 | Middleware SHALL have access to a mutable context object for cross-middleware communication. |

## FR-BUILTIN — Built-in Middleware

| ID | Requirement |
|----|-------------|
| FR-BUILTIN-001 | The system SHALL provide AuthMiddleware that validates bearer tokens and API keys. |
| FR-BUILTIN-002 | The system SHALL provide LoggingMiddleware with structured output and correlation IDs. |
| FR-BUILTIN-003 | The system SHALL provide TracingMiddleware that injects W3C TraceContext headers. |
| FR-BUILTIN-004 | The system SHALL provide RetryMiddleware with configurable backoff and jitter. |
| FR-BUILTIN-005 | The system SHALL provide RateLimitMiddleware using a token-bucket algorithm. |

## FR-PIPE — Pipeline

| ID | Requirement |
|----|-------------|
| FR-PIPE-001 | The system SHALL provide a Pipeline builder with a fluent use() API. |
| FR-PIPE-002 | Middleware SHALL execute in the order they are added to the pipeline. |
| FR-PIPE-003 | The system SHALL support conditional middleware that activates based on context predicates. |

## FR-INTEG — Integration

| ID | Requirement |
|----|-------------|
| FR-INTEG-001 | The system SHALL provide adapter base classes for FastAPI route handlers. |
| FR-INTEG-002 | The system SHALL provide adapter base classes for aiohttp middleware. |
| FR-INTEG-003 | Middleware pipelines SHALL attach to port adapters, not domain services. |
