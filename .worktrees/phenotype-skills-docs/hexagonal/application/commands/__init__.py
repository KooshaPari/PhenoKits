"""
Application Commands - Write operations for the application layer.

Commands:
- CreateSkill: Create a new skill
- UpdateSkill: Update an existing skill
- DeleteSkill: Delete a skill
- DeprecateSkill: Deprecate a skill
- ArchiveSkill: Archive a skill
- ReactivateSkill: Reactivate a deprecated/archived skill
- AddSkillTag: Add a tag to a skill
- RemoveSkillTag: Remove a tag from a skill
"""

from dataclasses import dataclass, field
from typing import Any, Dict, List, Optional, Set

from hexagonal.ports.inbound import (
    Command,
    CommandResult,
    CreateSkillCommand as InboundCreateSkillCommand,
    UpdateSkillCommand as InboundUpdateSkillCommand,
    DeleteSkillCommand as InboundDeleteSkillCommand,
    DeprecateSkillCommand as InboundDeprecateSkillCommand,
    ArchiveSkillCommand as InboundArchiveSkillCommand,
    AddSkillTagCommand as InboundAddSkillTagCommand,
    RemoveSkillTagCommand as InboundRemoveSkillTagCommand,
)


# Re-export inbound commands for convenience
__all__ = [
    "CreateSkillCommand",
    "UpdateSkillCommand",
    "DeleteSkillCommand",
    "DeprecateSkillCommand",
    "ArchiveSkillCommand",
    "ReactivateSkillCommand",
    "AddSkillTagCommand",
    "RemoveSkillTagCommand",
]


@dataclass
class CreateSkillCommand(InboundCreateSkillCommand):
    """Command to create a new skill."""
    pass


@dataclass
class UpdateSkillCommand(InboundUpdateSkillCommand):
    """Command to update an existing skill."""
    pass


@dataclass
class DeleteSkillCommand(InboundDeleteSkillCommand):
    """Command to delete a skill."""
    pass


@dataclass
class DeprecateSkillCommand(InboundDeprecateSkillCommand):
    """Command to deprecate a skill."""
    pass


@dataclass
class ArchiveSkillCommand(InboundArchiveSkillCommand):
    """Command to archive a skill."""
    pass


@dataclass
class ReactivateSkillCommand(Command):
    """Command to reactivate a deprecated or archived skill."""

    skill_id: str
    reactivated_by: Optional[str] = None

    @property
    def command_type(self) -> str:
        return "reactivate_skill"

    def validate(self) -> CommandResult:
        if not self.skill_id:
            return CommandResult.error("Skill ID is required")
        return CommandResult.ok()


@dataclass
class AddSkillTagCommand(InboundAddSkillTagCommand):
    """Command to add a tag to a skill."""
    pass


@dataclass
class RemoveSkillTagCommand(InboundRemoveSkillTagCommand):
    """Command to remove a tag from a skill."""
    pass
