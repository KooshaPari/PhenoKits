"""Mistral AI provider implementation.

This adapter implements the LLMProvider port using the Mistral AI API.
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
    from mistralai.client import MistralClient

SUPPORTED_MODELS = [
    "mistral-large-latest",
    "mistral-medium-latest",
    "mistral-small-latest",
    "mistral-tiny",
    "codestral-latest",
    "open-mistral-7b",
    "open-mixtral-8x7b",
    "open-mixtral-8x22b",
]


class MistralProvider(LLMProvider):
    """Mistral AI provider implementation.

    Implements the LLMProvider port using the Mistral AI API.

    Example:
        ```python
        from portkey.infrastructure.mistral_provider import MistralProvider

        provider = MistralProvider(api_key="your-mistral-api-key")
        response = provider.complete(
            CompletionRequest(
                messages=[Message(role="user", content="Hello!")],
                model="mistral-large-latest"
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
        """Initialize Mistral provider.

        Args:
            api_key: Mistral API key.
            cache: Optional cache instance for responses.
            default_temperature: Default temperature for completions.
            default_max_tokens: Default max tokens for completions.
        """
        try:
            from mistralai.client import MistralClient  # noqa: PLC0415
        except ImportError as e:
            msg = "mistralai package is required. Install with: pip install mistralai"
            raise ImportError(msg) from e

        if api_key is None:
            import os  # noqa: PLC0415

            api_key = os.environ.get("MISTRAL_API_KEY", "")

        self._client: MistralClient = MistralClient(api_key=api_key)
        self._cache = cache
        self._default_temperature = default_temperature
        self._default_max_tokens = default_max_tokens

    def _make_cache_key(self, request: CompletionRequest) -> str:
        """Generate cache key from request."""
        messages_str = "|".join(f"{m.role}:{m.content}" for m in request.messages)
        return f"mistral:{request.model}:{request.temperature}:{messages_str}"

    def _to_mistral_messages(
        self, messages: list[Message]
    ) -> list[dict[str, str]]:
        """Convert Message objects to Mistral format."""
        result: list[dict[str, str]] = []
        for msg in messages:
            item: dict[str, str] = {"role": msg.role, "content": msg.content}
            result.append(item)
        return result

    def complete(self, request: CompletionRequest) -> Response:
        """Generate a completion using Mistral API.

        Args:
            request: The completion request

        Returns:
            Response from Mistral

        Raises:
            LLMError: If the request fails
        """
        if self._cache is not None:
            cache_key = self._make_cache_key(request)
            cached = self._cache.get(cache_key)
            if cached is not None:
                return cached

        try:
            import httpx  # noqa: F401

            messages = self._to_mistral_messages(request.messages)

            chat_response = self._client.chat(
                model=request.model,
                messages=messages,  # type: ignore[arg-type]
                temperature=request.temperature or self._default_temperature,
                max_tokens=request.max_tokens or self._default_max_tokens,
            )

            content = chat_response.choices[0].message.content or ""

            usage = Usage(
                prompt_tokens=chat_response.usage.prompt_tokens,
                completion_tokens=chat_response.usage.completion_tokens,
                total_tokens=chat_response.usage.total_tokens,
            )

            result = Response(
                content=content,
                model=request.model,
                provider=Provider.MISTRAL,
                usage=usage,
            )

            if self._cache is not None:
                self._cache.set(cache_key, result)

            return result

        except Exception as e:
            msg = f"Mistral API error: {e}"
            raise LLMError(msg, provider=Provider.MISTRAL) from e

    def embed(self, request: EmbeddingRequest) -> list[list[float]]:
        """Generate embeddings using Mistral API.

        Args:
            request: The embedding request

        Returns:
            List of embedding vectors

        Raises:
            LLMError: If the request fails
        """
        try:
            import httpx  # noqa: F401

            embeddings_batch_response = self._client.embeddings(
                model=request.model,
                input=request.texts,
            )
            return [item.embedding for item in embeddings_batch_response.data]
        except Exception as e:
            msg = f"Mistral embedding error: {e}"
            raise LLMError(msg, provider=Provider.MISTRAL) from e

    @property
    def provider_name(self) -> str:
        """Return the provider name."""
        return "mistral"

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
