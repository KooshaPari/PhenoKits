"""
Code generator for portage adapters from YAML schemas.

This module provides functionality to generate adapter.py files from YAML schema definitions.
"""

from __future__ import annotations

import os
from pathlib import Path
from typing import Any, Dict, List, Optional

import yaml


class AdapterCodeGenerator:
    """
    Generate Python adapter classes from YAML schema definitions.
    
    This generator reads YAML schemas and produces complete adapter.py files
    with proper inheritance from BaseAdapter and proper metaclass usage.
    
    Example YAML schema:
        name: swe-bench
        category: coding
        timeout: 3600
        environment:
            docker_image: python:3.10
        tasks:
            - instance_id: ...
    
    Generated code:
        class SWEBenchAdapter(BaseAdapter, metaclass=RequireNameMeta):
            NAME = "swe-bench"
            ...
    """
    
    TEMPLATE_HEADER = '''"""
Generated adapter for {name} - {category} category.
DO NOT EDIT MANUALLY - This file is generated from {schema_path}
"""

from portage_adapter_core import BaseAdapter, RequireNameMeta


class {class_name}Adapter(BaseAdapter, metaclass=RequireNameMeta):
    """Generated adapter for {name} benchmark suite."""
    NAME = "{name}"
'''
    
    TEMPLATE_REQUIRED_FIELDS = '''
    def get_required_fields(self) -> list[str]:
        """Required configuration fields for this adapter."""
        return {required_fields}
'''
    
    TEMPLATE_GENERATE_TASK = '''
    def generate_task(self, **kwargs) -> dict[str, Any]:
        """Generate a task from the benchmark dataset.
        
        Args:
            **kwargs: Additional generation parameters
                - instance_id: Specific instance to generate
                - limit: Maximum number of tasks to generate
                
        Returns:
            Dictionary containing task data
        """
        # TODO: Implement task generation from YAML schema
        return {{}}
'''
    
    TEMPLATE_PREPARE_ENV = '''
    def prepare_environment(self, task_id: str, **kwargs) -> None:
        """Prepare execution environment for the task.
        
        Args:
            task_id: Unique task identifier
            **kwargs: Additional setup parameters
        """
        # TODO: Implement environment preparation from YAML schema
        pass
'''
    
    TEMPLATE_CLEANUP = '''
    def cleanup_environment(self, task_id: str) -> None:
        """Cleanup environment after task execution.
        
        Args:
            task_id: Task identifier to cleanup
        """
        # TODO: Implement cleanup from YAML schema
        pass
'''
    
    TEMPLATE_VALIDATE = '''
    def validate_task_result(self, result: dict[str, Any]) -> bool:
        """Validate task execution result.
        
        Args:
            result: Execution result dictionary
            
        Returns:
            True if result meets success criteria
        """
        return "status" in result
'''
    
    def __init__(self, schema_path: Path | str) -> None:
        """
        Initialize generator with YAML schema.
        
        Args:
            schema_path: Path to YAML schema file
        """
        self.schema_path = Path(schema_path)
        self.schema = self._load_schema()
        self.name = self.schema.get("name", "unknown")
        self.category = self.schema.get("category", "general")
        self.timeout = self.schema.get("timeout", 3600)
        self.environment = self.schema.get("environment", {})
        self.tasks = self.schema.get("tasks", [])
    
    def _load_schema(self) -> Dict[str, Any]:
        """Load YAML schema from file."""
        with open(self.schema_path) as f:
            schema = yaml.safe_load(f)
        
        # Handle nested structure (agents[0].name)
        if "agents" in schema and schema["agents"]:
            agent = schema["agents"][0]
            schema["name"] = agent.get("name", "unknown")
            schema["category"] = agent.get("type", "general")
            schema["timeout"] = agent.get("timeout", 3600)
            schema["environment"] = agent.get("environment", {})
        
        return schema
    
    def _get_class_name(self) -> str:
        """Generate Python class name from adapter name."""
        # Convert "swe-bench" -> "SWEBench"
        parts = self.name.replace("-", "_").split("_")
        return "".join(p.title() for p in parts)
    
    def _get_required_fields(self) -> List[str]:
        """Extract required fields from schema."""
        required = ["instance_id"]  # Always required
        # Add schema-specific required fields
        return required
    
    def generate(self, output_path: Optional[Path | str] = None) -> str:
        """
        Generate adapter.py code from schema.
        
        Args:
            output_path: Optional path to write generated code
            
        Returns:
            Generated Python code as string
        """
        class_name = self._get_class_name()
        required_fields = self._get_required_fields()
        
        code_parts = [
            self.TEMPLATE_HEADER.format(
                name=self.name,
                category=self.category,
                schema_path=str(self.schema_path),
                class_name=class_name
            ),
            self.TEMPLATE_REQUIRED_FIELDS.format(
                required_fields=required_fields
            ),
            self.TEMPLATE_GENERATE_TASK,
            self.TEMPLATE_PREPARE_ENV,
            self.TEMPLATE_CLEANUP,
            self.TEMPLATE_VALIDATE,
        ]
        
        code = "\n".join(code_parts)
        
        if output_path:
            Path(output_path).write_text(code)
        
        return code
    
    def generate_from_template(self, template_name: str) -> str:
        """
        Generate code using a specific template pattern.
        
        Args:
            template_name: Name of template pattern to use
                - "coding" - SWE-Bench style
                - "reasoning" - MMLU style
                - "database" - Spider style
                - "agent" - ARC-AGI style
                
        Returns:
            Generated code string
        """
        if template_name == "coding":
            return self._generate_coding_adapter()
        elif template_name == "reasoning":
            return self._generate_reasoning_adapter()
        elif template_name == "database":
            return self._generate_database_adapter()
        elif template_name == "agent":
            return self._generate_agent_adapter()
        else:
            return self.generate()
    
    def _generate_coding_adapter(self) -> str:
        """Generate adapter for coding benchmarks."""
        class_name = self._get_class_name()
        return f'''
from portage_adapter_core import BaseAdapter, RequireNameMeta


class {class_name}Adapter(BaseAdapter, metaclass=RequireNameMeta):
    """Coding benchmark adapter for {self.name}."""
    NAME = "{self.name}"
    CATEGORY = "coding"
    TIMEOUT = {self.timeout}
    
    def get_required_fields(self) -> list[str]:
        return ["instance_id", "problem"]
    
    def generate_task(self, **kwargs) -> dict[str, Any]:
        return {{}}
    
    def prepare_environment(self, task_id: str, **kwargs) -> None:
        pass
    
    def cleanup_environment(self, task_id: str) -> None:
        pass
    
    def validate_task_result(self, result: dict[str, Any]) -> bool:
        return result.get("status") == "success"
'''
    
    def _generate_reasoning_adapter(self) -> str:
        """Generate adapter for reasoning benchmarks."""
        class_name = self._get_class_name()
        return f'''
from portage_adapter_core import BaseAdapter, RequireNameMeta


class {class_name}Adapter(BaseAdapter, metaclass=RequireNameMeta):
    """Reasoning benchmark adapter for {self.name}."""
    NAME = "{self.name}"
    CATEGORY = "reasoning"
    TIMEOUT = {self.timeout}
    
    def get_required_fields(self) -> list[str]:
        return ["question", "options"]
    
    def generate_task(self, **kwargs) -> dict[str, Any]:
        return {{}}
    
    def prepare_environment(self, task_id: str, **kwargs) -> None:
        pass
    
    def cleanup_environment(self, task_id: str) -> None:
        pass
    
    def validate_task_result(self, result: dict[str, Any]) -> bool:
        return "answer" in result
'''
    
    def _generate_database_adapter(self) -> str:
        """Generate adapter for database benchmarks."""
        class_name = self._get_class_name()
        return f'''
from portage_adapter_core import BaseAdapter, RequireNameMeta


class {class_name}Adapter(BaseAdapter, metaclass=RequireNameMeta):
    """Database benchmark adapter for {self.name}."""
    NAME = "{self.name}"
    CATEGORY = "database"
    TIMEOUT = {self.timeout}
    
    def get_required_fields(self) -> list[str]:
        return ["schema", "question", "gold_sql"]
    
    def generate_task(self, **kwargs) -> dict[str, Any]:
        return {{}}
    
    def prepare_environment(self, task_id: str, **kwargs) -> None:
        pass
    
    def cleanup_environment(self, task_id: str) -> None:
        pass
    
    def validate_task_result(self, result: dict[str, Any]) -> bool:
        return result.get("execution_success") == True
'''
    
    def _generate_agent_adapter(self) -> str:
        """Generate adapter for agent benchmarks."""
        class_name = self._get_class_name()
        return f'''
from portage_adapter_core import BaseAdapter, RequireNameMeta


class {class_name}Adapter(BaseAdapter, metaclass=RequireNameMeta):
    """Agent benchmark adapter for {self.name}."""
    NAME = "{self.name}"
    CATEGORY = "agent"
    TIMEOUT = {self.timeout}
    
    def get_required_fields(self) -> list[str]:
        return ["task_description", "evaluation_fn"]
    
    def generate_task(self, **kwargs) -> dict[str, Any]:
        return {{}}
    
    def prepare_environment(self, task_id: str, **kwargs) -> None:
        pass
    
    def cleanup_environment(self, task_id: str) -> None:
        pass
    
    def validate_task_result(self, result: dict[str, Any]) -> bool:
        return result.get("score", 0) >= result.get("threshold", 0.8)
'''


class BatchGenerator:
    """
    Generate multiple adapters from directory of YAML schemas.
    
    Usage:
        generator = BatchGenerator("/path/to/adapters")
        generator.generate_all()
    """
    
    def __init__(self, base_path: Path | str) -> None:
        """
        Initialize batch generator.
        
        Args:
            base_path: Base directory containing adapter YAML schemas
        """
        self.base_path = Path(base_path)
    
    def discover_schemas(self) -> List[Path]:
        """Find all YAML schema files in directory tree."""
        return list(self.base_path.glob("*/**/*.yaml"))
    
    def generate_all(self, output_base: Optional[Path | str] = None) -> Dict[str, str]:
        """
        Generate adapter.py files for all discovered schemas.
        
        Args:
            output_base: Base path for generated files (default: same as input)
            
        Returns:
            Dictionary mapping schema paths to generated file paths
        """
        results = {}
        schemas = self.discover_schemas()
        
        for schema_path in schemas:
            generator = AdapterCodeGenerator(schema_path)
            
            # Determine output path
            if output_base:
                output_path = Path(output_base) / schema_path.parent.name / "adapter.py"
            else:
                output_path = schema_path.parent / "adapter.py"
            
            # Generate and write
            output_path.parent.mkdir(parents=True, exist_ok=True)
            generator.generate(output_path)
            results[str(schema_path)] = str(output_path)
        
        return results
