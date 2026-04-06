"""
Portage Adapter Core - Base classes for all portage adapters.

This module provides the foundation for code-generated adapters from YAML schemas.
"""

from __future__ import annotations

import os
from abc import ABC, abstractmethod
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any, ClassVar, Dict, List, Optional, Type


class RequireNameMeta(type(ABC)):
    """
    Metaclass enforcing that all adapter subclasses define a NAME class attribute.
    
    This ensures every adapter has a unique identifier for registry and discovery.
    """
    
    def __init__(cls, name: str, bases: tuple, dct: dict) -> None:
        super().__init__(name, bases, dct)
        # Skip the check for the base BaseAdapter class itself
        if name != "BaseAdapter" and not hasattr(cls, "NAME"):
            raise TypeError(
                f"Class {name} must define 'NAME' class attribute"
            )


class BaseAdapter(ABC, metaclass=RequireNameMeta):
    """
    Abstract base class for all portage adapters.
    
    Adapters are code-generated from YAML schemas. This base class provides:
    - Common interface for all adapters
    - Configuration from YAML schema
    - Environment preparation utilities
    - Task generation framework
    
    Example YAML schema:
        name: swe-bench
        category: coding
        timeout: 3600
        environment:
            docker_image: python:3.10
        tasks:
            - instance_id: 1
              gold_patch: ...
    """
    
    # Class variable set by code generator
    NAME: ClassVar[str] = "base"
    
    def __init__(self, config: Optional[Dict[str, Any]] = None) -> None:
        """
        Initialize adapter with configuration from YAML schema.
        
        Args:
            config: Configuration dictionary parsed from YAML schema
        """
        self.config = config or {}
        self.name = self.NAME
        self._validate_config()
    
    def _validate_config(self) -> None:
        """Validate configuration has required fields from schema."""
        required_fields = self.get_required_fields()
        for field in required_fields:
            if field not in self.config:
                raise ValueError(
                    f"Missing required field '{field}' in configuration for {self.NAME}"
                )
    
    @abstractmethod
    def get_required_fields(self) -> List[str]:
        """Return list of required configuration fields."""
        pass
    
    @abstractmethod
    def generate_task(self, **kwargs) -> Dict[str, Any]:
        """
        Generate a single task based on configuration.
        
        Args:
            **kwargs: Additional parameters for task generation
            
        Returns:
            Dictionary containing task data
        """
        pass
    
    @abstractmethod
    def prepare_environment(self, task_id: str, **kwargs) -> None:
        """
        Prepare environment for task execution.
        
        Args:
            task_id: Unique identifier for the task
            **kwargs: Additional environment parameters
        """
        pass
    
    @abstractmethod
    def cleanup_environment(self, task_id: str) -> None:
        """
        Cleanup environment after task execution.
        
        Args:
            task_id: Unique identifier for the task
        """
        pass
    
    def validate_task_result(self, result: Dict[str, Any]) -> bool:
        """
        Validate task execution result.
        
        Args:
            result: Result dictionary from task execution
            
        Returns:
            True if result is valid, False otherwise
        """
        return "status" in result and "output" in result


@dataclass
class TaskConfig:
    """Configuration for a single task."""
    
    instance_id: str
    timeout: int = 3600
    metadata: Dict[str, Any] = field(default_factory=dict)
    
    @property
    def task_id(self) -> str:
        """Generate unique task ID from instance_id."""
        return f"{self.NAME}:{self.instance_id}"


@dataclass 
class AdapterSchema:
    """Schema for code-generated adapter configuration."""
    
    name: str
    category: str
    timeout: int = 3600
    environment: Dict[str, str] = field(default_factory=dict)
    tasks: List[Dict[str, Any]] = field(default_factory=list)
    
    @classmethod
    def from_yaml(cls, yaml_path: Path) -> "AdapterSchema":
        """Load schema from YAML file."""
        import yaml
        with open(yaml_path) as f:
            data = yaml.safe_load(f)
        return cls(**data)
    
    def to_adapter_class(self) -> str:
        """Generate Python adapter class from schema."""
        return f'''
from portage_adapter_core import BaseAdapter, RequireNameMeta

class {self.name.title().replace("-", "")}Adapter(
    BaseAdapter,
    metaclass=RequireNameMeta
):
    """Generated adapter for {self.name} - {self.category}"""
    NAME = "{self.name}"
    
    def get_required_fields(self):
        return ["instance_id"]
    
    def generate_task(self, **kwargs):
        # Generated from YAML schema
        return {{}}
    
    def prepare_environment(self, task_id, **kwargs):
        # Generated from YAML schema
        pass
    
    def cleanup_environment(self, task_id):
        # Generated from YAML schema
        pass
'''


class AdapterRegistry:
    """
    Registry for discovered and loaded adapters.
    
    Maintains a mapping of adapter names to their classes for dynamic loading.
    """
    
    _adapters: Dict[str, Type[BaseAdapter]] = {}
    
    @classmethod
    def register(cls, adapter_class: Type[BaseAdapter]) -> None:
        """Register an adapter class in the registry."""
        cls._adapters[adapter_class.NAME] = adapter_class
    
    @classmethod
    def get(cls, name: str) -> Optional[Type[BaseAdapter]]:
        """Get adapter class by name."""
        return cls._adapters.get(name)
    
    @classmethod
    def list_adapters(cls) -> List[str]:
        """List all registered adapter names."""
        return list(cls._adapters.keys())


def auto_register(cls: Type[BaseAdapter]) -> Type[BaseAdapter]:
    """Decorator to auto-register adapter classes."""
    AdapterRegistry.register(cls)
    return cls
