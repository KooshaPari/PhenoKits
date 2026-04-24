# API Reference

## Domain Models

### Request

Immutable request object passed through the middleware chain.

```python
@dataclass(frozen=True)
class Request:
    path: str
    method: str
    headers: dict[str, str] = field(default_factory=dict)
    body: bytes | None = None
    context: dict[str, Any] = field(default_factory=dict)
```

**Methods:**

| Method | Description |
|--------|-------------|
| `with_context(key, value) -> Request` | Returns new request with added context (immutable) |

**Example:**

```python
from phenotype_middleware.domain import Request

request = Request(
    path="/api/users",
    method="POST",
    headers={"Content-Type": "application/json"},
    body=b'{"name": "John"}',
    context={"user_id": "123"}
)

# Add context immutably
modified = request.with_context("trace_id", "abc-123")
```

---

### Response

Mutable response object for building responses incrementally.

```python
@dataclass
class Response:
    status_code: int = 200
    headers: dict[str, str] = field(default_factory=dict)
    body: bytes | None = None
    error: str | None = None
```

**Methods:**

| Method | Description |
|--------|-------------|
| `set_header(key, value) -> None` | Add or update a header |
| `set_body(body: bytes) -> None` | Set body and auto-set Content-Length |

**Example:**

```python
from phenotype_middleware.domain import Response

response = Response(status_code=201)
response.set_header("X-Request-ID", "abc-123")
response.set_body(b'{"id": 1}')
```

---

### MiddlewareResult

Result pattern for explicit error handling.

```python
@dataclass
class MiddlewareResult:
    success: bool
    request: Request | None = None
    response: Response | None = None
    error: str | None = None
    metadata: dict[str, Any] = field(default_factory=dict)
```

**Factory Methods:**

| Method | Description |
|--------|-------------|
| `ok(request, response) -> MiddlewareResult` | Create successful result |
| `err(error, metadata) -> MiddlewareResult` | Create error result |

**Example:**

```python
from phenotype_middleware.domain import MiddlewareResult

# Success
return MiddlewareResult.ok(request=request)

# Success with response
return MiddlewareResult.ok(request=request, response=response)

# Error
return MiddlewareResult.err("Validation failed", {"field": "email"})
```

---

## Ports (Interfaces)

### MiddlewarePort

Base interface for all middleware implementations.

```python
class MiddlewarePort(ABC):
    @abstractmethod
    async def process(self, request: Request) -> MiddlewareResult:
        ...
```

### LoggingPort

Interface for logging adapters.

```python
class LoggingPort(ABC):
    @abstractmethod
    async def log(self, level: str, message: str, context: dict | None = None) -> None:
        ...
```

### MetricsPort

Interface for metrics collection.

```python
class MetricsPort(ABC):
    @abstractmethod
    async def record(self, name: str, value: float, labels: dict | None = None) -> None:
        ...
```

### AuthPort

Interface for authentication mechanisms.

```python
class AuthPort(ABC):
    @abstractmethod
    async def authenticate(self, request: Request) -> MiddlewareResult:
        ...
```

---

## Application Layer

### MiddlewareChain

Orchestrates middleware execution.

```python
class MiddlewareChain:
    def add(self, middleware: MiddlewarePort) -> "MiddlewareChain"
    def add_error_handler(self, handler: MiddlewarePort) -> "MiddlewareChain"
    async def handle(self, request: Request) -> MiddlewareResult
```

**Example:**

```python
from phenotype_middleware.application import MiddlewareChain
from phenotype_middleware.infrastructure import (
    StdoutLoggingAdapter,
    LoggingMiddleware,
)

chain = MiddlewareChain()
chain.add(LoggingMiddleware(StdoutLoggingAdapter()))

result = await chain.handle(request)
```

---

## Infrastructure Adapters

### StdoutLoggingAdapter

Simple logging to stdout with JSON output.

```python
class StdoutLoggingAdapter(LoggingPort):
    def __init__(self, min_level: str = "INFO")
    async def log(self, level: str, message: str, context: dict | None = None) -> None
```

### PrometheusMetricsAdapter

In-memory metrics collector.

```python
class PrometheusMetricsAdapter(MetricsPort):
    def __init__(self) -> None
    async def record(self, name: str, value: float, labels: dict | None = None) -> None
    def get_counter(self, name: str, labels: dict | None = None) -> float
```

### LoggingMiddleware

Middleware that logs requests.

```python
class LoggingMiddleware(MiddlewarePort):
    def __init__(self, logger: LoggingPort)
    async def process(self, request: Request) -> MiddlewareResult
```

### MetricsMiddleware

Middleware that records request metrics.

```python
class MetricsMiddleware(MiddlewarePort):
    def __init__(self, metrics: MetricsPort)
    async def process(self, request: Request) -> MiddlewareResult
```

---

## Error Handling

### Exception Hierarchy

```
MiddlewareError (base)
├── PipelineError
└── AdapterError
```

### Error Categories

```python
class ErrorCategory(Enum):
    MIDDLEWARE = "middleware"
    PIPELINE = "pipeline"
    ADAPTER = "adapter"
```

**Example:**

```python
from phenotype_middleware.domain import MiddlewareError, ErrorCategory

error = MiddlewareError(
    "Database connection failed",
    ErrorCategory.ADAPTER,
    context={"host": "localhost", "port": 5432}
)

# Add context
enriched = error.with_context("retry_count", 3)
```
