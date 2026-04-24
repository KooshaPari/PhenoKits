"""
Domain Entities - Core business objects with identity.

Entities:
- Skill: Represents a skill in the system
"""

from dataclasses import dataclass, field
from datetime import datetime
from enum import Enum
from typing import Any, Dict, List, Optional, Set
from uuid import uuid4


class SkillCategory(Enum):
    """Categories for skills."""

    DEVELOPMENT = "development"
    DEVOPS = "devops"
    DATA = "data"
    SECURITY = "security"
    DESIGN = "design"
    MANAGEMENT = "management"
    DOCUMENTATION = "documentation"
    TESTING = "testing"
    OTHER = "other"


@dataclass(frozen=True)
class SkillId:
    """Unique identifier for a skill."""

    value: str

    def __post_init__(self) -> None:
        if not self.value:
            raise ValueError("SkillId cannot be empty")

    def __str__(self) -> str:
        return self.value

    @classmethod
    def create(cls) -> "SkillId":
        """Generate a new unique skill ID."""
        return cls(value=str(uuid4()))

    @classmethod
    def from_string(cls, value: str) -> "SkillId":
        """Create a SkillId from a string value."""
        return cls(value=value)

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, SkillId):
            return False
        return self.value == other.value

    def __hash__(self) -> int:
        return hash(self.value)


@dataclass
class Skill:
    """
    Skill entity representing a reusable skill in the system.

    A skill has:
    - Unique identity (id)
    - Name and description
    - Category for organization
    - Tags for searchability
    - Version for tracking changes
    - Status (active, deprecated, archived)
    - Metadata for extensibility
    """

    name: str
    description: str
    category: SkillCategory
    id: SkillId = field(default_factory=SkillId.create)
    version: str = "1.0.0"
    tags: Set[str] = field(default_factory=set)
    status: str = "active"
    created_at: datetime = field(default_factory=datetime.utcnow)
    updated_at: datetime = field(default_factory=datetime.utcnow)
    metadata: Dict[str, Any] = field(default_factory=dict)

    def __post_init__(self) -> None:
        if not self.name or not self.name.strip():
            raise ValueError("Skill name cannot be empty")
        if not self.description or not self.description.strip():
            raise ValueError("Skill description cannot be empty")

    @property
    def entity_id(self) -> Any:
        """Return the unique identifier of this entity."""
        return self.id

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, Skill):
            return False
        return self.id == other.id

    def __hash__(self) -> int:
        return hash(self.id)

    def __repr__(self) -> str:
        return f"Skill(id={self.id!r}, name={self.name!r})"

    @classmethod
    def create(
        cls,
        name: str,
        description: str,
        category: SkillCategory,
        version: str = "1.0.0",
        tags: Optional[Set[str]] = None,
        metadata: Optional[Dict[str, Any]] = None,
    ) -> "Skill":
        """Factory method to create a new skill."""
        now = datetime.utcnow()
        skill = cls(
            id=SkillId.create(),
            name=name.strip(),
            description=description.strip(),
            category=category,
            version=version,
            tags=tags or set(),
            status="active",
            created_at=now,
            updated_at=now,
            metadata=metadata or {},
        )
        return skill

    def update(
        self,
        name: Optional[str] = None,
        description: Optional[str] = None,
        category: Optional[SkillCategory] = None,
        tags: Optional[Set[str]] = None,
        metadata: Optional[Dict[str, Any]] = None,
    ) -> "Skill":
        """Update the skill's attributes."""
        self.name = name.strip() if name else self.name
        self.description = description.strip() if description else self.description
        self.category = category if category else self.category
        self.tags = tags if tags is not None else self.tags
        self.metadata = metadata if metadata is not None else self.metadata
        self.updated_at = datetime.utcnow()
        return self

    def deprecate(self) -> "Skill":
        """Mark the skill as deprecated."""
        self.status = "deprecated"
        self.updated_at = datetime.utcnow()
        return self

    def archive(self) -> "Skill":
        """Archive the skill."""
        self.status = "archived"
        self.updated_at = datetime.utcnow()
        return self

    def reactivate(self) -> "Skill":
        """Reactivate a deprecated or archived skill."""
        if self.status in ("deprecated", "archived"):
            self.status = "active"
            self.updated_at = datetime.utcnow()
        return self

    def add_tag(self, tag: str) -> "Skill":
        """Add a tag to the skill."""
        self.tags.add(tag.lower().strip())
        self.updated_at = datetime.utcnow()
        return self

    def remove_tag(self, tag: str) -> "Skill":
        """Remove a tag from the skill."""
        self.tags.discard(tag.lower().strip())
        self.updated_at = datetime.utcnow()
        return self

    def has_tag(self, tag: str) -> bool:
        """Check if the skill has a specific tag."""
        return tag.lower().strip() in self.tags

    def to_dict(self) -> Dict[str, Any]:
        """Convert to dictionary representation."""
        return {
            "id": str(self.id),
            "name": self.name,
            "description": self.description,
            "category": self.category.value,
            "version": self.version,
            "tags": list(self.tags),
            "status": self.status,
            "created_at": self.created_at.isoformat(),
            "updated_at": self.updated_at.isoformat(),
            "metadata": self.metadata,
        }
