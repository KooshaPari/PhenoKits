"""Skill and event types."""

from __future__ import annotations
from dataclasses import dataclass, field
from datetime import datetime
from typing import Optional

from .types import SkillId, Version, SkillManifest


@dataclass
class Skill:
    """A registered skill with its metadata."""
    id: SkillId
    name: str
    version: Version
    manifest: SkillManifest
    path: Optional[str] = None
    created_at: datetime = field(default_factory=datetime.utcnow)
    updated_at: datetime = field(default_factory=datetime.utcnow)

    @classmethod
    def from_manifest(cls, manifest: SkillManifest, path: Optional[str] = None) -> "Skill":
        """Create a skill from a manifest."""
        return cls(
            id=SkillId.new(manifest.name),
            name=manifest.name,
            version=manifest.version,
            manifest=manifest,
            path=path
        )

    def is_compatible_with(self, other: Skill) -> bool:
        """Check if this skill is compatible with another."""
        return self.manifest.is_compatible_with(other.manifest)

    def requires_permission(self, permission: str) -> bool:
        """Check if the skill requires a specific permission."""
        return any(p.name == permission for p in self.manifest.permissions)

    def update_manifest(self, manifest: SkillManifest) -> None:
        """Update the manifest."""
        self.manifest = manifest
        self.updated_at = datetime.utcnow()

    def __str__(self) -> str:
        return f"{self.name}@{self.version}"


@dataclass
class SkillEvent:
    """Base class for skill events."""
    timestamp: datetime = field(default_factory=datetime.utcnow)


@dataclass
class SkillRegistered(SkillEvent):
    """Event emitted when a skill is registered."""
    skill_id: SkillId = None  # type: ignore
    name: str = ""
    version: str = ""


@dataclass
class SkillUnregistered(SkillEvent):
    """Event emitted when a skill is unregistered."""
    skill_id: SkillId = None  # type: ignore


@dataclass
class SkillUpdated(SkillEvent):
    """Event emitted when a skill is updated."""
    skill_id: SkillId = None  # type: ignore
    old_version: str = ""
    new_version: str = ""


@dataclass
class SkillExecuted(SkillEvent):
    """Event emitted when a skill is executed."""
    skill_id: SkillId = None  # type: ignore
    success: bool = False
    duration_ms: int = 0


@dataclass
class DependencyResolved(SkillEvent):
    """Event emitted when a dependency is resolved."""
    skill_id: SkillId = None  # type: ignore
    dependency_name: str = ""
    resolved_version: str = ""
