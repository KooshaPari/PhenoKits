"""
Outbound Ports - Driven ports for hexagonal architecture.

Outbound ports define how the domain interacts with infrastructure.
These are secondary/driven ports that are implemented by adapters (DB, Cache, External APIs, etc.).

Ports:
- Repository: Persistence operations for entities
- Cache: Caching operations
- EventBus: Event publishing/subscribing
- EventStore: Event sourcing storage
- Logger: Logging operations
- HttpClient: HTTP client for external APIs
- FileSystem: File system operations
"""

from abc import ABC, abstractmethod
from typing import Any, Dict, Generic, List, Optional, Set, TypeVar, Protocol, runtime_checkable

from hexagonal.domain.events import DomainEvent

T = TypeVar("T")
K = TypeVar("K")
V = TypeVar("V")


# Repository Protocol
@runtime_checkable
class Repository(Protocol[T, K]):
    """
    Repository port for entity persistence.
    
    Repositories provide CRUD operations for entities.
    They abstract the persistence mechanism from the domain.
    """

    async def save(self, entity: T) -> T:
        """Save an entity and return it."""
        ...

    async def find_by_id(self, id: K) -> Optional[T]:
        """Find an entity by its ID."""
        ...

    async def find_all(self) -> List[T]:
        """Find all entities."""
        ...

    async def delete(self, id: K) -> bool:
        """Delete an entity by its ID."""
        ...

    async def exists(self, id: K) -> bool:
        """Check if an entity exists by its ID."""
        ...


class Cache(ABC, Generic[K, V]):
    """
    Cache port for caching operations.
    
    Caches store data temporarily to improve performance.
    """

    @abstractmethod
    async def get(self, key: K) -> Optional[V]:
        """Get a value by key."""
        pass

    @abstractmethod
    async def set(self, key: K, value: V, ttl: Optional[int] = None) -> None:
        """Set a value with optional TTL in seconds."""
        pass

    @abstractmethod
    async def delete(self, key: K) -> None:
        """Delete a value by key."""
        pass

    @abstractmethod
    async def clear(self) -> None:
        """Clear all cached values."""
        pass

    @abstractmethod
    async def exists(self, key: K) -> bool:
        """Check if a key exists."""
        pass


class EventBus(ABC):
    """
    Event Bus port for event publishing/subscribing.
    
    Event buses allow publishing domain events and subscribing to them.
    """

    @abstractmethod
    async def publish(self, event: DomainEvent) -> None:
        """Publish a domain event."""
        pass

    @abstractmethod
    async def publish_all(self, events: List[DomainEvent]) -> None:
        """Publish multiple domain events."""
        pass

    @abstractmethod
    def subscribe(self, event_type: str, handler: Any) -> None:
        """Subscribe to an event type."""
        pass

    @abstractmethod
    def unsubscribe(self, event_type: str, handler: Any) -> None:
        """Unsubscribe from an event type."""
        pass


class EventStore(ABC):
    """
    Event Store port for event sourcing.
    
    Event stores persist domain events for replay.
    """

    @abstractmethod
    async def append(self, aggregate_id: str, event: DomainEvent) -> None:
        """Append an event to an aggregate's stream."""
        pass

    @abstractmethod
    async def append_all(self, aggregate_id: str, events: List[DomainEvent]) -> None:
        """Append multiple events to an aggregate's stream."""
        pass

    @abstractmethod
    async def get_events(self, aggregate_id: str) -> List[DomainEvent]:
        """Get all events for an aggregate."""
        pass

    @abstractmethod
    async def get_events_since(
        self, aggregate_id: str, timestamp: Any
    ) -> List[DomainEvent]:
        """Get events since a timestamp."""
        pass


class Logger(ABC):
    """
    Logger port for logging operations.
    
    Loggers capture diagnostic information.
    """

    @abstractmethod
    def debug(self, message: str, **kwargs: Any) -> None:
        """Log a debug message."""
        pass

    @abstractmethod
    def info(self, message: str, **kwargs: Any) -> None:
        """Log an info message."""
        pass

    @abstractmethod
    def warning(self, message: str, **kwargs: Any) -> None:
        """Log a warning message."""
        pass

    @abstractmethod
    def error(self, message: str, **kwargs: Any) -> None:
        """Log an error message."""
        pass

    @abstractmethod
    def critical(self, message: str, **kwargs: Any) -> None:
        """Log a critical message."""
        pass


@runtime_checkable
class HttpClient(Protocol):
    """
    HTTP Client port for making HTTP requests.
    
    HTTP clients abstract the HTTP library from the domain.
    """

    async def get(self, url: str, **kwargs: Any) -> Any:
        """Make a GET request."""
        ...

    async def post(self, url: str, **kwargs: Any) -> Any:
        """Make a POST request."""
        ...

    async def put(self, url: str, **kwargs: Any) -> Any:
        """Make a PUT request."""
        ...

    async def patch(self, url: str, **kwargs: Any) -> Any:
        """Make a PATCH request."""
        ...

    async def delete(self, url: str, **kwargs: Any) -> Any:
        """Make a DELETE request."""
        ...


class FileSystem(ABC):
    """
    File System port for file operations.
    
    File system abstraction for reading/writing files.
    """

    @abstractmethod
    async def read(self, path: str) -> bytes:
        """Read a file and return its contents."""
        pass

    @abstractmethod
    async def write(self, path: str, content: bytes) -> None:
        """Write content to a file."""
        pass

    @abstractmethod
    async def delete(self, path: str) -> None:
        """Delete a file."""
        pass

    @abstractmethod
    async def exists(self, path: str) -> bool:
        """Check if a file exists."""
        pass

    @abstractmethod
    async def list(self, directory: str) -> List[str]:
        """List files in a directory."""
        pass


class Config(ABC):
    """
    Configuration port for accessing configuration.
    
    Configuration abstraction for app settings.
    """

    @abstractmethod
    def get(self, key: str, default: Optional[Any] = None) -> Any:
        """Get a configuration value."""
        pass

    @abstractmethod
    def set(self, key: str, value: Any) -> None:
        """Set a configuration value."""
        pass

    @abstractmethod
    def has(self, key: str) -> bool:
        """Check if a configuration key exists."""
        pass

    @abstractmethod
    def all(self) -> Dict[str, Any]:
        """Get all configuration values."""
        pass
