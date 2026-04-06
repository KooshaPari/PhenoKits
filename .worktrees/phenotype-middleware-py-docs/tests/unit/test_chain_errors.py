"""Tests for middleware chain error handling.

Traces to: FR-PROTO-003, FR-PIPE-001, FR-PIPE-002
"""

import pytest

from phenotype_middleware.application import MiddlewareChain
from phenotype_middleware.domain import MiddlewareResult, Request
from phenotype_middleware.ports import MiddlewarePort


class FailingMiddleware(MiddlewarePort):
    """Middleware that always fails."""

    def __init__(self, error_message: str = "Test error") -> None:
        self.error_message = error_message

    async def process(self, request: Request) -> MiddlewareResult:
        return MiddlewareResult.err(error=self.error_message)


class ExceptionMiddleware(MiddlewarePort):
    """Middleware that raises an exception."""

    async def process(self, request: Request) -> MiddlewareResult:
        raise RuntimeError("Unexpected error")


class SuccessMiddleware(MiddlewarePort):
    """Middleware that always succeeds."""

    async def process(self, request: Request) -> MiddlewareResult:
        return MiddlewareResult.ok(request=request)


class ModifyingMiddleware(MiddlewarePort):
    """Middleware that modifies the request context."""

    def __init__(self, key: str, value: str) -> None:
        self.key = key
        self.value = value

    async def process(self, request: Request) -> MiddlewareResult:
        modified = request.with_context(self.key, self.value)
        return MiddlewareResult.ok(request=modified)


class ShortCircuitMiddleware(MiddlewarePort):
    """Middleware that returns early with a response."""

    async def process(self, request: Request) -> MiddlewareResult:
        from phenotype_middleware.domain import Response
        response = Response(status_code=200, body=b"short circuit")
        return MiddlewareResult.ok(request=request, response=response)


class TestMiddlewareChainErrorHandling:
    """Test error handling in middleware chain."""

    @pytest.mark.asyncio
    async def test_given_failing_middleware_when_handled_then_returns_error(self):
        """Given failing middleware, when handled, then returns error result."""
        chain = MiddlewareChain()
        chain.add(FailingMiddleware("Database timeout"))

        request = Request(path="/api", method="GET")
        result = await chain.handle(request)

        assert result.success is False
        assert "Database timeout" in result.error

    @pytest.mark.asyncio
    async def test_given_exception_middleware_when_handled_then_returns_error(self):
        """Given middleware that raises exception, when handled, then returns error."""
        chain = MiddlewareChain()
        chain.add(ExceptionMiddleware())

        request = Request(path="/api", method="GET")
        result = await chain.handle(request)

        assert result.success is False
        assert "Unexpected error" in result.error


class TestMiddlewareChainShortCircuit:
    """Test short-circuit behavior in middleware chain."""

    @pytest.mark.asyncio
    async def test_given_short_circuit_middleware_when_processed_then_stops_early(self):
        """Given short-circuit middleware, when processed, then stops chain early."""
        chain = MiddlewareChain()
        chain.add(ShortCircuitMiddleware())
        chain.add(SuccessMiddleware())  # Should not be reached

        request = Request(path="/api", method="GET")
        result = await chain.handle(request)

        assert result.success is True
        assert result.response is not None
        assert result.response.status_code == 200


class TestMiddlewareChainOrdering:
    """Test middleware execution order."""

    @pytest.mark.asyncio
    async def test_given_multiple_middleware_when_processed_then_executes_in_order(self):
        """Given multiple middleware, when processed, then executes in order."""
        chain = MiddlewareChain()
        chain.add(ModifyingMiddleware("first", "value1"))
        chain.add(ModifyingMiddleware("second", "value2"))

        request = Request(path="/api", method="GET")
        result = await chain.handle(request)

        assert result.success is True
        assert result.request.context.get("first") == "value1"
        assert result.request.context.get("second") == "value2"


class TestMiddlewareChainRequestModification:
    """Test request modification through middleware chain."""

    @pytest.mark.asyncio
    async def test_given_modifying_middleware_when_processed_then_request_updated(self):
        """Given modifying middleware, when processed, then request is updated."""
        chain = MiddlewareChain()
        chain.add(ModifyingMiddleware("user_id", "123"))

        request = Request(path="/api", method="GET")
        result = await chain.handle(request)

        assert result.success is True
        assert result.request.context.get("user_id") == "123"
