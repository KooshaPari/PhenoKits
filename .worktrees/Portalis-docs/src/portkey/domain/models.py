"""Domain models for portkey.

These models are pure domain objects with no external dependencies,
following Hexagonal Architecture principles.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from enum import Enum
from typing import Literal


class Role(str, Enum):
    """Message role in a conversation."""

    SYSTEM = "system"
    USER = "user"
    ASSISTANT = "assistant"
    TOOL = "tool"


class Provider(str, Enum):
    """Supported LLM providers."""

    OPENAI = "openai"
    ANTHROPIC = "anthropic"
    OLLAMA = "ollama"
    AZURE = "azure"
    VERTEX = "vertex"


@dataclass
class Message:
    """A single message in a conversation.

    Attributes:
        role: Who is speaking (system, user, assistant)
        content: The message content
        name: Optional name for the speaker
        tool_calls: Optional tool call requests
    """

    role: Role | Literal["system", "user", "assistant", "tool"]
    content: str
    name: str | None = None
    tool_calls: list[ToolCall] = field(default_factory=list)


@dataclass
class ToolCall:
    """A tool call request from the model."""

    id: str
    name: str
    arguments: dict[str, str]


@dataclass
class ToolDefinition:
    """Definition of a tool the model can call."""

    name: str
    description: str
    parameters: dict[str, object]  # JSON Schema


@dataclass
class Response:
    """Response from an LLM completion.

    Attributes:
        content: The text content of the response
        model: Model that generated the response
        provider: Provider that handled the request
        usage: Token usage statistics
        finish_reason: Why generation stopped
    """

    content: str
    model: str
    provider: Provider
    usage: Usage | None = None
    finish_reason: str | None = None
    tool_calls: list[ToolCall] = field(default_factory=list)


@dataclass
class Usage:
    """Token usage statistics."""

    prompt_tokens: int = 0
    completion_tokens: int = 0
    total_tokens: int = 0


@dataclass
class CompletionRequest:
    """Request for a completion.

    Attributes:
        messages: Conversation history
        model: Model identifier
        temperature: Sampling temperature (0-2)
        max_tokens: Maximum tokens to generate
        tools: Optional tool definitions
    """

    messages: list[Message]
    model: str
    temperature: float = 0.7
    max_tokens: int | None = None
    tools: list[ToolDefinition] | None = None
    stream: bool = False


@dataclass
class EmbeddingRequest:
    """Request for embeddings."""

    texts: list[str]
    model: str = "text-embedding-3-small"
    provider: Provider = Provider.OPENAI


@dataclass
class StreamingChunk:
    """A single chunk in a streaming response.

    Attributes:
        content: The text content of this chunk
        index: The index of this chunk in the stream
        is_final: Whether this is the final chunk
        usage: Token usage (only set on final chunk)
    """

    content: str
    index: int
    is_final: bool = False
    usage: Usage | None = None


@dataclass
class StreamingResponse:
    """Response from a streaming LLM completion.

    Attributes:
        model: Model that generated the response
        provider: Provider that handled the request
        chunks: The stream of content chunks
    """

    model: str
    provider: Provider
    chunks: list[StreamingChunk] = field(default_factory=list)

    def add_chunk(self, chunk: StreamingChunk) -> None:
        """Add a chunk to the response."""
        self.chunks.append(chunk)

    @property
    def full_content(self) -> str:
        """Get the full content from all chunks."""
        return "".join(chunk.content for chunk in self.chunks)

    @property
    def total_usage(self) -> Usage | None:
        """Get usage from the final chunk."""
        for chunk in reversed(self.chunks):
            if chunk.is_final and chunk.usage:
                return chunk.usage
        return None
