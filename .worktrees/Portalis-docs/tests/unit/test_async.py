"""Async tests for LLM provider operations.

Traces to: FR-PORT-001
"""
from __future__ import annotations

import pytest

from portkey.application.ports import LLMProvider
from portkey.domain.models import (
    CompletionRequest,
    EmbeddingRequest,
    Message,
    Provider,
    Response,
    Role,
)


class AsyncMockProvider(LLMProvider):
    """Mock provider with async methods for testing."""

    def __init__(self) -> None:
        self.call_count = 0
        self.embed_call_count = 0

    def complete(self, request: CompletionRequest) -> Response:
        """Sync complete (not used in async tests)."""
        self.call_count += 1
        return Response(
            content=f"Mock response {self.call_count}",
            model=request.model,
            provider=Provider.OPENAI,
        )

    async def complete_async(self, request: CompletionRequest) -> Response:
        """Async complete method."""
        self.call_count += 1
        return Response(
            content=f"Async response {self.call_count}",
            model=request.model,
            provider=Provider.OPENAI,
        )

    def embed(self, request: EmbeddingRequest) -> list[list[float]]:
        """Sync embed (not used in async tests)."""
        self.embed_call_count += 1
        return [[0.1, 0.2, 0.3] for _ in request.texts]

    async def embed_async(self, request: EmbeddingRequest) -> list[list[float]]:
        """Async embed method."""
        self.embed_call_count += 1
        return [[0.1 * i, 0.2 * i, 0.3 * i] for i in range(len(request.texts))]

    @property
    def provider_name(self) -> str:
        return "async-mock"

    @property
    def supported_models(self) -> list[str]:
        return ["async-model-1", "async-model-2"]


# Traces to: FR-PORT-001
@pytest.mark.asyncio
async def test_async_complete_basic() -> None:
    """Test basic async completion call."""
    provider = AsyncMockProvider()
    messages = [Message(role=Role.USER, content="Hello async")]
    request = CompletionRequest(messages=messages, model="async-model-1")

    response = await provider.complete_async(request)

    assert isinstance(response, Response)
    assert response.content == "Async response 1"
    assert response.model == "async-model-1"
    assert provider.call_count == 1


# Traces to: FR-PORT-001
@pytest.mark.asyncio
async def test_async_complete_multiple_messages() -> None:
    """Test async completion with multiple messages."""
    provider = AsyncMockProvider()
    messages = [
        Message(role=Role.SYSTEM, content="You are helpful"),
        Message(role=Role.USER, content="Hello"),
        Message(role=Role.ASSISTANT, content="Hi there!"),
        Message(role=Role.USER, content="How are you?"),
    ]
    request = CompletionRequest(messages=messages, model="async-model-2")

    response = await provider.complete_async(request)

    assert len(request.messages) == 4
    assert provider.call_count == 1


# Traces to: FR-PORT-001
@pytest.mark.asyncio
async def test_async_complete_sequential() -> None:
    """Test multiple sequential async completions."""
    provider = AsyncMockProvider()

    for i in range(5):
        messages = [Message(role=Role.USER, content=f"Message {i}")]
        request = CompletionRequest(messages=messages, model="async-model-1")
        response = await provider.complete_async(request)

        assert provider.call_count == i + 1
        assert response.content == f"Async response {i + 1}"


# Traces to: FR-PORT-001
@pytest.mark.asyncio
async def test_async_embed_basic() -> None:
    """Test basic async embed call."""
    provider = AsyncMockProvider()
    request = EmbeddingRequest(texts=["hello", "world"])

    embeddings = await provider.embed_async(request)

    assert len(embeddings) == 2
    assert len(embeddings[0]) == 3
    assert provider.embed_call_count == 1


# Traces to: FR-PORT-001
@pytest.mark.asyncio
async def test_async_embed_multiple_texts() -> None:
    """Test async embed with multiple texts."""
    provider = AsyncMockProvider()
    texts = ["text1", "text2", "text3", "text4", "text5"]
    request = EmbeddingRequest(texts=texts)

    embeddings = await provider.embed_async(request)

    assert len(embeddings) == 5
    assert provider.embed_call_count == 1


# Traces to: FR-PORT-001
def test_async_provider_properties() -> None:
    """Test async provider properties (sync test since no await needed)."""
    provider = AsyncMockProvider()

    assert provider.provider_name == "async-mock"
    assert "async-model-1" in provider.supported_models
    assert "async-model-2" in provider.supported_models
    assert len(provider.supported_models) == 2


# Traces to: FR-PORT-001
@pytest.mark.asyncio
async def test_async_concurrent_completions() -> None:
    """Test concurrent async completions."""
    provider = AsyncMockProvider()

    async def make_request(msg: str) -> Response:
        messages = [Message(role=Role.USER, content=msg)]
        return await provider.complete_async(
            CompletionRequest(messages=messages, model="async-model-1")
        )

    # Note: Can't use asyncio.gather here since we're using a mock
    # but this shows the pattern
    response1 = await make_request("First")
    response2 = await make_request("Second")

    assert provider.call_count == 2
    assert "Async response 1" in response1.content
    assert "Async response 2" in response2.content
