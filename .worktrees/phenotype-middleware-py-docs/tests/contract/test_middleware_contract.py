"""
Contract tests for middleware ports.

Following CDD (Contract-Driven Development):
- Tests verify adapters implement ports correctly
- Each adapter has a contract test file

xDD Principles:
- BDD: Given-When-Then scenario naming
- Contract: Port/adapter verification
"""

import pytest

from phenotype_middleware.domain import MiddlewareResult, Request
from phenotype_middleware.infrastructure import (
    LoggingMiddleware,
    MetricsMiddleware,
    PrometheusMetricsAdapter,
    StdoutLoggingAdapter,
)

# =============================================================================
# LoggingPort Contract Tests
# =============================================================================

class TestLoggingPortContract:
    """
    Contract tests for LoggingPort adapters.

    Given: A LoggingPort implementation
    When: log() is called with various levels and contexts
    Then: It should not raise and should complete successfully
    """

    @pytest.mark.asyncio
    async def test_logging_adapter_does_not_raise(self):
        """Log should not raise exceptions."""
        adapter = StdoutLoggingAdapter()
        # Should not raise
        await adapter.log("INFO", "test message")
        await adapter.log("DEBUG", "debug message", {"key": "value"})

    @pytest.mark.asyncio
    async def test_logging_adapter_all_levels(self):
        """Adapter should handle all standard log levels."""
        adapter = StdoutLoggingAdapter()
        for level in ["DEBUG", "INFO", "WARNING", "ERROR"]:
            await adapter.log(level, f"{level} message")

    @pytest.mark.asyncio
    async def test_logging_adapter_with_context(self):
        """Adapter should handle context dictionaries."""
        adapter = StdoutLoggingAdapter()
        context = {
            "request_id": "123",
            "user_id": "456",
            "ip": "127.0.0.1"
        }
        await adapter.log("INFO", "User action", context)


# =============================================================================
# MetricsPort Contract Tests
# =============================================================================

class TestMetricsPortContract:
    """
    Contract tests for MetricsPort adapters.

    Given: A MetricsPort implementation
    When: record() is called with various metrics
    Then: It should not raise and should store the metric
    """

    @pytest.mark.asyncio
    async def test_metrics_adapter_does_not_raise(self):
        """Record should not raise exceptions."""
        adapter = PrometheusMetricsAdapter()
        await adapter.record("test_metric", 1.0)

    @pytest.mark.asyncio
    async def test_metrics_adapter_with_labels(self):
        """Adapter should handle labeled metrics."""
        adapter = PrometheusMetricsAdapter()
        await adapter.record("requests_total", 1, {"method": "GET", "path": "/api"})
        await adapter.record("requests_total", 1, {"method": "POST", "path": "/api"})

    @pytest.mark.asyncio
    async def test_metrics_adapter_increments(self):
        """Multiple records should increment the counter."""
        adapter = PrometheusMetricsAdapter()
        await adapter.record("counter", 1)
        await adapter.record("counter", 1)
        await adapter.record("counter", 1)
        assert adapter.get_counter("counter") == 3


# =============================================================================
# MiddlewarePort Contract Tests
# =============================================================================

class TestMiddlewarePortContract:
    """
    Contract tests for MiddlewarePort adapters.

    Given: A MiddlewarePort implementation
    When: process() is called with a request
    Then: It should return a MiddlewareResult
    """

    @pytest.mark.asyncio
    async def test_logging_middleware_returns_result(self):
        """Middleware should return MiddlewareResult."""
        logger = StdoutLoggingAdapter()
        middleware = LoggingMiddleware(logger)

        request = Request(path="/test", method="GET")
        result = await middleware.process(request)

        assert isinstance(result, MiddlewareResult)
        assert result.success

    @pytest.mark.asyncio
    async def test_metrics_middleware_returns_result(self):
        """Middleware should return MiddlewareResult."""
        metrics = PrometheusMetricsAdapter()
        middleware = MetricsMiddleware(metrics)

        request = Request(path="/test", method="GET")
        result = await middleware.process(request)

        assert isinstance(result, MiddlewareResult)
        assert result.success

    @pytest.mark.asyncio
    async def test_middleware_preserves_request(self):
        """Middleware should preserve or return modified request."""
        logger = StdoutLoggingAdapter()
        middleware = LoggingMiddleware(logger)

        request = Request(path="/test", method="GET")
        result = await middleware.process(request)

        assert result.request is not None
        assert result.request.path == "/test"


# =============================================================================
# BDD Scenario Tests
# =============================================================================

class TestMiddlewareChainBehavior:
    """
    BDD-style tests for MiddlewareChain.

    Using Given-When-Then scenario naming.
    """

    @pytest.mark.asyncio
    async def test_given_middleware_chain_when_request_processed_then_returns_result(self):
        """Given middleware chain, when request processed, returns result."""
        from phenotype_middleware.application import MiddlewareChain
        from phenotype_middleware.infrastructure import StdoutLoggingAdapter

        chain = MiddlewareChain()
        chain.add(LoggingMiddleware(StdoutLoggingAdapter()))

        request = Request(path="/api", method="POST")
        result = await chain.handle(request)

        assert isinstance(result, MiddlewareResult)
        assert result.success

    @pytest.mark.asyncio
    async def test_given_empty_chain_when_request_processed_then_returns_success(self):
        """Given empty chain, when request processed, returns success."""
        from phenotype_middleware.application import MiddlewareChain

        chain = MiddlewareChain()
        request = Request(path="/test", method="GET")
        result = await chain.handle(request)

        assert result.success
        assert result.request is not None
