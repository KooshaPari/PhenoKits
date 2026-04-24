"""Domain errors for portkey.

Following PoLA (Principle of Least Astonishment), errors are descriptive
and indicate the error category.
"""


class PortkeyError(Exception):
    """Base exception for all portkey errors.

    Attributes:
        message: Human-readable error description
        provider: Optional provider identifier for context
    """

    def __init__(
        self,
        message: str,
        provider: str | None = None,
    ) -> None:
        self.message = message
        self.provider = provider
        super().__init__(self._format_message())

    def _format_message(self) -> str:
        parts = [self.message]
        if self.provider:
            parts.append(f"provider={self.provider}")
        return " | ".join(parts)


class LLMError(PortkeyError):
    """Base exception for LLM-related errors."""


class ModelNotSupportedError(LLMError):
    """Raised when a model is not supported by the provider."""


class RateLimitError(LLMError):
    """Raised when rate limit is exceeded."""


class AuthenticationError(LLMError):
    """Raised when authentication fails."""


class ContextLengthError(LLMError):
    """Raised when context length exceeds model limit."""


class InvalidRequestError(LLMError):
    """Raised when request parameters are invalid."""


class ProviderError(LLMError):
    """Raised when provider returns an unexpected error."""


class CacheError(PortkeyError):
    """Base exception for cache-related errors."""


class CacheKeyNotFoundError(CacheError):
    """Raised when a cache key is not found."""


class CacheSerializationError(CacheError):
    """Raised when cache serialization fails."""
