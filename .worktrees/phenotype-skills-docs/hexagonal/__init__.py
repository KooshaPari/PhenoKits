"""
Hexagonal Architecture for phenotype-skills-clone

Implements Clean Architecture with Ports and Adapters pattern.
"""

from hexagonal.domain.entities import Skill, SkillId, SkillCategory
from hexagonal.domain.value_objects import Identifier, Version, Email, Url
from hexagonal.domain.events import SkillCreated, SkillUpdated, SkillDeleted, SkillDeprecated, SkillArchived
from hexagonal.ports.inbound import Command, Query, CommandResult, QueryResult
from hexagonal.ports.outbound import (
    Repository,
    EventBus,
    Cache,
    EventStore,
    Logger,
    HttpClient,
    FileSystem,
)

__all__ = [
    # Entities
    "Skill",
    "SkillId",
    "SkillCategory",
    # Value Objects
    "Identifier",
    "Version",
    "Email",
    "Url",
    # Events
    "SkillCreated",
    "SkillUpdated",
    "SkillDeleted",
    "SkillDeprecated",
    "SkillArchived",
    # Inbound ports
    "Command",
    "Query",
    "CommandResult",
    "QueryResult",
    # Outbound ports
    "Repository",
    "EventBus",
    "Cache",
    "EventStore",
    "Logger",
    "HttpClient",
    "FileSystem",
]
