"""Port interfaces for portkey.

Following Hexagonal Architecture, ports define the boundaries between layers.
Infrastructure adapters implement these interfaces.
"""

from __future__ import annotations

from abc import ABC, abstractmethod
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from portkey.domain.models import CompletionRequest, EmbeddingRequest, Response


class LLMProvider(ABC):
    """Port for LLM providers.

    This port abstracts the interface for interacting with different
    LLM providers (OpenAI, Anthropic, Ollama, etc.).

    Example implementations:
        - OpenAIProvider
        - AnthropicProvider
        - OllamaProvider
    """

    @abstractmethod
    def complete(self, request: CompletionRequest) -> Response:
        """Generate a completion.

        Args:
            request: The completion request

        Returns:
            Response from the LLM

        Raises:
            LLMError: If the request fails
        """
        ...

    @abstractmethod
    def embed(self, request: EmbeddingRequest) -> list[list[float]]:
        """Generate embeddings.

        Args:
            request: The embedding request

        Returns:
            List of embedding vectors

        Raises:
            LLMError: If the request fails
        """
        ...

    @property
    @abstractmethod
    def provider_name(self) -> str:
        """Return the provider name."""
        ...

    @property
    @abstractmethod
    def supported_models(self) -> list[str]:
        """Return list of supported models."""
        ...


class TokenCounter(ABC):
    """Port for counting tokens.

    Different models have different tokenization schemes,
    so this allows swapping token counting strategies.
    """

    @abstractmethod
    def count_messages(self, messages: list[dict[str, str]]) -> int:
        """Count tokens in messages.

        Args:
            messages: List of message dicts

        Returns:
            Total token count
        """
        ...

    @abstractmethod
    def count_text(self, text: str) -> int:
        """Count tokens in text.

        Args:
            text: The text to count

        Returns:
            Token count
        """
        ...


class Cache(ABC):
    """Port for caching completions.

    Implementations:
        - InMemoryCache
        - RedisCache
        - FileCache
    """

    @abstractmethod
    def get(self, key: str) -> Response | None:
        """Get cached response.

        Args:
            key: Cache key

        Returns:
            Cached response or None
        """
        ...

    @abstractmethod
    def set(self, key: str, response: Response, ttl: int | None = None) -> None:
        """Cache a response.

        Args:
            key: Cache key
            response: Response to cache
            ttl: Time to live in seconds
        """
        ...

    @abstractmethod
    def delete(self, key: str) -> None:
        """Delete a cached response.

        Args:
            key: Cache key
        """
        ...

    @abstractmethod
    def clear(self) -> None:
        """Clear all cached responses."""
        ...

    @abstractmethod
    def keys(self) -> list[str]:
        """Return all cache keys."""
        ...
