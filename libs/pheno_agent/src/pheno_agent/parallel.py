"""Parallel agent for concurrent execution."""
from __future__ import annotations
import asyncio
from typing import Any
from pheno_agent.base import BaseAgent, AgentConfig, AgentResult

class ParallelAgent(BaseAgent[AgentConfig]):
    def __init__(self, config: AgentConfig, agents: list[BaseAgent], max_concurrent: int = 5) -> None:
        super().__init__(config)
        self._agents = agents
        self._max_concurrent = max_concurrent
    
    async def _execute(self, **kwargs) -> AgentResult:
        semaphore = asyncio.Semaphore(self._max_concurrent)
        
        async def run_with_limit(agent):
            async with semaphore:
                return await agent.run(**kwargs)
        
        results = await asyncio.gather(*[run_with_limit(a) for a in self._agents], return_exceptions=True)
        processed = [AgentResult(success=False, error=str(r)) if isinstance(r, Exception) else r for r in results]
        all_success = all(r.success for r in processed)
        if all_success:
            return AgentResult(success=True, output=[r.output for r in processed])
        return AgentResult(success=False, error=f"{sum(1 for r in processed if not r.success)} failed", output=processed)
