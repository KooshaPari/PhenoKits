"""Dependency resolver for skill graphs."""

from typing import Dict, List, Set, Iterable
from collections import defaultdict

from .types import SkillId
from .skill import Skill


class DependencyResolver:
    """Dependency resolver for skill graphs."""

    def resolve(self, skills: Iterable[Skill]) -> List[Skill]:
        """Resolve all dependencies for a set of skills.
        
        Returns skills in dependency order (dependencies first).
        """
        skill_list = list(skills)
        if not skill_list:
            return []

        # Build dependency graph
        graph: Dict[SkillId, List[SkillId]] = defaultdict(list)
        skill_map = {s.id: s for s in skill_list}

        for skill in skill_list:
            for dep in skill.manifest.dependencies:
                # Find skill that provides this dependency
                provider = next((s for s in skill_list if s.id.value == dep.name), None)
                if provider:
                    graph[skill.id].append(provider.id)

        # Topological sort
        result: List[Skill] = []
        visited: Set[SkillId] = set()
        temp: Set[SkillId] = set()

        def visit(skill_id: SkillId) -> None:
            if skill_id in temp:
                raise ValueError(f"Circular dependency detected involving: {skill_id}")
            if skill_id in visited:
                return

            temp.add(skill_id)
            for dep_id in graph.get(skill_id, []):
                visit(dep_id)
            temp.remove(skill_id)
            visited.add(skill_id)
            result.append(skill_map[skill_id])

        for skill in skill_list:
            visit(skill.id)

        # Reverse to get dependency order (dependencies before dependents)
        result.reverse()
        return result

    def check_circular(self, skills: Iterable[Skill]) -> None:
        """Check for circular dependencies."""
        # Just run resolve - it will raise on circular dependencies
        self.resolve(skills)

    def get_all_dependencies(self, skill: Skill, all_skills: Iterable[Skill]) -> List[Skill]:
        """Get all dependencies for a skill (recursively)."""
        all_skills_list = list(all_skills)
        result: List[Skill] = []
        visited: Set[str] = set()

        def collect_deps(s: Skill) -> None:
            for dep in s.manifest.dependencies:
                if dep.name in visited:
                    continue
                dep_skill = next((x for x in all_skills_list if x.id.value == dep.name), None)
                if dep_skill:
                    visited.add(dep.name)
                    result.append(dep_skill)
                    collect_deps(dep_skill)

        collect_deps(skill)
        return result

    def to_dot(self, skills: Iterable[Skill]) -> str:
        """Generate a DOT graph representation."""
        lines = ["digraph dependencies {", "  rankdir=BT;"]
        skill_list = list(skills)

        # Add nodes
        for skill in skill_list:
            lines.append(f'  "{skill.id}" [label="{skill.id} v{skill.version}"];')

        # Add edges
        for skill in skill_list:
            for dep in skill.manifest.dependencies:
                lines.append(f'  "{dep.name}" -> "{skill.id}";')

        lines.append("}")
        return "\n".join(lines)
