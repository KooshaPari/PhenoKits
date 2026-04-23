"""Bounded context management for multi-agent systems.

Provides isolation boundaries, data flow rules, and context management
for multi-agent workflows.
"""

from __future__ import annotations
from dataclasses import dataclass, field
from datetime import datetime
from enum import Enum
from typing import Any, Optional
import uuid


class ContextType(str, Enum):
    """Types of bounded contexts."""
    WORKFLOW = "workflow"
    AGENT = "agent"
    SESSION = "session"
    PROJECT = "project"


class DataClassification(str, Enum):
    """Data classification levels."""
    PUBLIC = "public"
    INTERNAL = "internal"
    CONFIDENTIAL = "confidential"
    RESTRICTED = "restricted"


@dataclass
class DataPolicy:
    """Policy for data flow between contexts."""
    source_context: str
    target_context: str
    allowed_fields: list[str] = field(default_factory=list)
    denied_fields: list[str] = field(default_factory=list)
    transformation: Optional[str] = None  # e.g., "redact", "hash", "mask"
    
    def can_transfer(self, field: str) -> bool:
        """Check if a field can be transferred."""
        if field in self.denied_fields:
            return False
        if self.allowed_fields and field not in self.allowed_fields:
            return False
        return True


@dataclass
class BoundaryRule:
    """Rule for context boundaries."""
    name: str
    description: str = ""
    from_context: Optional[str] = None
    to_context: Optional[str] = None
    data_policies: list[DataPolicy] = field(default_factory=list)
    required_roles: list[str] = field(default_factory=list)
    max_handoffs: int = 100


@dataclass
class BoundedContext:
    """A bounded context in the domain model.
    
    Represents an isolated area of the system with clear boundaries,
    data flow rules, and ownership.
    
    Attributes:
        context_id: Unique identifier
        name: Human-readable name
        context_type: Type of context
        parent: Parent context ID (if nested)
        children: Child context IDs
        boundaries: Boundary rules for this context
        metadata: Additional context metadata
    """
    context_id: str
    name: str
    context_type: ContextType
    description: str = ""
    parent: Optional[str] = None
    children: list[str] = field(default_factory=list)
    boundaries: list[BoundaryRule] = field(default_factory=list)
    metadata: dict[str, Any] = field(default_factory=dict)
    created_at: datetime = field(default_factory=datetime.utcnow)
    updated_at: datetime = field(default_factory=datetime.utcnow)
    
    def add_boundary(self, rule: BoundaryRule) -> None:
        """Add a boundary rule to this context."""
        self.boundaries.append(rule)
        self.updated_at = datetime.utcnow()
    
    def add_child(self, child_id: str) -> None:
        """Add a child context."""
        if child_id not in self.children:
            self.children.append(child_id)
            self.updated_at = datetime.utcnow()
    
    def remove_child(self, child_id: str) -> None:
        """Remove a child context."""
        if child_id in self.children:
            self.children.remove(child_id)
            self.updated_at = datetime.utcnow()
    
    def get_ancestors(self, context_map: dict[str, BoundedContext]) -> list[str]:
        """Get all ancestor context IDs up to root."""
        ancestors = []
        current_parent = self.parent
        while current_parent:
            ancestors.append(current_parent)
            parent_ctx = context_map.get(current_parent)
            current_parent = parent_ctx.parent if parent_ctx else None
        return ancestors
    
    def get_descendants(self, context_map: dict[str, BoundedContext]) -> list[str]:
        """Get all descendant context IDs down to leaves."""
        descendants = []
        stack = list(self.children)
        while stack:
            child_id = stack.pop()
            descendants.append(child_id)
            child_ctx = context_map.get(child_id)
            if child_ctx:
                stack.extend(child_ctx.children)
        return descendants
    
    def is_ancestor_of(self, other_id: str, context_map: dict[str, BoundedContext]) -> bool:
        """Check if this context is an ancestor of another."""
        return other_id in self.get_descendants(context_map)
    
    def is_descendant_of(self, other_id: str, context_map: dict[str, BoundedContext]) -> bool:
        """Check if this context is a descendant of another."""
        return other_id in self.get_ancestors(context_map)


@dataclass
class ContextManager:
    """Manager for bounded contexts.
    
    Provides CRUD operations and context relationship management.
    
    Attributes:
        contexts: Map of context_id to BoundedContext
        root_context: ID of the root context
    """
    contexts: dict[str, BoundedContext] = field(default_factory=dict)
    root_context: Optional[str] = None
    
    def create_context(
        self,
        name: str,
        context_type: ContextType,
        parent: Optional[str] = None,
        description: str = "",
        **metadata,
    ) -> BoundedContext:
        """Create a new bounded context.
        
        Args:
            name: Human-readable name
            context_type: Type of context
            parent: Parent context ID (defaults to root)
            description: Optional description
            **metadata: Additional metadata
            
        Returns:
            Created BoundedContext
        """
        context_id = str(uuid.uuid4())
        
        context = BoundedContext(
            context_id=context_id,
            name=name,
            context_type=context_type,
            description=description,
            parent=parent or self.root_context,
            metadata=metadata,
        )
        
        self.contexts[context_id] = context
        
        # Update parent's children
        if context.parent:
            parent_ctx = self.contexts.get(context.parent)
            if parent_ctx:
                parent_ctx.add_child(context_id)
        
        return context
    
    def get_context(self, context_id: str) -> Optional[BoundedContext]:
        """Get a context by ID."""
        return self.contexts.get(context_id)
    
    def find_by_name(self, name: str) -> Optional[BoundedContext]:
        """Find a context by name."""
        for context in self.contexts.values():
            if context.name == name:
                return context
        return None
    
    def find_by_type(self, context_type: ContextType) -> list[BoundedContext]:
        """Find all contexts of a given type."""
        return [
            ctx for ctx in self.contexts.values()
            if ctx.context_type == context_type
        ]
    
    def delete_context(self, context_id: str) -> bool:
        """Delete a context and its descendants.
        
        Args:
            context_id: ID of context to delete
            
        Returns:
            True if deleted, False if not found
        """
        context = self.contexts.get(context_id)
        if not context:
            return False
        
        # Delete descendants first
        for child_id in context.children:
            self.delete_context(child_id)
        
        # Remove from parent's children
        if context.parent:
            parent = self.contexts.get(context.parent)
            if parent:
                parent.remove_child(context_id)
        
        # Remove from registry
        del self.contexts[context_id]
        
        return True
    
    def can_transfer_data(
        self,
        source_id: str,
        target_id: str,
        field: str,
    ) -> tuple[bool, Optional[str]]:
        """Check if data transfer is allowed between contexts.
        
        Args:
            source_id: Source context ID
            target_id: Target context ID
            field: Field name to transfer
            
        Returns:
            Tuple of (allowed, reason)
        """
        source = self.contexts.get(source_id)
        target = self.contexts.get(target_id)
        
        if not source or not target:
            return False, "Context not found"
        
        # Find applicable boundary rule
        rule = self._find_boundary_rule(source_id, target_id)
        if not rule:
            # No rule means transfer not allowed by default
            return False, "No boundary rule defined"
        
        # Check data policies
        for policy in rule.data_policies:
            if policy.can_transfer(field):
                return True, None
        
        return False, f"Field '{field}' denied by policy"
    
    def transfer_data(
        self,
        source_id: str,
        target_id: str,
        data: dict[str, Any],
    ) -> tuple[dict[str, Any], list[str]]:
        """Transfer data between contexts with policy enforcement.
        
        Args:
            source_id: Source context ID
            target_id: Target context ID
            data: Data to transfer
            
        Returns:
            Tuple of (filtered_data, denied_fields)
        """
        filtered = {}
        denied = []
        
        for field, value in data.items():
            allowed, reason = self.can_transfer_data(source_id, target_id, field)
            if allowed:
                filtered[field] = value
            else:
                denied.append(field)
        
        return filtered, denied
    
    def _find_boundary_rule(
        self,
        source_id: str,
        target_id: str,
    ) -> Optional[BoundaryRule]:
        """Find boundary rule for context pair."""
        source = self.contexts.get(source_id)
        target = self.contexts.get(target_id)
        
        if not source or not target:
            return None
        
        # Check source's outgoing rules
        for rule in source.boundaries:
            if rule.to_context == target_id:
                return rule
        
        # Check target's incoming rules
        for rule in target.boundaries:
            if rule.from_context == source_id:
                return rule
        
        # Check hierarchical relationships
        # Parent -> child
        if target_id in source.children:
            for rule in source.boundaries:
                if rule.to_context is None:  # Generic parent rule
                    return rule
        
        # Child -> parent
        if source.parent == target_id:
            for rule in target.boundaries:
                if rule.from_context is None:  # Generic child rule
                    return rule
        
        return None
    
    def visualize_hierarchy(self) -> str:
        """Generate ASCII visualization of context hierarchy."""
        if not self.root_context:
            return "(no root context)"
        
        lines = []
        
        def render(context_id: str, prefix: str = "", is_last: bool = True):
            context = self.contexts.get(context_id)
            if not context:
                return
            
            connector = "└── " if is_last else "├── "
            lines.append(f"{prefix}{connector}{context.name} ({context.context_type.value})")
            
            new_prefix = prefix + ("    " if is_last else "│   ")
            
            for i, child_id in enumerate(context.children):
                is_child_last = i == len(context.children) - 1
                render(child_id, new_prefix, is_child_last)
        
        root = self.contexts.get(self.root_context)
        if root:
            lines.append(f"{root.name} ({root.context_type.value})")
            for i, child_id in enumerate(root.children):
                is_last = i == len(root.children) - 1
                render(child_id, "", is_last)
        
        return "\n".join(lines)
    
    def to_dict(self) -> dict[str, Any]:
        """Serialize to dictionary."""
        return {
            "root_context": self.root_context,
            "contexts": {
                cid: {
                    "context_id": ctx.context_id,
                    "name": ctx.name,
                    "context_type": ctx.context_type.value,
                    "description": ctx.description,
                    "parent": ctx.parent,
                    "children": ctx.children,
                    "metadata": ctx.metadata,
                }
                for cid, ctx in self.contexts.items()
            },
        }
    
    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> ContextManager:
        """Deserialize from dictionary."""
        manager = cls()
        manager.root_context = data.get("root_context")
        
        for cid, ctx_data in data.get("contexts", {}).items():
            manager.contexts[cid] = BoundedContext(
                context_id=ctx_data["context_id"],
                name=ctx_data["name"],
                context_type=ContextType(ctx_data["context_type"]),
                description=ctx_data.get("description", ""),
                parent=ctx_data.get("parent"),
                children=ctx_data.get("children", []),
                metadata=ctx_data.get("metadata", {}),
            )
        
        return manager
