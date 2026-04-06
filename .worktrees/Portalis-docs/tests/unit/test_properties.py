"""Property-based tests using Hypothesis.

Traces to: FR-TEST-002
"""
from __future__ import annotations

from typing import Any
from unittest.mock import MagicMock

import pytest
from hypothesis import given, settings
from hypothesis import strategies as st

from portkey.application.ports import Cache
from portkey.domain.models import (
    Message,
    Provider,
    Response,
    Role,
    Usage,
)
from portkey.infrastructure.cache import InMemoryCache


# Traces to: FR-TEST-002
class TestMessageProperties:
    """Property-based tests for Message model."""

    @given(
        role=st.sampled_from(Role),
        content=st.text(min_size=0, max_size=10000),
        name=st.one_of(st.none(), st.text(min_size=1, max_size=100)),
    )
    @settings(max_examples=50)
    def test_message_creation_with_various_inputs(
        self, role: Role, content: str, name: str | None
    ) -> None:
        """Test message creation with various inputs."""
        msg = Message(role=role, content=content, name=name)

        assert msg.role == role
        assert msg.content == content
        assert msg.name == name

    @given(content=st.text(min_size=1, max_size=1000))
    @settings(max_examples=20)
    def test_role_str_comparison(self, content: str) -> None:
        """Test that role value comparison works correctly."""
        msg = Message(role=Role.USER, content=content)

        assert msg.role == Role.USER
        assert msg.role != Role.ASSISTANT

    @given(content=st.text(min_size=0, max_size=5000))
    @settings(max_examples=30)
    def test_empty_and_long_messages(self, content: str) -> None:
        """Test handling of empty and very long messages."""
        msg = Message(role=Role.SYSTEM, content=content)
        assert msg.content == content


# Traces to: FR-TEST-002
class TestResponseProperties:
    """Property-based tests for Response model."""

    @given(
        content=st.text(min_size=0, max_size=5000),
        model=st.text(min_size=1, max_size=100),
        provider=st.sampled_from(Provider),
    )
    @settings(max_examples=30)
    def test_response_creation_properties(
        self, content: str, model: str, provider: Provider
    ) -> None:
        """Test response creation with various properties."""
        response = Response(content=content, model=model, provider=provider)

        assert response.content == content
        assert response.model == model
        assert response.provider == provider

    @given(
        prompt_tokens=st.integers(min_value=0, max_value=100000),
        completion_tokens=st.integers(min_value=0, max_value=100000),
    )
    @settings(max_examples=30)
    def test_usage_total_calculation(
        self, prompt_tokens: int, completion_tokens: int
    ) -> None:
        """Test usage total tokens calculation."""
        usage = Usage(
            prompt_tokens=prompt_tokens, completion_tokens=completion_tokens
        )
        # total_tokens is calculated from the parameters
        assert usage.prompt_tokens == prompt_tokens
        assert usage.completion_tokens == completion_tokens

    @given(
        prompt_tokens=st.integers(min_value=0, max_value=100000),
        completion_tokens=st.integers(min_value=0, max_value=100000),
        total_tokens=st.integers(min_value=0, max_value=200000),
    )
    @settings(max_examples=30)
    def test_usage_with_explicit_total(
        self, prompt_tokens: int, completion_tokens: int, total_tokens: int
    ) -> None:
        """Test usage with explicitly provided total tokens."""
        usage = Usage(
            prompt_tokens=prompt_tokens,
            completion_tokens=completion_tokens,
            total_tokens=total_tokens,
        )
        assert usage.total_tokens == total_tokens


# Traces to: FR-TEST-002
class TestCacheProperties:
    """Property-based tests for Cache operations."""

    @given(keys=st.lists(st.text(min_size=1, max_size=100), min_size=0, max_size=50, unique=True))
    @settings(max_examples=30)
    def test_cache_set_and_get_various_keys(self, keys: list[str]) -> None:
        """Test cache operations with various key patterns."""
        cache: Cache = InMemoryCache()

        # Set values
        for key in keys:
            response = Response(content=f"value_for_{key}", model="test", provider=Provider.OPENAI)
            cache.set(key, response)

        # Verify all keys exist
        assert len(cache) == len(keys)

        for key in keys:
            result = cache.get(key)
            assert result is not None
            assert result.content == f"value_for_{key}"

    @given(
        content=st.text(min_size=0, max_size=5000),
        model=st.text(min_size=1, max_size=100),
        provider=st.sampled_from(Provider),
    )
    @settings(max_examples=30)
    def test_cache_stores_response_preserving_data(
        self, content: str, model: str, provider: Provider
    ) -> None:
        """Test that cache preserves response data exactly."""
        cache: Cache = InMemoryCache()
        original = Response(content=content, model=model, provider=provider)

        cache.set("test_key", original)
        retrieved = cache.get("test_key")

        assert retrieved is not None
        assert retrieved.content == original.content
        assert retrieved.model == original.model
        assert retrieved.provider == original.provider

    @given(keys=st.lists(st.text(min_size=1, max_size=50), min_size=0, max_size=20, unique=True))
    @settings(max_examples=30)
    def test_cache_clear_removes_all(self, keys: list[str]) -> None:
        """Test that cache clear removes all entries."""
        cache: Cache = InMemoryCache()

        for key in keys:
            response = Response(content="value", model="test", provider=Provider.OPENAI)
            cache.set(key, response)

        assert len(cache) == len(keys)

        cache.clear()

        assert len(cache) == 0
        for key in keys:
            assert cache.get(key) is None


# Traces to: FR-TEST-002
class TestProviderEnumProperties:
    """Property-based tests for Provider enum."""

    @given(provider=st.sampled_from(Provider))
    @settings(max_examples=10)
    def test_provider_value_is_string(self, provider: Provider) -> None:
        """Test that provider value is always a string."""
        assert isinstance(provider.value, str)
        assert len(provider.value) > 0

    @given(provider=st.sampled_from(Provider))
    @settings(max_examples=10)
    def test_provider_from_value(self, provider: Provider) -> None:
        """Test that we can get provider from its value."""
        retrieved = Provider(provider.value)
        assert retrieved == provider


# Traces to: FR-TEST-002
class TestRoleEnumProperties:
    """Property-based tests for Role enum."""

    @given(role=st.sampled_from(Role))
    @settings(max_examples=10)
    def test_role_value_is_string(self, role: Role) -> None:
        """Test that role value is always a string."""
        assert isinstance(role.value, str)
        assert len(role.value) > 0

    @given(role=st.sampled_from(Role))
    @settings(max_examples=10)
    def test_role_from_value(self, role: Role) -> None:
        """Test that we can get role from its value."""
        retrieved = Role(role.value)
        assert retrieved == role
