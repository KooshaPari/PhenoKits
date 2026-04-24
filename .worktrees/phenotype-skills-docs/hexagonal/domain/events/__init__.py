"""
Domain Events - Events representing state changes.

Domain Events:
- DomainEvent: Base class for all domain events
- SkillCreated: Event fired when a skill is created
- SkillUpdated: Event fired when a skill is updated
- SkillDeleted: Event fired when a skill is deleted
- SkillDeprecated: Event fired when a skill is deprecated
- SkillArchived: Event fired when a skill is archived
"""

from abc import ABC, abstractmethod
from dataclasses import dataclass, field
from datetime import datetime
from typing import Any, Dict, Optional
from uuid import uuid4


@dataclass
class DomainEventData:
    """Data container for domain event common fields."""

    event_id: str = field(default_factory=lambda: str(uuid4()))
    occurred_on: datetime = field(default_factory=datetime.utcnow)


class DomainEvent(ABC):
    """
    Base class for all domain events.
    
    Domain events are:
    - Immutable
    - Named in past tense
    - Contain relevant data about the event
    - Have a unique event ID
    - Have a timestamp
    """

    event_id: str
    occurred_on: datetime

    def __init__(self) -> None:
        self.event_id = str(uuid4())
        self.occurred_on = datetime.utcnow()

    @property
    @abstractmethod
    def event_type(self) -> str:
        """Return the type name of this event (e.g., 'skill_created')."""
        pass

    @abstractmethod
    def metadata(self) -> Dict[str, Any]:
        """Return event metadata."""
        pass

    def __repr__(self) -> str:
        return f"{self.__class__.__name__}(id={self.event_id}, occurred_on={self.occurred_on.isoformat()})"


@dataclass
class SkillCreated:
    """Event fired when a skill is created."""

    skill_id: str
    skill_name: str
    skill_category: str
    created_by: Optional[str] = None
    event_id: str = field(default_factory=lambda: str(uuid4()))
    occurred_on: datetime = field(default_factory=datetime.utcnow)

    @property
    def event_type(self) -> str:
        return "skill_created"

    def metadata(self) -> Dict[str, Any]:
        return {
            "skill_id": self.skill_id,
            "skill_name": self.skill_name,
            "skill_category": self.skill_category,
            "created_by": self.created_by,
            "event_type": self.event_type,
            "occurred_on": self.occurred_on.isoformat(),
        }


@dataclass
class SkillUpdated:
    """Event fired when a skill is updated."""

    skill_id: str
    skill_name: str
    changes: Dict[str, Any]
    updated_by: Optional[str] = None
    event_id: str = field(default_factory=lambda: str(uuid4()))
    occurred_on: datetime = field(default_factory=datetime.utcnow)

    @property
    def event_type(self) -> str:
        return "skill_updated"

    def metadata(self) -> Dict[str, Any]:
        return {
            "skill_id": self.skill_id,
            "skill_name": self.skill_name,
            "changes": self.changes,
            "updated_by": self.updated_by,
            "event_type": self.event_type,
            "occurred_on": self.occurred_on.isoformat(),
        }


@dataclass
class SkillDeleted:
    """Event fired when a skill is deleted."""

    skill_id: str
    skill_name: str
    deleted_by: Optional[str] = None
    event_id: str = field(default_factory=lambda: str(uuid4()))
    occurred_on: datetime = field(default_factory=datetime.utcnow)

    @property
    def event_type(self) -> str:
        return "skill_deleted"

    def metadata(self) -> Dict[str, Any]:
        return {
            "skill_id": self.skill_id,
            "skill_name": self.skill_name,
            "deleted_by": self.deleted_by,
            "event_type": self.event_type,
            "occurred_on": self.occurred_on.isoformat(),
        }


@dataclass
class SkillDeprecated:
    """Event fired when a skill is deprecated."""

    skill_id: str
    skill_name: str
    reason: Optional[str] = None
    deprecated_by: Optional[str] = None
    event_id: str = field(default_factory=lambda: str(uuid4()))
    occurred_on: datetime = field(default_factory=datetime.utcnow)

    @property
    def event_type(self) -> str:
        return "skill_deprecated"

    def metadata(self) -> Dict[str, Any]:
        return {
            "skill_id": self.skill_id,
            "skill_name": self.skill_name,
            "reason": self.reason,
            "deprecated_by": self.deprecated_by,
            "event_type": self.event_type,
            "occurred_on": self.occurred_on.isoformat(),
        }


@dataclass
class SkillArchived:
    """Event fired when a skill is archived."""

    skill_id: str
    skill_name: str
    archived_by: Optional[str] = None
    event_id: str = field(default_factory=lambda: str(uuid4()))
    occurred_on: datetime = field(default_factory=datetime.utcnow)

    @property
    def event_type(self) -> str:
        return "skill_archived"

    def metadata(self) -> Dict[str, Any]:
        return {
            "skill_id": self.skill_id,
            "skill_name": self.skill_name,
            "archived_by": self.archived_by,
            "event_type": self.event_type,
            "occurred_on": self.occurred_on.isoformat(),
        }
