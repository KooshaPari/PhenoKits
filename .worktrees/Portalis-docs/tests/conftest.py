"""Pytest configuration and shared fixtures for Portalis tests.

Traces to: FR-TEST-001 - Test infrastructure
"""

from __future__ import annotations

import pytest

from portkey.domain import (
    CompletionRequest,
    EmbeddingRequest,
    Message,
    Provider,
    Response,
    Role,
    ToolCall,
    ToolDefinition,
    Usage,
)
from portkey.infrastructure import InMemoryCache


@pytest.fixture
def sample_message() -> Message:
    """Fixture providing a sample user message."""
    return Message(role=Role.USER, content="Hello, world!")


@pytest.fixture
def sample_system_message() -> Message:
    """Fixture providing a sample system message."""
    return Message(role=Role.SYSTEM, content="You are a helpful assistant.")


@pytest.fixture
def sample_assistant_message() -> Message:
    """Fixture providing a sample assistant message."""
    return Message(role=Role.ASSISTANT, content="How can I help you today?")


@pytest.fixture
def sample_messages(sample_user_message: Message) -> list[Message]:
    """Fixture providing a list of sample messages."""
    return [
        Message(role=Role.SYSTEM, content="You are a helpful assistant."),
        sample_user_message,
    ]


@pytest.fixture
def sample_tool_call() -> ToolCall:
    """Fixture providing a sample tool call."""
    return ToolCall(
        id="call_123",
        name="get_weather",
        arguments={"location": "San Francisco"},
    )


@pytest.fixture
def sample_tool_definition() -> ToolDefinition:
    """Fixture providing a sample tool definition."""
    return ToolDefinition(
        name="get_weather",
        description="Get the current weather for a location",
        parameters={
            "type": "object",
            "properties": {
                "location": {"type": "string", "description": "City name"},
            },
            "required": ["location"],
        },
    )


@pytest.fixture
def sample_usage() -> Usage:
    """Fixture providing sample usage statistics."""
    return Usage(prompt_tokens=10, completion_tokens=20, total_tokens=30)


@pytest.fixture
def sample_response(sample_usage: Usage) -> Response:
    """Fixture providing a sample response."""
    return Response(
        content="This is a test response",
        model="gpt-4",
        provider=Provider.OPENAI,
        usage=sample_usage,
        finish_reason="stop",
    )


@pytest.fixture
def sample_completion_request(sample_messages: list[Message]) -> CompletionRequest:
    """Fixture providing a sample completion request."""
    return CompletionRequest(
        messages=sample_messages,
        model="gpt-4",
        temperature=0.7,
        max_tokens=100,
    )


@pytest.fixture
def sample_embedding_request() -> EmbeddingRequest:
    """Fixture providing a sample embedding request."""
    return EmbeddingRequest(
        texts=["Hello, world!", "Test embedding"],
        model="text-embedding-3-small",
        provider=Provider.OPENAI,
    )


@pytest.fixture
def empty_cache() -> InMemoryCache:
    """Fixture providing an empty cache instance."""
    return InMemoryCache()


@pytest.fixture
def populated_cache(sample_response: Response) -> InMemoryCache:
    """Fixture providing a cache with one entry."""
    cache = InMemoryCache()
    cache.set("test-key", sample_response)
    return cache
