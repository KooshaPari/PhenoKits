"""phenotype-router-shared - Model routing system."""
from __future__ import annotations

import asyncio
import hashlib
import time
from dataclasses import dataclass, field
from enum import Enum
from typing import Any, Callable, Protocol

__version__ = "0.1.0"


class ModelProvider(str, Enum):
    """Supported model providers."""
    OPENAI = "openai"
    ANTHROPIC = "anthropic"
    GOOGLE = "google"
    MISTRAL = "mistral"
    DEEPSEEK = "deepseek"
    OLLAMA = "ollama"
    OPENROUTER = "openrouter"


@dataclass
class ModelSpec:
    """Model specification."""
    id: str
    provider: ModelProvider
    context_window: int = 128000
    max_output: int = 16384
    input_cost: float = 0.0
    output_cost: float = 0.0
    latency_ms: float = 0.0


@dataclass
class RoutingRequest:
    """Request for model routing."""
    task_type: str
    complexity: str = "medium"  # low, medium, high
    max_cost: float | None = None
    max_latency_ms: float | None = None
    required_capabilities: list[str] = field(default_factory=list)
    context_size: int = 0


@dataclass
class RoutingDecision:
    """Decision from router."""
    model: ModelSpec
    estimated_cost: float
    estimated_latency_ms: float
    reasoning: str


class RouterStrategy(Protocol):
    """Protocol for routing strategies."""
    
    def select(self, request: RoutingRequest, models: list[ModelSpec]) -> ModelSpec | None:
        """Select best model for request."""
        ...


class CostAwareRouter:
    """Router that optimizes for cost."""

    def __init__(self, balance_budget: float | None = None):
        self.balance_budget = balance_budget
        self._cost_history: list[float] = []
        self._lock = asyncio.Lock()

    async def route(self, request: RoutingRequest) -> RoutingDecision:
        """Route request to optimal model."""
        models = self._get_available_models()
        
        # Filter by capabilities
        candidates = [
            m for m in models
            if self._meets_requirements(m, request)
        ]
        
        if not candidates:
            raise ValueError(f"No models meet requirements: {request.required_capabilities}")
        
        # Sort by cost
        candidates.sort(key=lambda m: m.input_cost)
        
        selected = candidates[0]
        estimated_cost = self._estimate_cost(selected, request)
        
        return RoutingDecision(
            model=selected,
            estimated_cost=estimated_cost,
            estimated_latency_ms=selected.latency_ms,
            reasoning=f"Selected {selected.id} for cost optimization",
        )

    def _meets_requirements(self, model: ModelSpec, request: RoutingRequest) -> bool:
        """Check if model meets requirements."""
        if request.context_size > model.context_window:
            return False
        if request.max_cost and model.input_cost > request.max_cost:
            return False
        if request.max_latency_ms and model.latency_ms > request.max_latency_ms:
            return False
        return True

    def _estimate_cost(self, model: ModelSpec, request: RoutingRequest) -> float:
        """Estimate cost for request."""
        input_tokens = request.context_size // 4
        output_tokens = min(request.context_size // 2, model.max_output)
        
        return (input_tokens / 1000) * model.input_cost + \
               (output_tokens / 1000) * model.output_cost

    def _get_available_models(self) -> list[ModelSpec]:
        """Get available models (from config/registry)."""
        return [
            ModelSpec(
                id="gpt-4o-mini",
                provider=ModelProvider.OPENAI,
                input_cost=0.00015,
                output_cost=0.0006,
                latency_ms=100,
            ),
            ModelSpec(
                id="claude-3-haiku",
                provider=ModelProvider.ANTHROPIC,
                input_cost=0.00025,
                output_cost=0.00125,
                latency_ms=150,
            ),
            ModelSpec(
                id="gemini-2.0-flash",
                provider=ModelProvider.GOOGLE,
                input_cost=0.0001,
                output_cost=0.0004,
                latency_ms=80,
            ),
        ]


class LatencyRouter(CostAwareRouter):
    """Router that optimizes for latency."""

    async def route(self, request: RoutingRequest) -> RoutingDecision:
        """Route request to fastest model."""
        models = self._get_available_models()
        
        candidates = [
            m for m in models
            if self._meets_requirements(m, request)
        ]
        
        if not candidates:
            raise ValueError(f"No models meet requirements")
        
        # Sort by latency
        candidates.sort(key=lambda m: m.latency_ms)
        
        selected = candidates[0]
        
        return RoutingDecision(
            model=selected,
            estimated_cost=self._estimate_cost(selected, request),
            estimated_latency_ms=selected.latency_ms,
            reasoning=f"Selected {selected.id} for latency optimization",
        )


class ParetoRouter:
    """Router using Pareto frontier for cost-latency tradeoff."""

    async def route(self, request: RoutingRequest) -> RoutingDecision:
        """Route using Pareto optimization."""
        models = self._get_available_models()
        
        candidates = [
            m for m in models
            if self._meets_requirements(m, request)
        ]
        
        if not candidates:
            raise ValueError("No models meet requirements")
        
        # Find Pareto frontier
        frontier = self._find_pareto_frontier(candidates)
        
        # Pick middle ground (balanced)
        idx = len(frontier) // 2
        selected = frontier[idx]
        
        return RoutingDecision(
            model=selected,
            estimated_cost=self._estimate_cost(selected, request),
            estimated_latency_ms=selected.latency_ms,
            reasoning=f"Selected {selected.id} from Pareto frontier",
        )

    def _find_pareto_frontier(self, models: list[ModelSpec]) -> list[ModelSpec]:
        """Find Pareto-optimal models (cost vs latency)."""
        pareto = []
        
        for candidate in models:
            is_dominated = False
            for other in models:
                if other is candidate:
                    continue
                if (other.input_cost <= candidate.input_cost and 
                    other.latency_ms <= candidate.latency_ms and
                    (other.input_cost < candidate.input_cost or 
                     other.latency_ms < candidate.latency_ms)):
                    is_dominated = True
                    break
            
            if not is_dominated:
                pareto.append(candidate)
        
        pareto.sort(key=lambda m: m.input_cost)
        return pareto


__all__ = [
    "ModelProvider",
    "ModelSpec",
    "RoutingRequest",
    "RoutingDecision",
    "RouterStrategy",
    "CostAwareRouter",
    "LatencyRouter",
    "ParetoRouter",
]
