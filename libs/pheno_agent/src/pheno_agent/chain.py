"""Chain agent for sequential workflows."""
from __future__ import annotations
from typing import Any
from pheno_agent.base import BaseAgent, AgentConfig, AgentResult

class ChainAgent(BaseAgent[AgentConfig]):
    def __init__(self, config: AgentConfig, agents: list[BaseAgent]) -> None:
        super().__init__(config); self._agents = agents
    async def _execute(self, **kwargs) -> AgentResult:
        context = kwargs.copy(); results = []
        for agent in self._agents:
            result = await agent.run(**context); results.append(result)
            if not result.success: return AgentResult(success=False, error=f"Agent {agent.name} failed", output=results)
            context["previous_output"] = result.output
        return AgentResult(success=True, output=results[-1].output if results else None)
