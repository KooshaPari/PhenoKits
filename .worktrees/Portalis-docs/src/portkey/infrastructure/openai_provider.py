"""OpenAI provider implementation.

This adapter implements the LLMProvider port using the OpenAI API.
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
    from openai import OpenAI

SUPPORTED_MODELS = [
    "gpt-4o",
    "gpt-4o-mini",
    "gpt-4-turbo",
    "gpt-4",
    "gpt-3.5-turbo",
]


class OpenAIProvider(LLMProvider):
    """OpenAI provider implementation.

    Implements the LLMProvider port using the OpenAI API.

    Example:
        ```python
        from portkey.infrastructure.openai_provider import OpenAIProvider

        provider = OpenAIProvider(api_key="sk-...")
        response = provider.complete(
            CompletionRequest(
                messages=[Message(role="user", content="Hello!")],
                model="gpt-4o"
            )
        )
        print(response.content)
        ```
    """

    def __init__(
        self,
        api_key: str | None = None,
        base_url: str | None = None,
        cache: Cache | None = None,
        default_temperature: float = 0.7,
        default_max_tokens: int | None = None,
    ) -> None:
        """Initialize OpenAI provider.

        Args:
            api_key: OpenAI API key. If not provided, reads from OPENAI_API_KEY env var.
            base_url: Custom base URL for OpenAI-compatible APIs.
            cache: Optional cache instance for responses.
            default_temperature: Default temperature for completions.
            default_max_tokens: Default max tokens for completions.
        """
        try:
            from openai import OpenAI as OpenAIClient
        except ImportError as e:
            msg = "openai package is required. Install with: pip install portkey[openai]"
            raise ImportError(msg) from e

        if api_key is None:
            import os

            api_key = os.environ.get("OPENAI_API_KEY", "")

        self._client: OpenAI = OpenAIClient(api_key=api_key, base_url=base_url)
        self._cache = cache
        self._default_temperature = default_temperature
        self._default_max_tokens = default_max_tokens

    def _make_cache_key(self, request: CompletionRequest) -> str:
        """Generate cache key from request."""
        messages_str = "|".join(f"{m.role}:{m.content}" for m in request.messages)
        return f"openai:{request.model}:{request.temperature}:{messages_str}"

    def _to_openai_messages(
        self, messages: list[Message]
    ) -> list[dict[str, str]]:
        """Convert Message objects to OpenAI format."""
        result: list[dict[str, str]] = []
        for msg in messages:
            # Role is already a str (Role(str, Enum)), so just cast it
            item: dict[str, str] = {"role": msg.role, "content": msg.content}
            if msg.name:
                item["name"] = msg.name
            result.append(item)
        return result

    def _from_openai_response(
        self, completion: object, model: str
    ) -> Response:
        """Convert OpenAI response to Response model."""
        # Handle both chat and text completions
        if hasattr(completion, "choices"):
            # Chat completion
            choice = completion.choices[0]
            content = choice.message.content or ""

            usage_data = completion.usage  # type: ignore[attr-defined]
            usage = Usage(
                prompt_tokens=usage_data.prompt_tokens if usage_data else 0,
                completion_tokens=usage_data.completion_tokens if usage_data else 0,
                total_tokens=usage_data.total_tokens if usage_data else 0,
            )

            return Response(
                content=content,
                model=model,
                provider=Provider.OPENAI,
                usage=usage,
            )

        # Text completion (legacy) - should not happen with modern API
        msg = "Text completions not supported"
        raise NotImplementedError(msg)

    def complete(self, request: CompletionRequest) -> Response:
        """Generate a completion using OpenAI API.

        Args:
            request: The completion request

        Returns:
            Response from OpenAI

        Raises:
            LLMError: If the request fails
        """
        # Check cache first
        if self._cache is not None:
            cache_key = self._make_cache_key(request)
            cached = self._cache.get(cache_key)
            if cached is not None:
                return cached

        try:
            messages = self._to_openai_messages(request.messages)

            completion = self._client.chat.completions.create(
                model=request.model,
                messages=messages,  # type: ignore[arg-type]
                temperature=request.temperature or self._default_temperature,
                max_tokens=request.max_tokens or self._default_max_tokens,
                stream=False,
            )

            response = self._from_openai_response(completion, request.model)

            # Cache the response
            if self._cache is not None:
                self._cache.set(cache_key, response)

            return response

        except Exception as e:
            msg = f"OpenAI API error: {e}"
            raise LLMError(msg, provider=Provider.OPENAI) from e

    def embed(self, request: EmbeddingRequest) -> list[list[float]]:
        """Generate embeddings using OpenAI API.

        Args:
            request: The embedding request

        Returns:
            List of embedding vectors

        Raises:
            LLMError: If the request fails
        """
        try:
            response = self._client.embeddings.create(
                model=request.model,
                input=request.texts,
            )

            return [item.embedding for item in response.data]

        except Exception as e:
            msg = f"OpenAI embedding error: {e}"
            raise LLMError(msg, provider=Provider.OPENAI) from e

    @property
    def provider_name(self) -> str:
        """Return the provider name."""
        return "openai"

    @property
    def supported_models(self) -> list[str]:
        """Return list of supported models."""
        return SUPPORTED_MODELS.copy()

    def set_cache(self, cache: Cache) -> None:
        """Set or update the cache instance.

        Args:
            cache: The cache instance to use
        """
        self._cache = cache

    def clear_cache(self) -> None:
        """Clear the response cache if one is set."""
        if self._cache is not None:
            self._cache.clear()
