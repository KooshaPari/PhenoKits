"""
Unit tests for built-in middleware.

Traces to:
- FR-BUILTIN-001: Authentication middleware
- FR-BUILTIN-003: Tracing middleware
- FR-BUILTIN-004: Retry middleware
- FR-BUILTIN-005: Rate limiting middleware
- FR-PROTO-002: Sync-to-async wrapper
- FR-PIPE-003: Conditional middleware

xDD Principles:
- TDD: Test middleware in isolation
- BDD: Descriptive scenario naming
"""

import pytest

from phenotype_middleware.domain import MiddlewareResult, Request
from phenotype_middleware.infrastructure import (
    AuthMiddleware,
    ConditionalMiddleware,
    RateLimitMiddleware,
    RetryMiddleware,
    SyncMiddlewareAdapter,
    TracingMiddleware,
)

# =============================================================================
# AuthMiddleware Tests
# =============================================================================


class TestAuthMiddleware:
    """Tests for AuthMiddleware.

    Traces to: FR-BUILTIN-001
    """

    @pytest.mark.asyncio
    async def test_given_valid_token_when_processed_then_continues(self):
        """Given valid token, when processed, request continues."""
        auth = AuthMiddleware(token_validator=lambda t: t == "valid-token")

        request = Request(
            path="/api",
            method="GET",
            headers={"Authorization": "Bearer valid-token"}
        )
        result = await auth.process(request)

        assert result.success is True
        assert result.response is None  # No early response
        assert result.request.context.get("authenticated") is True

    @pytest.mark.asyncio
    async def test_given_invalid_token_when_processed_then_returns_401(self):
        """Given invalid token, when processed, returns 401 response."""
        auth = AuthMiddleware(token_validator=lambda t: t == "valid-token")

        request = Request(
            path="/api",
            method="GET",
            headers={"Authorization": "Bearer invalid-token"}
        )
        result = await auth.process(request)

        assert result.success is True  # Middleware succeeded
        assert result.response is not None
        assert result.response.status_code == 401

    @pytest.mark.asyncio
    async def test_given_no_auth_header_when_processed_then_returns_401(self):
        """Given no auth header, when processed, returns 401."""
        auth = AuthMiddleware(token_validator=lambda t: t == "token")

        request = Request(path="/api", method="GET", headers={})
        result = await auth.process(request)

        assert result.response.status_code == 401

    @pytest.mark.asyncio
    async def test_given_plain_token_when_processed_then_validates(self):
        """Given plain token (no Bearer prefix), when processed, validates."""
        auth = AuthMiddleware(token_validator=lambda t: t == "plain-token")

        request = Request(
            path="/api",
            method="GET",
            headers={"Authorization": "plain-token"}
        )
        result = await auth.process(request)

        assert result.success is True
        assert result.response is None

    @pytest.mark.asyncio
    async def test_given_custom_header_when_processed_then_uses_header(self):
        """Given custom header name, when processed, reads from that header."""
        auth = AuthMiddleware(
            token_validator=lambda t: t == "api-key",
            header_name="X-API-Key"
        )

        request = Request(
            path="/api",
            method="GET",
            headers={"X-API-Key": "api-key"}
        )
        result = await auth.process(request)

        assert result.response is None


# =============================================================================
# TracingMiddleware Tests
# =============================================================================


class TestTracingMiddleware:
    """Tests for TracingMiddleware.

    Traces to: FR-BUILTIN-003
    """

    @pytest.mark.asyncio
    async def test_given_request_without_trace_id_when_processed_then_generates_trace_id(self):
        """Given no trace ID, when processed, generates new trace ID."""
        tracing = TracingMiddleware()

        request = Request(path="/api", method="GET", headers={})
        result = await tracing.process(request)

        assert result.success is True
        trace_id = result.request.context.get("trace_id")
        assert trace_id is not None
        assert len(trace_id) == 36  # UUID format

    @pytest.mark.asyncio
    async def test_given_request_with_trace_id_when_processed_then_preserves_trace_id(self):
        """Given existing trace ID, when processed, preserves it."""
        tracing = TracingMiddleware()
        existing_trace = "existing-trace-123"

        request = Request(
            path="/api",
            method="GET",
            headers={"X-Trace-ID": existing_trace}
        )
        result = await tracing.process(request)

        assert result.request.context.get("trace_id") == existing_trace

    @pytest.mark.asyncio
    async def test_when_processed_then_generates_span_id(self):
        """When processed, generates span ID."""
        tracing = TracingMiddleware()

        request = Request(path="/api", method="GET")
        result = await tracing.process(request)

        span_id = result.request.context.get("span_id")
        assert span_id is not None
        assert len(span_id) == 16  # Short UUID


# =============================================================================
# RateLimitMiddleware Tests
# =============================================================================


class TestRateLimitMiddleware:
    """Tests for RateLimitMiddleware.

    Traces to: FR-BUILTIN-005
    """

    @pytest.mark.asyncio
    async def test_given_first_request_when_processed_then_allows(self):
        """Given first request, when processed, allows through."""
        rate_limit = RateLimitMiddleware(max_requests=2, window_seconds=60)

        request = Request(path="/api", method="GET", headers={"X-Client-ID": "client-1"})
        result = await rate_limit.process(request)

        assert result.success is True
        assert result.response is None
        assert result.request.context.get("rate_limit_remaining") == 1

    @pytest.mark.asyncio
    async def test_given_under_limit_when_processed_then_allows(self):
        """Given under limit, when processed, allows through."""
        rate_limit = RateLimitMiddleware(max_requests=3, window_seconds=60)

        client_id = "client-2"
        for _ in range(2):
            request = Request(
                path="/api",
                method="GET",
                headers={"X-Client-ID": client_id}
            )
            result = await rate_limit.process(request)
            assert result.response is None

    @pytest.mark.asyncio
    async def test_given_over_limit_when_processed_then_returns_429(self):
        """Given over limit, when processed, returns 429."""
        rate_limit = RateLimitMiddleware(max_requests=2, window_seconds=60)

        client_id = "client-3"
        # Exhaust limit
        for _ in range(2):
            request = Request(
                path="/api",
                method="GET",
                headers={"X-Client-ID": client_id}
            )
            await rate_limit.process(request)

        # Next request should be rejected
        request = Request(
            path="/api",
            method="GET",
            headers={"X-Client-ID": client_id}
        )
        result = await rate_limit.process(request)

        assert result.response is not None
        assert result.response.status_code == 429
        assert result.response.headers.get("Retry-After") is not None

    @pytest.mark.asyncio
    async def test_given_different_clients_when_processed_then_tracked_separately(self):
        """Given different clients, when processed, tracked separately."""
        rate_limit = RateLimitMiddleware(max_requests=1, window_seconds=60)

        # First client hits limit
        client1_request = Request(
            path="/api",
            method="GET",
            headers={"X-Client-ID": "client-a"}
        )
        await rate_limit.process(client1_request)
        result = await rate_limit.process(client1_request)
        assert result.response.status_code == 429

        # Second client still has quota
        client2_request = Request(
            path="/api",
            method="GET",
            headers={"X-Client-ID": "client-b"}
        )
        result = await rate_limit.process(client2_request)
        assert result.response is None

    @pytest.mark.asyncio
    async def test_given_custom_key_extractor_when_processed_then_uses_extractor(self):
        """Given custom key extractor, when processed, uses it."""
        rate_limit = RateLimitMiddleware(
            max_requests=1,
            window_seconds=60,
            key_extractor=lambda r: r.headers.get("X-User", "default")
        )

        request = Request(
            path="/api",
            method="GET",
            headers={"X-User": "user-123"}
        )
        result = await rate_limit.process(request)

        assert result.request.context.get("rate_limit_key") == "user-123"


# =============================================================================
# RetryMiddleware Tests
# =============================================================================


class TestRetryMiddleware:
    """Tests for RetryMiddleware.

    Traces to: FR-BUILTIN-004
    """

    @pytest.mark.asyncio
    async def test_given_retry_middleware_when_processed_then_adds_retry_context(self):
        """Given retry middleware, when processed, adds retry context."""
        retry = RetryMiddleware(max_retries=3)

        request = Request(path="/api", method="GET")
        result = await retry.process(request)

        assert result.success is True
        assert result.request.context.get("retry_attempt") == 0

    @pytest.mark.asyncio
    async def test_given_retry_with_jitter_when_calculated_then_returns_delay(self):
        """Given jitter enabled, when calculating delay, returns valid delay."""
        retry = RetryMiddleware(base_delay=0.1, max_delay=1.0, jitter=True)

        delay = retry._calculate_delay(0)
        assert 0 <= delay <= 1.0

        delay = retry._calculate_delay(1)
        assert 0 <= delay <= 1.0

        delay = retry._calculate_delay(10)  # Should be capped at max_delay
        assert 0 <= delay <= 1.0

    @pytest.mark.asyncio
    async def test_given_retry_without_jitter_when_calculated_then_returns_exponential_delay(self):
        """Given no jitter, when calculating delay, returns exponential backoff."""
        retry = RetryMiddleware(base_delay=0.1, max_delay=10.0, jitter=False)

        assert retry._calculate_delay(0) == 0.1
        assert retry._calculate_delay(1) == 0.2
        assert retry._calculate_delay(2) == 0.4
        assert retry._calculate_delay(3) == 0.8


# =============================================================================
# SyncMiddlewareAdapter Tests
# =============================================================================


class TestSyncMiddlewareAdapter:
    """Tests for SyncMiddlewareAdapter.

    Traces to: FR-PROTO-002
    """

    @pytest.mark.asyncio
    async def test_given_sync_middleware_when_processed_then_adapts(self):
        """Given sync middleware, when processed, adapter wraps it."""
        def sync_middleware(request: Request) -> MiddlewareResult:
            return MiddlewareResult.ok(request=request.with_context("sync", True))

        adapter = SyncMiddlewareAdapter(sync_middleware)

        request = Request(path="/api", method="GET")
        result = await adapter.process(request)

        assert result.success is True
        assert result.request.context.get("sync") is True


# =============================================================================
# ConditionalMiddleware Tests
# =============================================================================


class TestConditionalMiddleware:
    """Tests for ConditionalMiddleware.

    Traces to: FR-PIPE-003
    """

    @pytest.mark.asyncio
    async def test_given_condition_true_when_processed_then_runs_middleware(self):
        """Given condition true, when processed, runs wrapped middleware."""
        from phenotype_middleware.infrastructure import (
            LoggingMiddleware,
            StdoutLoggingAdapter,
        )

        inner = LoggingMiddleware(StdoutLoggingAdapter())

        def condition(r):
            return r.headers.get("X-Log") == "true"

        conditional = ConditionalMiddleware(inner, condition)

        request = Request(
            path="/api",
            method="GET",
            headers={"X-Log": "true"}
        )
        result = await conditional.process(request)

        # Should process (and log)
        assert result.success is True

    @pytest.mark.asyncio
    async def test_given_condition_false_when_processed_then_skips_middleware(self):
        """Given condition false, when processed, skips wrapped middleware."""
        from phenotype_middleware.infrastructure import (
            LoggingMiddleware,
            StdoutLoggingAdapter,
        )

        inner = LoggingMiddleware(StdoutLoggingAdapter())

        def condition(r):
            return r.headers.get("X-Log") == "true"

        conditional = ConditionalMiddleware(inner, condition)

        request = Request(
            path="/api",
            method="GET",
            headers={}  # No X-Log header
        )
        result = await conditional.process(request)

        # Should pass through unchanged
        assert result.success is True
        assert "logged" not in result.request.context  # Not modified by inner
