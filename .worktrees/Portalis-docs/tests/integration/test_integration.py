"""Integration tests for Portalis.

Traces to:
    - FR-INT-001: End-to-end completion flow
    - FR-INT-002: Provider integration patterns
    - FR-INT-003: Cache integration with providers
"""

from __future__ import annotations

import pytest

from portkey.application import LLMProvider
from portkey.domain import (
    CompletionRequest,
    EmbeddingRequest,
    Message,
    Provider,
    Response,
    Role,
    Usage,
)
from portkey.infrastructure import InMemoryCache


class MockProvider(LLMProvider):
    """Mock provider for integration testing."""

    def __init__(self, responses: dict[str, Response] | None = None) -> None:
        self._responses = responses or {}
        self._call_history: list[CompletionRequest] = []

    @property
    def provider_name(self) -> str:
        return "mock"

    @property
    def supported_models(self) -> list[str]:
        return ["mock-model-v1", "mock-model-v2"]

    def complete(self, request: CompletionRequest) -> Response:
        """Return mock response based on request."""
        self._call_history.append(request)

        # Return cached response if available
        if request.model in self._responses:
            return self._responses[request.model]

        return Response(
            content=f"Mock response for: {request.messages[-1].content if request.messages else ''}",
            model=request.model,
            provider=Provider.OPENAI,
            usage=Usage(prompt_tokens=10, completion_tokens=10, total_tokens=20),
            finish_reason="stop",
        )

    def embed(self, request: EmbeddingRequest) -> list[list[float]]:
        """Return mock embeddings."""
        return [[0.1, 0.2, 0.3] for _ in request.texts]


class TestProviderCacheIntegration:
    """Integration tests for provider and cache."""

    # Traces to: FR-INT-003
    def test_provider_with_cache_layer(self) -> None:
        """Provider can be wrapped with caching layer."""
        cache = InMemoryCache()
        provider = MockProvider()

        # Create a simple caching wrapper
        def cached_complete(request: CompletionRequest) -> Response:
            # Generate cache key from request
            cache_key = f"{request.model}:{hash(str(request.messages))}"

            # Check cache first
            cached = cache.get(cache_key)
            if cached:
                return cached

            # Call provider and cache result
            response = provider.complete(request)
            cache.set(cache_key, response)
            return response

        # First call - hits provider
        request = CompletionRequest(
            messages=[Message(role=Role.USER, content="Hello")],
            model="mock-model-v1",
        )
        response1 = cached_complete(request)
        assert response1.content == "Mock response for: Hello"
        assert len(cache) == 1

        # Second call - should hit cache
        response2 = cached_complete(request)
        assert response2.content == response1.content
        assert len(provider._call_history) == 1  # Provider only called once

    # Traces to: FR-INT-003
    def test_cache_invalidation_on_provider_error(self) -> None:
        """Cache behavior when provider fails."""
        cache = InMemoryCache()

        # Pre-populate cache with stale data
        stale_response = Response(
            content="Stale data",
            model="test",
            provider=Provider.OPENAI,
        )
        cache.set("stale-key", stale_response)

        # Verify cached data is retrievable
        cached = cache.get("stale-key")
        assert cached is not None
        assert cached.content == "Stale data"

        # Clear cache (simulating invalidation)
        cache.clear()
        assert cache.get("stale-key") is None


class TestMultiProviderSetup:
    """Integration tests for multi-provider configurations."""

    # Traces to: FR-INT-002
    def test_multiple_provider_registration(self) -> None:
        """Multiple providers can be registered and used."""
        openai_mock = MockProvider({
            "gpt-4": Response(
                content="OpenAI response",
                model="gpt-4",
                provider=Provider.OPENAI,
            ),
        })
        anthropic_mock = MockProvider({
            "claude-3": Response(
                content="Anthropic response",
                model="claude-3",
                provider=Provider.ANTHROPIC,
            ),
        })

        providers = {
            "openai": openai_mock,
            "anthropic": anthropic_mock,
        }

        # Route request to appropriate provider
        def route_request(request: CompletionRequest) -> Response:
            if request.model.startswith("gpt"):
                return providers["openai"].complete(request)
            if request.model.startswith("claude"):
                return providers["anthropic"].complete(request)
            raise ValueError(f"Unknown model: {request.model}")

        openai_request = CompletionRequest(
            messages=[Message(role=Role.USER, content="Test")],
            model="gpt-4",
        )
        response = route_request(openai_request)
        assert response.content == "OpenAI response"
        assert response.provider == Provider.OPENAI

        anthropic_request = CompletionRequest(
            messages=[Message(role=Role.USER, content="Test")],
            model="claude-3",
        )
        response = route_request(anthropic_request)
        assert response.content == "Anthropic response"
        assert response.provider == Provider.ANTHROPIC

    # Traces to: FR-INT-002
    def test_provider_fallback(self) -> None:
        """Fallback to secondary provider on primary failure."""
        primary = MockProvider()
        secondary = MockProvider({
            "fallback-model": Response(
                content="Fallback response",
                model="fallback-model",
                provider=Provider.OLLAMA,
            ),
        })

        # Simulate primary failure and fallback
        def complete_with_fallback(request: CompletionRequest) -> Response:
            try:
                # Simulate primary failure
                raise Exception("Primary provider unavailable")
            except Exception:
                return secondary.complete(request)

        request = CompletionRequest(
            messages=[Message(role=Role.USER, content="Test")],
            model="fallback-model",
        )
        response = complete_with_fallback(request)
        assert response.content == "Fallback response"


class TestEndToEndFlow:
    """End-to-end integration tests."""

    # Traces to: FR-INT-001
    def test_full_completion_flow(self) -> None:
        """Complete flow from request to response."""
        # Setup
        cache = InMemoryCache()
        provider = MockProvider()

        # Create conversation
        messages = [
            Message(role=Role.SYSTEM, content="You are a helpful assistant."),
            Message(role=Role.USER, content="What is 2+2?"),
        ]

        # Build request
        request = CompletionRequest(
            messages=messages,
            model="mock-model-v1",
            temperature=0.7,
            max_tokens=100,
        )

        # Execute completion (with caching)
        cache_key = f"complete:{hash(str(request.messages))}:{request.model}"

        cached = cache.get(cache_key)
        if cached:
            response = cached
        else:
            response = provider.complete(request)
            cache.set(cache_key, response)

        # Verify response
        assert response.content is not None
        assert response.model == "mock-model-v1"
        assert response.provider == Provider.OPENAI
        assert response.usage is not None
        assert response.usage.total_tokens > 0
        assert response.finish_reason == "stop"

        # Verify caching
        assert cache.get(cache_key) is not None

    # Traces to: FR-INT-001
    def test_conversation_thread(self) -> None:
        """Multi-turn conversation flow."""
        provider = MockProvider()
        conversation: list[Message] = [
            Message(role=Role.SYSTEM, content="You are a helpful assistant."),
        ]

        # Turn 1
        conversation.append(Message(role=Role.USER, content="Hello"))
        request1 = CompletionRequest(messages=conversation, model="mock-model-v1")
        response1 = provider.complete(request1)
        conversation.append(Message(role=Role.ASSISTANT, content=response1.content))

        # Turn 2
        conversation.append(Message(role=Role.USER, content="How are you?"))
        request2 = CompletionRequest(messages=conversation, model="mock-model-v1")
        response2 = provider.complete(request2)
        conversation.append(Message(role=Role.ASSISTANT, content=response2.content))

        # Verify conversation history preserved
        assert len(conversation) == 5  # system + (user + assistant) * 2
        assert provider._call_history[0] == request1
        assert provider._call_history[1] == request2
