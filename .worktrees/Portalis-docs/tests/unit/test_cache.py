"""Unit tests for InMemoryCache adapter.

Traces to:
    - FR-CACHE-001: Cache interface implementation
    - FR-CACHE-002: Get/set operations
    - FR-CACHE-003: Delete operations
    - FR-CACHE-004: Clear operations
    - FR-CACHE-005: TTL support (interface)
"""

from __future__ import annotations

import pytest

from portkey.domain import Provider, Response, Usage
from portkey.infrastructure import InMemoryCache


class TestInMemoryCacheCreation:
    """Tests for cache initialization."""

    # Traces to: FR-CACHE-001
    def test_cache_creation(self) -> None:
        """InMemoryCache can be instantiated."""
        cache = InMemoryCache()
        assert cache is not None
        assert len(cache) == 0

    # Traces to: FR-CACHE-001
    def test_cache_is_empty_on_creation(self) -> None:
        """New cache has no entries."""
        cache = InMemoryCache()
        assert len(cache) == 0


class TestCacheSetOperation:
    """Tests for cache set operation."""

    # Traces to: FR-CACHE-002
    def test_set_stores_response(self) -> None:
        """Set stores response in cache."""
        cache = InMemoryCache()
        response = Response(
            content="Test response",
            model="gpt-4",
            provider=Provider.OPENAI,
        )
        cache.set("test-key", response)

        assert len(cache) == 1
        retrieved = cache.get("test-key")
        assert retrieved is not None
        assert retrieved.content == "Test response"

    # Traces to: FR-CACHE-002
    def test_set_overwrites_existing(self) -> None:
        """Set overwrites existing entry with same key."""
        cache = InMemoryCache()
        response1 = Response(content="First", model="gpt-4", provider=Provider.OPENAI)
        response2 = Response(content="Second", model="gpt-4", provider=Provider.OPENAI)

        cache.set("key", response1)
        cache.set("key", response2)

        assert len(cache) == 1
        retrieved = cache.get("key")
        assert retrieved.content == "Second"

    # Traces to: FR-CACHE-005
    def test_set_with_ttl_parameter(self) -> None:
        """Set accepts TTL parameter (interface compliance)."""
        cache = InMemoryCache()
        response = Response(content="Test", model="gpt-4", provider=Provider.OPENAI)

        # TTL is accepted but not enforced in InMemoryCache
        cache.set("key", response, ttl=3600)
        assert cache.get("key") is not None

    # Traces to: FR-CACHE-002
    def test_set_multiple_keys(self) -> None:
        """Set can store multiple different keys."""
        cache = InMemoryCache()

        for i in range(5):
            response = Response(
                content=f"Response {i}",
                model="gpt-4",
                provider=Provider.OPENAI,
            )
            cache.set(f"key-{i}", response)

        assert len(cache) == 5


class TestCacheGetOperation:
    """Tests for cache get operation."""

    # Traces to: FR-CACHE-002
    def test_get_existing_key(self) -> None:
        """Get returns response for existing key."""
        cache = InMemoryCache()
        response = Response(
            content="Cached response",
            model="gpt-4",
            provider=Provider.OPENAI,
        )
        cache.set("my-key", response)

        retrieved = cache.get("my-key")
        assert retrieved is not None
        assert retrieved.content == "Cached response"
        assert retrieved.model == "gpt-4"
        assert retrieved.provider == Provider.OPENAI

    # Traces to: FR-CACHE-002
    def test_get_nonexistent_key(self) -> None:
        """Get returns None for non-existent key."""
        cache = InMemoryCache()
        result = cache.get("nonexistent-key")
        assert result is None

    # Traces to: FR-CACHE-002
    def test_get_preserves_response_attributes(self) -> None:
        """Get returns response with all attributes intact."""
        cache = InMemoryCache()
        usage = Usage(prompt_tokens=10, completion_tokens=20, total_tokens=30)
        response = Response(
            content="Test",
            model="gpt-4",
            provider=Provider.OPENAI,
            usage=usage,
            finish_reason="stop",
        )
        cache.set("key", response)

        retrieved = cache.get("key")
        assert retrieved.usage.prompt_tokens == 10
        assert retrieved.usage.completion_tokens == 20
        assert retrieved.finish_reason == "stop"


class TestCacheDeleteOperation:
    """Tests for cache delete operation."""

    # Traces to: FR-CACHE-003
    def test_delete_existing_key(self) -> None:
        """Delete removes existing key from cache."""
        cache = InMemoryCache()
        response = Response(content="Test", model="gpt-4", provider=Provider.OPENAI)
        cache.set("key-to-delete", response)

        assert len(cache) == 1
        cache.delete("key-to-delete")
        assert len(cache) == 0
        assert cache.get("key-to-delete") is None

    # Traces to: FR-CACHE-003
    def test_delete_nonexistent_key(self) -> None:
        """Delete handles non-existent key gracefully."""
        cache = InMemoryCache()
        # Should not raise
        cache.delete("nonexistent")
        assert len(cache) == 0

    # Traces to: FR-CACHE-003
    def test_delete_specific_key(self) -> None:
        """Delete only removes specified key."""
        cache = InMemoryCache()

        cache.set("key1", Response(content="1", model="gpt-4", provider=Provider.OPENAI))
        cache.set("key2", Response(content="2", model="gpt-4", provider=Provider.OPENAI))
        cache.set("key3", Response(content="3", model="gpt-4", provider=Provider.OPENAI))

        cache.delete("key2")

        assert len(cache) == 2
        assert cache.get("key1") is not None
        assert cache.get("key2") is None
        assert cache.get("key3") is not None


class TestCacheClearOperation:
    """Tests for cache clear operation."""

    # Traces to: FR-CACHE-004
    def test_clear_empty_cache(self) -> None:
        """Clear on empty cache does nothing."""
        cache = InMemoryCache()
        cache.clear()
        assert len(cache) == 0

    # Traces to: FR-CACHE-004
    def test_clear_removes_all_entries(self) -> None:
        """Clear removes all entries from cache."""
        cache = InMemoryCache()

        for i in range(10):
            response = Response(
                content=f"Response {i}",
                model="gpt-4",
                provider=Provider.OPENAI,
            )
            cache.set(f"key-{i}", response)

        assert len(cache) == 10
        cache.clear()
        assert len(cache) == 0

        # Verify all keys are gone
        for i in range(10):
            assert cache.get(f"key-{i}") is None


class TestCacheEdgeCases:
    """Tests for cache edge cases."""

    # Traces to: FR-CACHE-002
    def test_empty_key(self) -> None:
        """Cache handles empty string key."""
        cache = InMemoryCache()
        response = Response(content="Test", model="gpt-4", provider=Provider.OPENAI)

        cache.set("", response)
        assert cache.get("") is not None
        assert cache.get("").content == "Test"

    # Traces to: FR-CACHE-002
    def test_special_characters_in_key(self) -> None:
        """Cache handles special characters in keys."""
        cache = InMemoryCache()
        response = Response(content="Test", model="gpt-4", provider=Provider.OPENAI)

        special_keys = [
            "key with spaces",
            "key\twith\ttabs",
            "key\nwith\nnewlines",
            "🔑emoji-key",
            "key/with/slashes",
            "key-with-dashes_and_underscores.123",
        ]

        for key in special_keys:
            cache.set(key, response)
            assert cache.get(key) is not None

    # Traces to: FR-CACHE-002
    def test_large_key(self) -> None:
        """Cache handles large keys."""
        cache = InMemoryCache()
        response = Response(content="Test", model="gpt-4", provider=Provider.OPENAI)

        large_key = "x" * 10000
        cache.set(large_key, response)
        assert cache.get(large_key) is not None

    # Traces to: FR-CACHE-002
    def test_response_with_complex_content(self) -> None:
        """Cache handles responses with complex content."""
        cache = InMemoryCache()
        complex_content = """
        This is a multi-line response
        with "quotes" and 'apostrophes'
        and special chars: <>&%
        """
        response = Response(
            content=complex_content,
            model="gpt-4",
            provider=Provider.OPENAI,
        )

        cache.set("complex", response)
        retrieved = cache.get("complex")
        assert retrieved.content == complex_content


class TestCacheLenOperation:
    """Tests for cache __len__ operation."""

    # Traces to: FR-CACHE-001
    def test_len_increases_on_set(self) -> None:
        """Len increases when adding entries."""
        cache = InMemoryCache()
        response = Response(content="Test", model="gpt-4", provider=Provider.OPENAI)

        assert len(cache) == 0
        cache.set("a", response)
        assert len(cache) == 1
        cache.set("b", response)
        assert len(cache) == 2

    # Traces to: FR-CACHE-003
    def test_len_decreases_on_delete(self) -> None:
        """Len decreases when deleting entries."""
        cache = InMemoryCache()
        response = Response(content="Test", model="gpt-4", provider=Provider.OPENAI)

        cache.set("a", response)
        cache.set("b", response)
        assert len(cache) == 2

        cache.delete("a")
        assert len(cache) == 1

    # Traces to: FR-CACHE-004
    def test_len_zero_after_clear(self) -> None:
        """Len is zero after clear."""
        cache = InMemoryCache()
        response = Response(content="Test", model="gpt-4", provider=Provider.OPENAI)

        for i in range(5):
            cache.set(f"key-{i}", response)

        assert len(cache) == 5
        cache.clear()
        assert len(cache) == 0
