"""Agent specification schemas for phenotype-agents.

Provides Pydantic models for defining agents, teammates, and configurations.
"""

from __future__ import annotations
from pydantic import BaseModel, Field
from typing import Optional, Literal
from enum import Enum


class AgentCapability(str, Enum):
    """Agent capabilities."""
    READ = "read"
    WRITE = "write"
    EXECUTE = "execute"
    CODE_REVIEW = "code_review"
    TESTING = "testing"
    DOCUMENTATION = "documentation"
    PLANNING = "planning"
    ORCHESTRATION = "orchestration"


class ModelTier(str, Enum):
    """Model quality tiers."""
    FREE = "free"
    CHEAP = "cheap"
    STANDARD = "standard"
    PREMIUM = "premium"
    RESEARCH = "research"


class AgentConfig(BaseModel):
    """Agent configuration."""
    timeout_seconds: int = Field(default=300, ge=1, le=3600)
    max_retries: int = Field(default=3, ge=0, le=10)
    temperature: float = Field(default=0.7, ge=0.0, le=2.0)
    top_p: Optional[float] = Field(default=None, ge=0.0, le=1.0)
    max_tokens: Optional[int] = Field(default=None, ge=1)
    cache_enabled: bool = True


class AgentSpec(BaseModel):
    """Agent specification with metadata.
    
    Defines the capabilities, behavior, and configuration of an agent.
    Used to register agents in the agent registry.
    
    Attributes:
        name: Unique identifier for the agent
        description: Human-readable description
        model: Model identifier (e.g., "gpt-4", "claude-3-sonnet")
        tools: List of tools/capabilities the agent can use
        tier: Model quality tier
        config: Agent configuration options
        version: Agent version
        bounded_context: Optional bounded context for isolation
    """
    name: str = Field(..., min_length=1, max_length=100)
    description: str = Field(default="", max_length=500)
    model: str = Field(default="gpt-4")
    tools: list[AgentCapability] = Field(default_factory=list)
    tier: ModelTier = ModelTier.STANDARD
    config: AgentConfig = Field(default_factory=AgentConfig)
    version: str = Field(default="v1")
    bounded_context: Optional[str] = None
    system_prompt: Optional[str] = None
    max_concurrent: int = Field(default=5, ge=1, le=100)

    class Config:
        json_schema_extra = {
            "example": {
                "name": "code-reviewer",
                "description": "Reviews code for bugs and quality issues",
                "model": "gpt-4",
                "tools": ["read", "code_review"],
                "tier": "standard",
            }
        }


class TeammateSpec(BaseModel):
    """Specification for multi-agent teammate.
    
    Defines how an agent participates in a team workflow,
    including handoff protocols and communication patterns.
    """
    agent_name: str
    role: str
    priority: int = Field(default=1, ge=1, le=10)
    depends_on: list[str] = Field(default_factory=list)
    handoff_protocol: Literal["sequential", "parallel", "broadcast"] = "sequential"
    max_handoffs: int = Field(default=10, ge=1)
    timeout_seconds: int = Field(default=600, ge=1)
    retry_on_failure: bool = True
    fallback_agent: Optional[str] = None

    class Config:
        json_schema_extra = {
            "example": {
                "agent_name": "code-reviewer",
                "role": "review",
                "priority": 1,
                "depends_on": ["implementer"],
                "handoff_protocol": "sequential",
            }
        }


class WorkflowStep(BaseModel):
    """A single step in a workflow.
    
    Represents an executable unit of work that can be performed
    by an agent or team of agents.
    """
    step_id: str
    agent_name: str
    input_mapping: dict[str, str] = Field(default_factory=dict)
    output_key: str
    retry_policy: Optional[dict] = None
    timeout_seconds: int = Field(default=300, ge=1)
    required: bool = True
    condition: Optional[str] = None  # Jinja2 expression for conditional execution


class WorkflowSpec(BaseModel):
    """Workflow specification for multi-agent orchestration.
    
    Defines a complete workflow with steps, dependencies, and configuration.
    """
    workflow_id: str
    name: str
    description: str = ""
    version: str = "v1"
    steps: list[WorkflowStep]
    max_parallel: int = Field(default=3, ge=1)
    failure_strategy: Literal["stop", "continue", "retry"] = "stop"
    audit_enabled: bool = True

    class Config:
        json_schema_extra = {
            "example": {
                "workflow_id": "pr-review",
                "name": "PR Review Workflow",
                "steps": [
                    {
                        "step_id": "lint",
                        "agent_name": "linter",
                        "output_key": "lint_result",
                    },
                    {
                        "step_id": "review",
                        "agent_name": "reviewer",
                        "input_mapping": {"diff": "lint_result.diff"},
                        "output_key": "review_result",
                    },
                ],
            }
        }


class AgentMetrics(BaseModel):
    """Metrics for agent performance tracking."""
    agent_name: str
    total_runs: int = 0
    successful_runs: int = 0
    failed_runs: int = 0
    total_tokens: int = 0
    total_cost_usd: float = 0.0
    avg_latency_ms: float = 0.0

    @property
    def success_rate(self) -> float:
        """Calculate success rate."""
        if self.total_runs == 0:
            return 0.0
        return self.successful_runs / self.total_runs


class AgentRegistry:
    """Registry for managing agent specifications.
    
    Provides centralized storage and lookup for agent definitions.
    """
    
    def __init__(self):
        self._agents: dict[str, AgentSpec] = {}
        self._metrics: dict[str, AgentMetrics] = {}
    
    def register(self, spec: AgentSpec) -> None:
        """Register an agent specification."""
        self._agents[spec.name] = spec
        self._metrics[spec.name] = AgentMetrics(agent_name=spec.name)
    
    def get(self, name: str) -> AgentSpec | None:
        """Get agent specification by name."""
        return self._agents.get(name)
    
    def list_agents(self) -> list[AgentSpec]:
        """List all registered agents."""
        return list(self._agents.values())
    
    def update_metrics(self, name: str, metrics: AgentMetrics) -> None:
        """Update agent metrics."""
        if name in self._metrics:
            self._metrics[name] = metrics
    
    def get_metrics(self, name: str) -> AgentMetrics | None:
        """Get agent metrics."""
        return self._metrics.get(name)
