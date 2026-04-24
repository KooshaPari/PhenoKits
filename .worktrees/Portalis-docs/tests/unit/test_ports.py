"""Unit tests for port interfaces.

Traces to:
    - FR-PORT-001: LLMProvider interface
    - FR-PORT-002: Cache interface
    - FR-PORT-003: TokenCounter interface
"""

from __future__ import annotations

from abc import ABC

import pytest

from portkey.application import Cache, LLMProvider, TokenCounter
from portkey.domain import CompletionRequest, EmbeddingRequest, Message, Provider, Response


class TestLLMProviderInterface:
    """Tests for LLMProvider port interface."""

    # Traces to: FR-PORT-001
    def test_llm_provider_is_abstract(self) -> None:
        """LLMProvider is an abstract base class."""
        assert issubclass(LLMProvider, ABC)

    # Traces to: FR-PORT-001
    def test_llm_provider_cannot_be_instantiated(self) -> None:
        """LLMProvider cannot be instantiated directly."""
        with pytest.raises(TypeError):
            LLMProvider()  # type: ignore

    # Traces to: FR-PORT-001
    def test_llm_provider_required_methods(self) -> None:
        """LLMProvider defines required abstract methods."""
        assert hasattr(LLMProvider, "complete")
        assert hasattr(LLMProvider, "embed")
        assert hasattr(LLMProvider, "provider_name")
        assert hasattr(LLMProvider, "supported_models")


class TestCacheInterface:
    """Tests for Cache port interface."""

    # Traces to: FR-PORT-002
    def test_cache_is_abstract(self) -> None:
        """Cache is an abstract base class."""
        assert issubclass(Cache, ABC)

    # Traces to: FR-PORT-002
    def test_cache_cannot_be_instantiated(self) -> None:
        """Cache cannot be instantiated directly."""
        with pytest.raises(TypeError):
            Cache()  # type: ignore

    # Traces to: FR-PORT-002
    def test_cache_required_methods(self) -> None:
        """Cache defines required abstract methods."""
        assert hasattr(Cache, "get")
        assert hasattr(Cache, "set")
        assert hasattr(Cache, "delete")
        assert hasattr(Cache, "clear")


class TestTokenCounterInterface:
    """Tests for TokenCounter port interface."""

    # Traces to: FR-PORT-003
    def test_token_counter_is_abstract(self) -> None:
        """TokenCounter is an abstract base class."""
        assert issubclass(TokenCounter, ABC)

    # Traces to: FR-PORT-003
    def test_token_counter_cannot_be_instantiated(self) -> None:
        """TokenCounter cannot be instantiated directly."""
        with pytest.raises(TypeError):
            TokenCounter()  # type: ignore

    # Traces to: FR-PORT-003
    def test_token_counter_required_methods(self) -> None:
        """TokenCounter defines required abstract methods."""
        assert hasattr(TokenCounter, "count_messages")
        assert hasattr(TokenCounter, "count_text")


class TestPortImplementations:
    """Tests for concrete port implementations."""

    # Traces to: FR-PORT-002
    def test_in_memory_cache_implements_cache(self) -> None:
        """InMemoryCache properly implements Cache port."""
        from portkey.infrastructure import InMemoryCache

        cache = InMemoryCache()
        assert isinstance(cache, Cache)

        # Verify all methods work
        response = Response(content="Test", model="gpt-4", provider=Provider.OPENAI)
        cache.set("key", response)
        assert cache.get("key") == response
        cache.delete("key")
        assert cache.get("key") is None
        cache.clear()
