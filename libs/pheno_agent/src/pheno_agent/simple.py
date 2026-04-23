"""Simple agent for one-shot operations."""
from __future__ import annotations
from typing import Any, Callable
from pheno_agent.base import BaseAgent, AgentConfig, AgentResult
import asyncio

class SimpleAgent(BaseAgent[AgentConfig]):
    def __init__(self, name: str, handler: Callable, config: AgentConfig | None = None) -> None:
        super().__init__(config or AgentConfig(name=name)); self._handler = handler
    async def _execute(self, **kwargs) -> AgentResult:
        try:
            result = await self._handler(**kwargs) if asyncio.iscoroutinefunction(self._handler) else self._handler(**kwargs)
            return AgentResult(success=True, output=result)
        except Exception as e: return AgentResult(success=False, error=str(e))
