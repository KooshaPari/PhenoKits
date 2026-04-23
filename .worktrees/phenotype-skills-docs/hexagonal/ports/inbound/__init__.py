"""
Inbound Ports - Driving ports for hexagonal architecture.

Inbound ports define how the outside world interacts with the domain.
These are primary/driving ports that are called by adapters (UI, API, CLI, etc.).

Commands:
- CreateSkill: Command to create a new skill
- UpdateSkill: Command to update an existing skill
- DeleteSkill: Command to delete a skill
- DeprecateSkill: Command to deprecate a skill
- ArchiveSkill: Command to archive a skill
- AddSkillTag: Command to add a tag to a skill
- RemoveSkillTag: Command to remove a tag from a skill

Queries:
- GetSkill: Query to get a single skill by ID
- ListSkills: Query to list skills with filters
- SearchSkills: Query to search skills

Command/Query Results:
- CommandResult: Result of a command execution
- QueryResult: Result of a query execution
"""

from abc import ABC, abstractmethod
from dataclasses import dataclass, field
from datetime import datetime
from enum import Enum
from typing import Any, Dict, Generic, List, Optional, Set, TypeVar

T = TypeVar("T")
C = TypeVar("C")
Q = TypeVar("Q")


class CommandResult:
    """Result of a command execution."""

    def __init__(
        self,
        success: bool,
        message: Optional[str] = None,
        data: Optional[Dict[str, Any]] = None,
        errors: Optional[List[str]] = None,
    ):
        self.success = success
        self.message = message
        self.data = data or {}
        self.errors = errors or []

    @classmethod
    def ok(cls, message: Optional[str] = None, data: Optional[Dict[str, Any]] = None) -> "CommandResult":
        """Create a successful result."""
        return cls(success=True, message=message, data=data)

    @classmethod
    def error(cls, message: str, errors: Optional[List[str]] = None) -> "CommandResult":
        """Create an error result."""
        return cls(success=False, message=message, errors=errors or [message])

    @property
    def is_success(self) -> bool:
        return self.success


class QueryResult(Generic[T]):
    """Result of a query execution."""

    def __init__(
        self,
        success: bool,
        data: Optional[T] = None,
        errors: Optional[List[str]] = None,
        metadata: Optional[Dict[str, Any]] = None,
    ):
        self.success = success
        self.data = data
        self.errors = errors or []
        self.metadata = metadata or {}

    @classmethod
    def ok(cls, data: T, metadata: Optional[Dict[str, Any]] = None) -> "QueryResult[T]":
        """Create a successful query result."""
        return cls(success=True, data=data, metadata=metadata)

    @classmethod
    def error(cls, message: str) -> "QueryResult[T]":
        """Create an error result."""
        return cls(success=False, errors=[message])

    @property
    def is_success(self) -> bool:
        return self.success

    @property
    def is_empty(self) -> bool:
        """Check if the result has no data."""
        return self.data is None


class Command(ABC):
    """
    Base class for all commands.
    
    Commands represent write operations that change state.
    They should be named in imperative mood (CreateX, UpdateX, DeleteX).
    """

    @property
    @abstractmethod
    def command_type(self) -> str:
        """Return the type name of this command."""
        pass

    @abstractmethod
    def validate(self) -> CommandResult:
        """Validate the command data."""
        pass


class Query(ABC, Generic[T]):
    """
    Base class for all queries.
    
    Queries represent read operations that don't change state.
    They should be named in past tense or as questions (GetX, ListX, SearchX).
    """

    @property
    @abstractmethod
    def query_type(self) -> str:
        """Return the type name of this query."""
        pass


@dataclass
class CreateSkillCommand(Command):
    """Command to create a new skill."""

    name: str
    description: str
    category: str
    version: str = "1.0.0"
    tags: Set[str] = field(default_factory=set)
    metadata: Dict[str, Any] = field(default_factory=dict)
    created_by: Optional[str] = None

    @property
    def command_type(self) -> str:
        return "create_skill"

    def validate(self) -> CommandResult:
        errors = []
        if not self.name or not self.name.strip():
            errors.append("Name is required")
        if not self.description or not self.description.strip():
            errors.append("Description is required")
        if not self.category:
            errors.append("Category is required")
        if errors:
            return CommandResult.error("Validation failed", errors)
        return CommandResult.ok()


@dataclass
class UpdateSkillCommand(Command):
    """Command to update an existing skill."""

    skill_id: str
    name: Optional[str] = None
    description: Optional[str] = None
    category: Optional[str] = None
    tags: Optional[Set[str]] = None
    metadata: Optional[Dict[str, Any]] = None
    updated_by: Optional[str] = None

    @property
    def command_type(self) -> str:
        return "update_skill"

    def validate(self) -> CommandResult:
        errors = []
        if not self.skill_id:
            errors.append("Skill ID is required")
        if errors:
            return CommandResult.error("Validation failed", errors)
        return CommandResult.ok()


@dataclass
class DeleteSkillCommand(Command):
    """Command to delete a skill."""

    skill_id: str
    deleted_by: Optional[str] = None

    @property
    def command_type(self) -> str:
        return "delete_skill"

    def validate(self) -> CommandResult:
        if not self.skill_id:
            return CommandResult.error("Skill ID is required")
        return CommandResult.ok()


@dataclass
class DeprecateSkillCommand(Command):
    """Command to deprecate a skill."""

    skill_id: str
    reason: Optional[str] = None
    deprecated_by: Optional[str] = None

    @property
    def command_type(self) -> str:
        return "deprecate_skill"

    def validate(self) -> CommandResult:
        if not self.skill_id:
            return CommandResult.error("Skill ID is required")
        return CommandResult.ok()


@dataclass
class ArchiveSkillCommand(Command):
    """Command to archive a skill."""

    skill_id: str
    archived_by: Optional[str] = None

    @property
    def command_type(self) -> str:
        return "archive_skill"

    def validate(self) -> CommandResult:
        if not self.skill_id:
            return CommandResult.error("Skill ID is required")
        return CommandResult.ok()


@dataclass
class AddSkillTagCommand(Command):
    """Command to add a tag to a skill."""

    skill_id: str
    tag: str
    added_by: Optional[str] = None

    @property
    def command_type(self) -> str:
        return "add_skill_tag"

    def validate(self) -> CommandResult:
        errors = []
        if not self.skill_id:
            errors.append("Skill ID is required")
        if not self.tag or not self.tag.strip():
            errors.append("Tag is required")
        if errors:
            return CommandResult.error("Validation failed", errors)
        return CommandResult.ok()


@dataclass
class RemoveSkillTagCommand(Command):
    """Command to remove a tag from a skill."""

    skill_id: str
    tag: str
    removed_by: Optional[str] = None

    @property
    def command_type(self) -> str:
        return "remove_skill_tag"

    def validate(self) -> CommandResult:
        errors = []
        if not self.skill_id:
            errors.append("Skill ID is required")
        if not self.tag or not self.tag.strip():
            errors.append("Tag is required")
        if errors:
            return CommandResult.error("Validation failed", errors)
        return CommandResult.ok()
