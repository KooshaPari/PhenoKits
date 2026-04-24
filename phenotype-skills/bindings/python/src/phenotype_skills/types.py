"""Core types for the skills system."""

from __future__ import annotations
from dataclasses import dataclass, field
from datetime import datetime
from enum import Enum, auto
from typing import Optional, Dict, Any, List


class Runtime(Enum):
    """Runtime environment for skill execution."""
    WASM = "wasm"
    PYTHON = "python"
    JAVASCRIPT = "javascript"
    RUST = "rust"
    CSHARP = "csharp"
    GO = "go"
    SHELL = "shell"
    BINARY = "binary"
    CUSTOM = "custom"


@dataclass(frozen=True)
class SkillId:
    """Unique identifier for a skill."""
    value: str

    @classmethod
    def new(cls, value: str) -> SkillId:
        return cls(value)

    def __str__(self) -> str:
        return self.value


@dataclass(frozen=True, order=True)
class Version:
    """Semantic version."""
    major: int
    minor: int
    patch: int
    prerelease: Optional[str] = None

    @classmethod
    def parse(cls, version: str) -> Version:
        parts = version.split(".")
        if len(parts) != 3:
            raise ValueError(f"Invalid version: {version}")
        return cls(
            major=int(parts[0]),
            minor=int(parts[1]),
            patch=int(parts[2])
        )

    @classmethod
    def new(cls, major: int, minor: int, patch: int) -> Version:
        return cls(major, minor, patch)

    def satisfies(self, constraint: str) -> bool:
        """Check if version satisfies a constraint (e.g., ^1.0.0, >=1.2.0)."""
        if constraint.startswith("^"):
            base = Version.parse(constraint[1:])
            return self.major == base.major and self >= base
        elif constraint.startswith("~"):
            base = Version.parse(constraint[1:])
            return (self.major == base.major and 
                    self.minor == base.minor and 
                    self >= base)
        elif constraint.startswith(">="):
            base = Version.parse(constraint[2:])
            return self >= base
        else:
            base = Version.parse(constraint)
            return self == base

    def __str__(self) -> str:
        if self.prerelease:
            return f"{self.major}.{self.minor}.{self.patch}-{self.prerelease}"
        return f"{self.major}.{self.minor}.{self.patch}"


@dataclass
class Permission:
    """Permission for skill execution."""
    name: str
    description: str = ""

    def __str__(self) -> str:
        return self.name


@dataclass
class SkillDependency:
    """Skill dependency specification."""
    name: str
    version_constraint: str
    optional: bool = False

    @classmethod
    def new(cls, name: str, version_constraint: str) -> SkillDependency:
        return cls(name, version_constraint)

    @classmethod
    def optional(cls, name: str, version_constraint: str) -> SkillDependency:
        return cls(name, version_constraint, optional=True)

    def __str__(self) -> str:
        return f"{self.name}@{self.version_constraint}"


@dataclass
class SkillManifest:
    """Skill manifest - definition of a skill."""
    name: str
    version: Version
    runtime: Runtime
    entry_point: str
    description: Optional[str] = None
    author: Optional[str] = None
    permissions: List[Permission] = field(default_factory=list)
    dependencies: List[SkillDependency] = field(default_factory=list)
    config: Dict[str, Any] = field(default_factory=dict)

    @classmethod
    def create(
        cls,
        name: str,
        version: str,
        runtime: Runtime,
        entry_point: str
    ) -> SkillManifest:
        return cls(
            name=name,
            version=Version.parse(version),
            runtime=runtime,
            entry_point=entry_point
        )

    def is_compatible_with(self, other: SkillManifest) -> bool:
        """Check if compatible with another manifest."""
        return self.runtime == other.runtime
