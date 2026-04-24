# Examples

## Basic Middleware Chain

```python
import asyncio
from phenotype_middleware.domain import Request
from phenotype_middleware.application import MiddlewareChain
from phenotype_middleware.infrastructure import (
    StdoutLoggingAdapter,
    PrometheusMetricsAdapter,
    LoggingMiddleware,
    MetricsMiddleware,
)

async def main():
    # Create chain
    chain = MiddlewareChain()

    # Add logging
    logger = StdoutLoggingAdapter()
    chain.add(LoggingMiddleware(logger))

    # Add metrics
    metrics = PrometheusMetricsAdapter()
    chain.add(MetricsMiddleware(metrics))

    # Process request
    request = Request(path="/api/users", method="GET")
    result = await chain.handle(request)

    print(f"Success: {result.success}")
    print(f"Metrics: {metrics.get_counter('requests_total')}")

asyncio.run(main())
```

---

## Custom Middleware

```python
from phenotype_middleware.domain import Request, Response, MiddlewareResult
from phenotype_middleware.ports import MiddlewarePort

class AuthMiddleware(MiddlewarePort):
    """Simple token-based auth middleware."""

    def __init__(self, valid_tokens: set[str]) -> None:
        self.valid_tokens = valid_tokens

    async def process(self, request: Request) -> MiddlewareResult:
        auth_header = request.headers.get("Authorization", "")
        token = auth_header.replace("Bearer ", "")

        if token not in self.valid_tokens:
            response = Response(status_code=401)
            response.set_body(b'{"error": "Unauthorized"}')
            return MiddlewareResult.ok(request=request, response=response)

        # Add user context and continue
        modified = request.with_context("user", token)
        return MiddlewareResult.ok(request=modified)


# Usage
chain = MiddlewareChain()
chain.add(AuthMiddleware({"token-123", "token-456"}))
chain.add(LoggingMiddleware(StdoutLoggingAdapter()))
```

---

## Error Handling

```python
from phenotype_middleware.domain import MiddlewareResult

class ErrorLoggingMiddleware:
    """Logs errors and transforms them into responses."""

    def __init__(self, logger: LoggingPort) -> None:
        self.logger = logger

    async def process(self, request: Request) -> MiddlewareResult:
        # Check if this is an error context
        if "error" in request.context:
            error = request.context["error"]
            await self.logger.log("ERROR", f"Request failed: {error}")

            response = Response(status_code=500)
            response.set_body(f'{{"error": "{error}"}}'.encode())
            return MiddlewareResult.ok(request=request, response=response)

        # Pass through
        return MiddlewareResult.ok(request=request)


# Setup with error handler
chain = MiddlewareChain()
chain.add(SomeRiskyMiddleware())
chain.add_error_handler(ErrorLoggingMiddleware(logger))
```

---

## Request/Response Modification

```python
class AddTimestampMiddleware(MiddlewarePort):
    """Adds timestamp to request context."""

    async def process(self, request: Request) -> MiddlewareResult:
        from datetime import datetime, timezone
        timestamp = datetime.now(timezone.utc).isoformat()
        modified = request.with_context("timestamp", timestamp)
        return MiddlewareResult.ok(request=modified)


class AddRequestIDMiddleware(MiddlewarePort):
    """Adds unique request ID."""

    def __init__(self) -> None:
        self.counter = 0

    async def process(self, request: Request) -> MiddlewareResult:
        self.counter += 1
        request_id = f"req-{self.counter:06d}"
        modified = request.with_context("request_id", request_id)
        return MiddlewareResult.ok(request=modified)


# Chain them together
chain = MiddlewareChain()
chain.add(AddRequestIDMiddleware())
chain.add(AddTimestampMiddleware())
```

---

## Short-Circuit (Early Response)

```python
class RateLimitMiddleware(MiddlewarePort):
    """Returns 429 if rate limit exceeded."""

    def __init__(self, max_requests: int = 100) -> None:
        self.max_requests = max_requests
        self.requests: dict[str, int] = {}

    async def process(self, request: Request) -> MiddlewareResult:
        client_id = request.headers.get("X-Client-ID", "anonymous")
        count = self.requests.get(client_id, 0)

        if count >= self.max_requests:
            response = Response(status_code=429)
            response.set_body(b'{"error": "Rate limit exceeded"}')
            # Short-circuit: return response without calling rest of chain
            return MiddlewareResult.ok(request=request, response=response)

        self.requests[client_id] = count + 1
        return MiddlewareResult.ok(request=request)
```

---

## Complete Application Example

```python
from fastapi import FastAPI
from phenotype_middleware.application import MiddlewareChain
from phenotype_middleware.domain import Request, MiddlewareResult
from phenotype_middleware.infrastructure import (
    StdoutLoggingAdapter,
    PrometheusMetricsAdapter,
    LoggingMiddleware,
    MetricsMiddleware,
)

# Create FastAPI app
app = FastAPI()

# Create middleware chain
chain = MiddlewareChain()
chain.add(LoggingMiddleware(StdoutLoggingAdapter()))
chain.add(MetricsMiddleware(PrometheusMetricsAdapter()))

# Middleware adapter for FastAPI
@app.middleware("http")
async def phenotype_middleware(request, call_next):
    # Convert FastAPI request to phenotype Request
    pheno_request = Request(
        path=str(request.url.path),
        method=request.method,
        headers=dict(request.headers),
    )

    # Process through chain
    result = await chain.handle(pheno_request)

    # If chain returned response, use it
    if result.response:
        from fastapi.responses import Response as FastAPIResponse
        return FastAPIResponse(
            content=result.response.body,
            status_code=result.response.status_code,
            headers=result.response.headers,
        )

    # Otherwise continue to handler
    return await call_next(request)

@app.get("/health")
async def health():
    return {"status": "healthy"}
```
