"""
Domain Layer - Core business logic with no external dependencies.

This layer contains:
- Entities: Core business objects with identity
- Value Objects: Immutable objects representing concepts
- Events: Domain events representing state changes
- Services: Domain services and business rules
"""

from hexagonal.domain.entities import Skill, SkillId, SkillCategory
from hexagonal.domain.value_objects import (
    Identifier,
    StringId,
    UuidId,
    Email,
    Url,
    Version,
    Timestamp,
)
from hexagonal.domain.events import DomainEvent

__all__ = [
    # Entities
    "Skill",
    "SkillId",
    "SkillCategory",
    # Value Objects
    "Identifier",
    "StringId",
    "UuidId",
    "Email",
    "Url",
    "Version",
    "Timestamp",
    # Events
    "DomainEvent",
]
