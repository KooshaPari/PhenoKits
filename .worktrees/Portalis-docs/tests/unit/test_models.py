"""Unit tests for domain models.

Traces to:
    - FR-DOM-001: Message model
    - FR-DOM-002: Response model
    - FR-DOM-003: CompletionRequest model
    - FR-DOM-004: Provider enum
    - FR-DOM-005: Role enum
    - FR-DOM-006: Usage model
    - FR-DOM-007: ToolCall model
    - FR-DOM-008: ToolDefinition model
    - FR-DOM-009: EmbeddingRequest model
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


class TestRole:
    """Tests for Role enum."""

    # Traces to: FR-DOM-005
    def test_role_values(self) -> None:
        """Role enum contains expected values."""
        assert Role.SYSTEM == "system"
        assert Role.USER == "user"
        assert Role.ASSISTANT == "assistant"
        assert Role.TOOL == "tool"

    # Traces to: FR-DOM-005
    def test_role_str_enum_behavior(self) -> None:
        """Role enum behaves as string enum."""
        # Note: str(Enum) returns repr, not value; use .value for the string
        assert Role.USER.value == "user"
        assert str(Role.USER.value) == "user"

    # Traces to: FR-DOM-005
    def test_role_comparison_with_string(self) -> None:
        """Role can be compared with string literals."""
        assert Role.USER == "user"
        assert Role.SYSTEM == "system"


class TestProvider:
    """Tests for Provider enum."""

    # Traces to: FR-DOM-004
    def test_provider_values(self) -> None:
        """Provider enum contains expected values."""
        assert Provider.OPENAI == "openai"
        assert Provider.ANTHROPIC == "anthropic"
        assert Provider.OLLAMA == "ollama"
        assert Provider.AZURE == "azure"
        assert Provider.VERTEX == "vertex"

    # Traces to: FR-DOM-004
    def test_provider_str_enum_behavior(self) -> None:
        """Provider enum behaves as string enum."""
        # Note: str(Enum) returns repr, not value; use .value for the string
        assert Provider.ANTHROPIC.value == "anthropic"
        assert str(Provider.ANTHROPIC.value) == "anthropic"


class TestMessage:
    """Tests for Message dataclass."""

    # Traces to: FR-DOM-001
    def test_message_creation(self) -> None:
        """Message can be created with required fields."""
        message = Message(role=Role.USER, content="Hello")
        assert message.role == Role.USER
        assert message.content == "Hello"
        assert message.name is None
        assert message.tool_calls == []

    # Traces to: FR-DOM-001
    def test_message_with_name(self) -> None:
        """Message can have optional name field."""
        message = Message(role=Role.USER, content="Hello", name="Alice")
        assert message.name == "Alice"

    # Traces to: FR-DOM-001
    def test_message_with_tool_calls(self) -> None:
        """Message can have tool calls."""
        tool_call = ToolCall(id="call_1", name="test", arguments={})
        message = Message(role=Role.ASSISTANT, content="", tool_calls=[tool_call])
        assert len(message.tool_calls) == 1
        assert message.tool_calls[0].id == "call_1"

    # Traces to: FR-DOM-001
    def test_message_with_string_role(self) -> None:
        """Message accepts string role (for compatibility)."""
        message = Message(role="user", content="Hello")  # type: ignore
        assert message.role == "user"


class TestToolCall:
    """Tests for ToolCall dataclass."""

    # Traces to: FR-DOM-007
    def test_tool_call_creation(self) -> None:
        """ToolCall can be created with all fields."""
        tool_call = ToolCall(
            id="call_123",
            name="get_weather",
            arguments={"location": "NYC"},
        )
        assert tool_call.id == "call_123"
        assert tool_call.name == "get_weather"
        assert tool_call.arguments == {"location": "NYC"}


class TestToolDefinition:
    """Tests for ToolDefinition dataclass."""

    # Traces to: FR-DOM-008
    def test_tool_definition_creation(self) -> None:
        """ToolDefinition can be created with all fields."""
        tool_def = ToolDefinition(
            name="calculator",
            description="Performs calculations",
            parameters={"type": "object", "properties": {}},
        )
        assert tool_def.name == "calculator"
        assert tool_def.description == "Performs calculations"
        assert tool_def.parameters == {"type": "object", "properties": {}}


class TestUsage:
    """Tests for Usage dataclass."""

    # Traces to: FR-DOM-006
    def test_usage_creation(self) -> None:
        """Usage can be created with default values."""
        usage = Usage()
        assert usage.prompt_tokens == 0
        assert usage.completion_tokens == 0
        assert usage.total_tokens == 0

    # Traces to: FR-DOM-006
    def test_usage_with_values(self) -> None:
        """Usage can be created with specific values."""
        usage = Usage(prompt_tokens=10, completion_tokens=20, total_tokens=30)
        assert usage.prompt_tokens == 10
        assert usage.completion_tokens == 20
        assert usage.total_tokens == 30


class TestResponse:
    """Tests for Response dataclass."""

    # Traces to: FR-DOM-002
    def test_response_creation(self) -> None:
        """Response can be created with required fields."""
        response = Response(
            content="Hello!",
            model="gpt-4",
            provider=Provider.OPENAI,
        )
        assert response.content == "Hello!"
        assert response.model == "gpt-4"
        assert response.provider == Provider.OPENAI
        assert response.usage is None
        assert response.finish_reason is None
        assert response.tool_calls == []

    # Traces to: FR-DOM-002
    def test_response_with_usage(self) -> None:
        """Response can include usage statistics."""
        usage = Usage(prompt_tokens=5, completion_tokens=10, total_tokens=15)
        response = Response(
            content="Hello!",
            model="gpt-4",
            provider=Provider.OPENAI,
            usage=usage,
        )
        assert response.usage == usage
        assert response.usage.prompt_tokens == 5

    # Traces to: FR-DOM-002
    def test_response_with_tool_calls(self) -> None:
        """Response can include tool calls."""
        tool_call = ToolCall(id="call_1", name="test", arguments={})
        response = Response(
            content="",
            model="gpt-4",
            provider=Provider.OPENAI,
            tool_calls=[tool_call],
        )
        assert len(response.tool_calls) == 1


class TestCompletionRequest:
    """Tests for CompletionRequest dataclass."""

    # Traces to: FR-DOM-003
    def test_completion_request_creation(self) -> None:
        """CompletionRequest can be created with required fields."""
        messages = [Message(role=Role.USER, content="Hello")]
        request = CompletionRequest(messages=messages, model="gpt-4")
        assert request.messages == messages
        assert request.model == "gpt-4"
        assert request.temperature == 0.7  # default
        assert request.max_tokens is None
        assert request.tools is None
        assert request.stream is False

    # Traces to: FR-DOM-003
    def test_completion_request_with_options(self) -> None:
        """CompletionRequest can include optional parameters."""
        messages = [Message(role=Role.USER, content="Hello")]
        tool_def = ToolDefinition(
            name="test",
            description="test tool",
            parameters={},
        )
        request = CompletionRequest(
            messages=messages,
            model="gpt-4",
            temperature=0.5,
            max_tokens=100,
            tools=[tool_def],
            stream=True,
        )
        assert request.temperature == 0.5
        assert request.max_tokens == 100
        assert request.tools == [tool_def]
        assert request.stream is True

    # Traces to: FR-DOM-003
    def test_completion_request_temperature_validation(self) -> None:
        """Temperature accepts valid range values."""
        messages = [Message(role=Role.USER, content="Hello")]
        request = CompletionRequest(messages=messages, model="gpt-4", temperature=0.0)
        assert request.temperature == 0.0

        request = CompletionRequest(messages=messages, model="gpt-4", temperature=2.0)
        assert request.temperature == 2.0


class TestEmbeddingRequest:
    """Tests for EmbeddingRequest dataclass."""

    # Traces to: FR-DOM-009
    def test_embedding_request_creation(self) -> None:
        """EmbeddingRequest can be created with defaults."""
        request = EmbeddingRequest(texts=["Hello", "World"])
        assert request.texts == ["Hello", "World"]
        assert request.model == "text-embedding-3-small"  # default
        assert request.provider == Provider.OPENAI  # default

    # Traces to: FR-DOM-009
    def test_embedding_request_with_custom_values(self) -> None:
        """EmbeddingRequest can use custom model and provider."""
        request = EmbeddingRequest(
            texts=["Test"],
            model="custom-model",
            provider=Provider.ANTHROPIC,
        )
        assert request.model == "custom-model"
        assert request.provider == Provider.ANTHROPIC
