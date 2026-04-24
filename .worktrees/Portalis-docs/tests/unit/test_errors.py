"""Unit tests for error hierarchy.

Traces to:
    - FR-ERR-001: PortkeyError base class
    - FR-ERR-002: LLMError hierarchy
    - FR-ERR-003: CacheError hierarchy
    - FR-ERR-004: Error context with provider
"""

from __future__ import annotations

import pytest

from portkey.domain import (
    AuthenticationError,
    CacheError,
    CacheKeyNotFoundError,
    CacheSerializationError,
    ContextLengthError,
    InvalidRequestError,
    LLMError,
    ModelNotSupportedError,
    PortkeyError,
    ProviderError,
    RateLimitError,
)


class TestPortkeyError:
    """Tests for PortkeyError base class."""

    # Traces to: FR-ERR-001
    def test_base_error_message(self) -> None:
        """Base error stores and formats message."""
        error = PortkeyError("Something went wrong")
        assert error.message == "Something went wrong"
        assert str(error) == "Something went wrong"

    # Traces to: FR-ERR-001
    def test_base_error_with_provider(self) -> None:
        """Base error includes provider context."""
        error = PortkeyError("Failed", provider="openai")
        assert error.provider == "openai"
        assert "provider=openai" in str(error)

    # Traces to: FR-ERR-001
    def test_base_error_message_and_provider(self) -> None:
        """Base error formats message and provider together."""
        error = PortkeyError("Request failed", provider="anthropic")
        assert str(error) == "Request failed | provider=anthropic"


class TestLLMErrorHierarchy:
    """Tests for LLMError and its subclasses."""

    # Traces to: FR-ERR-002
    def test_llm_error_is_portkey_error(self) -> None:
        """LLMError inherits from PortkeyError."""
        error = LLMError("LLM failed")
        assert isinstance(error, PortkeyError)
        assert error.message == "LLM failed"

    # Traces to: FR-ERR-002
    def test_model_not_supported_error(self) -> None:
        """ModelNotSupportedError for unsupported models."""
        error = ModelNotSupportedError("Model 'gpt-99' not supported", provider="openai")
        assert isinstance(error, LLMError)
        assert isinstance(error, PortkeyError)
        assert "gpt-99" in str(error)
        assert "provider=openai" in str(error)

    # Traces to: FR-ERR-002
    def test_rate_limit_error(self) -> None:
        """RateLimitError for rate limiting."""
        error = RateLimitError("Rate limit exceeded", provider="anthropic")
        assert isinstance(error, LLMError)
        assert "Rate limit exceeded" in str(error)

    # Traces to: FR-ERR-002
    def test_authentication_error(self) -> None:
        """AuthenticationError for auth failures."""
        error = AuthenticationError("Invalid API key", provider="openai")
        assert isinstance(error, LLMError)
        assert "Invalid API key" in str(error)

    # Traces to: FR-ERR-002
    def test_context_length_error(self) -> None:
        """ContextLengthError for token limit exceeded."""
        error = ContextLengthError("Context too long", provider="openai")
        assert isinstance(error, LLMError)
        assert "Context too long" in str(error)

    # Traces to: FR-ERR-002
    def test_invalid_request_error(self) -> None:
        """InvalidRequestError for bad requests."""
        error = InvalidRequestError("Invalid parameters", provider="vertex")
        assert isinstance(error, LLMError)
        assert "Invalid parameters" in str(error)

    # Traces to: FR-ERR-002
    def test_provider_error(self) -> None:
        """ProviderError for unexpected provider errors."""
        error = ProviderError("Provider error", provider="ollama")
        assert isinstance(error, LLMError)
        assert "Provider error" in str(error)

    # Traces to: FR-ERR-002
    def test_llm_error_without_provider(self) -> None:
        """LLMError works without provider context."""
        error = LLMError("Generic LLM error")
        assert error.provider is None
        assert str(error) == "Generic LLM error"


class TestCacheErrorHierarchy:
    """Tests for CacheError and its subclasses."""

    # Traces to: FR-ERR-003
    def test_cache_error_is_portkey_error(self) -> None:
        """CacheError inherits from PortkeyError."""
        error = CacheError("Cache operation failed")
        assert isinstance(error, PortkeyError)
        assert error.message == "Cache operation failed"

    # Traces to: FR-ERR-003
    def test_cache_key_not_found_error(self) -> None:
        """CacheKeyNotFoundError for missing keys."""
        error = CacheKeyNotFoundError("Key 'abc' not found")
        assert isinstance(error, CacheError)
        assert isinstance(error, PortkeyError)
        assert "Key 'abc' not found" in str(error)

    # Traces to: FR-ERR-003
    def test_cache_serialization_error(self) -> None:
        """CacheSerializationError for serialization failures."""
        error = CacheSerializationError("Failed to serialize")
        assert isinstance(error, CacheError)
        assert "Failed to serialize" in str(error)


class TestErrorCatching:
    """Tests for error catching behavior."""

    # Traces to: FR-ERR-001, FR-ERR-002
    def test_catch_llm_error_as_portkey_error(self) -> None:
        """Can catch LLMError as PortkeyError."""
        try:
            raise RateLimitError("Too many requests", provider="openai")
        except PortkeyError as e:
            assert "Too many requests" in str(e)
            assert e.provider == "openai"

    # Traces to: FR-ERR-001, FR-ERR-003
    def test_catch_cache_error_as_portkey_error(self) -> None:
        """Can catch CacheError as PortkeyError."""
        try:
            raise CacheKeyNotFoundError("Key missing")
        except PortkeyError as e:
            assert "Key missing" in str(e)

    # Traces to: FR-ERR-002
    def test_catch_specific_llm_errors(self) -> None:
        """Can catch specific LLM error types."""
        errors = [
            RateLimitError("Rate limited", provider="openai"),
            AuthenticationError("Auth failed", provider="anthropic"),
            ModelNotSupportedError("Bad model", provider="vertex"),
        ]

        for error in errors:
            with pytest.raises(type(error)):
                raise error


class TestErrorEquality:
    """Tests for error comparison behavior."""

    # Traces to: FR-ERR-001
    def test_error_message_comparison(self) -> None:
        """Errors with same message are not equal (no __eq__ defined)."""
        error1 = PortkeyError("Same message")
        error2 = PortkeyError("Same message")
        # Without custom __eq__, they compare by identity
        assert error1 is not error2
        assert str(error1) == str(error2)


class TestErrorExceptionBehavior:
    """Tests for error as exception behavior."""

    # Traces to: FR-ERR-001
    def test_error_is_exception(self) -> None:
        """PortkeyError is an Exception subclass."""
        error = PortkeyError("Test")
        assert isinstance(error, Exception)

    # Traces to: FR-ERR-001
    def test_error_can_be_raised(self) -> None:
        """PortkeyError can be raised and caught."""
        with pytest.raises(PortkeyError) as exc_info:
            raise PortkeyError("Test error", provider="test")

        assert exc_info.value.message == "Test error"
        assert exc_info.value.provider == "test"

    # Traces to: FR-ERR-002
    def test_llm_error_traceback(self) -> None:
        """LLMError preserves traceback information."""
        try:
            raise RateLimitError("Rate limited", provider="openai")
        except RateLimitError as e:
            import sys

            tb = sys.exc_info()[2]
            assert tb is not None
            assert e.message == "Rate limited"
