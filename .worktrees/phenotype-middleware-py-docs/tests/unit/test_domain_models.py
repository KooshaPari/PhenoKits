"""Tests for domain models.

Traces to: FR-PROTO-004 (mutable context), domain model design
"""

import pytest

from phenotype_middleware.domain import (
    AdapterError,
    ErrorCategory,
    MiddlewareError,
    MiddlewareResult,
    PipelineError,
    Request,
    Response,
)


class TestErrorCategory:
    """Test ErrorCategory enum values."""

    def test_error_category_values(self):
        """ErrorCategory should have expected values."""
        assert ErrorCategory.MIDDLEWARE.value == "middleware"
        assert ErrorCategory.PIPELINE.value == "pipeline"
        assert ErrorCategory.ADAPTER.value == "adapter"


class TestRequest:
    """Test Request dataclass behavior."""

    def test_request_creation(self):
        """Request should be creatable with minimal fields."""
        request = Request(path="/test", method="GET")
        assert request.path == "/test"
        assert request.method == "GET"
        assert request.headers == {}
        assert request.body is None
        assert request.context == {}

    def test_request_with_optional_fields(self):
        """Request should accept optional fields."""
        request = Request(
            path="/api",
            method="POST",
            headers={"Content-Type": "application/json"},
            body=b'{"key": "value"}',
        )
        assert request.headers == {"Content-Type": "application/json"}
        assert request.body == b'{"key": "value"}'

    def test_request_immutability(self):
        """Frozen Request should be immutable."""
        request = Request(path="/test", method="GET")
        with pytest.raises(AttributeError):
            request.path = "/modified"

    def test_request_with_context_returns_new_instance(self):
        """with_context should return new instance with updated context."""
        request = Request(path="/test", method="GET")
        modified = request.with_context("user_id", "123")

        assert request.context == {}  # Original unchanged
        assert modified.context.get("user_id") == "123"
        assert modified.path == "/test"  # Other fields unchanged

    def test_request_frozen_dataclass(self):
        """Request should be frozen dataclass (immutable)."""
        request = Request(path="/test", method="GET")
        with pytest.raises(AttributeError):
            request.path = "/modified"


class TestResponse:
    """Test Response dataclass behavior."""

    def test_response_defaults(self):
        """Response should have sensible defaults."""
        response = Response()
        assert response.status_code == 200
        assert response.body is None
        assert response.headers == {}

    def test_response_custom_status(self):
        """Response should accept custom status code."""
        response = Response(status_code=404, body=b"Not found")
        assert response.status_code == 404
        assert response.body == b"Not found"

    def test_set_header(self):
        """set_header should add header to response."""
        response = Response()
        response.set_header("X-Custom", "value")
        assert response.headers.get("X-Custom") == "value"

    def test_set_body(self):
        """set_body should update response body."""
        response = Response()
        response.set_body(b"Hello")
        assert response.body == b"Hello"


class TestMiddlewareResult:
    """Test MiddlewareResult factory methods."""

    def test_ok_factory_method(self):
        """ok() factory should create success result."""
        result = MiddlewareResult.ok()
        assert result.success is True
        assert result.request is None
        assert result.response is None
        assert result.error is None

    def test_ok_with_response(self):
        """ok() should accept response parameter."""
        request = Request(path="/", method="GET")
        response = Response(status_code=200)
        result = MiddlewareResult.ok(request=request, response=response)
        assert result.success is True
        assert result.response.status_code == 200

    def test_err_factory_method(self):
        """err() factory should create error result."""
        result = MiddlewareResult.err(error="Something went wrong")
        assert result.success is False
        assert result.error == "Something went wrong"
        assert result.request is None

    def test_err_with_metadata(self):
        """err() should accept metadata parameter."""
        result = MiddlewareResult.err(
            error="Validation failed",
            metadata={"field": "username", "reason": "too short"}
        )
        assert result.metadata.get("field") == "username"


class TestMiddlewareError:
    """Test MiddlewareError exception."""

    def test_basic_error(self):
        """MiddlewareError should be creatable with message."""
        error = MiddlewareError("Test error")
        assert str(error) == "Test error"

    def test_error_with_context(self):
        """MiddlewareError should support context dict."""
        error = MiddlewareError(
            "Auth failed",
            context={"user_id": "123", "attempt": 3}
        )
        assert error.context.get("user_id") == "123"

    def test_error_with_context_method(self):
        """with_context should add to error context."""
        error = MiddlewareError("Error", context={"a": 1})
        modified = error.with_context("b", 2)
        assert modified.context.get("a") == 1
        assert modified.context.get("b") == 2


class TestPipelineError:
    """Test PipelineError exception."""

    def test_pipeline_error_with_message(self):
        """PipelineError should have message."""
        error = PipelineError("Pipeline failed")
        assert str(error) == "Pipeline failed"

    def test_pipeline_error_with_context(self):
        """PipelineError should support context."""
        error = PipelineError(
            "Chain broken",
            context={"timeout_ms": 5000}
        )
        assert error.context.get("timeout_ms") == 5000


class TestAdapterError:
    """Test AdapterError exception."""

    def test_adapter_error_with_message(self):
        """AdapterError should have message."""
        error = AdapterError("Adapter failed")
        assert str(error) == "Adapter failed"

    def test_adapter_error_with_context(self):
        """AdapterError should support context."""
        error = AdapterError(
            "Connection failed",
            context={"adapter_name": "PrometheusMetricsAdapter"}
        )
        assert error.context.get("adapter_name") == "PrometheusMetricsAdapter"
