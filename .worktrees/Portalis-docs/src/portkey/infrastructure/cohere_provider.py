"""Cohere AI provider implementation.

This adapter implements the LLMProvider port using the Cohere API.
"""
from __future__ import annotations

from typing import TYPE_CHECKING

from portkey.application.ports import Cache, LLMProvider
from portkey.domain.errors import LLMError
from portkey.domain.models import (
    CompletionRequest,
    EmbeddingRequest,
    Message,
    Provider,
    Response,
    Usage,
)

if TYPE_CHECKING:
    from cohere import Client

SUPPORTED_MODELS = [
    "command",
    "command-r",
    "command-r-plus",
    "command-r7b-12-2024",
    "c4ai-aya-expanse-8b",
    "c4ai-aya-expanse-32k",
]


class CohereProvider(LLMProvider):
    """Cohere AI provider implementation.

    Implements the LLMProvider port using the Cohere API.

    Example:
        ```python
        from portkey.infrastructure.cohere_provider import CohereProvider

        provider = CohereProvider(api_key="your-cohere-api-key")
        response = provider.complete(
            CompletionRequest(
                messages=[Message(role="user", content="Hello!")],
                model="command-r"
            )
        )
        print(response.content)
        ```
    """

    def __init__(
        self,
        api_key: str | None = None,
        cache: Cache | None = None,
        default_temperature: float = 0.7,
        default_max_tokens: int = 1024,
    ) -> None:
        """Initialize Cohere provider.

        Args:
            api_key: Cohere API key.
            cache: Optional cache instance for responses.
            default_temperature: Default temperature for completions.
            default_max_tokens: Default max tokens for completions.
        """
        try:
            from cohere import Client  # noqa: PLC0415
        except ImportError as e:
            msg = "cohere package is required. Install with: pip install cohere"
            raise ImportError(msg) from e

        if api_key is None:
            import os  # noqa: PLC0415

            api_key = os.environ.get("COHERE_API_KEY", "")

        self._client: Client = Client(api_key=api_key)
        self._cache = cache
        self._default_temperature = default_temperature
        self._default_max_tokens = default_max_tokens

    def _make_cache_key(self, request: CompletionRequest) -> str:
        """Generate cache key from request."""
        messages_str = "|".join(f"{m.role}:{m.content}" for m in request.messages)
        return f"cohere:{request.model}:{request.temperature}:{messages_str}"

    def _to_cohere_messages(
        self, messages: list[Message]
    ) -> list[dict[str, str]]:
        """Convert Message objects to Cohere format."""
        result: list[dict[str, str]] = []
        for msg in messages:
            # Cohere uses 'USER' and 'CHATBOT' instead of lowercase
            role = msg.role.upper() if msg.role != "system" else "SYSTEM"
            item: dict[str, str] = {"role": role, "content": msg.content}
            result.append(item)
        return result

    def complete(self, request: CompletionRequest) -> Response:
        """Generate a completion using Cohere API.

        Args:
            request: The completion request

        Returns:
            Response from Cohere

        Raises:
            LLMError: If the request fails
        """
        if self._cache is not None:
            cache_key = self._make_cache_key(request)
            cached = self._cache.get(cache_key)
            if cached is not None:
                return cached

        try:
            messages = self._to_cohere_messages(request.messages)

            # Cohere chat API
            chat_response = self._client.chat(
                model=request.model,
                message=messages[-1]["content"] if messages else "",  # type: ignore[arg-type]
                chat_history=messages[:-1] if len(messages) > 1 else [],  # type: ignore[arg-type]
                temperature=request.temperature or self._default_temperature,
                max_tokens=request.max_tokens or self._default_max_tokens,
            )

            content = chat_response.text or ""

            usage = Usage(
                prompt_tokens=0,  # Cohere doesn't always provide token counts
                completion_tokens=0,
                total_tokens=0,
            )

            result = Response(
                content=content,
                model=request.model,
                provider=Provider.COHERE,
                usage=usage,
            )

            if self._cache is not None:
                self._cache.set(cache_key, result)

            return result

        except Exception as e:
            msg = f"Cohere API error: {e}"
            raise LLMError(msg, provider=Provider.COHERE) from e

    def embed(self, request: EmbeddingRequest) -> list[list[float]]:
        """Generate embeddings using Cohere API.

        Args:
            request: The embedding request

        Returns:
            List of embedding vectors

        Raises:
            LLMError: If the request fails
        """
        try:
            embeddings_response = self._client.embed(
                model="embed-english-v3.0",
                texts=request.texts,
            )
            return embeddings_response.embeddings
        except Exception as e:
            msg = f"Cohere embedding error: {e}"
            raise LLMError(msg, provider=Provider.COHERE) from e

    @property
    def provider_name(self) -> str:
        """Return the provider name."""
        return "cohere"

    @property
    def supported_models(self) -> list[str]:
        """Return list of supported models."""
        return SUPPORTED_MODELS.copy()

    def set_cache(self, cache: Cache) -> None:
        """Set or update the cache instance."""
        self._cache = cache

    def clear_cache(self) -> None:
        """Clear the response cache if one is set."""
        if self._cache is not None:
            self._cache.clear()
