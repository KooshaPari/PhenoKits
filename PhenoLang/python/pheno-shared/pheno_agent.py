"""Pheno Agent - Unified agent framework."""
from dataclasses import dataclass
from enum import Enum, auto

class AgentStatus(Enum):
    IDLE = auto()
    RUNNING = auto()
    COMPLETED = auto()
    FAILED = auto()

@dataclass
class AgentConfig:
    name: str
    timeout: float = 300.0

class BaseAgent:
    def __init__(self, config: AgentConfig):
        self.config = config
        self.status = AgentStatus.IDLE
    
    async def run(self, **kwargs):
        self.status = AgentStatus.RUNNING
        try:
            result = await self._execute(**kwargs)
            self.status = AgentStatus.COMPLETED
            return result
        except Exception as e:
            self.status = AgentStatus.FAILED
            raise

class SimpleAgent(BaseAgent):
    def __init__(self, name, handler, config=None):
        super().__init__(config or AgentConfig(name=name))
        self._handler = handler
    
    async def _execute(self, **kwargs):
        return await self._handler(**kwargs)

class ChainAgent(BaseAgent):
    def __init__(self, config: AgentConfig, agents):
        super().__init__(config)
        self._agents = agents
    
    async def _execute(self, **kwargs):
        result = kwargs
        for agent in self._agents:
            result = await agent.run(**result)
        return result

class ParallelAgent(BaseAgent):
    def __init__(self, config: AgentConfig, agents, max_concurrent=5):
        super().__init__(config)
        self._agents = agents
        self._max = max_concurrent
    
    async def _execute(self, **kwargs):
        import asyncio
        semaphore = asyncio.Semaphore(self._max)
        
        async def run_with_limit(agent):
            async with semaphore:
                return await agent.run(**kwargs)
        
        results = await asyncio.gather(*[run_with_limit(a) for a in self._agents], return_exceptions=True)
        return [r for r in results if not isinstance(r, Exception)]

__all__ = ["AgentConfig", "AgentStatus", "BaseAgent", "SimpleAgent", "ChainAgent", "ParallelAgent"]
