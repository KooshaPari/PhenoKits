"""Streaming response tests.

Traces to: FR-DOM-010
"""
from __future__ import annotations

import pytest

from portkey.domain.models import (
    Provider,
    StreamingChunk,
    StreamingResponse,
    Usage,
)


# Traces to: FR-DOM-010
class TestStreamingChunk:
    """Tests for StreamingChunk model."""

    def test_chunk_creation(self) -> None:
        """Test basic chunk creation."""
        chunk = StreamingChunk(content="Hello", index=0)
        assert chunk.content == "Hello"
        assert chunk.index == 0
        assert chunk.is_final is False
        assert chunk.usage is None

    def test_chunk_with_usage(self) -> None:
        """Test chunk with usage information."""
        usage = Usage(prompt_tokens=10, completion_tokens=20, total_tokens=30)
        chunk = StreamingChunk(content="Final", index=5, is_final=True, usage=usage)

        assert chunk.is_final is True
        assert chunk.usage == usage
        assert chunk.usage.total_tokens == 30


# Traces to: FR-DOM-010
class TestStreamingResponse:
    """Tests for StreamingResponse model."""

    def test_response_creation(self) -> None:
        """Test basic response creation."""
        response = StreamingResponse(model="gpt-4", provider=Provider.OPENAI)

        assert response.model == "gpt-4"
        assert response.provider == Provider.OPENAI
        assert len(response.chunks) == 0

    def test_add_chunk(self) -> None:
        """Test adding chunks to response."""
        response = StreamingResponse(model="gpt-4", provider=Provider.OPENAI)

        response.add_chunk(StreamingChunk(content="Hello ", index=0))
        response.add_chunk(StreamingChunk(content="world", index=1))

        assert len(response.chunks) == 2
        assert response.chunks[0].content == "Hello "
        assert response.chunks[1].content == "world"

    def test_full_content(self) -> None:
        """Test getting full content from chunks."""
        response = StreamingResponse(model="gpt-4", provider=Provider.OPENAI)

        chunks = [
            StreamingChunk(content="Hello ", index=0),
            StreamingChunk(content="beautiful ", index=1),
            StreamingChunk(content="world!", index=2),
        ]
        for chunk in chunks:
            response.add_chunk(chunk)

        assert response.full_content == "Hello beautiful world!"

    def test_full_content_empty(self) -> None:
        """Test full content with no chunks."""
        response = StreamingResponse(model="gpt-4", provider=Provider.OPENAI)
        assert not response.full_content

    def test_total_usage_with_final_chunk(self) -> None:
        """Test getting usage from final chunk."""
        response = StreamingResponse(model="gpt-4", provider=Provider.OPENAI)

        usage = Usage(prompt_tokens=10, completion_tokens=20, total_tokens=30)

        response.add_chunk(StreamingChunk(content="Part1 ", index=0))
        response.add_chunk(StreamingChunk(content="Part2 ", index=1))
        response.add_chunk(
            StreamingChunk(content="Done", index=2, is_final=True, usage=usage)
        )

        assert response.total_usage == usage
        assert response.total_usage.total_tokens == 30

    def test_total_usage_without_final_chunk(self) -> None:
        """Test getting usage when no final chunk has usage."""
        response = StreamingResponse(model="gpt-4", provider=Provider.OPENAI)

        response.add_chunk(StreamingChunk(content="Part1 ", index=0))
        response.add_chunk(StreamingChunk(content="Part2 ", index=1))
        response.add_chunk(StreamingChunk(content="Done", index=2, is_final=True))

        assert response.total_usage is None

    def test_total_usage_no_final_chunk(self) -> None:
        """Test getting usage when there's no final chunk."""
        response = StreamingResponse(model="gpt-4", provider=Provider.OPENAI)

        response.add_chunk(StreamingChunk(content="Part1 ", index=0))
        response.add_chunk(StreamingChunk(content="Part2 ", index=1))

        assert response.total_usage is None


# Traces to: FR-DOM-010
class TestStreamingWorkflow:
    """Tests simulating streaming workflow."""

    def test_simulate_streaming(self) -> None:
        """Simulate a complete streaming response."""
        response = StreamingResponse(model="claude-3", provider=Provider.ANTHROPIC)

        words = ["This ", "is ", "a ", "simulated ", "streaming ", "response!"]
        for i, word in enumerate(words):
            is_final = i == len(words) - 1
            if is_final:
                chunk = StreamingChunk(
                    content=word,
                    index=i,
                    is_final=True,
                    usage=Usage(prompt_tokens=5, completion_tokens=6, total_tokens=11),
                )
            else:
                chunk = StreamingChunk(content=word, index=i)
            response.add_chunk(chunk)

        assert len(response.chunks) == 6
        assert response.full_content == "This is a simulated streaming response!"
        assert response.total_usage.total_tokens == 11

    def test_streaming_chunks_accumulation(self) -> None:
        """Test that chunks accumulate correctly."""
        response = StreamingResponse(model="gpt-4-turbo", provider=Provider.OPENAI)

        for i in range(100):
            response.add_chunk(StreamingChunk(content=f"chunk{i} ", index=i))

        assert len(response.chunks) == 100
        assert response.full_content == "".join(f"chunk{i} " for i in range(100))
