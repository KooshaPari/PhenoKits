# Advanced Patterns

This guide covers advanced usage patterns for `phenotype-middleware-py`.

## Table of Contents

- [Composite Middleware](#composite-middleware)
- [Middleware Factory](#middleware-factory)
- [Dynamic Chain Building](#dynamic-chain-building)
- [Request/Response Transformers](#requestresponse-transformers)
- [Circuit Breaker Pattern](#circuit-breaker-pattern)
- [Caching Middleware](#caching-middleware)
- [Audit Logging](#audit-logging)
- [Multi-Tenant Support](#multi-tenant-support)

---

## Composite Middleware

Combine multiple middleware into a single reusable component:

```python
from phenotype_middleware.application import MiddlewareChain
from phenotype_middleware.infrastructure import (
    AuthMiddleware,
    TracingMiddleware,
    RateLimitMiddleware,
    LoggingMiddleware,
    StdoutLoggingAdapter,
)

class SecureApiMiddleware:
    """Pre-configured middleware stack for secure API endpoints."""
    
    def __init__(self, auth_tokens: set[str], rate_limit: int = 100):
        self.chain = MiddlewareChain()
        
        # Add security middleware in order
        self.chain.add(TracingMiddleware())
        self.chain.add(RateLimitMiddleware(max_requests=rate_limit))
        self.chain.add(AuthMiddleware(token_validator=lambda t: t in auth_tokens))
        self.chain.add(LoggingMiddleware(StdoutLoggingAdapter()))
    
    async def process(self, request):
        return await self.chain.handle(request)

# Usage
secure_middleware = SecureApiMiddleware(
    auth_tokens={"token-1", "token-2"},
    rate_limit=1000
)
result = await secure_middleware.process(request)
```

---

## Middleware Factory

Create middleware instances based on configuration:

```python
from typing import Any
from phenotype_middleware.ports import MiddlewarePort
from phenotype_middleware.infrastructure import *

class MiddlewareFactory:
    """Factory for creating middleware from configuration."""
    
    @staticmethod
    def create(config: dict[str, Any]) -> MiddlewarePort:
        middleware_type = config.get("type")
        
        if middleware_type == "auth":
            return AuthMiddleware(
                token_validator=lambda t: t in config.get("tokens", set())
            )
        
        elif middleware_type == "rate_limit":
            return RateLimitMiddleware(
                max_requests=config.get("max_requests", 100),
                window_seconds=config.get("window", 60)
            )
        
        elif middleware_type == "tracing":
            return TracingMiddleware(
                header_name=config.get("header", "X-Trace-ID")
            )
        
        elif middleware_type == "retry":
            return RetryMiddleware(
                max_retries=config.get("max_retries", 3),
                base_delay=config.get("base_delay", 0.1)
            )
        
        elif middleware_type == "conditional":
            inner = MiddlewareFactory.create(config.get("middleware", {}))
            condition = config.get("condition", lambda r: True)
            return ConditionalMiddleware(inner, condition)
        
        raise ValueError(f"Unknown middleware type: {middleware_type}")

# Usage from config
config = {
    "type": "rate_limit",
    "max_requests": 100,
    "window": 60
}
middleware = MiddlewareFactory.create(config)
```

---

## Dynamic Chain Building

Build chains dynamically based on route or context:

```python
from phenotype_middleware.application import MiddlewareChain

class DynamicChainBuilder:
    """Builds middleware chains based on request characteristics."""
    
    def __init__(self):
        self.middleware_registry = {}
    
    def register(self, name: str, middleware, condition=None):
        """Register middleware with optional activation condition."""
        self.middleware_registry[name] = (middleware, condition)
    
    async def build_chain(self, request) -> MiddlewareChain:
        """Build chain based on request."""
        chain = MiddlewareChain()
        
        for name, (middleware, condition) in self.middleware_registry.items():
            if condition is None or condition(request):
                chain.add(middleware)
        
        return chain

# Usage
builder = DynamicChainBuilder()

# Register with conditions
builder.register("auth", auth_middleware, 
    condition=lambda r: r.path.startswith("/api"))

builder.register("logging", logging_middleware)

builder.register("admin_only", admin_middleware,
    condition=lambda r: r.path.startswith("/admin"))

# Build chain dynamically
chain = await builder.build_chain(request)
result = await chain.handle(request)
```

---

## Request/Response Transformers

Transform requests and responses at the middleware level:

```python
from phenotype_middleware.domain import Request, Response, MiddlewareResult
from phenotype_middleware.ports import MiddlewarePort

class JsonRequestTransformer(MiddlewarePort):
    """Parses JSON body and adds to context."""
    
    async def process(self, request: Request) -> MiddlewareResult:
        if request.body and request.headers.get("Content-Type") == "application/json":
            try:
                import json
                json_body = json.loads(request.body.decode())
                modified = request.with_context("json_body", json_body)
                return MiddlewareResult.ok(request=modified)
            except json.JSONDecodeError:
                response = Response(status_code=400)
                response.set_body(b'{"error": "Invalid JSON"}')
                return MiddlewareResult.ok(request=request, response=response)
        
        return MiddlewareResult.ok(request=request)

class JsonResponseTransformer(MiddlewarePort):
    """Ensures JSON responses have proper headers."""
    
    async def process(self, request: Request) -> MiddlewareResult:
        # This would wrap downstream middleware in practice
        # For now, demonstrates the pattern
        return MiddlewareResult.ok(request=request)
```

---

## Circuit Breaker Pattern

Prevent cascade failures with circuit breaker:

```python
import time
from enum import Enum, auto
from phenotype_middleware.ports import MiddlewarePort
from phenotype_middleware.domain import Request, MiddlewareResult, Response

class CircuitState(Enum):
    CLOSED = auto()      # Normal operation
    OPEN = auto()        # Failing, reject requests
    HALF_OPEN = auto()   # Testing if recovered

class CircuitBreakerMiddleware(MiddlewarePort):
    """
    Circuit breaker pattern for downstream services.
    
    Wraps another middleware and stops calling it after
    threshold failures, periodically testing recovery.
    """
    
    def __init__(
        self,
        wrapped: MiddlewarePort,
        failure_threshold: int = 5,
        recovery_timeout: float = 30.0,
        half_open_max_calls: int = 3
    ):
        self.wrapped = wrapped
        self.failure_threshold = failure_threshold
        self.recovery_timeout = recovery_timeout
        self.half_open_max_calls = half_open_max_calls
        
        self.state = CircuitState.CLOSED
        self.failures = 0
        self.last_failure_time = 0.0
        self.half_open_calls = 0
    
    async def process(self, request: Request) -> MiddlewareResult:
        if self.state == CircuitState.OPEN:
            if time.time() - self.last_failure_time >= self.recovery_timeout:
                self.state = CircuitState.HALF_OPEN
                self.half_open_calls = 0
            else:
                # Circuit open - return error immediately
                response = Response(status_code=503)
                response.set_body(b'{"error": "Service temporarily unavailable"}')
                return MiddlewareResult.ok(request=request, response=response)
        
        try:
            if self.state == CircuitState.HALF_OPEN:
                self.half_open_calls += 1
            
            result = await self.wrapped.process(request)
            
            if result.success:
                self._on_success()
            else:
                self._on_failure()
            
            return result
            
        except Exception:
            self._on_failure()
            raise
    
    def _on_success(self):
        self.failures = 0
        if self.state == CircuitState.HALF_OPEN:
            if self.half_open_calls >= self.half_open_max_calls:
                self.state = CircuitState.CLOSED
                self.half_open_calls = 0
    
    def _on_failure(self):
        self.failures += 1
        self.last_failure_time = time.time()
        
        if self.failures >= self.failure_threshold:
            self.state = CircuitState.OPEN
```

---

## Caching Middleware

Add response caching:

```python
from phenotype_middleware.ports import MiddlewarePort
from phenotype_middleware.domain import Request, MiddlewareResult, Response

class CacheMiddleware(MiddlewarePort):
    """
    Simple in-memory response cache.
    
    For production, use Redis or similar.
    """
    
    def __init__(self, ttl_seconds: float = 60.0):
        self.ttl = ttl_seconds
        self._cache: dict[str, tuple[Response, float]] = {}
    
    def _make_key(self, request: Request) -> str:
        """Create cache key from request."""
        return f"{request.method}:{request.path}"
    
    def _is_valid(self, entry: tuple[Response, float]) -> bool:
        """Check if cache entry is still valid."""
        _, timestamp = entry
        return (time.time() - timestamp) < self.ttl
    
    async def process(self, request: Request) -> MiddlewareResult:
        # Check cache (only for GET requests)
        if request.method == "GET":
            key = self._make_key(request)
            if key in self._cache:
                response, _ = self._cache[key]
                if self._is_valid(self._cache[key]):
                    return MiddlewareResult.ok(request=request, response=response)
        
        # Add cache info to context for downstream to potentially use
        modified = request.with_context("cache_skip", True)
        return MiddlewareResult.ok(request=modified)
```

---

## Audit Logging

Comprehensive audit trail for sensitive operations:

```python
import json
from datetime import datetime, timezone
from phenotype_middleware.ports import MiddlewarePort, LoggingPort
from phenotype_middleware.domain import Request, MiddlewareResult

class AuditLogMiddleware(MiddlewarePort):
    """
    Audit logging for compliance and security.
    
    Logs: who, what, when, where, and result
    """
    
    def __init__(
        self,
        logger: LoggingPort,
        sensitive_paths: set[str] | None = None
    ):
        self.logger = logger
        self.sensitive_paths = sensitive_paths or {"/api/admin", "/api/payments"}
    
    async def process(self, request: Request) -> MiddlewareResult:
        start_time = datetime.now(timezone.utc)
        
        # Log request
        await self.logger.log(
            "INFO",
            "Audit: Request started",
            {
                "timestamp": start_time.isoformat(),
                "path": request.path,
                "method": request.method,
                "user_id": request.context.get("user_id"),
                "trace_id": request.context.get("trace_id"),
                "ip": request.headers.get("X-Forwarded-For", "unknown"),
                "sensitive": request.path in self.sensitive_paths
            }
        )
        
        # Pass through - in real implementation, would need to wrap
        # downstream middleware to capture response
        return MiddlewareResult.ok(request=request)
```

---

## Multi-Tenant Support

Support multiple tenants in a single application:

```python
from phenotype_middleware.ports import MiddlewarePort
from phenotype_middleware.domain import Request, MiddlewareResult

class TenantResolutionMiddleware(MiddlewarePort):
    """
    Resolves tenant from request and adds to context.
    
    Supports multiple resolution strategies:
    - Subdomain: tenant.example.com
    - Header: X-Tenant-ID
    - Path: /api/tenant-id/...
    """
    
    def __init__(self, strategy: str = "header"):
        self.strategy = strategy
    
    async def process(self, request: Request) -> MiddlewareResult:
        tenant_id = None
        
        if self.strategy == "header":
            tenant_id = request.headers.get("X-Tenant-ID")
        
        elif self.strategy == "subdomain":
            # In real implementation, extract from Host header
            tenant_id = request.context.get("subdomain")
        
        elif self.strategy == "path":
            # Extract from path like /api/tenant-123/users
            parts = request.path.strip("/").split("/")
            if len(parts) >= 2 and parts[0] == "api":
                tenant_id = parts[1] if len(parts) > 1 else None
        
        if tenant_id:
            modified = request.with_context("tenant_id", tenant_id)
            return MiddlewareResult.ok(request=modified)
        
        return MiddlewareResult.ok(request=request)

class TenantRateLimitMiddleware(MiddlewarePort):
    """Rate limiting per tenant."""
    
    def __init__(self, default_limit: int = 100):
        self.default_limit = default_limit
        self._tenant_limits: dict[str, int] = {}
        self._tenant_counters: dict[str, int] = {}
    
    async def process(self, request: Request) -> MiddlewareResult:
        tenant_id = request.context.get("tenant_id", "default")
        
        # Get or initialize counter for tenant
        count = self._tenant_counters.get(tenant_id, 0)
        limit = self._tenant_limits.get(tenant_id, self.default_limit)
        
        if count >= limit:
            from phenotype_middleware.domain import Response
            response = Response(status_code=429)
            response.set_body(
                json.dumps({
                    "error": "Rate limit exceeded",
                    "tenant": tenant_id
                }).encode()
            )
            return MiddlewareResult.ok(request=request, response=response)
        
        self._tenant_counters[tenant_id] = count + 1
        return MiddlewareResult.ok(request=request)
```

---

## Best Practices

1. **Order matters**: Place authentication before rate limiting, tracing at the start
2. **Fail fast**: Return early errors to avoid unnecessary processing
3. **Context enrichment**: Each middleware adds useful context for downstream
4. **Immutable requests**: Always use `with_context()` instead of mutating
5. **Test in isolation**: Use contract tests for adapters, unit tests for logic

---

## Performance Tips

```python
# Cache expensive lookups
class CachedAuthMiddleware(MiddlewarePort):
    def __init__(self):
        self._token_cache: dict[str, bool] = {}
    
    async def process(self, request: Request) -> MiddlewareResult:
        token = request.headers.get("Authorization", "")
        
        # Check cache first
        if token in self._token_cache:
            is_valid = self._token_cache[token]
        else:
            is_valid = await self._validate_token(token)
            self._token_cache[token] = is_valid
        
        # ... rest of logic

# Batch metrics recording
class BatchedMetricsMiddleware(MiddlewarePort):
    """Batch metrics for fewer external calls."""
    
    def __init__(self, metrics: MetricsPort, batch_size: int = 10):
        self.metrics = metrics
        self.batch_size = batch_size
        self._buffer: list[dict] = []
    
    async def process(self, request: Request) -> MiddlewareResult:
        self._buffer.append({
            "path": request.path,
            "method": request.method,
            "timestamp": time.time()
        })
        
        if len(self._buffer) >= self.batch_size:
            await self._flush()
        
        return MiddlewareResult.ok(request=request)
    
    async def _flush(self):
        # Send batched metrics
        for entry in self._buffer:
            await self.metrics.record("request", 1, entry)
        self._buffer.clear()
```
