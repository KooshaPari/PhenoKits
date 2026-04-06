"""
Application Queries - Read operations for the application layer.

Queries:
- GetSkill: Get a single skill by ID
- ListSkills: List skills with optional filters
- SearchSkills: Search skills by text
- GetSkillTags: Get all tags for a skill
- GetSkillVersions: Get all versions of a skill
"""

from dataclasses import dataclass, field
from typing import Any, Dict, List, Optional, Set

from hexagonal.ports.inbound import Query, QueryResult


@dataclass
class GetSkillQuery(Query[Any]):
    """Query to get a single skill by ID."""

    skill_id: str
    include_metadata: bool = True

    @property
    def query_type(self) -> str:
        return "get_skill"


@dataclass
class ListSkillsQuery(Query[List[Any]]):
    """Query to list skills with optional filters."""

    category: Optional[str] = None
    tags: Optional[Set[str]] = None
    status: Optional[str] = None
    limit: int = 100
    offset: int = 0
    include_metadata: bool = False

    @property
    def query_type(self) -> str:
        return "list_skills"


@dataclass
class SearchSkillsQuery(Query[List[Any]]):
    """Query to search skills by text."""

    query: str
    categories: Optional[List[str]] = None
    tags: Optional[Set[str]] = None
    limit: int = 50
    include_metadata: bool = False

    @property
    def query_type(self) -> str:
        return "search_skills"


@dataclass
class GetSkillTagsQuery(Query[Set[str]]):
    """Query to get all tags for a skill."""

    skill_id: str

    @property
    def query_type(self) -> str:
        return "get_skill_tags"


@dataclass
class GetSkillByNameQuery(Query[Any]):
    """Query to get a skill by name."""

    name: str
    category: Optional[str] = None

    @property
    def query_type(self) -> str:
        return "get_skill_by_name"


@dataclass
class GetSkillCountQuery(Query[int]):
    """Query to get the count of skills."""

    category: Optional[str] = None
    status: Optional[str] = None

    @property
    def query_type(self) -> str:
        return "get_skill_count"


@dataclass
class ListSkillCategoriesQuery(Query[List[str]]):
    """Query to list all skill categories."""

    @property
    def query_type(self) -> str:
        return "list_skill_categories"
