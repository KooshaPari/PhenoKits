"""
Application Handlers - Combined command and query handlers.

SkillHandler:
- Handles all skill-related commands and queries
- Orchestrates domain objects and ports
- Implements CQRS pattern
"""

from dataclasses import dataclass, field
from typing import Any, Dict, List, Optional, Set, TypeVar, Generic

from hexagonal.domain.entities import Skill, SkillCategory
from hexagonal.domain.events import (
    DomainEvent,
    SkillCreated,
    SkillUpdated,
    SkillDeleted,
    SkillDeprecated,
    SkillArchived,
)
from hexagonal.domain.services import SkillValidator, ValidationResult
from hexagonal.ports.inbound import Command, Query, CommandResult, QueryResult
from hexagonal.ports.outbound import Repository, Cache, EventBus

T = TypeVar("T")


@dataclass
class SkillHandler(Generic[T]):
    """
    Handler for skill-related commands and queries.
    
    This handler:
    - Receives commands and queries from inbound ports
    - Coordinates domain objects
    - Uses outbound ports for persistence and events
    - Returns results to the caller
    """

    repository: Repository[Skill, str]
    cache: Optional[Cache[str, Dict[str, Any]]] = None
    event_bus: Optional[EventBus] = None
    validator: Optional[SkillValidator] = None

    def __post_init__(self) -> None:
        if self.validator is None:
            self.validator = SkillValidator()

    # Command Handlers

    async def handle_create_skill(
        self,
        command: Any,
    ) -> CommandResult:
        """Handle create skill command."""
        # Validate command
        validation = command.validate()
        if not validation.is_success:
            return validation

        # Create skill entity
        skill = Skill.create(
            name=command.name,
            description=command.description,
            category=SkillCategory(command.category),
            version=command.version,
            tags=command.tags,
            metadata=command.metadata,
        )

        # Validate skill
        validation_result = self.validator.validate(skill)
        if not validation_result.is_valid:
            return CommandResult.error(
                "Validation failed",
                [e.message for e in validation_result.errors],
            )

        # Save to repository
        saved_skill = await self.repository.save(skill)

        # Publish event
        if self.event_bus:
            event = SkillCreated(
                skill_id=str(saved_skill.id),
                skill_name=saved_skill.name,
                skill_category=saved_skill.category.value,
                created_by=command.created_by,
            )
            await self.event_bus.publish(event)

        # Cache the skill
        if self.cache:
            await self.cache.set(
                str(saved_skill.id),
                saved_skill.to_dict(),
                ttl=3600,
            )

        return CommandResult.ok(
            message="Skill created successfully",
            data={"skill": saved_skill.to_dict()},
        )

    async def handle_update_skill(
        self,
        command: Any,
    ) -> CommandResult:
        """Handle update skill command."""
        # Validate command
        validation = command.validate()
        if not validation.is_success:
            return validation

        # Find existing skill
        skill = await self.repository.find_by_id(command.skill_id)
        if not skill:
            return CommandResult.error(f"Skill not found: {command.skill_id}")

        # Track changes for event
        changes = {}
        if command.name and command.name != skill.name:
            changes["name"] = {"old": skill.name, "new": command.name}
        if command.description and command.description != skill.description:
            changes["description"] = {"old": skill.description, "new": command.description}
        if command.category and command.category != skill.category.value:
            changes["category"] = {"old": skill.category.value, "new": command.category}

        # Update skill
        updated_skill = skill.update(
            name=command.name,
            description=command.description,
            category=SkillCategory(command.category) if command.category else None,
            tags=command.tags,
            metadata=command.metadata,
        )

        # Validate
        validation_result = self.validator.validate(updated_skill)
        if not validation_result.is_valid:
            return CommandResult.error(
                "Validation failed",
                [e.message for e in validation_result.errors],
            )

        # Save
        await self.repository.save(updated_skill)

        # Publish event
        if self.event_bus and changes:
            event = SkillUpdated(
                skill_id=str(updated_skill.id),
                skill_name=updated_skill.name,
                changes=changes,
                updated_by=command.updated_by,
            )
            await self.event_bus.publish(event)

        # Invalidate cache
        if self.cache:
            await self.cache.delete(command.skill_id)

        return CommandResult.ok(
            message="Skill updated successfully",
            data={"skill": updated_skill.to_dict()},
        )

    async def handle_delete_skill(
        self,
        command: Any,
    ) -> CommandResult:
        """Handle delete skill command."""
        # Validate command
        validation = command.validate()
        if not validation.is_success:
            return validation

        # Find skill
        skill = await self.repository.find_by_id(command.skill_id)
        if not skill:
            return CommandResult.error(f"Skill not found: {command.skill_id}")

        skill_name = skill.name

        # Delete
        deleted = await self.repository.delete(command.skill_id)
        if not deleted:
            return CommandResult.error("Failed to delete skill")

        # Publish event
        if self.event_bus:
            event = SkillDeleted(
                skill_id=command.skill_id,
                skill_name=skill_name,
                deleted_by=command.deleted_by,
            )
            await self.event_bus.publish(event)

        # Invalidate cache
        if self.cache:
            await self.cache.delete(command.skill_id)

        return CommandResult.ok(message="Skill deleted successfully")

    async def handle_deprecate_skill(
        self,
        command: Any,
    ) -> CommandResult:
        """Handle deprecate skill command."""
        validation = command.validate()
        if not validation.is_success:
            return validation

        skill = await self.repository.find_by_id(command.skill_id)
        if not skill:
            return CommandResult.error(f"Skill not found: {command.skill_id}")

        deprecated_skill = skill.deprecate()
        await self.repository.save(deprecated_skill)

        if self.event_bus:
            event = SkillDeprecated(
                skill_id=str(deprecated_skill.id),
                skill_name=deprecated_skill.name,
                reason=command.reason,
                deprecated_by=command.deprecated_by,
            )
            await self.event_bus.publish(event)

        if self.cache:
            await self.cache.delete(command.skill_id)

        return CommandResult.ok(message="Skill deprecated successfully")

    async def handle_archive_skill(
        self,
        command: Any,
    ) -> CommandResult:
        """Handle archive skill command."""
        validation = command.validate()
        if not validation.is_success:
            return validation

        skill = await self.repository.find_by_id(command.skill_id)
        if not skill:
            return CommandResult.error(f"Skill not found: {command.skill_id}")

        archived_skill = skill.archive()
        await self.repository.save(archived_skill)

        if self.event_bus:
            event = SkillArchived(
                skill_id=str(archived_skill.id),
                skill_name=archived_skill.name,
                archived_by=command.archived_by,
            )
            await self.event_bus.publish(event)

        if self.cache:
            await self.cache.delete(command.skill_id)

        return CommandResult.ok(message="Skill archived successfully")

    # Query Handlers

    async def handle_get_skill(
        self,
        query: Any,
    ) -> QueryResult[Dict[str, Any]]:
        """Handle get skill query."""
        # Check cache first
        if self.cache:
            cached = await self.cache.get(query.skill_id)
            if cached:
                return QueryResult.ok(data=cached, metadata={"cached": True})

        # Get from repository
        skill = await self.repository.find_by_id(query.skill_id)
        if not skill:
            return QueryResult.error(f"Skill not found: {query.skill_id}")

        data = skill.to_dict()

        # Cache the result
        if self.cache:
            await self.cache.set(query.skill_id, data, ttl=3600)

        return QueryResult.ok(data=data, metadata={"cached": False})

    async def handle_list_skills(
        self,
        query: Any,
    ) -> QueryResult[List[Dict[str, Any]]]:
        """Handle list skills query."""
        skills = await self.repository.find_all()

        # Apply filters
        if query.category:
            skills = [s for s in skills if s.category.value == query.category]
        if query.status:
            skills = [s for s in skills if s.status == query.status]
        if query.tags:
            skills = [s for s in skills if s.tags & query.tags]

        # Apply pagination
        total = len(skills)
        skills = skills[query.offset : query.offset + query.limit]

        data = [s.to_dict() for s in skills]
        return QueryResult.ok(
            data=data,
            metadata={
                "total": total,
                "limit": query.limit,
                "offset": query.offset,
            },
        )
