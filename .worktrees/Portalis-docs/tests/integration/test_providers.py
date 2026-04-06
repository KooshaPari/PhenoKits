"""Tests for provider implementations.

Traces to: FR-PROV-001, FR-PROV-002, FR-PROV-003
"""
from __future__ import annotations

from unittest.mock import MagicMock, patch

import pytest

from portkey.domain.errors import LLMError
from portkey.domain.models import (
    CompletionRequest,
    Message,
    Provider,
    Response,
    Role,
)
from portkey.infrastructure.anthropic_provider import AnthropicProvider
from portkey.infrastructure.cache import InMemoryCache
from portkey.infrastructure.ollama_provider import OllamaProvider
from portkey.infrastructure.openai_provider import OpenAIProvider


# =============================================================================
# OpenAI Provider Tests
# =============================================================================
class TestOpenAIProvider:
    """Tests for OpenAIProvider."""

    def test_initialization(self) -> None:
        """Test provider initialization."""
        with patch("openai.OpenAI"):
            provider = OpenAIProvider(api_key="test-key")
            assert provider.provider_name == "openai"
            assert "gpt-4o" in provider.supported_models

    def test_initialization_with_env_key(self) -> None:
        """Test provider initialization reads API key from environment."""
        with patch.dict("os.environ", {"OPENAI_API_KEY": "env-key"}), patch("openai.OpenAI"):
            provider = OpenAIProvider()
            assert provider is not None

    def test_initialization_with_base_url(self) -> None:
        """Test provider initialization with custom base URL."""
        with patch("openai.OpenAI"):
            provider = OpenAIProvider(
                api_key="test-key", base_url="https://custom.api.com/v1"
            )
            assert provider.provider_name == "openai"

    def test_missing_openai_package(self) -> None:
        """Test error when openai package is not installed."""
        with patch.dict("sys.modules", {"openai": None}), pytest.raises(
            ImportError, match="openai package is required"
        ):
            OpenAIProvider(api_key="test-key")


# =============================================================================
# Anthropic Provider Tests
# =============================================================================
class TestAnthropicProvider:
    """Tests for AnthropicProvider."""

    def test_initialization(self) -> None:
        """Test provider initialization."""
        with patch("anthropic.Anthropic"):
            provider = AnthropicProvider(api_key="test-key")
            assert provider.provider_name == "anthropic"
            assert "claude-3-5-sonnet-latest" in provider.supported_models

    def test_initialization_with_env_key(self) -> None:
        """Test provider initialization reads API key from environment."""
        with patch.dict("os.environ", {"ANTHROPIC_API_KEY": "env-key"}), patch(
            "anthropic.Anthropic"
        ):
            provider = AnthropicProvider()
            assert provider is not None

    def test_message_conversion(self) -> None:
        """Test conversion of messages to Anthropic format."""
        with patch("anthropic.Anthropic"):
            provider = AnthropicProvider(api_key="test-key")

            messages = [
                Message(role=Role.SYSTEM, content="You are helpful."),
                Message(role=Role.USER, content="Hello"),
            ]

            conv_messages = provider._to_anthropic_messages(messages)
            assert len(conv_messages) == 1
            assert conv_messages[0]["role"] == "user"
            assert conv_messages[0]["content"] == "Hello"

            system = provider._get_system_prompt(messages)
            assert system == "You are helpful."

    def test_missing_anthropic_package(self) -> None:
        """Test error when anthropic package is not installed."""
        with patch.dict("sys.modules", {"anthropic": None}), pytest.raises(
            ImportError, match="anthropic package is required"
        ):
            AnthropicProvider(api_key="test-key")


# =============================================================================
# Ollama Provider Tests
# =============================================================================
class TestOllamaProvider:
    """Tests for OllamaProvider."""

    def test_initialization(self) -> None:
        """Test provider initialization."""
        provider = OllamaProvider(base_url="http://localhost:11434")
        assert provider.provider_name == "ollama"
        assert "llama3" in provider.supported_models

    def test_initialization_custom_url(self) -> None:
        """Test provider initialization with custom URL."""
        provider = OllamaProvider(base_url="http://192.168.1.100:11434")
        assert provider.provider_name == "ollama"

    def test_message_conversion(self) -> None:
        """Test conversion of messages to Ollama format."""
        provider = OllamaProvider()

        messages = [
            Message(role=Role.USER, content="Hello"),
            Message(role=Role.ASSISTANT, content="Hi there!"),
        ]

        ollama_messages = provider._to_ollama_messages(messages)
        assert len(ollama_messages) == 2
        assert ollama_messages[0]["role"] == "user"
        assert ollama_messages[1]["role"] == "assistant"

    def test_cache_key_generation(self) -> None:
        """Test cache key generation."""
        provider = OllamaProvider()

        request = CompletionRequest(
            messages=[Message(role=Role.USER, content="Hello")],
            model="llama3",
            temperature=0.7,
        )

        key = provider._make_cache_key(request)
        assert "ollama" in key
        assert "llama3" in key


# =============================================================================
# Provider Integration Tests
# =============================================================================
class TestProviderCacheIntegration:
    """Tests for provider with cache integration."""

    def test_openai_provider_with_cache(self) -> None:
        """Test OpenAI provider with cache."""
        with patch("openai.OpenAI"):
            provider = OpenAIProvider(api_key="test-key")
            cache = InMemoryCache()
            provider.set_cache(cache)
            assert provider._cache is cache

    def test_anthropic_provider_with_cache(self) -> None:
        """Test Anthropic provider with cache."""
        with patch("anthropic.Anthropic"):
            provider = AnthropicProvider(api_key="test-key")
            cache = InMemoryCache()
            provider.set_cache(cache)
            assert provider._cache is cache

    def test_ollama_provider_with_cache(self) -> None:
        """Test Ollama provider with cache."""
        provider = OllamaProvider()
        cache = InMemoryCache()
        provider.set_cache(cache)
        assert provider._cache is cache

    def test_provider_clear_cache(self) -> None:
        """Test clearing provider cache."""
        provider = OllamaProvider()
        cache = InMemoryCache()
        cache.set("test", Response(content="test", model="m", provider=Provider.OLLAMA))
        provider.set_cache(cache)
        assert len(cache) == 1
        provider.clear_cache()
        assert len(cache) == 0


# =============================================================================
# Provider Model Tests
# =============================================================================
class TestProviderModels:
    """Tests for provider models and constants."""

    def test_openai_supported_models(self) -> None:
        """Test OpenAI supported models list."""
        with patch("openai.OpenAI"):
            provider = OpenAIProvider(api_key="test-key")
            models = provider.supported_models
            assert "gpt-4o" in models
            assert "gpt-4" in models
            assert "gpt-3.5-turbo" in models

    def test_anthropic_supported_models(self) -> None:
        """Test Anthropic supported models list."""
        with patch("anthropic.Anthropic"):
            provider = AnthropicProvider(api_key="test-key")
            models = provider.supported_models
            assert "claude-3-5-sonnet-latest" in models
            assert "claude-3-opus-latest" in models

    def test_ollama_supported_models(self) -> None:
        """Test Ollama supported models list."""
        provider = OllamaProvider()
        models = provider.supported_models
        assert "llama3" in models
        assert "mistral" in models


# =============================================================================
# Error Handling Tests
# =============================================================================
class TestProviderErrorHandling:
    """Tests for provider error handling."""

    def test_openai_api_error_raises_llm_error(self) -> None:
        """Test that OpenAI API errors raise LLMError."""
        from openai import OpenAI

        mock_client = MagicMock()
        mock_client.chat.completions.create.side_effect = Exception("API Error")

        with patch("openai.OpenAI", return_value=mock_client):
            provider = OpenAIProvider(api_key="test-key")
            request = CompletionRequest(
                messages=[Message(role=Role.USER, content="test")],
                model="gpt-4o",
            )

            with pytest.raises(LLMError) as exc_info:
                provider.complete(request)

            assert exc_info.value.provider == Provider.OPENAI
            assert "OpenAI API error" in str(exc_info.value)

    def test_anthropic_api_error_raises_llm_error(self) -> None:
        """Test that Anthropic API errors raise LLMError."""
        mock_client = MagicMock()
        mock_client.messages.create.side_effect = Exception("API Error")

        with patch("anthropic.Anthropic", return_value=mock_client):
            provider = AnthropicProvider(api_key="test-key")
            request = CompletionRequest(
                messages=[Message(role=Role.USER, content="test")],
                model="claude-3-5-sonnet-latest",
            )

            with pytest.raises(LLMError) as exc_info:
                provider.complete(request)

            assert exc_info.value.provider == Provider.ANTHROPIC
            assert "Anthropic API error" in str(exc_info.value)

    def test_ollama_http_error_raises_llm_error(self) -> None:
        """Test that Ollama HTTP errors raise LLMError."""
        provider = OllamaProvider(base_url="http://localhost:11434")
        request = CompletionRequest(
            messages=[Message(role=Role.USER, content="test")],
            model="llama3",
        )

        with pytest.raises(LLMError) as exc_info:
            provider.complete(request)

        assert exc_info.value.provider == Provider.OLLAMA
