"""Azure OpenAI provider implementation.

This adapter implements the LLMProvider port using the Azure OpenAI API.
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
    from openai import AzureOpenAI

SUPPORTED_MODELS = [
    "gpt-4o",
    "gpt-4-turbo",
    "gpt-4",
    "gpt-4-32k",
    "gpt-35-turbo",
    "gpt-35-turbo-16k",
    "gpt-4o-mini",
]


class AzureOpenAIProvider(LLMProvider):
    """Azure OpenAI provider implementation.

    Implements the LLMProvider port using the Azure OpenAI API.

    Example:
        ```python
        from portkey.infrastructure.azure_provider import AzureOpenAIProvider

        provider = AzureOpenAIProvider(
            api_key="your-azure-key",
            azure_endpoint="https://your-resource.openai.azure.com",
            api_version="2024-02-01",
        )
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
        azure_endpoint: str | None = None,
        api_version: str = "2024-02-01",
        cache: Cache | None = None,
        default_temperature: float = 0.7,
        default_max_tokens: int = 1024,
        azure_deployment: str | None = None,
    ) -> None:
        """Initialize Azure OpenAI provider.

        Args:
            api_key: Azure OpenAI API key.
            azure_endpoint: Azure OpenAI endpoint URL.
            api_version: Azure API version.
            cache: Optional cache instance for responses.
            default_temperature: Default temperature for completions.
            default_max_tokens: Default max tokens for completions.
            azure_deployment: Optional Azure deployment name.
        """
        try:
            from openai import AzureOpenAI  # noqa: PLC0415
        except ImportError as e:
            msg = "openai package is required. Install with: pip install portkey[openai]"
            raise ImportError(msg) from e

        if api_key is None:
            import os  # noqa: PLC0415

            api_key = os.environ.get("AZURE_OPENAI_API_KEY", "")

        if azure_endpoint is None:
            import os  # noqa: PLC0415

            azure_endpoint = os.environ.get("AZURE_OPENAI_ENDPOINT", "")

        self._client: AzureOpenAI = AzureOpenAI(
            api_key=api_key,
            azure_endpoint=azure_endpoint,
            api_version=api_version,
        )
        self._cache = cache
        self._default_temperature = default_temperature
        self._default_max_tokens = default_max_tokens
        self._azure_deployment = azure_deployment

    def _make_cache_key(self, request: CompletionRequest) -> str:
        """Generate cache key from request."""
        messages_str = "|".join(f"{m.role}:{m.content}" for m in request.messages)
        return f"azure:{request.model}:{request.temperature}:{messages_str}"

    def _to_openai_messages(
        self, messages: list[Message]
    ) -> list[dict[str, str]]:
        """Convert Message objects to OpenAI format."""
        result: list[dict[str, str]] = []
        for msg in messages:
            item: dict[str, str] = {"role": msg.role, "content": msg.content}
            if msg.name:
                item["name"] = msg.name
            result.append(item)
        return result

    def _from_openai_response(
        self, completion: object, model: str
    ) -> Response:
        """Convert OpenAI response to Response model."""
        if hasattr(completion, "choices"):
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
                provider=Provider.AZURE,
                usage=usage,
            )

        msg = "Invalid response format"
        raise ValueError(msg)

    def complete(self, request: CompletionRequest) -> Response:
        """Generate a completion using Azure OpenAI API.

        Args:
            request: The completion request

        Returns:
            Response from Azure OpenAI

        Raises:
            LLMError: If the request fails
        """
        if self._cache is not None:
            cache_key = self._make_cache_key(request)
            cached = self._cache.get(cache_key)
            if cached is not None:
                return cached

        try:
            messages = self._to_openai_messages(request.messages)
            deployment = self._azure_deployment or request.model

            completion = self._client.chat.completions.create(
                model=deployment,
                messages=messages,  # type: ignore[arg-type]
                temperature=request.temperature or self._default_temperature,
                max_tokens=request.max_tokens or self._default_max_tokens,
            )

            response = self._from_openai_response(completion, request.model)

            if self._cache is not None:
                self._cache.set(cache_key, response)

            return response

        except Exception as e:
            msg = f"Azure OpenAI API error: {e}"
            raise LLMError(msg, provider=Provider.AZURE) from e

    def embed(self, request: EmbeddingRequest) -> list[list[float]]:
        """Generate embeddings using Azure OpenAI API.

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
            msg = f"Azure OpenAI embedding error: {e}"
            raise LLMError(msg, provider=Provider.AZURE) from e

    @property
    def provider_name(self) -> str:
        """Return the provider name."""
        return "azure-openai"

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
