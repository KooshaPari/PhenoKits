"""
Ports (interfaces) for middleware adapters.

Following hexagonal architecture:
- Ports define the interface contract
- Adapters implement the ports
- Domain depends only on ports

xDD Principles:
- Interface segregation: focused port interfaces
- Dependency inversion: domain depends on abstraction
- Contract testing: adapters verified against ports
"""

from abc import ABC, abstractmethod
from typing import Optional
from collections.abc import Callable, Awaitable
from phenotype_middleware.domain import Request, Response, MiddlewareResult


# Type alias for middleware functions
MiddlewareFunc = Callable[[Request], Awaitable[MiddlewareResult]]


class MiddlewarePort(ABC):
    """
    Port interface for middleware implementations.

    Following Interface Segregation Principle (SOLID):
    - Single abstract method for processing
    - Can be implemented by functions or classes
    """

    @abstractmethod
    async def process(self, request: Request) -> MiddlewareResult:
        """
        Process a request through this middleware.

        Args:
            request: The incoming request

        Returns:
            MiddlewareResult with success/failure and optional modified request/response
        """
        ...


class LoggingPort(ABC):
    """
    Port interface for logging adapters.

    Allows different logging implementations (stdout, file, external service).
    """

    @abstractmethod
    async def log(self, level: str, message: str, context: dict | None = None) -> None:
        """
        Log a message at the specified level.

        Args:
            level: Log level (DEBUG, INFO, WARNING, ERROR)
            message: Log message
            context: Additional context for the log entry
        """
        ...


class MetricsPort(ABC):
    """
    Port interface for metrics collection.

    Allows different metrics backends (Prometheus, StatsD, etc.).
    """

    @abstractmethod
    async def record(self, name: str, value: float, labels: dict | None = None) -> None:
        """
        Record a metric value.

        Args:
            name: Metric name
            value: Metric value
            labels: Optional labels/tags
        """
        ...


class AuthPort(ABC):
    """
    Port interface for authentication.

    Allows different auth mechanisms (JWT, OAuth, API key).
    """

    @abstractmethod
    async def authenticate(self, request: Request) -> MiddlewareResult:
        """
        Authenticate a request.

        Args:
            request: The request to authenticate

        Returns:
            MiddlewareResult with auth status and user info in metadata
        """
        ...
