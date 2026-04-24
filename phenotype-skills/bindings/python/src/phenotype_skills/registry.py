"""Skill registry implementation."""

from typing import Dict, List, Optional
from threading import RLock

from .types import SkillId, SkillManifest
from .skill import Skill


class SkillRegistry:
    """Main registry for managing skills."""

    def __init__(self):
        self._skills: Dict[SkillId, Skill] = {}
        self._lock = RLock()

    def register(self, skill: Skill) -> None:
        """Register a new skill."""
        with self._lock:
            if skill.id in self._skills:
                raise ValueError(f"Skill already registered: {skill.id}")
            self._skills[skill.id] = skill

    def unregister(self, skill_id: SkillId) -> Skill:
        """Unregister a skill."""
        with self._lock:
            if skill_id not in self._skills:
                raise KeyError(f"Skill not found: {skill_id}")
            return self._skills.pop(skill_id)

    def get(self, skill_id: SkillId) -> Skill:
        """Get a skill by ID."""
        with self._lock:
            if skill_id not in self._skills:
                raise KeyError(f"Skill not found: {skill_id}")
            return self._skills[skill_id]

    def try_get(self, skill_id: SkillId) -> Optional[Skill]:
        """Try to get a skill by ID."""
        with self._lock:
            return self._skills.get(skill_id)

    def list(self) -> List[Skill]:
        """List all registered skills."""
        with self._lock:
            return list(self._skills.values())

    def update(self, skill_id: SkillId, manifest: SkillManifest) -> Skill:
        """Update a skill's manifest."""
        with self._lock:
            if skill_id not in self._skills:
                raise KeyError(f"Skill not found: {skill_id}")
            skill = self._skills[skill_id]
            skill.update_manifest(manifest)
            return skill

    def exists(self, skill_id: SkillId) -> bool:
        """Check if a skill exists."""
        with self._lock:
            return skill_id in self._skills

    @property
    def count(self) -> int:
        """Get the count of registered skills."""
        with self._lock:
            return len(self._skills)


class SkillRegistryBuilder:
    """Builder for creating configured SkillRegistry instances."""

    def build(self) -> SkillRegistry:
        """Build the registry."""
        return SkillRegistry()
