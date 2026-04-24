"""Base agent classes."""
from __future__ import annotations
import uuid, asyncio
from abc import ABC, abstractmethod
from dataclasses import dataclass
from datetime import datetime
from enum import Enum, auto
from typing import Any, Generic, TypeVar
from pydantic import BaseModel, Field

class AgentStatus(Enum): IDLE = auto(); RUNNING = auto(); PAUSED = auto(); FAILED = auto(); COMPLETED = auto()
class AgentResult(BaseModel): success: bool; output: Any = None; error: str | None = None; duration_ms: float = 0.0; metadata: dict = Field(default_factory=dict)
class AgentConfig(BaseModel): name: str; description: str = ""; timeout_seconds: float = 300.0
T = TypeVar("T", bound=AgentConfig)

class BaseAgent(ABC, Generic[T]):
    def __init__(self, config: T) -> None:
        self.config = config; self.id = str(uuid.uuid4()); self.status = AgentStatus.IDLE
    @property
    def name(self) -> str: return self.config.name
    async def run(self, **kwargs) -> AgentResult:
        self.status = AgentStatus.RUNNING; start = datetime.utcnow()
        try:
            result = await self._execute(**kwargs)
            self.status = AgentStatus.COMPLETED; result.duration_ms = (datetime.utcnow() - start).total_seconds() * 1000
            return result
        except Exception as e:
            self.status = AgentStatus.FAILED; return AgentResult(success=False, error=str(e))
    @abstractmethod
    async def _execute(self, **kwargs) -> AgentResult: pass
