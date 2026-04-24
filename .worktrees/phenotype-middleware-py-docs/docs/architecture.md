# Architecture Guide

This guide explains the hexagonal architecture of `phenotype-middleware-py` and how the different layers interact.

## Hexagonal Architecture Overview

The library follows **hexagonal architecture** (also known as ports and adapters), which separates concerns into concentric layers:

```mermaid
flowchart TB
    subgraph Infrastructure["🔌 Infrastructure Layer"]
        direction TB
        Auth[AuthMiddleware]
        Trace[TracingMiddleware]
        Retry[RetryMiddleware]
        RateLimit[RateLimitMiddleware]
        LogAdapter[StdoutLoggingAdapter]
        MetricsAdapter[PrometheusMetricsAdapter]
    end

    subgraph Ports["🔌 Ports (Interfaces)"]
        direction TB
        MP[MiddlewarePort]
        LP[LoggingPort]
        MetP[MetricsPort]
        AP[AuthPort]
    end

    subgraph Application["⚙️ Application Layer"]
        MC[MiddlewareChain]
    end

    subgraph Domain["💎 Domain Layer"]
        direction TB
        Req[Request]
        Res[Response]
        MR[MiddlewareResult]
        Err[Error Hierarchy]
    end

    Infrastructure -->|implements| Ports
    Ports -->|depends on| Domain
    Application -->|orchestrates| Ports
    Application -->|uses| Domain
```

## Layer Responsibilities

### Domain Layer (`domain/`)

The innermost layer containing pure business logic with zero external dependencies.

**Contains:**
- `Request` - Immutable request object with context
- `Response` - Mutable response builder
- `MiddlewareResult` - Result pattern for explicit error handling
- `MiddlewareError`, `PipelineError`, `AdapterError` - Exception hierarchy

**Principles:**
- **Immutable data**: `Request` is frozen to prevent accidental mutation
- **Explicit errors**: `MiddlewareResult` forces handling of success/failure
- **No dependencies**: No imports from other layers

### Application Layer (`application/`)

Contains use cases that orchestrate domain objects.

**Contains:**
- `MiddlewareChain` - Orchestrates middleware execution

**Responsibilities:**
- Executes middleware in order
- Handles errors and error handlers
- Manages request/response flow
- Short-circuits when response is returned early

### Ports Layer (`ports/`)

Defines interface contracts (abstract base classes) that adapters must implement.

**Contains:**
- `MiddlewarePort` - Base interface for all middleware
- `LoggingPort` - Interface for logging adapters
- `MetricsPort` - Interface for metrics adapters
- `AuthPort` - Interface for authentication mechanisms

**Purpose:**
- Enables swapping implementations without changing domain/application code
- Facilitates contract testing
- Supports multiple backends (stdout vs file logging, Prometheus vs StatsD)

### Infrastructure Layer (`infrastructure/`)

Contains concrete implementations of ports and external service adapters.

**Contains:**
- Adapters: `StdoutLoggingAdapter`, `PrometheusMetricsAdapter`
- Middleware: `LoggingMiddleware`, `MetricsMiddleware`
- Built-in: `AuthMiddleware`, `TracingMiddleware`, `RetryMiddleware`, `RateLimitMiddleware`

## Data Flow

```mermaid
sequenceDiagram
    participant Client
    participant Chain as MiddlewareChain
    participant M1 as AuthMiddleware
    participant M2 as LoggingMiddleware
    participant M3 as MetricsMiddleware
    participant Handler as Request Handler

    Client->>Chain: handle(request)
    
    Chain->>M1: process(request)
    M1-->>Chain: MiddlewareResult(success, request')
    
    Chain->>M2: process(request')
    M2->>M2: Log request
    M2-->>Chain: MiddlewareResult(success, request'')
    
    Chain->>M3: process(request'')
    M3->>M3: Record metrics
    M3-->>Chain: MiddlewareResult(success, request''')
    
    Chain->>Handler: Final request
    Handler-->>Chain: Response
    
    Chain-->>Client: MiddlewareResult with response
```

## Error Handling Flow

```mermaid
sequenceDiagram
    participant Client
    participant Chain as MiddlewareChain
    participant M1 as FailingMiddleware
    participant EH as ErrorHandler

    Client->>Chain: handle(request)
    
    Chain->>M1: process(request)
    M1-->>Chain: MiddlewareResult(error="Database timeout")
    
    alt Error handler configured
        Chain->>EH: process(error_request)
        EH-->>Chain: MiddlewareResult(success, response)
        Chain-->>Client: Success with error response
    else No error handler
        Chain-->>Client: Error result
    end
```

## Short-Circuit Pattern

Middleware can short-circuit the chain by returning a response:

```mermaid
sequenceDiagram
    participant Client
    participant Chain as MiddlewareChain
    participant Auth as AuthMiddleware
    participant Log as LoggingMiddleware
    participant Handler as Request Handler

    Client->>Chain: handle(request)
    
    Chain->>Auth: process(request)
    Auth->>Auth: Validate token
    Note over Auth: Invalid token!
    Auth-->>Chain: MiddlewareResult(response=401)
    
    Note over Chain: Short-circuit!<br/>Skip remaining middleware
    
    Chain-->>Client: 401 response
```

## Adding Custom Middleware

To add custom middleware, implement `MiddlewarePort`:

```python
from phenotype_middleware.ports import MiddlewarePort
from phenotype_middleware.domain import Request, MiddlewareResult

class CustomMiddleware(MiddlewarePort):
    async def process(self, request: Request) -> MiddlewareResult:
        # Your logic here
        modified = request.with_context("custom_key", "value")
        return MiddlewareResult.ok(request=modified)
```

## Testing Architecture

The architecture supports testing at multiple levels:

```mermaid
flowchart LR
    subgraph Contract["Contract Tests"]
        CT1[Test LoggingPort adapters]
        CT2[Test MetricsPort adapters]
        CT3[Test MiddlewarePort implementations]
    end
    
    subgraph Unit["Unit Tests"]
        UT1[Test domain models]
        UT2[Test middleware in isolation]
    end
    
    subgraph Integration["Integration Tests"]
        IT1[Test chain with real middleware]
        IT2[Test error handlers]
    end
```

- **Contract tests**: Verify adapters implement ports correctly
- **Unit tests**: Test individual components in isolation
- **Integration tests**: Test complete chains end-to-end

## Design Patterns Used

| Pattern | Usage |
|---------|-------|
| **Port/Adapter** | Decouple domain from infrastructure |
| **Result** | Explicit error handling with `MiddlewareResult` |
| **Chain of Responsibility** | Middleware execution order |
| **Builder** | `MiddlewareChain` fluent API |
| **Factory** | `MiddlewareResult.ok()` / `err()` |
| **Immutable** | `Request` frozen dataclass |

## xDD Methodologies

The codebase follows several xDD practices:

| Methodology | Implementation |
|-------------|----------------|
| **TDD** | Tests written before/parallel to implementation |
| **BDD** | Given-When-Then scenario naming in tests |
| **DDD** | Domain models with bounded context |
| **CDD** | Port/Adapter contract verification |

## Performance Considerations

- **Request immutability**: Creates new objects but enables safe caching
- **Async/await**: Non-blocking I/O for logging and metrics
- **In-memory metrics**: Fast collection with optional external flush
- **Lazy evaluation**: Error handlers only invoked on errors
