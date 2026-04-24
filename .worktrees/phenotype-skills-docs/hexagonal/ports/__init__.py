"""
Ports Layer - Interface definitions for hexagonal architecture.

Inbound Ports:
- Commands: Write operations (Create, Update, Delete, etc.)
- Queries: Read operations (Get, List, Search)

Outbound Ports:
- Repository: Entity persistence
- Cache: Caching operations
- EventBus: Event publishing/subscribing
- EventStore: Event sourcing storage
- Logger: Logging operations
- HttpClient: HTTP client for external APIs
- FileSystem: File system operations
- Config: Configuration management
"""

from hexagonal.ports.inbound import (
    Command,
    Query,
    CommandResult,
    QueryResult,
    CreateSkillCommand,
    UpdateSkillCommand,
    DeleteSkillCommand,
    DeprecateSkillCommand,
    ArchiveSkillCommand,
    AddSkillTagCommand,
    RemoveSkillTagCommand,
)

from hexagonal.ports.outbound import (
    Repository,
    Cache,
    EventBus,
    EventStore,
    Logger,
    HttpClient,
    FileSystem,
    Config,
)

__all__ = [
    # Inbound
    "Command",
    "Query",
    "CommandResult",
    "QueryResult",
    "CreateSkillCommand",
    "UpdateSkillCommand",
    "DeleteSkillCommand",
    "DeprecateSkillCommand",
    "ArchiveSkillCommand",
    "AddSkillTagCommand",
    "RemoveSkillTagCommand",
    # Outbound
    "Repository",
    "Cache",
    "EventBus",
    "EventStore",
    "Logger",
    "HttpClient",
    "FileSystem",
    "Config",
]
