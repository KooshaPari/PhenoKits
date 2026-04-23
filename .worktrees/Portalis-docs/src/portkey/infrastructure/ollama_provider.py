"""Ollama provider implementation.

This adapter implements the LLMProvider port using the Ollama API.
Ollama provides an OpenAI-compatible API for running LLMs locally.
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
    pass  # httpx imported at runtime

SUPPORTED_MODELS = [
    "llama3",
    "llama3.1",
    "llama3.2",
    "mistral",
    "mixtral",
    "codellama",
    "phi3",
    "qwen2",
    "gemma2",
    "nomic-embed-text",
]


class OllamaProvider(LLMProvider):
    """Ollama provider implementation.

    Implements the LLMProvider port using the Ollama API.
    Ollama runs LLMs locally and provides an OpenAI-compatible API.

    Example:
        ```python
        from portkey.infrastructure.ollama_provider import OllamaProvider

        provider = OllamaProvider(base_url="http://localhost:11434")
        response = provider.complete(
            CompletionRequest(
                messages=[Message(role="user", content="Hello!")],
                model="llama3"
            )
        )
        print(response.content)
        ```
    """

    def __init__(
        self,
        base_url: str = "http://localhost:11434",
        cache: Cache | None = None,
        default_temperature: float = 0.7,
        default_num_predict: int = 256,
    ) -> None:
        """Initialize Ollama provider.

        Args:
            base_url: Ollama server URL. Defaults to localhost.
            cache: Optional cache instance for responses.
            default_temperature: Default temperature for completions.
            default_num_predict: Default max tokens for completions.
        """
        try:
            import httpx  # noqa: F401
        except ImportError as e:
            msg = "httpx package is required. Install with: pip install httpx"
            raise ImportError(msg) from e

        self._base_url = base_url.rstrip("/")
        self._cache = cache
        self._default_temperature = default_temperature
        self._default_num_predict = default_num_predict

    def _make_cache_key(self, request: CompletionRequest) -> str:
        """Generate cache key from request."""
        messages_str = "|".join(f"{m.role}:{m.content}" for m in request.messages)
        return f"ollama:{request.model}:{request.temperature}:{messages_str}"

    def _to_ollama_messages(
        self, messages: list[Message]
    ) -> list[dict[str, str]]:
        """Convert Message objects to Ollama format."""
        result: list[dict[str, str]] = []
        for msg in messages:
            # Role is already a str (Role(str, Enum)), so just use it directly
            item: dict[str, str] = {"role": msg.role, "content": msg.content}
            result.append(item)
        return result

    def _complete_sync(self, request: CompletionRequest) -> Response:
        """Synchronous completion using httpx."""
        import httpx

        messages = self._to_ollama_messages(request.messages)

        payload = {
            "model": request.model,
            "messages": messages,
            "stream": False,
        }

        if request.temperature is not None:
            payload["temperature"] = request.temperature
        else:
            payload["temperature"] = self._default_temperature

        if request.max_tokens is not None:
            payload["options"] = {"num_predict": request.max_tokens}
        else:
            payload["options"] = {"num_predict": self._default_num_predict}

        try:
            with httpx.Client(timeout=120.0) as client:
                response = client.post(
                    f"{self._base_url}/api/chat",
                    json=payload,
                )
                response.raise_for_status()
                data = response.json()

            content = ""
            if data.get("message"):
                content = data["message"].get("content", "")

            # Ollama may not always return usage info
            usage = None
            if data.get("usage"):
                usage = Usage(
                    prompt_tokens=data["usage"].get("prompt_eval_count", 0),
                    completion_tokens=data["usage"].get("eval_count", 0),
                    total_tokens=(
                        data["usage"].get("prompt_eval_count", 0)
                        + data["usage"].get("eval_count", 0)
                    ),
                )

            return Response(
                content=content,
                model=request.model,
                provider=Provider.OLLAMA,
                usage=usage,
            )

        except httpx.HTTPError as e:
            msg = f"Ollama HTTP error: {e}"
            raise LLMError(msg, provider=Provider.OLLAMA) from e
        except Exception as e:
            msg = f"Ollama error: {e}"
            raise LLMError(msg, provider=Provider.OLLAMA) from e

    def complete(self, request: CompletionRequest) -> Response:
        """Generate a completion using Ollama API.

        Args:
            request: The completion request

        Returns:
            Response from Ollama

        Raises:
            LLMError: If the request fails
        """
        # Check cache first
        if self._cache is not None:
            cache_key = self._make_cache_key(request)
            cached = self._cache.get(cache_key)
            if cached is not None:
                return cached

        result = self._complete_sync(request)

        # Cache the response
        if self._cache is not None:
            self._cache.set(cache_key, result)

        return result

    def embed(self, request: EmbeddingRequest) -> list[list[float]]:
        """Generate embeddings using Ollama API.

        Args:
            request: The embedding request

        Returns:
            List of embedding vectors

        Raises:
            LLMError: If the request fails
        """
        try:
            import httpx

            embeddings: list[list[float]] = []

            with httpx.Client(timeout=60.0) as client:
                for text in request.texts:
                    response = client.post(
                        f"{self._base_url}/api/embeddings",
                        json={"model": request.model, "prompt": text},
                    )
                    response.raise_for_status()
                    data = response.json()
                    embeddings.append(data.get("embedding", []))

            return embeddings

        except httpx.HTTPError as e:
            msg = f"Ollama embedding HTTP error: {e}"
            raise LLMError(msg, provider=Provider.OLLAMA) from e
        except Exception as e:
            msg = f"Ollama embedding error: {e}"
            raise LLMError(msg, provider=Provider.OLLAMA) from e

    @property
    def provider_name(self) -> str:
        """Return the provider name."""
        return "ollama"

    @property
    def supported_models(self) -> list[str]:
        """Return list of supported models."""
        return SUPPORTED_MODELS.copy()

    def list_models(self) -> list[str]:
        """List models available on the Ollama server.

        Returns:
            List of model names available on the server
        """
        try:
            import httpx

            with httpx.Client(timeout=10.0) as client:
                response = client.get(f"{self._base_url}/api/tags")
                response.raise_for_status()
                data = response.json()
                return [m["name"] for m in data.get("models", [])]
        except Exception:
            return self.supported_models

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
