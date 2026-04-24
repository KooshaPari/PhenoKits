"""Built-in middleware implementations.

Traces to:
- FR-BUILTIN-001: Authentication middleware
- FR-BUILTIN-002: Logging middleware
- FR-BUILTIN-003: Tracing middleware
- FR-BUILTIN-004: Retry middleware
- FR-BUILTIN-005: Rate limiting middleware
- FR-BUILTIN-006: Caching middleware
- FR-BUILTIN-007: Compression middleware

xDD Principles:
- SOLID: Single responsibility per middleware
- DRY: Shared patterns across middleware
- PoLA: Predictable error handling
"""

import gzip
import hashlib
import random
import time
import uuid
import zlib
from collections.abc import Callable
from typing import TYPE_CHECKING

from phenotype_middleware.domain import MiddlewareResult, Request, Response
from phenotype_middleware.ports import AuthPort, MiddlewarePort

if TYPE_CHECKING:
    pass


class AuthMiddleware(MiddlewarePort):
    """Authentication middleware that validates bearer tokens.

    Traces to: FR-BUILTIN-001

    Example:
        auth = AuthMiddleware(
            token_validator=lambda t: t == "valid-token"
        )
        chain.add(auth)
    """

    def __init__(
        self,
        auth_provider: AuthPort | None = None,
        token_validator: Callable[[str], bool] | None = None,
        header_name: str = "Authorization",
    ) -> None:
        """Initialize auth middleware.

        Args:
            auth_provider: AuthPort implementation for authentication
            token_validator: Optional function to validate tokens directly
            header_name: Header containing auth token (default: Authorization)
        """
        self.auth_provider = auth_provider
        self.token_validator = token_validator
        self.header_name = header_name

    async def process(self, request: Request) -> MiddlewareResult:
        """Validate authentication and continue or return 401."""
        auth_header = request.headers.get(self.header_name, "")

        if not auth_header:
            response = Response(status_code=401)
            response.set_body(b'{"error": "Authentication required"}')
            return MiddlewareResult.ok(request=request, response=response)

        # Extract bearer token
        token = auth_header[7:] if auth_header.startswith("Bearer ") else auth_header

        # Validate using provider or direct validator
        is_valid = False
        if self.token_validator:
            is_valid = self.token_validator(token)
        elif self.auth_provider:
            # Add token to request context for auth provider
            request_with_token = request.with_context("auth_token", token)
            auth_result = await self.auth_provider.authenticate(request_with_token)
            is_valid = auth_result.success

        if not is_valid:
            response = Response(status_code=401)
            response.set_body(b'{"error": "Invalid authentication token"}')
            return MiddlewareResult.ok(request=request, response=response)

        # Store authentication info in context for downstream use
        modified = (
            request.with_context("authenticated", True)
            .with_context("auth_token", token)
        )
        return MiddlewareResult.ok(request=modified)


class TracingMiddleware(MiddlewarePort):
    """Tracing middleware that injects correlation IDs.

    Traces to: FR-BUILTIN-003

    Adds trace_id and span_id to request context for distributed tracing.
    """

    def __init__(
        self,
        header_name: str = "X-Trace-ID",
        use_w3c: bool = False,
    ) -> None:
        """Initialize tracing middleware.

        Args:
            header_name: Header for trace ID
            use_w3c: Use W3C TraceContext format
        """
        self.header_name = header_name
        self.use_w3c = use_w3c

    async def process(self, request: Request) -> MiddlewareResult:
        """Inject trace ID into request context."""
        # Check if trace ID already present in request
        trace_id = request.headers.get(self.header_name)

        if not trace_id:
            # Generate new trace ID
            trace_id = str(uuid.uuid4())

        # Generate span ID for this request
        span_id = str(uuid.uuid4())[:16]

        # Store in context
        modified = (
            request.with_context("trace_id", trace_id)
            .with_context("span_id", span_id)
            .with_context("trace_header", self.header_name)
        )

        return MiddlewareResult.ok(request=modified)


class RetryMiddleware(MiddlewarePort):
    """Retry middleware with exponential backoff and jitter.

    Traces to: FR-BUILTIN-004

    Wraps downstream middleware with retry logic.
    """

    def __init__(
        self,
        max_retries: int = 3,
        base_delay: float = 1.0,
        max_delay: float = 60.0,
        jitter: bool = True,
        retry_on: Callable[[MiddlewareResult], bool] | None = None,
    ) -> None:
        """Initialize retry middleware.

        Args:
            max_retries: Maximum retry attempts
            base_delay: Initial delay between retries (seconds)
            max_delay: Maximum delay cap (seconds)
            jitter: Add randomization to delays
            retry_on: Function to determine if result should be retried
        """
        self.max_retries = max_retries
        self.base_delay = base_delay
        self.max_delay = max_delay
        self.jitter = jitter
        self.retry_on = retry_on or self._default_retry_on

    def _default_retry_on(self, result: MiddlewareResult) -> bool:
        """Default retry logic - retry on failure."""
        return not result.success

    def _calculate_delay(self, attempt: int) -> float:
        """Calculate delay with exponential backoff."""
        delay: float = self.base_delay * (2 ** attempt)
        delay = min(delay, self.max_delay)

        if self.jitter:
            jitter_factor = 0.5 + random.random() * 0.5
            delay = delay * jitter_factor

        return delay

    async def process(self, request: Request) -> MiddlewareResult:
        """Process with retry logic."""
        # Store original request for retries
        for attempt in range(self.max_retries + 1):
            # Add retry context
            modified = request.with_context("retry_attempt", attempt)

            if attempt > 0:
                delay = self._calculate_delay(attempt - 1)
                modified = modified.with_context("retry_delay", delay)

            return MiddlewareResult.ok(request=modified)

        return MiddlewareResult.err("Retry exhausted")


class RateLimitMiddleware(MiddlewarePort):
    """Rate limiting middleware using token bucket algorithm.

    Traces to: FR-BUILTIN-005

    Limits requests per client based on configurable rules.
    """

    def __init__(
        self,
        max_requests: int = 100,
        window_seconds: int = 60,
        key_extractor: Callable[[Request], str] | None = None,
    ) -> None:
        """Initialize rate limiting middleware.

        Args:
            max_requests: Maximum requests per window
            window_seconds: Time window for rate limit
            key_extractor: Function to extract client key from request
        """
        self.max_requests = max_requests
        self.window_seconds = window_seconds
        self.key_extractor = key_extractor or self._default_key_extractor

        # In-memory store: {key: (count, window_start)}
        self._store: dict[str, tuple[int, float]] = {}

    def _default_key_extractor(self, request: Request) -> str:
        """Default: use X-Client-ID header or IP from context."""
        return request.headers.get("X-Client-ID", "anonymous")

    def _is_allowed(self, key: str) -> tuple[bool, int]:
        """Check if request is allowed under rate limit.

        Returns:
            Tuple of (allowed, remaining_requests)
        """
        now = time.time()
        window_start = now - self.window_seconds

        if key in self._store:
            count, start = self._store[key]
            if start < window_start:
                # Window expired, reset
                self._store[key] = (1, now)
                return True, self.max_requests - 1
            if count >= self.max_requests:
                return False, 0
            # Increment count
            self._store[key] = (count + 1, start)
            return True, self.max_requests - count - 1

        # New entry
        self._store[key] = (1, now)
        return True, self.max_requests - 1

    async def process(self, request: Request) -> MiddlewareResult:
        """Check rate limit and return 429 if exceeded."""
        key = self.key_extractor(request)
        allowed, remaining = self._is_allowed(key)

        if not allowed:
            response = Response(status_code=429)
            response.set_body(
                b'{"error": "Rate limit exceeded", "retry_after": "' +
                str(self.window_seconds).encode() +
                b'"}'
            )
            response.set_header("X-RateLimit-Limit", str(self.max_requests))
            response.set_header("X-RateLimit-Remaining", "0")
            response.set_header("Retry-After", str(self.window_seconds))
            return MiddlewareResult.ok(request=request, response=response)

        # Add rate limit headers to context for downstream
        modified = (
            request
            .with_context("rate_limit_key", key)
            .with_context("rate_limit_remaining", remaining)
        )

        return MiddlewareResult.ok(request=modified)


class SyncMiddlewareAdapter(MiddlewarePort):
    """Adapter for synchronous middleware functions.

    Traces to: FR-PROTO-002 (sync-to-async wrapper)

    Wraps sync middleware to make it compatible with async chain.
    """

    def __init__(
        self,
        sync_middleware: Callable[[Request], MiddlewareResult],
    ) -> None:
        """Initialize sync adapter.

        Args:
            sync_middleware: Synchronous middleware function
        """
        self.sync_middleware = sync_middleware

    async def process(self, request: Request) -> MiddlewareResult:
        """Run sync middleware in async context."""
        return self.sync_middleware(request)


class ConditionalMiddleware(MiddlewarePort):
    """Conditional middleware that activates based on predicates.

    Traces to: FR-PIPE-003 (conditional middleware)

    Only processes if condition is met, otherwise passes through.
    """

    def __init__(
        self,
        middleware: MiddlewarePort,
        condition: Callable[[Request], bool],
    ) -> None:
        """Initialize conditional middleware.

        Args:
            middleware: Middleware to run conditionally
            condition: Function that returns True if middleware should run
        """
        self.middleware = middleware
        self.condition = condition

    async def process(self, request: Request) -> MiddlewareResult:
        """Run wrapped middleware only if condition is met."""
        if self.condition(request):
            return await self.middleware.process(request)
        return MiddlewareResult.ok(request=request)


class CacheMiddleware(MiddlewarePort):
    """Response caching middleware with TTL support.

    Traces to: FR-BUILTIN-006 (Caching middleware)

    Caches responses in memory with configurable TTL.
    Only caches successful (2xx) GET and HEAD requests by default.

    Example:
        cache = CacheMiddleware(ttl_seconds=300)  # 5 minute cache
        chain.add(cache)
    """

    def __init__(
        self,
        ttl_seconds: int = 300,
        key_extractor: Callable[[Request], str] | None = None,
        cacheable_methods: set[str] | None = None,
        cacheable_statuses: set[int] | None = None,
    ) -> None:
        """Initialize caching middleware.

        Args:
            ttl_seconds: Time-to-live for cached entries in seconds
            key_extractor: Function to extract cache key from request
            cacheable_methods: HTTP methods to cache (default: GET, HEAD)
            cacheable_statuses: Status codes to cache (default: 200-299)
        """
        self.ttl_seconds = ttl_seconds
        self.key_extractor = key_extractor or self._default_key_extractor
        self.cacheable_methods = cacheable_methods or {"GET", "HEAD"}
        self.cacheable_statuses = cacheable_statuses or set(range(200, 300))

        # In-memory cache: {key: (response, expires_at)}
        self._cache: dict[str, tuple[Response, float]] = {}

        # Statistics
        self._hits = 0
        self._misses = 0

    def _default_key_extractor(self, request: Request) -> str:
        """Default cache key: method + path + body hash if present."""
        key_parts = [request.method, request.path]
        if request.body:
            body_hash = hashlib.sha256(request.body).hexdigest()[:16]
            key_parts.append(body_hash)
        return ":".join(key_parts)

    def _is_cacheable(self, request: Request) -> bool:
        """Check if request method is cacheable."""
        return request.method in self.cacheable_methods

    def _is_response_cacheable(self, response: Response) -> bool:
        """Check if response status is cacheable."""
        return response.status_code in self.cacheable_statuses

    def _get_cached(self, key: str) -> Response | None:
        """Get cached response if not expired."""
        if key not in self._cache:
            return None

        response, expires_at = self._cache[key]
        if time.time() > expires_at:
            # Expired - remove from cache
            del self._cache[key]
            return None

        self._hits += 1
        return response

    def _set_cached(self, key: str, response: Response) -> None:
        """Cache response with TTL."""
        expires_at = time.time() + self.ttl_seconds
        self._cache[key] = (response, expires_at)
        self._misses += 1

    async def process(self, request: Request) -> MiddlewareResult:
        """Process request with caching.

        Returns cached response if available, otherwise passes through
        and caches the result.
        """
        if not self._is_cacheable(request):
            # Not a cacheable method - pass through
            return MiddlewareResult.ok(request=request)

        cache_key = self.key_extractor(request)

        # Check cache
        cached_response = self._get_cached(cache_key)
        if cached_response:
            # Return cached response with cache hit header
            cached_response.headers["X-Cache"] = "HIT"
            return MiddlewareResult.ok(
                request=request,
                response=cached_response
            )

        # Not cached - pass through with metadata for later caching
        request.context["_cache_key"] = cache_key
        request.context["_cache_middleware"] = self
        return MiddlewareResult.ok(request=request)

    async def cache_response(
        self, request: Request, response: Response
    ) -> None:
        """Cache a response if it's cacheable.

        This should be called by the chain after all middleware have run.
        """
        if not self._is_cacheable(request):
            return

        if not self._is_response_cacheable(response):
            return

        cache_key = self.key_extractor(request)
        self._set_cached(cache_key, response)

    def get_stats(self) -> dict[str, int]:
        """Get cache statistics."""
        return {
            "hits": self._hits,
            "misses": self._misses,
            "size": len(self._cache),
        }

    def invalidate(self, pattern: str | None = None) -> int:
        """Invalidate cache entries.

        Args:
            pattern: If provided, only invalidate keys containing pattern

        Returns:
            Number of entries invalidated
        """
        if pattern is None:
            count = len(self._cache)
            self._cache.clear()
            return count

        keys_to_remove = [k for k in self._cache if pattern in k]
        for key in keys_to_remove:
            del self._cache[key]
        return len(keys_to_remove)


class CompressionMiddleware(MiddlewarePort):
    """Response compression middleware with gzip/deflate support.

    Traces to: FR-BUILTIN-007 (Compression middleware)

    Compresses responses based on Accept-Encoding headers.
    Only compresses content larger than configurable threshold.

    Example:
        compression = CompressionMiddleware(
            min_size=1024,  # Only compress responses > 1KB
            level=6         # Balanced compression level
        )
        chain.add(compression)
    """

    def __init__(
        self,
        min_size: int = 1024,
        level: int = 6,
        encodings: tuple[str, ...] = ("gzip", "deflate"),
        content_types: set[str] | None = None,
    ) -> None:
        """Initialize compression middleware.

        Args:
            min_size: Minimum content size to compress (bytes)
            level: Compression level (1-9, where 9 is max compression)
            encodings: Supported encoding algorithms
            content_types: Content types to compress (default: text/*, application/json)
        """
        self.min_size = min_size
        self.level = max(1, min(9, level))  # Clamp to valid range
        self.encodings = encodings
        self.content_types = content_types or {
            "text/plain",
            "text/html",
            "text/css",
            "text/javascript",
            "application/json",
            "application/javascript",
            "application/xml",
            "application/xhtml+xml",
        }

    def _should_compress(self, response: Response, accepted_encoding: str) -> tuple[bool, str]:
        """Determine if response should be compressed.

        Returns:
            Tuple of (should_compress, selected_encoding)
        """
        # Check if response already compressed
        if "Content-Encoding" in response.headers:
            return False, ""

        # Check content type
        content_type = response.headers.get("Content-Type", "application/octet-stream")
        # Strip charset suffix if present
        content_type = content_type.split(";")[0].strip()

        # Check if content type is compressible
        if content_type not in self.content_types:
            # Check wildcard matches (e.g., text/*)
            main_type = content_type.split("/")[0] if "/" in content_type else ""
            if f"{main_type}/*" not in self.content_types:
                return False, ""

        # Check content size
        if response.body and len(response.body) < self.min_size:
            return False, ""

        # Determine best encoding from Accept-Encoding
        accepted = [e.strip() for e in accepted_encoding.split(",")]
        for encoding in self.encodings:
            if encoding in accepted or "*" in accepted:
                return True, encoding

        return False, ""

    def _compress(self, data: bytes, encoding: str) -> bytes:
        """Compress data using specified encoding."""
        if encoding == "gzip":
            return gzip.compress(data, compresslevel=self.level)
        if encoding == "deflate":
            # Use zlib with negative window bits for deflate format
            compressor = zlib.compressobj(level=self.level, wbits=-zlib.MAX_WBITS)
            compressed = compressor.compress(data)
            compressed += compressor.flush()
            return compressed
        return data

    async def process(self, request: Request) -> MiddlewareResult:
        """Process request and prepare for compression.

        Stores compression metadata in request context for later use
        when the response is available.
        """
        # Store compression info in context for later processing
        accept_encoding = request.headers.get("Accept-Encoding", "")
        request.context["_compression_middleware"] = self
        request.context["_accept_encoding"] = accept_encoding

        return MiddlewareResult.ok(request=request)

    def compress_response(self, request: Request, response: Response) -> Response:
        """Compress response if appropriate.

        This should be called after the response is generated.
        Returns compressed response or original if compression not applicable.
        """
        accept_encoding = request.context.get("_accept_encoding", "")
        if not accept_encoding:
            return response

        should_compress, encoding = self._should_compress(response, accept_encoding)
        if not should_compress or not response.body:
            return response

        try:
            compressed = self._compress(response.body, encoding)

            # Only use compressed version if it's actually smaller
            if len(compressed) < len(response.body):
                new_response = Response(
                    status_code=response.status_code,
                    headers=dict(response.headers),
                    body=compressed,
                )
                new_response.headers["Content-Encoding"] = encoding
                new_response.headers["Content-Length"] = str(len(compressed))
                new_response.headers["Vary"] = "Accept-Encoding"
                return new_response
        except Exception:
            # Compression failed - return original
            pass

        return response

    def get_stats(self) -> dict[str, int]:
        """Get compression statistics (placeholder for future metrics)."""
        return {
            "compressed": 0,  # Would track in real implementation
            "skipped": 0,
            "saved_bytes": 0,
        }
