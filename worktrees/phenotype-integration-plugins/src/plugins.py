"""phenotype-integration-plugins - Integration plugin system."""
from __future__ import annotations

import asyncio
from abc import ABC, abstractmethod
from dataclasses import dataclass, field
from enum import Enum
from typing import Any, Generic, TypeVar

__version__ = "0.1.0"

T = TypeVar("T")


class PluginError(Exception):
    """Base plugin error."""
    pass


class ConnectionError(PluginError):
    """Connection failed."""
    pass


class OperationError(PluginError):
    """Operation failed."""
    pass


class PluginState(str, Enum):
    """Plugin lifecycle states."""
    UNINITIALIZED = "uninitialized"
    CONNECTING = "connecting"
    CONNECTED = "connected"
    DISCONNECTED = "disconnected"
    ERROR = "error"


@dataclass
class PluginMetadata:
    """Plugin metadata."""
    name: str
    version: str
    description: str
    author: str = ""
    capabilities: list[str] = field(default_factory=list)


@dataclass 
class Connection:
    """Represents a connection to external service."""
    id: str
    endpoint: str
    state: PluginState = PluginState.UNINITIALIZED


class PluginHost:
    """Host for integration plugins."""

    def __init__(self):
        self._plugins: dict[str, IntegrationPlugin] = {}
        self._connections: dict[str, Connection] = {}

    def register(self, plugin: IntegrationPlugin) -> None:
        """Register a plugin."""
        self._plugins[plugin.metadata.name] = plugin

    async def connect(self, name: str) -> Connection:
        """Connect to a plugin."""
        if name not in self._plugins:
            raise ValueError(f"Unknown plugin: {name}")
        
        plugin = self._plugins[name]
        connection = await plugin.connect()
        self._connections[name] = connection
        return connection

    async def disconnect(self, name: str) -> None:
        """Disconnect a plugin."""
        if name in self._plugins:
            await self._plugins[name].disconnect()
            if name in self._connections:
                del self._connections[name]

    def get_plugin(self, name: str) -> IntegrationPlugin | None:
        """Get a registered plugin."""
        return self._plugins.get(name)

    def list_plugins(self) -> list[PluginMetadata]:
        """List all plugins."""
        return [p.metadata for p in self._plugins.values()]


class IntegrationPlugin(ABC):
    """Base integration plugin."""

    def __init__(self):
        self.state = PluginState.UNINITIALIZED
        self._connection: Connection | None = None

    @property
    @abstractmethod
    def metadata(self) -> PluginMetadata:
        """Return plugin metadata."""
        ...

    @abstractmethod
    async def connect(self) -> Connection:
        """Connect to external service."""
        ...

    @abstractmethod
    async def disconnect(self) -> None:
        """Disconnect from external service."""
        ...

    @abstractmethod
    async def execute(self, operation: str, params: dict[str, Any]) -> Any:
        """Execute an operation."""
        ...

    async def health_check(self) -> bool:
        """Check plugin health."""
        return self.state == PluginState.CONNECTED


class TicketingPlugin(IntegrationPlugin):
    """Plugin for ticketing/project management integrations."""

    @abstractmethod
    async def create_ticket(self, title: str, description: str, **kwargs) -> dict[str, Any]:
        """Create a ticket."""
        ...

    @abstractmethod
    async def get_ticket(self, ticket_id: str) -> dict[str, Any]:
        """Get ticket by ID."""
        ...

    @abstractmethod
    async def list_tickets(self, status: str | None = None) -> list[dict[str, Any]]:
        """List tickets."""
        ...


class MonitoringPlugin(IntegrationPlugin):
    """Plugin for monitoring/observability integrations."""

    @abstractmethod
    async def emit_metric(self, name: str, value: float, labels: dict[str, str] | None = None) -> None:
        """Emit a metric."""
        ...

    @abstractmethod
    async def emit_event(self, name: str, message: str, severity: str = "info") -> None:
        """Emit an event."""
        ...


class CICDPlugin(IntegrationPlugin):
    """Plugin for CI/CD integrations."""

    @abstractmethod
    async def trigger_build(self, branch: str, commit: str | None = None) -> dict[str, Any]:
        """Trigger a build."""
        ...

    @abstractmethod
    async def get_build_status(self, build_id: str) -> dict[str, Any]:
        """Get build status."""
        ...


class GenericAdapter(IntegrationPlugin):
    """Generic REST API adapter."""

    def __init__(self, base_url: str, api_key: str | None = None):
        super().__init__()
        self.base_url = base_url
        self.api_key = api_key
        self._session: asyncio.ClientSession | None = None

    @property
    def metadata(self) -> PluginMetadata:
        return PluginMetadata(
            name="generic-rest",
            version="1.0.0",
            description="Generic REST API adapter",
        )

    async def connect(self) -> Connection:
        """Connect to REST API."""
        self.state = PluginState.CONNECTING
        self._connection = Connection(
            id=f"generic-{self.base_url}",
            endpoint=self.base_url,
            state=PluginState.CONNECTED,
        )
        self.state = PluginState.CONNECTED
        return self._connection

    async def disconnect(self) -> None:
        """Disconnect."""
        if self._session:
            await self._session.close()
        self.state = PluginState.DISCONNECTED

    async def execute(self, operation: str, params: dict[str, Any]) -> Any:
        """Execute REST operation."""
        raise NotImplementedError("Subclass must implement")


__all__ = [
    "PluginError",
    "ConnectionError", 
    "OperationError",
    "PluginState",
    "PluginMetadata",
    "Connection",
    "PluginHost",
    "IntegrationPlugin",
    "TicketingPlugin",
    "MonitoringPlugin",
    "CI/CDPlugin",
    "CI/CDPlugin",
    "GenericAdapter",
]
