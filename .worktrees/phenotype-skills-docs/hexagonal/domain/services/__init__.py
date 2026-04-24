"""
Domain Services - Business logic services.

Domain Services:
- Specification: Specification pattern for complex business rules
- Validator: Validation service for domain objects
"""

from abc import ABC, abstractmethod
from dataclasses import dataclass, field
from datetime import datetime
from typing import Any, List, Optional, Set


class Specification(ABC):
    """
    Specification pattern for complex business rules.
    
    A specification defines a rule that can be evaluated against an object.
    Specifications can be combined using AND, OR, NOT operations.
    """

    @abstractmethod
    def is_satisfied_by(self, candidate: Any) -> bool:
        """Check if the candidate satisfies this specification."""
        pass

    def and_(self, other: "Specification") -> "AndSpecification":
        """Combine with AND operation."""
        return AndSpecification(self, other)

    def or_(self, other: "Specification") -> "OrSpecification":
        """Combine with OR operation."""
        return OrSpecification(self, other)

    def not_(self) -> "NotSpecification":
        """Negate this specification."""
        return NotSpecification(self)


@dataclass
class AndSpecification(Specification):
    """Specification that is satisfied if both operands are satisfied."""

    left: Specification
    right: Specification

    def is_satisfied_by(self, candidate: Any) -> bool:
        return self.left.is_satisfied_by(candidate) and self.right.is_satisfied_by(candidate)


@dataclass
class OrSpecification(Specification):
    """Specification that is satisfied if either operand is satisfied."""

    left: Specification
    right: Specification

    def is_satisfied_by(self, candidate: Any) -> bool:
        return self.left.is_satisfied_by(candidate) or self.right.is_satisfied_by(candidate)


@dataclass
class NotSpecification(Specification):
    """Specification that negates another specification."""

    specification: Specification

    def is_satisfied_by(self, candidate: Any) -> bool:
        return not self.specification.is_satisfied_by(candidate)


@dataclass
class SkillNameSpecification(Specification):
    """Specification for valid skill names."""

    min_length: int = 1
    max_length: int = 100

    def is_satisfied_by(self, candidate: str) -> bool:
        if not candidate:
            return False
        stripped = candidate.strip()
        return self.min_length <= len(stripped) <= self.max_length


@dataclass
class SkillCategorySpecification(Specification):
    """Specification for valid skill categories."""

    valid_categories: Set[str]

    def is_satisfied_by(self, candidate: str) -> bool:
        return candidate in self.valid_categories


@dataclass
class SkillTagSpecification(Specification):
    """Specification for valid skill tags."""

    min_tags: int = 0
    max_tags: int = 10
    min_tag_length: int = 1
    max_tag_length: int = 30

    def is_satisfied_by(self, candidate: Set[str]) -> bool:
        if len(candidate) < self.min_tags or len(candidate) > self.max_tags:
            return False
        for tag in candidate:
            if len(tag) < self.min_tag_length or len(tag) > self.max_tag_length:
                return False
        return True


@dataclass
class ValidationError:
    """Represents a validation error."""

    field: str
    message: str
    code: Optional[str] = None


@dataclass
class ValidationResult:
    """Result of a validation operation."""

    errors: List[ValidationError] = field(default_factory=list)

    @property
    def is_valid(self) -> bool:
        """Check if validation passed."""
        return len(self.errors) == 0

    def add_error(self, field: str, message: str, code: Optional[str] = None) -> None:
        """Add a validation error."""
        self.errors.append(ValidationError(field=field, message=message, code=code))

    def merge(self, other: "ValidationResult") -> "ValidationResult":
        """Merge another validation result."""
        result = ValidationResult(errors=[*self.errors, *other.errors])
        return result


class Validator(ABC):
    """
    Abstract base class for validators.
    
    Validators check that domain objects meet business rules.
    """

    @abstractmethod
    def validate(self, entity: Any) -> ValidationResult:
        """Validate an entity and return the result."""
        pass


@dataclass
class SkillValidator(Validator):
    """Validator for Skill entities."""

    name_spec: Optional[SkillNameSpecification] = None
    category_spec: Optional[SkillCategorySpecification] = None
    tag_spec: Optional[SkillTagSpecification] = None

    def __post_init__(self) -> None:
        if self.name_spec is None:
            self.name_spec = SkillNameSpecification(min_length=1, max_length=100)
        if self.category_spec is None:
            self.category_spec = SkillCategorySpecification(
                valid_categories={"development", "devops", "data", "security", 
                                "design", "management", "documentation", "testing", "other"}
            )
        if self.tag_spec is None:
            self.tag_spec = SkillTagSpecification(min_tags=0, max_tags=10)

    def validate(self, entity: Any) -> ValidationResult:
        """Validate a skill entity."""
        result = ValidationResult()

        # Validate name
        if not hasattr(entity, "name"):
            result.add_error("name", "Entity must have a name", "MISSING_NAME")
        elif not self.name_spec.is_satisfied_by(entity.name):
            result.add_error(
                "name", 
                f"Name must be between 1 and 100 characters", 
                "INVALID_NAME_LENGTH"
            )

        # Validate description
        if not hasattr(entity, "description"):
            result.add_error("description", "Entity must have a description", "MISSING_DESCRIPTION")
        elif not entity.description or not entity.description.strip():
            result.add_error("description", "Description cannot be empty", "EMPTY_DESCRIPTION")

        # Validate category
        if not hasattr(entity, "category"):
            result.add_error("category", "Entity must have a category", "MISSING_CATEGORY")
        elif hasattr(entity.category, "value"):
            # It's an enum
            if not self.category_spec.is_satisfied_by(entity.category.value):
                result.add_error(
                    "category", 
                    f"Invalid category", 
                    "INVALID_CATEGORY"
                )

        # Validate tags
        if hasattr(entity, "tags"):
            if not self.tag_spec.is_satisfied_by(entity.tags):
                result.add_error(
                    "tags", 
                    f"Tags must be between 0 and 10, each between 1 and 30 characters", 
                    "INVALID_TAGS"
                )

        return result
