"""
Domain Value Objects - Immutable objects representing concepts.

Value Objects:
- Identifier: Abstract base for all identifiers
- StringId: String-based identifier
- UuidId: UUID-based identifier
- Email: Email address
- Url: URL/link
- Version: Semantic version
- Timestamp: Date/time value
"""

from abc import ABC, abstractmethod
from dataclasses import dataclass
from datetime import datetime
from typing import Any, Optional, Pattern, Union
import re


class ValueObject(ABC):
    """
    Base class for all value objects.
    
    Value Objects are:
    - Immutable
    - Equal based on their values, not identity
    - Self-validating
    """

    @abstractmethod
    def __eq__(self, other: object) -> bool:
        pass

    @abstractmethod
    def __hash__(self) -> int:
        pass


@dataclass(frozen=True)
class Identifier(ValueObject):
    """
    Abstract base class for identifiers.
    
    Identifiers are immutable strings that uniquely identify entities.
    """

    value: str

    def __post_init__(self) -> None:
        if not self.value:
            raise ValueError("Identifier value cannot be empty")

    def __str__(self) -> str:
        return self.value

    def __repr__(self) -> str:
        return f"{self.__class__.__name__}({self.value!r})"

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, Identifier):
            return False
        return self.value == other.value

    def __hash__(self) -> int:
        return hash(self.value)


@dataclass(frozen=True)
class StringId(Identifier):
    """String-based identifier."""

    def matches(self, pattern: str) -> bool:
        """Check if the ID matches a pattern."""
        return bool(re.match(pattern, self.value))


@dataclass(frozen=True)
class UuidId(Identifier):
    """UUID-based identifier."""

    def __post_init__(self) -> None:
        if not self.value:
            raise ValueError("UUID cannot be empty")
        # Validate UUID format
        uuid_pattern = re.compile(
            r"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$",
            re.IGNORECASE,
        )
        if not uuid_pattern.match(self.value):
            raise ValueError(f"Invalid UUID format: {self.value}")


@dataclass(frozen=True)
class Email(ValueObject):
    """Email address value object."""

    value: str

    def __post_init__(self) -> None:
        pattern = re.compile(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
        if not pattern.match(self.value):
            raise ValueError(f"Invalid email address: {self.value}")

    def __str__(self) -> str:
        return self.value

    def __repr__(self) -> str:
        return f"Email({self.value!r})"

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, Email):
            return False
        return self.value.lower() == other.value.lower()

    def __hash__(self) -> int:
        return hash(self.value.lower())


@dataclass(frozen=True)
class Url(ValueObject):
    """URL value object."""

    value: str

    def __post_init__(self) -> None:
        # Basic URL validation
        pattern = re.compile(
            r"^https?://"  # http:// or https://
            r"(?:(?:[A-Z0-9](?:[A-Z0-9-]{0,61}[A-Z0-9])?\.)+[A-Z]{2,6}\.?|"  # domain
            r"localhost|"  # localhost
            r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})"  # or IP
            r"(?::\d+)?"  # optional port
            r"(?:/?|[/?]\S+)$",
            re.IGNORECASE,
        )
        if not pattern.match(self.value):
            raise ValueError(f"Invalid URL: {self.value}")

    def __str__(self) -> str:
        return self.value

    def __repr__(self) -> str:
        return f"Url({self.value!r})"

    @property
    def scheme(self) -> str:
        """Get the URL scheme (http or https)."""
        return self.value.split("://")[0]

    @property
    def host(self) -> str:
        """Get the URL host."""
        parts = self.value.split("://")
        if len(parts) < 2:
            return ""
        host_path = parts[1].split("/")[0]
        return host_path.split(":")[0]


@dataclass(frozen=True)
class Version(ValueObject):
    """Semantic version value object (MAJOR.MINOR.PATCH)."""

    major: int
    minor: int
    patch: int
    prerelease: Optional[str] = None
    build_metadata: Optional[str] = None

    def __post_init__(
        self,
    ) -> None:
        if self.major < 0:
            raise ValueError("Major version cannot be negative")
        if self.minor < 0:
            raise ValueError("Minor version cannot be negative")
        if self.patch < 0:
            raise ValueError("Patch version cannot be negative")

    def __str__(self) -> str:
        version = f"{self.major}.{self.minor}.{self.patch}"
        if self.prerelease:
            version += f"-{self.prerelease}"
        if self.build_metadata:
            version += f"+{self.build_metadata}"
        return version

    def __repr__(self) -> str:
        return f"Version({self.major}, {self.minor}, {self.patch})"

    @classmethod
    def parse(cls, version_str: str) -> "Version":
        """Parse a version string."""
        build = None
        prerelease = None

        # Split build metadata
        if "+" in version_str:
            version_str, build = version_str.split("+", 1)

        # Split prerelease
        if "-" in version_str:
            version_str, prerelease = version_str.split("-", 1)

        # Parse version numbers
        parts = version_str.split(".")
        if len(parts) < 3:
            raise ValueError(f"Invalid version format: {version_str}")

        return cls(
            major=int(parts[0]),
            minor=int(parts[1]),
            patch=int(parts[2]),
            prerelease=prerelease,
            build_metadata=build,
        )

    def bump_major(self) -> "Version":
        """Create a new version with bumped major."""
        return Version(self.major + 1, 0, 0)

    def bump_minor(self) -> "Version":
        """Create a new version with bumped minor."""
        return Version(self.major, self.minor + 1, 0)

    def bump_patch(self) -> "Version":
        """Create a new version with bumped patch."""
        return Version(self.major, self.minor, self.patch + 1)


@dataclass(frozen=True)
class Timestamp(ValueObject):
    """Timestamp value object."""

    value: datetime

    def __str__(self) -> str:
        return self.value.isoformat()

    @property
    def unix(self) -> int:
        """Get Unix timestamp."""
        return int(self.value.timestamp())

    @property
    def iso(self) -> str:
        """Get ISO format string."""
        return self.value.isoformat()

    @classmethod
    def now(cls) -> "Timestamp":
        """Create a timestamp for current time."""
        return cls(value=datetime.utcnow())

    @classmethod
    def from_unix(cls, timestamp: int) -> "Timestamp":
        """Create a timestamp from Unix timestamp."""
        return cls(value=datetime.fromtimestamp(timestamp))

    @classmethod
    def from_iso(cls, iso_str: str) -> "Timestamp":
        """Create a timestamp from ISO format string."""
        return cls(value=datetime.fromisoformat(iso_str))
