"""
Domain models for middleware.

Following hexagonal architecture, domain models are pure business logic
with no external dependencies.

xDD Principles:
- KISS: Simple dataclasses for requests/responses
- DRY: Shared model definitions
- PoLA: Descriptive error messages
"""

from dataclasses import dataclass, field
from enum import Enum
from typing import Any


class ErrorCategory(Enum):
    """Error categories following xDD error handling standards."""
    MIDDLEWARE = "middleware"
    PIPELINE = "pipeline"
    ADAPTER = "adapter"


@dataclass(frozen=True)
class Request:
    """
    Immutable request object passed through middleware chain.

    Following PoLA (Principle of Least Astonishment):
    - Immutable: prevents accidental mutation
    - Frozen: hashable for caching
    - Type hints: clear contract
    """
    path: str
    method: str
    headers: dict[str, str] = field(default_factory=dict)
    body: bytes | None = None
    context: dict[str, Any] = field(default_factory=dict)

    def with_context(self, key: str, value: Any) -> "Request":
        """Return new request with added context (immutable)."""
        return Request(
            path=self.path,
            method=self.method,
            headers=self.headers,
            body=self.body,
            context={**self.context, key: value},
        )


@dataclass
class Response:
    """
    Response object returned from middleware chain.

    Mutable for building up response incrementally.
    """
    status_code: int = 200
    headers: dict[str, str] = field(default_factory=dict)
    body: bytes | None = None
    error: str | None = None

    def set_header(self, key: str, value: str) -> None:
        """Add or update a header."""
        self.headers[key] = value

    def set_body(self, body: bytes) -> None:
        """Set response body."""
        self.body = body
        self.set_header("Content-Length", str(len(body)))


@dataclass
class MiddlewareResult:
    """
    Result from a single middleware operation.

    Follows the Result pattern for explicit error handling.
    """
    success: bool
    request: Request | None = None
    response: Response | None = None
    error: str | None = None
    metadata: dict[str, Any] = field(default_factory=dict)

    @classmethod
    def ok(cls, request: Request | None = None, response: Response | None = None) -> "MiddlewareResult":
        """Create a successful result."""
        return cls(success=True, request=request, response=response)

    @classmethod
    def err(cls, error: str, metadata: dict[str, Any] | None = None) -> "MiddlewareResult":
        """Create an error result."""
        return cls(success=False, error=error, metadata=metadata or {})


class MiddlewareError(Exception):
    """
    Base exception for middleware operations.

    Following PoLA:
    - Descriptive messages
    - Context attachment
    - Categorized errors
    """
    def __init__(self, message: str, category: ErrorCategory = ErrorCategory.MIDDLEWARE, context: dict | None = None):
        super().__init__(message)
        self.message = message
        self.category = category
        self.context = context or {}

    def with_context(self, key: str, value: Any) -> "MiddlewareError":
        """Return new exception with added context."""
        return MiddlewareError(
            message=self.message,
            category=self.category,
            context={**self.context, key: value},
        )


class PipelineError(MiddlewareError):
    """Raised when pipeline execution fails."""
    def __init__(self, message: str, context: dict | None = None):
        super().__init__(message, ErrorCategory.PIPELINE, context)


class AdapterError(MiddlewareError):
    """Raised when an adapter operation fails."""
    def __init__(self, message: str, context: dict | None = None):
        super().__init__(message, ErrorCategory.ADAPTER, context)
