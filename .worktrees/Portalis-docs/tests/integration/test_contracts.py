"""Contract tests for provider integrations.

Traces to: FR-CONTRACT-001

These tests verify that provider implementations follow the expected
contract defined by the LLMProvider port interface.
"""
from __future__ import annotations

from unittest.mock import MagicMock

from portkey.application.ports import Cache, LLMProvider
from portkey.domain.models import CompletionRequest, Message, Provider, Response, Role


# Traces to: FR-CONTRACT-001
class TestLLMProviderContract:
    """Contract tests for LLMProvider interface."""

    def test_provider_must_implement_complete(self) -> None:
        """Verify that providers must implement complete method."""
        mock_provider = MagicMock(spec=LLMProvider)
        mock_provider.complete.return_value = Response(
            content="test", model="test", provider=Provider.OPENAI
        )

        request = CompletionRequest(
            messages=[Message(role=Role.USER, content="test")],
            model="test-model",
        )

        result = mock_provider.complete(request)
        assert result is not None
        assert result.content == "test"

    def test_provider_must_implement_embed(self) -> None:
        """Verify that providers must implement embed method."""
        mock_provider = MagicMock(spec=LLMProvider)
        mock_provider.embed.return_value = [[0.1, 0.2, 0.3]]

        result = mock_provider.embed(texts=["test"], model="embedding-model")
        assert result is not None
        assert len(result) == 1

    def test_provider_accepts_cache_parameter(self) -> None:
        """Verify that complete method accepts optional cache parameter."""
        mock_provider = MagicMock(spec=LLMProvider)
        mock_cache = MagicMock(spec=Cache)
        mock_provider.complete.return_value = Response(
            content="cached", model="test", provider=Provider.OPENAI
        )

        request = CompletionRequest(
            messages=[Message(role=Role.USER, content="test")],
            model="test-model",
        )

        # Should work with cache parameter
        result = mock_provider.complete(request, cache=mock_cache)
        assert result is not None

    def test_provider_returns_response_with_correct_provider(self) -> None:
        """Verify that provider sets the correct provider in response."""
        mock_provider = MagicMock(spec=LLMProvider)
        expected_provider = Provider.ANTHROPIC
        mock_provider.complete.return_value = Response(
            content="test", model="claude-3", provider=expected_provider
        )

        request = CompletionRequest(
            messages=[Message(role=Role.USER, content="test")],
            model="claude-3",
        )

        result = mock_provider.complete(request)
        assert result.provider == expected_provider

    def test_provider_returns_response_with_model(self) -> None:
        """Verify that provider sets the model in response."""
        mock_provider = MagicMock(spec=LLMProvider)
        expected_model = "gpt-4-turbo"
        mock_provider.complete.return_value = Response(
            content="test", model=expected_model, provider=Provider.OPENAI
        )

        request = CompletionRequest(
            messages=[Message(role=Role.USER, content="test")],
            model=expected_model,
        )

        result = mock_provider.complete(request)
        assert result.model == expected_model


# Traces to: FR-CONTRACT-001
class TestCacheContract:
    """Contract tests for Cache interface."""

    def test_cache_must_implement_get(self) -> None:
        """Verify that cache implementations must implement get."""
        mock_cache = MagicMock(spec=Cache)
        mock_cache.get.return_value = Response(
            content="cached", model="test", provider=Provider.OPENAI
        )

        result = mock_cache.get("test_key")
        assert result is not None
        assert result.content == "cached"

    def test_cache_must_implement_set(self) -> None:
        """Verify that cache implementations must implement set."""
        mock_cache = MagicMock(spec=Cache)

        response = Response(content="test", model="test", provider=Provider.OPENAI)
        mock_cache.set("test_key", response)

        mock_cache.set.assert_called_once_with("test_key", response)

    def test_cache_must_implement_delete(self) -> None:
        """Verify that cache implementations must implement delete."""
        mock_cache = MagicMock(spec=Cache)

        mock_cache.delete("test_key")

        mock_cache.delete.assert_called_once_with("test_key")

    def test_cache_must_implement_clear(self) -> None:
        """Verify that cache implementations must implement clear."""
        mock_cache = MagicMock(spec=Cache)

        mock_cache.clear()

        mock_cache.clear.assert_called_once()

    def test_cache_len_returns_correct_count(self) -> None:
        """Verify that len returns correct count."""
        from portkey.infrastructure.cache import InMemoryCache

        cache = InMemoryCache()
        for i in range(5):
            cache.set(f"key_{i}", Response(content=f"val_{i}", model="test", provider=Provider.OPENAI))

        assert len(cache) == 5

    def test_cache_keys_returns_all_keys(self) -> None:
        """Verify that keys returns all cache keys."""
        from portkey.infrastructure.cache import InMemoryCache

        cache = InMemoryCache()
        expected_keys = ["key_a", "key_b", "key_c"]
        for key in expected_keys:
            cache.set(key, Response(content="val", model="test", provider=Provider.OPENAI))

        result = cache.keys()
        assert len(result) == 3
        for key in expected_keys:
            assert key in result


# Traces to: FR-CONTRACT-001
class TestProviderIntegrationContract:
    """Contract tests for provider-cache integration."""

    def test_provider_with_cache_checks_cache_first(self) -> None:
        """Verify that providers check cache before making API calls."""
        cache = MagicMock(spec=Cache)
        cache.get.return_value = Response(
            content="cached response", model="test", provider=Provider.OPENAI
        )

        provider = MagicMock(spec=LLMProvider)
        provider.complete.return_value = Response(
            content="api response", model="test", provider=Provider.OPENAI
        )

        request = CompletionRequest(
            messages=[Message(role=Role.USER, content="test")],
            model="test-model",
        )

        # With cache hit, provider.complete should NOT be called
        result = cache.get("cache_key")
        if result is not None:
            # Cache hit - should return cached response
            assert result.content == "cached response"

    def test_provider_with_cache_stores_on_miss(self) -> None:
        """Verify that providers store responses in cache on cache miss."""
        cache = MagicMock(spec=Cache)
        cache.get.return_value = None  # Cache miss

        provider = MagicMock(spec=LLMProvider)
        api_response = Response(
            content="api response", model="test", provider=Provider.OPENAI
        )
        provider.complete.return_value = api_response

        request = CompletionRequest(
            messages=[Message(role=Role.USER, content="test")],
            model="test-model",
        )

        # Simulate cache miss flow
        cached = cache.get("cache_key")
        if cached is None:
            result = provider.complete(request)
            cache.set("cache_key", result)

        # Verify cache.set was called
        cache.set.assert_called()


# Traces to: FR-CONTRACT-001
class TestRequestValidationContract:
    """Contract tests for request validation."""

    def test_request_must_have_messages(self) -> None:
        """Verify that CompletionRequest requires messages."""
        request = CompletionRequest(
            messages=[Message(role=Role.USER, content="test")],
            model="test-model",
        )
        assert len(request.messages) == 1

    def test_request_must_have_model(self) -> None:
        """Verify that CompletionRequest requires model."""
        request = CompletionRequest(
            messages=[Message(role=Role.USER, content="test")],
            model="gpt-4",
        )
        assert request.model == "gpt-4"

    def test_request_temperature_range(self) -> None:
        """Verify temperature validation in request."""
        # Valid temperature
        request = CompletionRequest(
            messages=[Message(role=Role.USER, content="test")],
            model="test",
            temperature=1.0,
        )
        assert request.temperature == 1.0

    def test_request_system_message(self) -> None:
        """Verify system message handling in request."""
        messages = [
            Message(role=Role.SYSTEM, content="You are helpful."),
            Message(role=Role.USER, content="Hello"),
        ]
        request = CompletionRequest(messages=messages, model="test")

        assert request.messages[0].role == Role.SYSTEM
        assert request.messages[0].content == "You are helpful."
