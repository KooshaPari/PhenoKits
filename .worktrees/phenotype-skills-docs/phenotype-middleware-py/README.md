# phenotype-middleware-py

Python middleware patterns following hexagonal architecture. Pluggable middleware for web frameworks.

## Features

- Hexagonal architecture
- Framework-agnostic middleware interface
- Request/Response transformers
- Authentication middleware
- Logging middleware
- Rate limiting middleware
- Circuit breaker

## Installation

```bash
pip install phenotype-middleware-py
```

## Usage

### Basic Middleware

```python
from phenotype_middleware import Middleware, MiddlewareChain

class LoggingMiddleware(Middleware):
    async def process(self, request, next_handler):
        print(f"Request: {request.method} {request.url}")
        response = await next_handler(request)
        print(f"Response: {response.status}")
        return response
```

### Middleware Chain

```python
from phenotype_middleware import MiddlewareChain

chain = MiddlewareChain()
chain.add(AuthMiddleware())
chain.add(RateLimitMiddleware(max_requests=100))
chain.add(LoggingMiddleware())

response = await chain.handle(request)
```

### Framework Adapters

```python
from phenotype_middleware.adapters import FastAPIMiddleware, StarletteMiddleware

# FastAPI
app.add_middleware(FastAPIMiddleware(chain))

# Starlette
app.add_middleware(StarletteMiddleware(chain))
```

## Architecture

```
src/
├── domain/           # Core middleware concepts
│   ├── Middleware.py
│   ├── Request.py
│   └── Response.py
├── application/       # Chain orchestration
│   ├── MiddlewareChain.py
│   └── PipelineBuilder.py
├── ports/           # Interfaces
│   └── MiddlewarePort.py
└── adapters/        # Implementations
    ├── logging/
    ├── auth/
    ├── ratelimit/
    ├── circuitbreaker/
    ├── fastapi/
    └── starlette/
```

## License

MIT
