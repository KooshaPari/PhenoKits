"""Google AI (Gemini) provider implementation.

This adapter implements the LLMProvider port using the Google Gemini API.
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
    import google.generativeai as genai

SUPPORTED_MODELS = [
    "gemini-2.0-flash",
    "gemini-1.5-pro",
    "gemini-1.5-flash",
    "gemini-1.5-flash-8b",
    "gemini-pro",
    "gemini-pro-vision",
]


class GoogleAIProvider(LLMProvider):
    """Google AI (Gemini) provider implementation.

    Implements the LLMProvider port using the Google Gemini API.

    Example:
        ```python
        from portkey.infrastructure.gemini_provider import GoogleAIProvider

        provider = GoogleAIProvider(api_key="your-google-api-key")
        response = provider.complete(
            CompletionRequest(
                messages=[Message(role="user", content="Hello!")],
                model="gemini-2.0-flash"
            )
        )
        print(response.content)
        ```
    """

    def __init__(
        self,
        api_key: str | None = None,
        cache: Cache | None = None,
        default_temperature: float = 0.9,
        default_max_tokens: int = 2048,
    ) -> None:
        """Initialize Google AI provider.

        Args:
            api_key: Google AI API key.
            cache: Optional cache instance for responses.
            default_temperature: Default temperature for completions.
            default_max_tokens: Default max tokens for completions.
        """
        try:
            import google.generativeai as genai  # noqa: PLC0415
        except ImportError as e:
            msg = "google-generativeai package is required. Install with: pip install google-generativeai"
            raise ImportError(msg) from e

        if api_key is None:
            import os  # noqa: PLC0415

            api_key = os.environ.get("GOOGLE_API_KEY", "")

        genai.configure(api_key=api_key)
        self._cache = cache
        self._default_temperature = default_temperature
        self._default_max_tokens = default_max_tokens

    def _make_cache_key(self, request: CompletionRequest) -> str:
        """Generate cache key from request."""
        messages_str = "|".join(f"{m.role}:{m.content}" for m in request.messages)
        return f"gemini:{request.model}:{request.temperature}:{messages_str}"

    def _to_gemini_contents(
        self, messages: list[Message]
    ) -> list[str | object]:
        """Convert Message objects to Gemini format."""
        result: list[str | object] = []
        for msg in messages:
            if msg.role.value == "user":
                result.append(msg.content)
            elif msg.role.value == "model" or msg.role.value == "assistant":
                result.append(msg.content)
        return result

    def _extract_text_from_response(self, response: object) -> str:
        """Extract text from Gemini response."""
        if hasattr(response, "text"):
            return response.text
        if hasattr(response, "parts"):
            parts = response.parts
            if parts:
                return "".join(part.text for part in parts if hasattr(part, "text"))
        return ""

    def complete(self, request: CompletionRequest) -> Response:
        """Generate a completion using Gemini API.

        Args:
            request: The completion request

        Returns:
            Response from Gemini

        Raises:
            LLMError: If the request fails
        """
        if self._cache is not None:
            cache_key = self._make_cache_key(request)
            cached = self._cache.get(cache_key)
            if cached is not None:
                return cached

        try:
            import google.generativeai as genai  # noqa: PLC0415, F401

            model = genai.GenerativeModel(request.model)
            contents = self._to_gemini_contents(request.messages)

            generation_config = {
                "temperature": request.temperature or self._default_temperature,
                "max_output_tokens": request.max_tokens or self._default_max_tokens,
            }

            response = model.generate_content(
                contents,
                generation_config=generation_config,  # type: ignore[arg-type]
            )

            content = self._extract_text_from_response(response)

            usage = Usage(
                prompt_tokens=0,  # Gemini doesn't provide token counts
                completion_tokens=0,
                total_tokens=0,
            )

            result = Response(
                content=content,
                model=request.model,
                provider=Provider.GOOGLE,
                usage=usage,
            )

            if self._cache is not None:
                self._cache.set(cache_key, result)

            return result

        except Exception as e:
            msg = f"Google AI API error: {e}"
            raise LLMError(msg, provider=Provider.GOOGLE) from e

    def embed(self, request: EmbeddingRequest) -> list[list[float]]:
        """Generate embeddings using Gemini API.

        Args:
            request: The embedding request

        Returns:
            List of embedding vectors

        Raises:
            NotImplementedError: Gemini doesn't support embeddings
        """
        msg = "Gemini doesn't support embeddings API"
        raise NotImplementedError(msg)

    @property
    def provider_name(self) -> str:
        """Return the provider name."""
        return "google-ai"

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
