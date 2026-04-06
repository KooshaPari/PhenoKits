"""phenotype_agents - Agent SDK for Phenotype ecosystem.

This package provides:
- AgentSpec: Define agent capabilities and behavior
- Workflow: Topological execution engine for multi-agent workflows
- BoundedContext: Isolated execution contexts
- AgentPool: Concurrent agent execution
- AgentTeam: Multi-agent coordination

Usage:
    from phenotype_agents import AgentSpec, Workflow, AgentPool

    spec = AgentSpec(name="test-agent", model="gpt-4")
    workflow = Workflow()
    pool = AgentPool()
"""

__version__ = "0.1.0"
__all__ = [
    "AgentSpec",
    "TeammateSpec",
    "Workflow",
    "BoundedContext",
    "AgentPool",
    "AgentTeam",
    "AgentConfig",
    "WorkflowResult",
    "TaskResult",
]

from phenotype_agents.schema import AgentSpec, TeammateSpec, AgentConfig
from phenotype_agents.workflow import Workflow, WorkflowResult, TaskResult
from phenotype_agents.context import BoundedContext, ContextManager
from phenotype_agents.runtime import AgentPool, AgentTeam
from phenotype_agents.teammate import AgentTeam
