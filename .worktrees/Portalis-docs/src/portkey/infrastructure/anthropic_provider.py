"""Anthropic provider implementation.

This adapter implements the LLMProvider port using the Anthropic API.
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
    Role,
    Usage,
)

if TYPE_CHECKING:
    from anthropic import Anthropic as AnthropicClient

SUPPORTED_MODELS = [
    "claude-opus-4-5",
    "claude-sonnet-4-5",
    "claude-3-5-sonnet-latest",
    "claude-3-opus-latest",
    "claude-3-sonnet-latest",
    "claude-3-haiku-latest",
]


class AnthropicProvider(LLMProvider):
    """Anthropic provider implementation.

    Implements the LLMProvider port using the Anthropic API.

    Example:
        ```python
        from portkey.infrastructure.anthropic_provider import AnthropicProvider

        provider = AnthropicProvider(api_key="sk-ant-...")
        response = provider.complete(
            CompletionRequest(
                messages=[Message(role="user", content="Hello!")],
                model="claude-3-5-sonnet-latest"
            )
        )
        print(response.content)
        ```
    """

    def __init__(
        self,
        api_key: str | None = None,
        cache: Cache | None = None,
        default_temperature: float = 1.0,
        default_max_tokens: int = 1024,
    ) -> None:
        """Initialize Anthropic provider.

        Args:
            api_key: Anthropic API key. If not provided, reads from ANTHROPIC_API_KEY env var.
            cache: Optional cache instance for responses.
            default_temperature: Default temperature for completions.
            default_max_tokens: Default max tokens for completions.
        """
        try:
            from anthropic import Anthropic
        except ImportError as e:
            msg = "anthropic package is required. Install with: pip install portkey[anthropic]"
            raise ImportError(msg) from e

        if api_key is None:
            import os

            api_key = os.environ.get("ANTHROPIC_API_KEY", "")

        self._client: AnthropicClient = Anthropic(api_key=api_key)
        self._cache = cache
        self._default_temperature = default_temperature
        self._default_max_tokens = default_max_tokens

    def _make_cache_key(self, request: CompletionRequest) -> str:
        """Generate cache key from request."""
        messages_str = "|".join(f"{m.role}:{m.content}" for m in request.messages)
        return f"anthropic:{request.model}:{request.temperature}:{messages_str}"

    def _to_anthropic_messages(
        self, messages: list[Message]
    ) -> list[dict[str, str]]:
        """Convert Message objects to Anthropic format.

        Anthropic uses 'user' and 'assistant' roles, plus 'system' as a separate parameter.
        """
        system_messages: list[Message] = []
        conversation_messages: list[dict[str, str]] = []

        for msg in messages:
            if msg.role == Role.SYSTEM:
                system_messages.append(msg)
            elif msg.role == Role.USER:
                conversation_messages.append({"role": "user", "content": msg.content})
            elif msg.role == Role.ASSISTANT:
                conversation_messages.append({"role": "assistant", "content": msg.content})
            elif msg.role == Role.TOOL:
                # Anthropic doesn't support tool roles directly
                conversation_messages.append({"role": "user", "content": msg.content})
            else:
                # Default to user for unknown roles
                conversation_messages.append({"role": "user", "content": msg.content})

        return conversation_messages

    def _get_system_prompt(self, messages: list[Message]) -> str | None:
        """Extract system prompt from messages."""
        for msg in messages:
            if msg.role == Role.SYSTEM:
                return msg.content
        return None

    def complete(self, request: CompletionRequest) -> Response:
        """Generate a completion using Anthropic API.

        Args:
            request: The completion request

        Returns:
            Response from Anthropic

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
            messages = self._to_anthropic_messages(request.messages)
            system = self._get_system_prompt(request.messages)

            response = self._client.messages.create(
                model=request.model,
                messages=messages,  # type: ignore[arg-type]
                system=system,  # type: ignore[arg-type]
                temperature=request.temperature or self._default_temperature,
                max_tokens=request.max_tokens or self._default_max_tokens,
            )

            content = ""
            if response.content:
                # Get text from content blocks
                for block in response.content:
                    if hasattr(block, "text"):
                        content += block.text

            usage = Usage(
                prompt_tokens=response.usage.input_tokens,
                completion_tokens=response.usage.output_tokens,
                total_tokens=response.usage.input_tokens + response.usage.output_tokens,
            )

            result = Response(
                content=content,
                model=request.model,
                provider=Provider.ANTHROPIC,
                usage=usage,
            )

            # Cache the response
            if self._cache is not None:
                self._cache.set(cache_key, result)

            return result

        except Exception as e:
            msg = f"Anthropic API error: {e}"
            raise LLMError(msg, provider=Provider.ANTHROPIC) from e

    def embed(self, request: EmbeddingRequest) -> list[list[float]]:
        """Generate embeddings using Anthropic API.

        Note: Anthropic doesn't have a native embedding API.
        This method raises NotImplementedError.

        Args:
            request: The embedding request

        Raises:
            NotImplementedError: Anthropic doesn't support embeddings
        """
        msg = "Anthropic doesn't support embeddings API"
        raise NotImplementedError(msg)

    @property
    def provider_name(self) -> str:
        """Return the provider name."""
        return "anthropic"

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
